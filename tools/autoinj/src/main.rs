use anyhow::{anyhow, bail, Context, Result};
use proc_macro2::{LineColumn, Span};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
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
    name: Option<String>,
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
    path: Option<String>,
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
    callee_name: Option<String>,
    raw_pointer_deref: bool,
    source_line: Option<String>,
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let cargo_dir = PathBuf::from(args.next().ok_or_else(usage)?);
    let meta_json = PathBuf::from(args.next().ok_or_else(usage)?);
    let dest_dir = PathBuf::from(args.next().ok_or_else(usage)?);
    if args.next().is_some() {
        return Err(usage());
    }

    run(&cargo_dir, &meta_json, &dest_dir)
}

fn usage() -> anyhow::Error {
    anyhow!("usage: autoinj <cargo-dir> <meta-json> <dest-dir>")
}

fn run(cargo_dir: &Path, meta_json: &Path, dest_dir: &Path) -> Result<()> {
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
    rebase_relative_dependency_paths(cargo_dir, dest_dir)?;
    add_klee_ext_bind_dependency(dest_dir)?;

    let mut meta: Meta = serde_json::from_str(
        &fs::read_to_string(meta_json)
            .with_context(|| format!("failed to read {}", meta_json.display()))?,
    )
    .with_context(|| format!("failed to parse {}", meta_json.display()))?;

    normalize_callsite_ids(&mut meta);
    inject_from_meta(dest_dir, &meta)?;
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

fn rebase_relative_dependency_paths(source_dir: &Path, dest_dir: &Path) -> Result<()> {
    let manifest_path = dest_dir.join("Cargo.toml");
    let manifest_text = fs::read_to_string(&manifest_path)?;
    let mut doc = manifest_text.parse::<DocumentMut>()?;
    rebase_paths_in_table(doc.as_table_mut(), false, source_dir, dest_dir)?;
    fs::write(&manifest_path, doc.to_string())?;
    Ok(())
}

fn rebase_paths_in_table(
    table: &mut Table,
    dependency_context: bool,
    source_dir: &Path,
    dest_dir: &Path,
) -> Result<()> {
    for (key, item) in table.iter_mut() {
        let key = key.get();
        let nested_dependency_context = dependency_context
            || matches!(
                key,
                "dependencies" | "dev-dependencies" | "build-dependencies" | "patch" | "replace"
            );
        if dependency_context && key == "path" {
            if let Some(path) = item.as_str() {
                let path = Path::new(path);
                if path.is_relative() {
                    let target = source_dir.join(path);
                    *item = value(relative_path(dest_dir, &target)?);
                }
            }
            continue;
        }
        match item {
            Item::Table(child) => {
                rebase_paths_in_table(child, nested_dependency_context, source_dir, dest_dir)?
            }
            Item::ArrayOfTables(children) => {
                for child in children.iter_mut() {
                    rebase_paths_in_table(child, nested_dependency_context, source_dir, dest_dir)?;
                }
            }
            _ => {}
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
        let path = target_path(target).unwrap_or("unknown");
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
        if let Some(path) = target_path(target) {
            if crate_dir.join(path).is_file() {
                files.insert(path.to_string());
            }
        }
    }

    for rel_file in files {
        let path = crate_dir.join(&rel_file);
        let mut seen_callsites = BTreeSet::new();
        let injections = meta
            .report
            .targets
            .iter()
            .filter(|target| target_path(target) == Some(rel_file.as_str()))
            .filter_map(|target| {
                let id = target.callsite.id.clone().unwrap_or_else(|| {
                    callsite_id(&rel_file, target.callsite.line, target.callsite.col)
                });
                let callee_name = target
                    .callee
                    .as_ref()
                    .and_then(|callee| callee.name.clone());
                let raw_pointer_deref = callee_name
                    .as_deref()
                    .is_some_and(|name| name == "core::ptr::__raw_ptr_deref__");
                // A MIR location can be reported once for an inlined unsafe
                // operation and again for a raw-pointer dereference. Both map
                // to the same source marker id, which must only be injected
                // once.
                seen_callsites.insert(id.clone()).then(|| Injection {
                    id,
                    line: target.callsite.line,
                    col: target.callsite.col,
                    callee_name,
                    raw_pointer_deref,
                    source_line: None,
                })
            })
            .collect::<Vec<_>>();
        inject_file(&path, &injections)
            .with_context(|| format!("failed to inject {}", path.display()))?;
    }
    Ok(())
}

fn target_path(target: &Target) -> Option<&str> {
    target.callsite.path.as_deref().or_else(|| {
        target
            .caller
            .as_ref()
            .and_then(|caller| caller.path.as_deref())
    })
}

fn inject_file(path: &Path, injections: &[Injection]) -> Result<()> {
    let source = fs::read_to_string(path)?;
    let mut ast = syn::parse_file(&source)?;
    let mut raw_pointer_collector = RawPointerPlaceCollector::default();
    raw_pointer_collector.visit_file(&ast);
    let raw_pointer_places = raw_pointer_collector.places;
    let mut next_temp_index = 0usize;
    let mut macro_collector = MacroRangeCollector::default();
    macro_collector.visit_file(&ast);
    let macro_ranges = macro_collector.ranges;
    let const_ranges = ast
        .items
        .iter()
        .flat_map(|item| match item {
            syn::Item::Fn(item_fn) if item_fn.sig.constness.is_some() => {
                let span = item_fn.span();
                vec![(span.start().line, span.end().line)]
            }
            syn::Item::Impl(item_impl) => item_impl
                .items
                .iter()
                .filter_map(|item| match item {
                    syn::ImplItem::Fn(method) if method.sig.constness.is_some() => {
                        let span = method.span();
                        Some((span.start().line, span.end().line))
                    }
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        })
        .collect::<Vec<_>>();
    let ordered_injections = order_injections_for_source(&source, injections)
        .into_iter()
        .filter(|injection| {
            let in_macro = macro_ranges
                .iter()
                .any(|(start, end)| injection.line >= *start && injection.line <= *end);
            if in_macro {
                eprintln!(
                    "warning: skipping callsite {} at {}:{} because it is inside a macro",
                    injection.id,
                    path.display(),
                    injection.line,
                );
            }
            let in_const = const_ranges
                .iter()
                .any(|(start, end)| injection.line >= *start && injection.line <= *end);
            if in_const {
                eprintln!(
                    "warning: skipping callsite {} at {}:{} because it is inside a const function",
                    injection.id,
                    path.display(),
                    injection.line,
                );
            }
            !in_macro && !in_const
        })
        .collect::<Vec<_>>();
    for injection in &ordered_injections {
        let injection = injection.with_source_line(&source);
        let mut injector = Injector {
            injection: &injection,
            inserted: false,
            next_temp_index,
            raw_pointer_places: &raw_pointer_places,
        };
        injector.visit_file_mut(&mut ast);
        if !injector.inserted {
            if injection.raw_pointer_deref {
                eprintln!(
                    "warning: skipping raw pointer deref callsite {} at {}:{} because no source-level dereference expression was found",
                    injection.id,
                    injection.line,
                    injection.col,
                );
                continue;
            }
            bail!(
                "no call expression found at {}:{} in {}",
                injection.line,
                injection.col,
                path.display()
            );
        }
        next_temp_index = injector.next_temp_index;
    }
    let mut cleaner = DiscardedRetStmtCleaner;
    cleaner.visit_file_mut(&mut ast);
    fs::write(path, prettyplease::unparse(&ast))?;
    Ok(())
}

#[derive(Default)]
struct MacroRangeCollector {
    ranges: Vec<(usize, usize)>,
}

impl<'ast> Visit<'ast> for MacroRangeCollector {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let span = node.span();
        self.ranges.push((span.start().line, span.end().line));
        visit::visit_macro(self, node);
    }
}

impl Injection {
    fn with_source_line(&self, source: &str) -> Self {
        let mut injection = self.clone();
        injection.source_line = source
            .lines()
            .skip(self.line.saturating_sub(1))
            .take(32)
            .map(ToString::to_string)
            .reduce(|window, line| window + "\n" + &line);
        injection
    }
}

fn order_injections_for_source(source: &str, injections: &[Injection]) -> Vec<Injection> {
    let lines = source.lines().collect::<Vec<_>>();
    let mut ordered = injections
        .iter()
        .cloned()
        .enumerate()
        .collect::<Vec<(usize, Injection)>>();

    ordered.sort_by(|(left_index, left), (right_index, right)| {
        if left.line == right.line && left.col == right.col {
            let left_pos = callee_position_in_chain(&lines, left);
            let right_pos = callee_position_in_chain(&lines, right);
            right_pos
                .cmp(&left_pos)
                .then_with(|| left_index.cmp(right_index))
        } else {
            left_index.cmp(right_index)
        }
    });

    ordered
        .into_iter()
        .map(|(_, injection)| injection)
        .collect()
}

fn callee_position_in_chain(lines: &[&str], injection: &Injection) -> usize {
    let Some(callee_name) = injection.callee_name.as_deref() else {
        return 0;
    };
    let Some(callee_leaf) = callee_leaf(callee_name) else {
        return 0;
    };
    let start_line = injection.line.saturating_sub(1);
    let Some(line) = lines.get(start_line) else {
        return 0;
    };
    let search_start = injection.col.saturating_sub(1).min(line.len());
    // rustc assigns every call in a multi-line method chain the location of
    // the chain's first token. Search a small statement-sized window so outer
    // calls (which must be rewritten first) sort after inner calls even when
    // their method names occur on later lines.
    let window = lines[start_line..lines.len().min(start_line + 32)].join("\n");
    window[search_start..]
        .find(callee_leaf)
        .map(|offset| search_start + offset)
        .unwrap_or(0)
}

struct DiscardedRetStmtCleaner;

impl VisitMut for DiscardedRetStmtCleaner {
    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        visit_mut::visit_block_mut(self, block);

        block.stmts.retain(|stmt| !is_discarded_klee_ret_stmt(stmt));
    }
}

struct Injector<'a> {
    injection: &'a Injection,
    inserted: bool,
    next_temp_index: usize,
    raw_pointer_places: &'a BTreeSet<String>,
}

impl VisitMut for Injector<'_> {
    fn visit_item_fn_mut(&mut self, function: &mut syn::ItemFn) {
        let inserted_before = self.inserted;
        visit_mut::visit_item_fn_mut(self, function);
        if !inserted_before && self.inserted {
            preserve_target_caller_symbol(&mut function.attrs);
        }
    }

    fn visit_impl_item_fn_mut(&mut self, function: &mut syn::ImplItemFn) {
        let inserted_before = self.inserted;
        visit_mut::visit_impl_item_fn_mut(self, function);
        if !inserted_before && self.inserted {
            preserve_target_caller_symbol(&mut function.attrs);
        }
    }

    fn visit_trait_item_fn_mut(&mut self, function: &mut syn::TraitItemFn) {
        let inserted_before = self.inserted;
        visit_mut::visit_trait_item_fn_mut(self, function);
        if !inserted_before && self.inserted {
            preserve_target_caller_symbol(&mut function.attrs);
        }
    }

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

            if self.injection.raw_pointer_deref
                && stmt_matches_raw_deref_location(&block.stmts[index], self.injection)
            {
                let mut rewriter = RawPointerDerefRewriter {
                    injection: self.injection,
                    inserted: false,
                    raw_pointer_places: self.raw_pointer_places,
                };
                rewriter.visit_stmt_mut(&mut block.stmts[index]);
                if rewriter.inserted {
                    self.inserted = true;
                    return;
                }
            }

            let rewrite = rewrite_stmt_matching_call_args(
                &mut block.stmts[index],
                self.injection,
                self.next_temp_index,
            );
            if let Some(rewrite) = rewrite {
                let mut stmts = rewrite.lift_stmts;
                stmts.push(callsite_stmt(&self.injection.id));
                stmts.extend(rewrite.ret_stmts);
                // Rewriting turns target call expressions into `__klee_ret`.
                // If the original statement was just a discarded expression (`...;`),
                // keeping that rewritten `__klee_ret;` is redundant, so replace it.
                if is_discarded_klee_ret_stmt(&block.stmts[index]) {
                    block.stmts.splice(index..index + 1, stmts);
                } else {
                    block.stmts.splice(index..index, stmts);
                }
                self.next_temp_index = rewrite.next_temp_index;
                self.inserted = true;
                return;
            }
            index += 1;
        }
    }
}

