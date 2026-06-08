use anyhow::{anyhow, bail, Context, Result};
use proc_macro2::{LineColumn, Span};
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::{self, VisitMut};
use toml_edit::{value, DocumentMut, Item, Table};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    #[serde(default)]
    crate_dir: Option<String>,
    #[serde(default)]
    crate_name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    report: Report,
}

#[derive(Debug, Deserialize, Serialize)]
struct Report {
    targets: Vec<Target>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Target {
    #[serde(default)]
    caller: Option<FnInfo>,
    #[serde(default)]
    callee: Option<FnInfo>,
    callsite: Callsite,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FnInfo {
    #[serde(default)]
    path: Option<String>,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Callsite {
    line: usize,
    col: usize,
    #[serde(default)]
    id: Option<String>,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
struct Injection {
    id: String,
    line: usize,
    col: usize,
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let cargo_dir = PathBuf::from(args.next().ok_or_else(usage)?);
    let meta_json = PathBuf::from(args.next().ok_or_else(usage)?);
    let dest_dir = PathBuf::from(args.next().ok_or_else(usage)?);
    let generated_meta_json = args.next().map(PathBuf::from);
    if args.next().is_some() {
        return Err(usage());
    }

    run(
        &cargo_dir,
        &meta_json,
        &dest_dir,
        generated_meta_json.as_deref(),
    )
}

fn usage() -> anyhow::Error {
    anyhow!("usage: autoinj <cargo-dir> <meta-json> <dest-dir> [generated-meta-json]")
}

fn run(
    cargo_dir: &Path,
    meta_json: &Path,
    dest_dir: &Path,
    generated_meta_json: Option<&Path>,
) -> Result<()> {
    if !cargo_dir.join("Cargo.toml").is_file() {
        bail!(
            "cargo dir does not contain Cargo.toml: {}",
            cargo_dir.display()
        );
    }
    if dest_dir.exists() {
        bail!("destination already exists: {}", dest_dir.display());
    }

    copy_crate(cargo_dir, dest_dir)?;
    add_klee_ext_bind_dependency(dest_dir)?;

    let mut meta: Meta = serde_json::from_str(
        &fs::read_to_string(meta_json)
            .with_context(|| format!("failed to read {}", meta_json.display()))?,
    )
    .with_context(|| format!("failed to parse {}", meta_json.display()))?;

    normalize_callsite_ids(&mut meta);
    inject_from_meta(dest_dir, &meta)?;

    let out_meta = generated_meta_json
        .map(PathBuf::from)
        .unwrap_or_else(|| dest_dir.join("generated-meta.json"));
    fs::write(&out_meta, serde_json::to_string_pretty(&meta)? + "\n")
        .with_context(|| format!("failed to write {}", out_meta.display()))?;
    Ok(())
}

fn copy_crate(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).with_context(|| format!("failed to create {}", dst.display()))?;

    for entry in WalkDir::new(src).into_iter() {
        let entry = entry?;
        let path = entry.path();
        let rel = path.strip_prefix(src)?;
        if rel.as_os_str().is_empty() {
            continue;
        }
        if rel.components().any(|component| {
            let name = component.as_os_str();
            name == "target" || name == ".git"
        }) {
            continue;
        }

        let dest_path = dst.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    path.display(),
                    dest_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn add_klee_ext_bind_dependency(crate_dir: &Path) -> Result<()> {
    let manifest_path = crate_dir.join("Cargo.toml");
    let manifest_text = fs::read_to_string(&manifest_path)?;
    let mut doc = manifest_text.parse::<DocumentMut>()?;

    let dep_path = relative_path(crate_dir, &repo_root()?.join("tools/klee-ext-bind"))?;
    let deps = doc
        .entry("dependencies")
        .or_insert(Item::Table(Table::new()))
        .as_table_mut()
        .ok_or_else(|| anyhow!("[dependencies] is not a table"))?;

    let mut dep = Table::new();
    dep["path"] = value(dep_path);
    deps["klee-ext-bind"] = Item::Table(dep);

    fs::write(&manifest_path, doc.to_string())?;
    Ok(())
}

fn repo_root() -> Result<PathBuf> {
    let start = env::current_dir()?;
    for dir in start.ancestors() {
        if dir.join("tools/klee-ext-bind").is_dir() {
            return Ok(dir.to_path_buf());
        }
    }
    bail!("could not locate repo root containing tools/klee-ext-bind")
}

fn relative_path(from_dir: &Path, to: &Path) -> Result<String> {
    let from = from_dir
        .canonicalize()
        .unwrap_or_else(|_| from_dir.to_path_buf());
    let to = to.canonicalize().unwrap_or_else(|_| to.to_path_buf());
    let from_parts: Vec<_> = from.components().collect();
    let to_parts: Vec<_> = to.components().collect();
    let common = from_parts
        .iter()
        .zip(to_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut rel = PathBuf::new();
    for _ in common..from_parts.len() {
        rel.push("..");
    }
    for component in &to_parts[common..] {
        rel.push(component.as_os_str());
    }
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

fn normalize_callsite_ids(meta: &mut Meta) {
    for target in &mut meta.report.targets {
        let path = target
            .caller
            .as_ref()
            .and_then(|caller| caller.path.as_deref())
            .unwrap_or("unknown");
        target.callsite.id = Some(callsite_id(path, target.callsite.line, target.callsite.col));
    }
}

fn callsite_id(path: &str, line: usize, col: usize) -> String {
    let normalized = path.replace(['\\', '/', '.'], "-");
    format!("{normalized}-{line}-{col}")
}

fn inject_from_meta(crate_dir: &Path, meta: &Meta) -> Result<()> {
    let mut files = BTreeSet::new();
    for target in &meta.report.targets {
        if let Some(path) = target
            .caller
            .as_ref()
            .and_then(|caller| caller.path.as_ref())
        {
            files.insert(path.clone());
        }
    }

    for rel_file in files {
        let path = crate_dir.join(&rel_file);
        let injections = meta
            .report
            .targets
            .iter()
            .filter(|target| {
                target
                    .caller
                    .as_ref()
                    .and_then(|caller| caller.path.as_ref())
                    == Some(&rel_file)
            })
            .map(|target| Injection {
                id: target.callsite.id.clone().unwrap_or_else(|| {
                    callsite_id(&rel_file, target.callsite.line, target.callsite.col)
                }),
                line: target.callsite.line,
                col: target.callsite.col,
            })
            .collect::<Vec<_>>();
        inject_file(&path, &injections)
            .with_context(|| format!("failed to inject {}", path.display()))?;
    }
    Ok(())
}

fn inject_file(path: &Path, injections: &[Injection]) -> Result<()> {
    let source = fs::read_to_string(path)?;
    let mut ast = syn::parse_file(&source)?;
    for injection in injections {
        let mut injector = Injector {
            injection,
            inserted: false,
        };
        injector.visit_file_mut(&mut ast);
        if !injector.inserted {
            bail!(
                "no call expression found at {}:{} in {}",
                injection.line,
                injection.col,
                path.display()
            );
        }
    }
    fs::write(path, prettyplease::unparse(&ast))?;
    Ok(())
}

struct Injector<'a> {
    injection: &'a Injection,
    inserted: bool,
}

impl VisitMut for Injector<'_> {
    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        if self.inserted {
            return;
        }

        let mut index = 0;
        while index < block.stmts.len() {
            visit_mut::visit_stmt_mut(self, &mut block.stmts[index]);
            if self.inserted {
                return;
            }

            let args = stmt_matching_call_args(&block.stmts[index], self.injection);
            if let Some(args) = args {
                let mut stmts = bind_stmts(&args);
                stmts.push(callsite_stmt(&self.injection.id));
                block.stmts.splice(index..index, stmts);
                self.inserted = true;
                return;
            }
            index += 1;
        }
    }
}

fn bind_stmts(args: &[String]) -> Vec<syn::Stmt> {
    args.iter()
        .filter_map(|arg| {
            let ident = syn::parse_str::<syn::Ident>(arg).ok()?;
            syn::parse_str(&format!("klee_ext_bind::bind!(&{}, {:?});", ident, arg)).ok()
        })
        .collect()
}

fn callsite_stmt(id: &str) -> syn::Stmt {
    let literal = syn::LitStr::new(id, Span::call_site());
    parse_quote! { klee_ext_bind::callsite!(#literal); }
}

fn stmt_matching_call_args(stmt: &syn::Stmt, injection: &Injection) -> Option<Vec<String>> {
    let mut finder = CallFinder {
        injection,
        args: None,
    };
    finder.visit_stmt(stmt);
    finder.args
}

struct CallFinder<'a> {
    injection: &'a Injection,
    args: Option<Vec<String>>,
}

impl<'ast> Visit<'ast> for CallFinder<'_> {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if self.args.is_none() && span_matches(node.span(), self.injection) {
            self.args = Some(node.args.iter().map(expr_name).collect());
            return;
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if self.args.is_none() && span_matches(node.span(), self.injection) {
            self.args = Some(node.args.iter().map(expr_name).collect());
            return;
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

fn span_matches(span: Span, injection: &Injection) -> bool {
    let start = span.start();
    let end = span.end();
    contains(start, end, injection.line, injection.col)
        || contains(start, end, injection.line, injection.col.saturating_sub(1))
        || contains(start, end, injection.line, injection.col.saturating_add(1))
}

fn contains(start: LineColumn, end: LineColumn, line: usize, col: usize) -> bool {
    let start_col = start.column + 1;
    let end_col = end.column + 1;
    (line > start.line || (line == start.line && col >= start_col))
        && (line < end.line || (line == end.line && col <= end_col))
}

fn expr_name(expr: &syn::Expr) -> String {
    if let syn::Expr::Path(path) = expr {
        if path.qself.is_none() && path.path.segments.len() == 1 {
            return path.path.segments[0].ident.to_string();
        }
    }
    expr.to_token_stream().to_string().replace(' ', "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TMP_ID: AtomicUsize = AtomicUsize::new(0);

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(name: &str) -> Result<Self> {
            let id = NEXT_TMP_ID.fetch_add(1, Ordering::Relaxed);
            let path =
                env::temp_dir().join(format!("autoinj-test-{name}-{}-{id}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path)?;
            }
            fs::create_dir_all(&path)?;
            Ok(Self { path })
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write(path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    fn fixture_crate(root: &Path) -> Result<()> {
        write(
            &root.join("Cargo.toml"),
            r#"[package]
name = "fixture"
version = "0.1.0"
edition = "2021"
"#,
        )?;
        write(
            &root.join("src/lib.rs"),
            r#"pub fn dealloc(p: *mut u8, layout: usize) {
    unsafe {
        callee(p, layout)
    }
}

unsafe fn callee(_p: *mut u8, _layout: usize) {}
"#,
        )?;
        write(&root.join("target/ignored.txt"), "do not copy")?;
        Ok(())
    }

    fn fixture_meta(path: &Path) -> Result<()> {
        write(
            path,
            r#"{
  "crate_dir": "fixture",
  "crate_name": "fixture",
  "description": null,
  "report": {
    "targets": [
      {
        "caller": { "name": "dealloc", "path": "src/lib.rs" },
        "callee": { "name": "callee", "path": "src/lib.rs", "line_start": 7 },
        "callsite": { "line": 3, "col": 9, "id": "old-id" }
      }
    ]
  }
}
"#,
        )
    }

    #[test]
    fn span_matches_accepts_adjacent_columns() {
        let expr: syn::Expr = syn::parse_quote! { ptr.add(index) };
        let span = expr.span();
        let start = span.start();
        let start_col = start.column + 1;
        assert!(span_matches(
            span,
            &Injection {
                id: "src-vec-rs-206-20".to_string(),
                line: start.line,
                col: start_col.saturating_sub(1),
            }
        ));
        assert!(span_matches(
            span,
            &Injection {
                id: "src-vec-rs-206-20".to_string(),
                line: start.line,
                col: start_col + 1,
            }
        ));
    }

    #[test]
    fn callsite_id_uses_relative_path_line_and_col() {
        assert_eq!(
            callsite_id(r"src\nested/foo.rs", 43, 13),
            "src-nested-foo-rs-43-13"
        );
    }

    #[test]
    fn normalize_callsite_ids_replaces_existing_ids() -> Result<()> {
        let mut meta: Meta = serde_json::from_str(
            r#"{
  "report": {
    "targets": [
      {
        "caller": { "path": "src/lib.rs" },
        "callsite": { "line": 10, "col": 2, "id": "stale" }
      }
    ]
  }
}
"#,
        )?;

        normalize_callsite_ids(&mut meta);

        assert_eq!(
            meta.report.targets[0].callsite.id.as_deref(),
            Some("src-lib-rs-10-2")
        );
        Ok(())
    }

    #[test]
    fn inject_file_inserts_binds_and_callsite_inside_unsafe_block() -> Result<()> {
        let tmp = TempDir::new("inject-file")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub fn dealloc(p: *mut u8, layout: usize) {
    unsafe {
        callee(p, layout)
    }
}

unsafe fn callee(_p: *mut u8, _layout: usize) {}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-3-9".to_string(),
                line: 3,
                col: 9,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(compact.contains("unsafe{klee_ext_bind::bind!(&p,\"p\");"));
        assert!(compact.contains("klee_ext_bind::bind!(&layout,\"layout\");"));
        assert!(compact.contains("klee_ext_bind::callsite!(\"src-lib-rs-3-9\");"));
        assert!(compact.contains("callee(p,layout)"));
        Ok(())
    }

    #[test]
    fn run_copies_crate_adds_dependency_injects_and_writes_meta() -> Result<()> {
        let tmp = TempDir::new("run")?;
        let source_crate = tmp.path().join("source");
        let dest_crate = tmp.path().join("dest");
        let meta_path = tmp.path().join("meta.json");
        let out_meta = tmp.path().join("generated-meta.json");
        fixture_crate(&source_crate)?;
        fixture_meta(&meta_path)?;

        run(&source_crate, &meta_path, &dest_crate, Some(&out_meta))?;

        let manifest = fs::read_to_string(dest_crate.join("Cargo.toml"))?;
        assert!(manifest.contains("[dependencies.klee-ext-bind]"));
        assert!(manifest.contains("path = "));
        assert!(!dest_crate.join("target/ignored.txt").exists());

        let source = fs::read_to_string(dest_crate.join("src/lib.rs"))?;
        let compact = source.replace(char::is_whitespace, "");
        assert!(compact.contains("klee_ext_bind::bind!(&p,\"p\");"));
        assert!(compact.contains("klee_ext_bind::bind!(&layout,\"layout\");"));
        assert!(compact.contains("klee_ext_bind::callsite!(\"src-lib-rs-3-9\");"));

        let generated: Meta = serde_json::from_str(&fs::read_to_string(out_meta)?)?;
        assert_eq!(
            generated.report.targets[0].callsite.id.as_deref(),
            Some("src-lib-rs-3-9")
        );
        Ok(())
    }
}
