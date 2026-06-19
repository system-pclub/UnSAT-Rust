use anyhow::{anyhow, Context, Result};
use proc_macro2::Span;
use quote::ToTokens;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    Expr, ExprBlock, ExprCall, ExprMacro, ExprPath, ExprUnsafe, Fields, File, GenericParam,
    ImplItem, ImplItemFn, Item, ItemFn, ItemImpl, ItemStruct, PathArguments, Type, Visibility,
};
use tokio::sync::{mpsc, Semaphore};
use tokio::task;
use walkdir::WalkDir;

const DEFAULT_THREADS: usize = 8;

#[derive(Debug, Clone)]
struct Config {
    root: PathBuf,
    threads: usize,
    blacklist: Vec<String>,
}

#[derive(Debug, Clone)]
struct StructInfo {
    name: String,
    visibility: String,
    template_number: usize,
    fields: Vec<FieldInfo>,
}

#[derive(Debug, Clone, Serialize)]
struct FieldInfo {
    name: String,
    visibility: String,
    template_number: usize,
}

#[derive(Debug, Clone, Serialize)]
struct Finding {
    file: String,
    function: String,
    function_kind: String,
    function_line: usize,
    function_col: usize,
    unsafe_line: usize,
    unsafe_col: usize,
    reference: String,
    reference_line: usize,
    reference_col: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    struct_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    struct_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    struct_template_number: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<FieldInfo>,
}

fn main() -> Result<()> {
    let config = parse_args()?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.threads)
        .enable_all()
        .build()?;
    runtime.block_on(run(config))
}

async fn run(config: Config) -> Result<()> {
    let (file_sender, mut file_receiver) = mpsc::unbounded_channel();
    let collector = task::spawn_blocking({
        let root = config.root.clone();
        let blacklist = config.blacklist.clone();
        move || collect_rust_files(root, blacklist, file_sender)
    });

    let semaphore = Arc::new(Semaphore::new(config.threads));
    let (sender, mut receiver) = mpsc::unbounded_channel();
    let scheduler = tokio::spawn(async move {
        let mut tasks = Vec::new();

        while let Some(file) = file_receiver.recv().await {
            let file = file.map_err(anyhow::Error::msg)?;
            let permit = semaphore.clone().acquire_owned().await?;
            let sender = sender.clone();
            tasks.push(tokio::spawn(async move {
                let _permit = permit;
                process_file(file, sender).await
            }));
        }
        drop(sender);

        for task in tasks {
            task.await??;
        }
        Ok::<_, anyhow::Error>(())
    });

    while let Some(finding) = receiver.recv().await {
        print_finding(&finding);
    }

    collector.await?;
    scheduler.await??;
    Ok(())
}

async fn process_file(path: PathBuf, sender: mpsc::UnboundedSender<Finding>) -> Result<()> {
    let source = match tokio::fs::read_to_string(&path).await {
        Ok(source) => source,
        Err(error) => {
            eprintln!("{}: failed to read file: {error}", path.display());
            return Ok(());
        }
    };
    let parsed = match syn::parse_file(&source) {
        Ok(parsed) => parsed,
        Err(error) => {
            let start = error.span().start();
            eprintln!(
                "{}:{}:{}: syn parse error: {error}",
                path.display(),
                start.line,
                start.column
            );
            return Ok(());
        }
    };
    for finding in analyze_file(&path, &parsed) {
        if sender.send(finding).is_err() {
            break;
        }
    }
    Ok(())
}

fn print_finding(finding: &Finding) {
    let reference = finding.reference.replace(' ', "");
    let is_struct_method = if finding.struct_name.is_some() {
        "yes"
    } else {
        "no"
    };
    println!(
        "{}:{},{},{}",
        finding.file, finding.reference_line, reference, is_struct_method
    );
}

fn analyze_file(path: &Path, file: &File) -> Vec<Finding> {
    let structs = collect_structs(file);
    let mut findings = Vec::new();

    for item in &file.items {
        match item {
            Item::Fn(function) if is_pub_safe_fn(function) => {
                findings.extend(analyze_fn(path, function, "free", None));
            }
            Item::Impl(item_impl) => {
                findings.extend(analyze_impl(path, item_impl, &structs));
            }
            _ => {}
        }
    }

    findings
}

fn collect_structs(file: &File) -> BTreeMap<String, StructInfo> {
    let mut structs = BTreeMap::new();
    for item in &file.items {
        if let Item::Struct(item_struct) = item {
            let info = struct_info(item_struct);
            structs.insert(info.name.clone(), info);
        }
    }
    structs
}

fn struct_info(item_struct: &ItemStruct) -> StructInfo {
    StructInfo {
        name: item_struct.ident.to_string(),
        visibility: visibility_name(&item_struct.vis),
        template_number: generic_param_count(&item_struct.generics.params),
        fields: fields_info(&item_struct.fields),
    }
}