fn preserve_target_caller_symbol(attrs: &mut Vec<syn::Attribute>) {
    attrs.retain(|attr| !attr.path().is_ident("inline"));
    attrs.push(parse_quote! { #[inline(never)] });
}

struct CallRewrite {
    lift_stmts: Vec<syn::Stmt>,
    ret_stmts: Vec<syn::Stmt>,
    next_temp_index: usize,
}

fn is_discarded_klee_ret_stmt(stmt: &syn::Stmt) -> bool {
    match stmt {
        syn::Stmt::Expr(expr, Some(_)) => {
            matches!(
                expr,
                syn::Expr::Path(path)
                    if path.path.segments.last().is_some_and(|segment| {
                        segment.ident.to_string().starts_with("__klee_ret_")
                    })
            )
        }
        _ => false,
    }
}

fn callsite_stmt(id: &str) -> syn::Stmt {
    let literal = syn::LitStr::new(id, Span::call_site());
    parse_quote! { klee_ext_bind::callsite!(#literal); }
}

fn stmt_matches_raw_deref_location(stmt: &syn::Stmt, injection: &Injection) -> bool {
    span_matches(stmt.span(), injection)
}

struct RawPointerDerefRewriter<'a> {
    injection: &'a Injection,
    inserted: bool,
    raw_pointer_places: &'a BTreeSet<String>,
}

impl VisitMut for RawPointerDerefRewriter<'_> {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if self.inserted {
            return;
        }

        if let syn::Expr::Unary(unary) = expr {
            if matches!(unary.op, syn::UnOp::Deref(_))
                && span_matches(unary.span(), self.injection)
                && is_known_raw_pointer_operand(&unary.expr, self.raw_pointer_places)
            {
                let pointer = (*unary.expr).clone();
                let literal = syn::LitStr::new(&self.injection.id, Span::call_site());
                unary.expr = Box::new(parse_quote! {
                    klee_ext_bind::raw_pointer_deref!(#literal, #pointer)
                });
                self.inserted = true;
                return;
            }
        }

        visit_mut::visit_expr_mut(self, expr);
    }
}

#[derive(Default)]
struct RawPointerPlaceCollector {
    places: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for RawPointerPlaceCollector {
    fn visit_field(&mut self, field: &'ast syn::Field) {
        if matches!(field.ty, syn::Type::Ptr(_)) {
            if let Some(ident) = &field.ident {
                self.places.insert(ident.to_string());
            }
        }
        visit::visit_field(self, field);
    }

    fn visit_fn_arg(&mut self, arg: &'ast syn::FnArg) {
        if let syn::FnArg::Typed(arg) = arg {
            if matches!(*arg.ty, syn::Type::Ptr(_)) {
                if let syn::Pat::Ident(ident) = &*arg.pat {
                    self.places.insert(ident.ident.to_string());
                }
            }
        }
        visit::visit_fn_arg(self, arg);
    }

    fn visit_pat_type(&mut self, pat: &'ast syn::PatType) {
        if matches!(*pat.ty, syn::Type::Ptr(_)) {
            if let syn::Pat::Ident(ident) = &*pat.pat {
                self.places.insert(ident.ident.to_string());
            }
        }
        visit::visit_pat_type(self, pat);
    }
}

fn is_known_raw_pointer_operand(expr: &syn::Expr, places: &BTreeSet<String>) -> bool {
    match expr {
        syn::Expr::Cast(cast) => matches!(&*cast.ty, syn::Type::Ptr(_)),
        syn::Expr::MethodCall(call) => call.method == "cast" && call.args.is_empty(),
        syn::Expr::Path(path) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| places.contains(&segment.ident.to_string())),
        syn::Expr::Field(field) => match &field.member {
            syn::Member::Named(ident) => places.contains(&ident.to_string()),
            syn::Member::Unnamed(_) => false,
        },
        syn::Expr::Group(group) => is_known_raw_pointer_operand(&group.expr, places),
        syn::Expr::Paren(paren) => is_known_raw_pointer_operand(&paren.expr, places),
        _ => false,
    }
}

fn rewrite_stmt_matching_call_args(
    stmt: &mut syn::Stmt,
    injection: &Injection,
    next_temp_index: usize,
) -> Option<CallRewrite> {
    let mut rewriter = CallRewriter {
        injection,
        rewrite: None,
        next_temp_index,
    };
    rewriter.visit_stmt_mut(stmt);
    rewriter.rewrite
}

struct CallRewriter<'a> {
    injection: &'a Injection,
    rewrite: Option<CallRewrite>,
    next_temp_index: usize,
}

impl VisitMut for CallRewriter<'_> {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        visit_mut::visit_expr_mut(self, expr);
        if self.rewrite.is_some() {
            return;
        }
        match expr {
            syn::Expr::Call(node)
                if (call_expr_matches_location(node.span(), self.injection)
                    || call_expr_matches_enclosing_source(node, self.injection))
                    && (expr_call_matches_callee(node, self.injection.callee_name.as_deref())
                        || (span_starts_at_location(node.span(), self.injection)
                            && !source_statement_contains_callee(self.injection))) =>
            {
                let mut next_temp_index = self.next_temp_index;
                let (_, lift_stmts) = lift_call_args_only(&mut node.args, &mut next_temp_index);
                let call_expr = syn::Expr::Call(node.clone());
                let ret_ident =
                    callsite_ret_ident(&self.injection.id, self.injection.callee_name.as_deref());
                self.rewrite = Some(CallRewrite {
                    lift_stmts,
                    ret_stmts: make_return_stmts(call_expr, &ret_ident),
                    next_temp_index,
                });
                *expr = parse_quote! { #ret_ident };
            }
            syn::Expr::MethodCall(node)
                if method_call_expr_matches_location(node, self.injection)
                    && (method_call_matches_callee(
                        node,
                        self.injection.callee_name.as_deref(),
                    ) || (span_starts_at_location(node.span(), self.injection)
                        && !source_statement_contains_callee(self.injection))) =>
            {
                let mut next_temp_index = self.next_temp_index;
                let callee_method = self
                    .injection
                    .callee_name
                    .as_deref()
                    .and_then(callee_leaf)
                    .unwrap_or("");
                let source_method = node.method.to_string();
                let effective_method = if callee_method == source_method {
                    callee_method
                } else {
                    source_method.as_str()
                };
                let mutable_receiver = effective_method.ends_with("_mut")
                    || effective_method.ends_with("_unsafe")
                    || matches!(effective_method, "as_mut_ptr" | "copy_within");
                let bind_receiver_in_place = effective_method == "set_len";
                let ret_ident =
                    callsite_ret_ident(&self.injection.id, self.injection.callee_name.as_deref());
                self.rewrite = Some(lift_method_call_parts(
                    node,
                    &mut next_temp_index,
                    mutable_receiver,
                    bind_receiver_in_place,
                    effective_method,
                    &ret_ident,
                ));
                *expr = parse_quote! { #ret_ident };
            }
            _ => {}
        }
    }
}