fn fields_info(fields: &Fields) -> Vec<FieldInfo> {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| FieldInfo {
            name: field
                .ident
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| index.to_string()),
            visibility: visibility_name(&field.vis),
            template_number: type_template_number(&field.ty),
        })
        .collect()
}

fn analyze_impl(
    path: &Path,
    item_impl: &ItemImpl,
    structs: &BTreeMap<String, StructInfo>,
) -> Vec<Finding> {
    let Some(struct_name) = impl_self_type_name(&item_impl.self_ty) else {
        return Vec::new();
    };
    let struct_info = structs.get(&struct_name);
    let mut findings = Vec::new();

    for item in &item_impl.items {
        if let ImplItem::Fn(method) = item {
            if is_pub_safe_method(method) {
                findings.extend(analyze_method(path, method, struct_info));
            }
        }
    }

    findings
}

fn analyze_fn(
    path: &Path,
    function: &ItemFn,
    function_kind: &str,
    struct_info: Option<&StructInfo>,
) -> Vec<Finding> {
    analyze_block(
        path,
        function.sig.ident.to_string(),
        function_kind,
        function.sig.ident.span(),
        &function.block,
        struct_info,
    )
}

fn analyze_method(
    path: &Path,
    method: &ImplItemFn,
    struct_info: Option<&StructInfo>,
) -> Vec<Finding> {
    analyze_block(
        path,
        method.sig.ident.to_string(),
        "method",
        method.sig.ident.span(),
        &method.block,
        struct_info,
    )
}

fn analyze_block(
    path: &Path,
    function: String,
    function_kind: &str,
    function_span: Span,
    block: &syn::Block,
    struct_info: Option<&StructInfo>,
) -> Vec<Finding> {
    let mut visitor = UnsafeCoreStdVisitor::default();
    visitor.visit_block(block);
    let function_start = function_span.start();
    let file = path.to_string_lossy().into_owned();

    visitor
        .references
        .into_iter()
        .map(|reference| {
            let reference_start = reference.span.start();
            let unsafe_start = reference.unsafe_span.start();
            Finding {
                file: file.clone(),
                function: function.clone(),
                function_kind: function_kind.to_string(),
                function_line: function_start.line,
                function_col: function_start.column,
                unsafe_line: unsafe_start.line,
                unsafe_col: unsafe_start.column,
                reference: reference.text,
                reference_line: reference_start.line,
                reference_col: reference_start.column,
                struct_name: struct_info.map(|info| info.name.clone()),
                struct_visibility: struct_info.map(|info| info.visibility.clone()),
                struct_template_number: struct_info.map(|info| info.template_number),
                fields: struct_info
                    .map(|info| info.fields.clone())
                    .unwrap_or_default(),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct UnsafeReference {
    text: String,
    span: Span,
    unsafe_span: Span,
}

#[derive(Default)]
struct UnsafeCoreStdVisitor {
    unsafe_spans: Vec<Span>,
    references: Vec<UnsafeReference>,
    seen: BTreeSet<(usize, usize, String)>,
}

impl UnsafeCoreStdVisitor {
    fn current_unsafe_span(&self) -> Option<Span> {
        self.unsafe_spans.last().copied()
    }

    fn push_reference(&mut self, span: Span, text: String) {
        let Some(unsafe_span) = self.current_unsafe_span() else {
            return;
        };
        let start = span.start();
        if self.seen.insert((start.line, start.column, text.clone())) {
            self.references.push(UnsafeReference {
                text,
                span,
                unsafe_span,
            });
        }
    }

    fn visit_core_std_path(&mut self, span: Span, path: &syn::Path) {
        if self.current_unsafe_span().is_none() {
            return;
        }
        if path_starts_core_or_std(path) {
            self.push_reference(span, path.to_token_stream().to_string());
        }
    }
}

impl<'ast> Visit<'ast> for UnsafeCoreStdVisitor {
    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        self.unsafe_spans.push(node.unsafe_token.span);
        visit::visit_block(self, &node.block);
        self.unsafe_spans.pop();
    }

    fn visit_expr_block(&mut self, node: &'ast ExprBlock) {
        visit::visit_expr_block(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast ExprPath) {
        self.visit_core_std_path(node.span(), &node.path);
        visit::visit_expr_path(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(path) = node.func.as_ref() {
            self.visit_core_std_path(path.span(), &path.path);
        }
        visit::visit_expr_call(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        self.visit_core_std_path(node.mac.path.span(), &node.mac.path);
        visit::visit_expr_macro(self, node);
    }
}

fn is_pub_safe_fn(function: &ItemFn) -> bool {
    is_public(&function.vis) && function.sig.unsafety.is_none()
}

fn is_pub_safe_method(method: &ImplItemFn) -> bool {
    is_public(&method.vis) && method.sig.unsafety.is_none()
}

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

fn visibility_name(vis: &Visibility) -> String {
    match vis {
        Visibility::Public(_) => "pub".to_string(),
        Visibility::Restricted(restricted) => restricted.to_token_stream().to_string(),
        Visibility::Inherited => "private".to_string(),
    }
}

fn impl_self_type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

fn path_starts_core_or_std(path: &syn::Path) -> bool {
    path.segments
        .first()
        .map(|segment| segment.ident == "core" || segment.ident == "std")
        .unwrap_or(false)
}

fn generic_param_count(
    params: &syn::punctuated::Punctuated<GenericParam, syn::token::Comma>,
) -> usize {
    params.len()
}

fn type_template_number(ty: &Type) -> usize {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| match &segment.arguments {
                PathArguments::AngleBracketed(args) => args.args.len(),
                PathArguments::Parenthesized(args) => {
                    args.inputs.len()
                        + usize::from(!matches!(args.output, syn::ReturnType::Default))
                }
                PathArguments::None => 0,
            })
            .unwrap_or(0),
        _ => 0,
    }
}

fn collect_rust_files(
    root: PathBuf,
    blacklist: Vec<String>,
    sender: mpsc::UnboundedSender<Result<PathBuf, String>>,
) {
    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            !is_skipped_dir(entry.path()) && !is_blacklisted(entry.path(), &blacklist)
        })
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                let _ = sender.send(Err(error.to_string()));
                return;
            }
        };
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "rs") {
            if sender.send(Ok(entry.path().to_path_buf())).is_err() {
                return;
            }
        }
    }
}