fn call_expr_matches_location(span: Span, injection: &Injection) -> bool {
    span_matches(span, injection)
}

fn method_call_expr_matches_location(node: &syn::ExprMethodCall, injection: &Injection) -> bool {
    if span_matches(node.span(), injection) {
        return true;
    }

    // rustc often reports the source location for an unsafe call in a
    // multi-line method chain at the beginning of the chain, while syn's span
    // for the specific method call can start at the later `.method(...)`.
    // Treat that chain start as matching the method call if the target method
    // appears in the same chain expression.
    if !source_line_starts_call_expr(injection) {
        return false;
    }
    let span = node.span();
    if injection.line < span.start().line || injection.line > span.end().line {
        return false;
    }
    let Some(callee_name) = injection.callee_name.as_deref() else {
        return false;
    };
    method_call_matches_callee(node, Some(callee_name))
}

fn source_line_starts_call_expr(injection: &Injection) -> bool {
    let Some(line) = injection.source_line.as_deref() else {
        return false;
    };
    let Some(from_col) = injection.col.checked_sub(1) else {
        return false;
    };
    let Some(tail) = line.get(from_col..) else {
        return false;
    };
    let trimmed = tail.trim_start();
    trimmed.starts_with("self.")
        || trimmed.starts_with("Self::")
        || trimmed
            .chars()
            .next()
            .map(|ch| ch == '_' || ch.is_ascii_alphabetic())
            .unwrap_or(false)
}

fn call_expr_matches_enclosing_source(node: &syn::ExprCall, injection: &Injection) -> bool {
    let start = node.span().start();
    source_line_starts_call_expr(injection)
        && start.line >= injection.line
        && start.line < injection.line.saturating_add(32)
}

fn source_statement_contains_callee(injection: &Injection) -> bool {
    let Some(callee) = injection.callee_name.as_deref().and_then(callee_leaf) else {
        return false;
    };
    let Some(source) = injection.source_line.as_deref() else {
        return false;
    };
    let end = source.find([';', '}']).unwrap_or(source.len());
    source[..end].contains(callee)
}

fn expr_call_matches_callee(node: &syn::ExprCall, callee_name: Option<&str>) -> bool {
    let Some(callee_name) = callee_name else {
        return true;
    };
    let syn::Expr::Path(path) = &*node.func else {
        return true;
    };
    let Some(segment) = path.path.segments.last() else {
        return true;
    };
    callee_leaf_matches(callee_name, &segment.ident.to_string())
}

fn method_call_matches_callee(node: &syn::ExprMethodCall, callee_name: Option<&str>) -> bool {
    callee_name
        .map(|callee_name| callee_leaf_matches(callee_name, &node.method.to_string()))
        .unwrap_or(true)
}

fn callee_leaf_matches(callee_name: &str, expr_leaf: &str) -> bool {
    callee_leaf(callee_name)
        .map(|callee_leaf| callee_leaf == expr_leaf)
        .unwrap_or(true)
}

fn callee_leaf(callee_name: &str) -> Option<&str> {
    callee_name.rsplit("::").next()
}

fn callsite_ret_ident(callsite_id: &str, callee_name: Option<&str>) -> syn::Ident {
    let raw = format!(
        "{callsite_id}_{}",
        callee_name.and_then(callee_leaf).unwrap_or("call")
    );
    let suffix = raw
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>();
    syn::Ident::new(&format!("__klee_ret_{suffix}"), Span::call_site())
}

fn make_return_stmts(call_expr: syn::Expr, ret_ident: &syn::Ident) -> Vec<syn::Stmt> {
    let ret_stmt: syn::Stmt = parse_quote! {
        // The original call result may immediately be mutably borrowed by the
        // surrounding expression (for example `&mut mem::zeroed()`). Keep the
        // lifted temporary mutable so injection preserves that valid source.
        let mut #ret_ident = #call_expr;
    };
    vec![ret_stmt]
}

fn lift_call_args_only(
    args: &mut syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>,
    next_temp_index: &mut usize,
) -> (Vec<String>, Vec<syn::Stmt>) {
    let mut arg_names = Vec::new();
    let mut lift_stmts = Vec::new();

    for arg in args.iter_mut() {
        if let Some(name) = simple_ident_name(arg) {
            arg_names.push(name);
            continue;
        }
        if matches!(arg, syn::Expr::Path(_)) {
            // Qualified constants can be const-generic arguments after MIR
            // lowering (for example `_mm_prefetch(ptr, _MM_HINT_T0)`). Turning
            // them into `let` temporaries makes the reconstructed source fail
            // const checking, while leaving a path expression in place emits
            // no side-effecting IR between the marker and target call.
            arg_names.push(String::new());
            continue;
        }

        let index = *next_temp_index;
        *next_temp_index += 1;
        let name = format!("__klee_arg{index}");
        let ident = syn::Ident::new(&name, Span::call_site());
        let original = arg.clone();
        let lift_stmt: syn::Stmt = parse_quote! {
            let #ident = #original;
        };
        *arg = parse_quote! { #ident };
        lift_stmts.push(lift_stmt);
        arg_names.push(name);
    }

    (arg_names, lift_stmts)
}

fn lift_method_call_parts(
    node: &mut syn::ExprMethodCall,
    next_temp_index: &mut usize,
    mutable_receiver: bool,
    bind_receiver_in_place: bool,
    callee_method: &str,
    ret_ident: &syn::Ident,
) -> CallRewrite {
    let mut lift_stmts = Vec::new();

    if bind_receiver_in_place {
        // set_len's receiver must remain an in-place expression. The executor
        // captures the receiver directly from the target CallInst, so no
        // temporary or explicit binding is needed.
    } else if simple_ident_name(&node.receiver).is_some() {
    } else {
        let name = format!("__klee_arg{}", *next_temp_index);
        *next_temp_index += 1;
        let ident = syn::Ident::new(&name, Span::call_site());
        let original = (*node.receiver).clone();
        // A field receiver is a place expression. Moving it into a temporary can
        // make a valid auto-borrowed method call invalid (for example moving a
        // Box out of &self before calling slice::get_unchecked). Borrow the place
        // so method-call autoderef/autoref keeps the original ownership.
        let receiver_is_place = is_place_expr(&original);
        if receiver_is_place && mutable_receiver {
            // Keep mutable place receivers in the call expression. Rust's
            // two-phase borrow permits arguments such as
            // `self.buf.copy_within(self.range(), 0)` to inspect `self`
            // before activating the receiver borrow; lifting `&mut self.buf`
            // ahead of the arguments rejects otherwise valid source.
        } else {
            let lift_stmt: syn::Stmt = if receiver_is_place {
                parse_quote! { let #ident = &#original; }
            } else {
                parse_quote! { let #ident = #original; }
            };
            node.receiver = Box::new(parse_quote! { #ident });
            lift_stmts.push(lift_stmt);
        }
    }

    let bind_first_tail_arg_as_u64 = node
        .args
        .first()
        .is_some_and(is_scalar_index_bind_candidate);
    let (tail_arg_names, mut tail_stmts) = lift_call_args_only(&mut node.args, next_temp_index);
    lift_stmts.append(&mut tail_stmts);
    if matches!(callee_method, "get_unchecked" | "get_unchecked_mut") && bind_first_tail_arg_as_u64
    {
        let receiver = (*node.receiver).clone();
        let bind_len_stmt: syn::Stmt = parse_quote! {
            klee_ext_bind::bind_arg_u64(1, (#receiver).len() as u64);
        };
        lift_stmts.push(bind_len_stmt);
        if let Some(index_arg) = tail_arg_names.first() {
            let index_ident = syn::Ident::new(index_arg, Span::call_site());
            let bind_stmt: syn::Stmt = parse_quote! {
                klee_ext_bind::bind_arg_u64(2, #index_ident as u64);
            };
            lift_stmts.push(bind_stmt);
        }
    }

    let call_expr = syn::Expr::MethodCall(node.clone());

    CallRewrite {
        lift_stmts,
        ret_stmts: make_return_stmts(call_expr, ret_ident),
        next_temp_index: *next_temp_index,
    }
}

fn is_place_expr(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Field(_) | syn::Expr::Index(_) | syn::Expr::Path(_) => true,
        syn::Expr::Unary(unary) => matches!(unary.op, syn::UnOp::Deref(_)),
        syn::Expr::Group(group) => is_place_expr(&group.expr),
        syn::Expr::Paren(paren) => is_place_expr(&paren.expr),
        _ => false,
    }
}

fn is_scalar_index_bind_candidate(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Array(_)
        | syn::Expr::Async(_)
        | syn::Expr::Await(_)
        | syn::Expr::Block(_)
        | syn::Expr::Call(_)
        | syn::Expr::Closure(_)
        | syn::Expr::ForLoop(_)
        | syn::Expr::Loop(_)
        | syn::Expr::Macro(_)
        | syn::Expr::MethodCall(_)
        | syn::Expr::Range(_)
        | syn::Expr::Repeat(_)
        | syn::Expr::Struct(_)
        | syn::Expr::Try(_)
        | syn::Expr::TryBlock(_)
        | syn::Expr::Tuple(_)
        | syn::Expr::Unsafe(_)
        | syn::Expr::While(_)
        | syn::Expr::Yield(_) => false,
        syn::Expr::Group(group) => is_scalar_index_bind_candidate(&group.expr),
        syn::Expr::Paren(paren) => is_scalar_index_bind_candidate(&paren.expr),
        _ => true,
    }
}

fn span_matches(span: Span, injection: &Injection) -> bool {
    let start = span.start();
    let end = span.end();
    contains(start, end, injection.line, injection.col)
        || contains(start, end, injection.line, injection.col.saturating_sub(1))
        || contains(start, end, injection.line, injection.col.saturating_add(1))
}

fn span_starts_at_location(span: Span, injection: &Injection) -> bool {
    let start = span.start();
    let start_col = start.column + 1;
    start.line == injection.line && start_col.abs_diff(injection.col) <= 1
}

fn contains(start: LineColumn, end: LineColumn, line: usize, col: usize) -> bool {
    let start_col = start.column + 1;
    let end_col = end.column + 1;
    (line > start.line || (line == start.line && col >= start_col))
        && (line < end.line || (line == end.line && col <= end_col))
}

fn simple_ident_name(expr: &syn::Expr) -> Option<String> {
    if let syn::Expr::Path(path) = expr {
        if path.qself.is_none() && path.path.segments.len() == 1 {
            return Some(path.path.segments[0].ident.to_string());
        }
    }
    None
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
                callee_name: None,
                raw_pointer_deref: false,
                source_line: None,
            }
        ));
        assert!(span_matches(
            span,
            &Injection {
                id: "src-vec-rs-206-20".to_string(),
                line: start.line,
                col: start_col + 1,
                callee_name: None,
                raw_pointer_deref: false,
                source_line: None,
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
    fn inject_file_inserts_callsite_inside_unsafe_block() -> Result<()> {
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
                callee_name: Some("callee".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(!compact.contains("klee_ext_bind::bind!"));
        assert!(compact.contains("unsafe{klee_ext_bind::callsite!"));
        assert!(compact.contains("klee_ext_bind::callsite!(\"src-lib-rs-3-9\");"));
        assert!(compact.contains("callee(p,layout)"));
        Ok(())
    }

    #[test]
    fn inject_file_keeps_inline_always_target_as_callable_symbol() -> Result<()> {
        let tmp = TempDir::new("preserve-target-caller")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"struct Values(Vec<u8>);
impl Values {
    #[inline(always)]
    pub fn get(&self, index: usize) -> &u8 {
        unsafe { self.0.get_unchecked(index) }
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-5-18".to_string(),
                line: 5,
                col: 18,
                callee_name: Some("core::slice::<impl [T]>::get_unchecked".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        assert!(!injected.contains("inline(always)"));
        assert!(injected.contains("inline(never)"));
        assert!(injected.contains("klee_ext_bind::callsite!"));
        Ok(())
    }

    #[test]
    fn inject_file_skips_raw_pointer_target_without_source_deref() -> Result<()> {
        let tmp = TempDir::new("skip-implicit-raw-deref")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub fn init(content: &[u64], rank: &[u64]) {
    let _ = Select::new(&content, &rank);
}

struct Select;
impl Select {
    fn new(_: &[u64], _: &[u64]) -> Self { Select }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-2-25".to_string(),
                line: 2,
                col: 25,
                callee_name: Some("core::ptr::__raw_ptr_deref__".to_string()),
                raw_pointer_deref: true,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        assert!(!injected.contains("klee_ext_bind::raw_pointer_deref!"));
        assert!(!injected.contains("klee_ext_bind::callsite!"));
        Ok(())
    }

    #[test]
    fn inject_file_lifts_complex_call_arguments_before_callsite() -> Result<()> {
        let tmp = TempDir::new("lift-complex-args")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub fn init(ptr: *mut u8, index: usize) {
    unsafe {
        core::ptr::write(ptr.add(index), make_value())
    }

}

fn make_value() -> u8 { 0 }
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-3-9".to_string(),
                line: 3,
                col: 9,
                callee_name: Some("core::ptr::write".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(compact.contains("let__klee_arg0=ptr.add(index);"));
        assert!(compact.contains("let__klee_arg1=make_value();"));
        assert!(!compact.contains("klee_ext_bind::bind!"));
        assert!(compact.contains("klee_ext_bind::callsite!(\"src-lib-rs-3-9\");"));
        assert!(
            compact.contains("core::ptr::write(__klee_arg0,__klee_arg1"),
            "unexpected injection:\n{injected}"
        );
        Ok(())
    }

    #[test]
    fn inject_file_keeps_qualified_constant_call_argument_in_place() -> Result<()> {
        let tmp = TempDir::new("const-call-argument")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"const HINT: i32 = 3;
unsafe fn prefetch(_ptr: *const i8, _hint: i32) {}
fn run(ptr: *const i8) {
    unsafe { prefetch(ptr, crate::HINT) }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-4-14".to_string(),
                line: 4,
                col: 14,
                callee_name: Some("prefetch".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(compact.contains("prefetch(ptr,crate::HINT)"));
        assert!(!compact.contains("let__klee_arg0=crate::HINT"));
        Ok(())
    }

    #[test]
    fn inject_file_prefers_innermost_matching_call() -> Result<()> {
        let tmp = TempDir::new("innermost-call")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub fn init(ptr: *mut u8, index: usize) {
    unsafe {
        core::ptr::write(ptr.add(index), make_value())
    }
}

fn make_value() -> u8 { 0 }
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-3-30".to_string(),
                line: 3,
                col: 30,
                callee_name: Some("std::ptr::mut_ptr::<impl *mut T>::add".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(!compact.contains("klee_ext_bind::bind!"));
        assert!(compact.contains("klee_ext_bind::callsite!(\"src-lib-rs-3-30\");"));
        assert!(compact.contains("=ptr.add(index);"));
        assert!(compact.contains("core::ptr::write(__klee_ret_src_lib_rs_3_30_add,make_value())"));
        Ok(())
    }

    #[test]
    fn inject_file_binds_slice_get_unchecked_len_and_index() -> Result<()> {
        let tmp = TempDir::new("slice-get-unchecked-bindings")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub struct Rank {
    ranks: Box<[u32]>,
}

impl Rank {
    pub fn read(&self, block: usize) -> u32 {
        unsafe { *self.ranks.get_unchecked(block) }
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-7-19".to_string(),
                line: 7,
                col: 19,
                callee_name: Some("core::slice::<impl [T]>::get_unchecked".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(
            compact.contains("let__klee_arg0=&self.ranks;"),
            "unexpected injection:\n{injected}"
        );
        assert!(compact.contains("klee_ext_bind::bind_arg_u64(1,(__klee_arg0).len()asu64);"));
        assert!(compact.contains("klee_ext_bind::bind_arg_u64(2,blockasu64);"));
        assert!(compact.contains("=__klee_arg0.get_unchecked(block);"));
        Ok(())
    }

    #[test]
    fn inject_file_does_not_rewrite_later_function_calls_to_previous_ret() -> Result<()> {
        let tmp = TempDir::new("same-callee-function-calls")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub fn init(data: *const u8) {
    unsafe {
        y0 ^= ptr::read_unaligned(data);
        y1 ^= ptr::read_unaligned(data);
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[
                Injection {
                    id: "src-lib-rs-3-15".to_string(),
                    line: 3,
                    col: 15,
                    callee_name: Some("core::ptr::read_unaligned".to_string()),
                    raw_pointer_deref: false,
                    source_line: None,
                },
                Injection {
                    id: "src-lib-rs-4-15".to_string(),
                    line: 4,
                    col: 15,
                    callee_name: Some("core::ptr::read_unaligned".to_string()),
                    raw_pointer_deref: false,
                    source_line: None,
                },
            ],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(compact.contains(
            "letmut__klee_ret_src_lib_rs_3_15_read_unaligned=ptr::read_unaligned(data);"
        ));
        assert!(compact.contains(
            "letmut__klee_ret_src_lib_rs_4_15_read_unaligned=ptr::read_unaligned(data);"
        ));
        assert!(!compact.contains(
            "let__klee_ret_src_lib_rs_4_15_read_unaligned=__klee_ret_src_lib_rs_3_15_read_unaligned;"
        ));
        Ok(())
    }

    #[test]
    fn inject_file_handles_same_span_chained_method_calls() -> Result<()> {
        let tmp = TempDir::new("same-span-chain")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub fn init(ptr: *mut u8, index: usize) {
    unsafe {
        let dst = ptr.add(index).as_mut().unwrap();
        consume(dst);
    }
}

fn consume(_: &mut u8) {}
"#,
        )?;

        inject_file(
            &source,
            &[
                Injection {
                    id: "src-lib-rs-3-19".to_string(),
                    line: 3,
                    col: 19,
                    callee_name: Some("std::ptr::mut_ptr::<impl *mut T>::add".to_string()),
                    raw_pointer_deref: false,
                    source_line: None,
                },
                Injection {
                    id: "src-lib-rs-3-19".to_string(),
                    line: 3,
                    col: 19,
                    callee_name: Some("std::ptr::mut_ptr::<impl *mut T>::as_mut".to_string()),
                    raw_pointer_deref: false,
                    source_line: None,
                },
            ],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(compact.contains("let__klee_arg0=__klee_ret_src_lib_rs_3_19_add;"));
        assert!(compact.contains("=__klee_arg0.as_mut();"));
        assert!(compact.contains("=ptr.add(index);"));
        assert!(compact.contains("__klee_ret_src_lib_rs_3_19_as_mut.unwrap()"));
        Ok(())
    }

    #[test]
    fn inject_file_handles_multiline_chain_start_callsite() -> Result<()> {
        let tmp = TempDir::new("multiline-chain-start")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub fn add(storage: &mut [u8], cursor: usize, component: u8) {
    unsafe {
        storage
            .as_mut_ptr()
            .add(cursor)
            .cast::<u8>()
            .write_unaligned(component);
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-3-9".to_string(),
                line: 3,
                col: 9,
                callee_name: Some("core::ptr::mut_ptr::<impl *mut T>::add".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(
            compact.contains("let__klee_arg0=storage.as_mut_ptr();"),
            "unexpected injection:\n{injected}"
        );
        assert!(!compact.contains("klee_ext_bind::bind!"));
        assert!(compact.contains("klee_ext_bind::callsite!(\"src-lib-rs-3-9\");"));
        assert!(compact.contains("=__klee_arg0.add(cursor);"));
        assert!(compact
            .contains("__klee_ret_src_lib_rs_3_9_add.cast::<u8>().write_unaligned(component)"));
        Ok(())
    }

    #[test]
    fn inject_file_borrows_raw_pointer_field_receiver_without_moving_it() -> Result<()> {
        let tmp = TempDir::new("raw-pointer-field-receiver")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub struct Window {
    end: *const u8,
}

impl Window {
    pub fn advance(&mut self, r: usize) {
        unsafe {
            self.end = self.end.add(r);
        }
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-8-33".to_string(),
                line: 8,
                col: 33,
                callee_name: Some("core::ptr::const_ptr::<impl *const T>::add".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(compact.contains("let__klee_arg0=&self.end;"));
        assert!(compact.contains("=__klee_arg0.add(r);"));
        Ok(())
    }

    #[test]
    fn inject_file_wraps_exact_raw_pointer_operand() -> Result<()> {
        let tmp = TempDir::new("raw-pointer-deref")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"pub struct Interpreter {
    pub instruction_pointer: *const u8,
}

impl Interpreter {
    pub fn current_opcode(&self) -> u8 {
        unsafe { *self.instruction_pointer }
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-7-18".to_string(),
                line: 7,
                col: 18,
                callee_name: Some("core::ptr::__raw_ptr_deref__".to_string()),
                raw_pointer_deref: true,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(compact.contains(
            "*klee_ext_bind::raw_pointer_deref!(\"src-lib-rs-7-18\",self.instruction_pointer)"
        ));
        assert!(!compact.contains("klee_ext_bind::callsite!"));
        Ok(())
    }

    #[test]
    fn inject_file_does_not_treat_box_deref_as_raw_pointer_deref() -> Result<()> {
        let tmp = TempDir::new("box-deref-is-not-raw")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"struct Error(Box<u8>);
impl Error {
    fn into_inner(self) -> u8 {
        *self.0
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-4-9".to_string(),
                line: 4,
                col: 9,
                callee_name: Some("core::ptr::__raw_ptr_deref__".to_string()),
                raw_pointer_deref: true,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        assert!(!injected.contains("raw_pointer_deref!"));
        Ok(())
    }

    #[test]
    fn inject_file_preserves_mutable_field_receiver_for_copy_within() -> Result<()> {
        let tmp = TempDir::new("copy-within-receiver")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"struct Buffer { buf: Vec<u8> }
impl Buffer {
    fn shift(&mut self) {
        self.buf.copy_within(1.., 0);
    }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-4-9".to_string(),
                line: 4,
                col: 9,
                callee_name: Some("core::ptr::__raw_ptr_deref__".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(!compact.contains("let__klee_arg0="));
        assert!(compact.contains("self.buf.copy_within"));
        Ok(())
    }

    #[test]
    fn inject_file_mutably_borrows_unsafe_field_receiver() -> Result<()> {
        let tmp = TempDir::new("unsafe-field-receiver")?;
        let source = tmp.path().join("lib.rs");
        write(
            &source,
            r#"struct Stack;

impl Stack {
    unsafe fn pop_unsafe(&mut self) -> u8 { 0 }
}

struct Interpreter { stack: Stack }

fn pop(interpreter: &mut Interpreter) -> u8 {
    unsafe { interpreter.stack.pop_unsafe() }
}
"#,
        )?;

        inject_file(
            &source,
            &[Injection {
                id: "src-lib-rs-10-14".to_string(),
                line: 10,
                col: 14,
                // mirscan can report an inlined unsafe callee while the source
                // location names the enclosing unsafe method.
                callee_name: Some("std::option::Option::<T>::unwrap_unchecked".to_string()),
                raw_pointer_deref: false,
                source_line: None,
            }],
        )?;

        let injected = fs::read_to_string(source)?;
        let compact = injected.replace(char::is_whitespace, "");
        assert!(
            !compact.contains("let__klee_arg0="),
            "unexpected injection:\n{injected}"
        );
        assert!(compact.contains("=interpreter.stack.pop_unsafe();"));
        Ok(())
    }

    #[test]
    fn run_copies_crate_adds_dependency_injects_and_writes_meta() -> Result<()> {
        let tmp = TempDir::new("run")?;
        let source_crate = tmp.path().join("source");
        let dest_crate = tmp.path().join("dest");
        let meta_path = tmp.path().join("meta.json");
        fixture_crate(&source_crate)?;
        fixture_meta(&meta_path)?;

        run(&source_crate, &meta_path, &dest_crate)?;

        let manifest = fs::read_to_string(dest_crate.join("Cargo.toml"))?;
        assert!(manifest.contains("[dependencies.klee-ext-bind]"));
        assert!(manifest.contains("path = "));
        assert!(!dest_crate.join("target/ignored.txt").exists());

        let source = fs::read_to_string(dest_crate.join("src/lib.rs"))?;
        let compact = source.replace(char::is_whitespace, "");
        assert!(!compact.contains("klee_ext_bind::bind!"));
        assert!(compact.contains("klee_ext_bind::callsite!(\"src-lib-rs-3-9\");"));

        Ok(())
    }

    #[test]
    fn copy_rebases_relative_dependency_paths_but_not_lib_paths() -> Result<()> {
        let tmp = TempDir::new("rebase-path-dependency")?;
        let source_crate = tmp.path().join("source/crate");
        let dependency = tmp.path().join("source/local-dep");
        let dest_crate = tmp.path().join("generated/deep/crate");
        write(
            &dependency.join("Cargo.toml"),
            "[package]\nname='local-dep'\nversion='0.1.0'\n",
        )?;
        write(
            &source_crate.join("Cargo.toml"),
            r#"[package]
name = "fixture"
version = "0.1.0"

[lib]
path = "custom.rs"

[dependencies.local-dep]
path = "../local-dep"
"#,
        )?;
        write(&source_crate.join("custom.rs"), "pub fn fixture() {}\n")?;

        copy_crate(&source_crate, &dest_crate)?;
        rebase_relative_dependency_paths(&source_crate, &dest_crate)?;

        let manifest = fs::read_to_string(dest_crate.join("Cargo.toml"))?;
        let doc = manifest.parse::<DocumentMut>()?;
        let rebased = doc["dependencies"]["local-dep"]["path"]
            .as_str()
            .ok_or_else(|| anyhow!("missing rebased dependency path"))?;
        assert_eq!(
            dest_crate.join(rebased).canonicalize()?,
            dependency.canonicalize()?
        );
        assert_eq!(doc["lib"]["path"].as_str(), Some("custom.rs"));
        Ok(())
    }
}