fn is_skipped_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| matches!(name, "target" | ".git"))
        .unwrap_or(false)
}

fn is_blacklisted(path: &Path, blacklist: &[String]) -> bool {
    let path = path.to_string_lossy();
    blacklist
        .iter()
        .any(|pattern| !pattern.is_empty() && path.contains(pattern))
}

fn parse_args() -> Result<Config> {
    let mut root = None;
    let mut threads = DEFAULT_THREADS;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-j" | "--threads" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow!("{arg} requires a value"))?;
                threads = value
                    .parse()
                    .with_context(|| format!("invalid thread count: {value}"))?;
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            _ if arg.starts_with("--threads=") => {
                let value = arg.trim_start_matches("--threads=");
                threads = value
                    .parse()
                    .with_context(|| format!("invalid thread count: {value}"))?;
            }
            _ if arg.starts_with('-') => return Err(anyhow!("unknown option: {arg}")),
            _ => {
                if root.replace(PathBuf::from(&arg)).is_some() {
                    return Err(anyhow!("only one directory argument is supported"));
                }
            }
        }
    }

    if threads == 0 {
        return Err(anyhow!("thread count must be greater than zero"));
    }

    Ok(Config {
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        threads,
        blacklist: vec!["windows".to_string()],
    })
}

fn print_usage() {
    eprintln!("usage: fastcheck [--threads N|-j N] [directory]");
    eprintln!("default threads: {DEFAULT_THREADS}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_core_reference_under_public_safe_free_fn() {
        let parsed = syn::parse_file(
            r#"
pub fn init(ptr: *mut u8) {
    unsafe {
        core::ptr::write(ptr, 1);
    }
}
"#,
        )
        .unwrap();

        let findings = analyze_file(Path::new("sample.rs"), &parsed);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].function, "init");
        assert_eq!(findings[0].function_kind, "free");
        assert_eq!(findings[0].reference, "core :: ptr :: write");
        assert!(findings[0].fields.is_empty());
    }

    #[test]
    fn includes_struct_fields_for_public_safe_method() {
        let parsed = syn::parse_file(
            r#"
pub struct Bag<T> {
    pub item: Option<T>,
    secret: Vec<T>,
}

impl<T> Bag<T> {
    pub fn put(&mut self, ptr: *mut T, value: T) {
        unsafe {
            std::ptr::write(ptr, value);
        }
    }
}
"#,
        )
        .unwrap();

        let findings = analyze_file(Path::new("sample.rs"), &parsed);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].function, "put");
        assert_eq!(findings[0].function_kind, "method");
        assert_eq!(findings[0].struct_name.as_deref(), Some("Bag"));
        assert_eq!(findings[0].struct_template_number, Some(1));
        assert_eq!(findings[0].fields.len(), 2);
        assert_eq!(findings[0].fields[0].name, "item");
        assert_eq!(findings[0].fields[0].visibility, "pub");
        assert_eq!(findings[0].fields[0].template_number, 1);
        assert_eq!(findings[0].fields[1].name, "secret");
        assert_eq!(findings[0].fields[1].visibility, "private");
        assert_eq!(findings[0].fields[1].template_number, 1);
    }
}
