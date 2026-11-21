//! Fast syntactic screen for unsafe std/core callsites reachable from public safe functions.
//!
//! A callsite is reported when, inside an `unsafe` block of a public safe function, one of the
//! tracked unsafe APIs receives a value that is derived from a function parameter (or a pub field
//! of `self`) and no earlier condition, assertion or sanitising expression mentions that value.
//! Output lines: `path:line<TAB>caller<TAB>callee`.
//!
//! `--family ast` (see `ast_family.rs`) instead screens the DSL rules whose preconditions are plain field
//! comparisons and appends a fourth column with the matching `step2-<corpus>-rule-<n>` tags;
//! `--family all` runs both. The default (`mem`) output is unchanged.
use anyhow::{anyhow, Context, Result};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    Attribute, Expr, ExprCall, ExprField, ExprMacro, ExprMethodCall, ExprPath, ExprUnsafe, FnArg,
    ImplItem, Item, ItemImpl, ItemStruct, Member, Pat, PatIdent, Type, Visibility,
};
use tokio::sync::{mpsc, Semaphore};
use tokio::task;
use walkdir::WalkDir;

mod ast_family;

const DEFAULT_THREADS: usize = 8;
const SKIPPED_DIRS: &[&str] = &["target", ".git", "tests", "benches", "examples"];
const SANITIZERS: &[&str] = &[
    "min", "clamp", "checked_", "saturating_", "try_from", "try_into", "rem_euclid",
];

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Method,
    Path,
}

/// One tracked unsafe API. `quals` lists the accepted path qualifiers. `ptr_only` methods are ambiguous with safe traits and need a pointer-typed receiver.
struct Api {
    name: &'static str,
    kind: Kind,
    quals: &'static [&'static str],
    recv: bool,
    args: &'static [usize],
    ptr_only: bool,
}

const fn method(name: &'static str, recv: bool, args: &'static [usize]) -> Api {
    Api { name, kind: Kind::Method, quals: &[], recv, args, ptr_only: false }
}

const fn ptr_method(name: &'static str, args: &'static [usize]) -> Api {
    Api { name, kind: Kind::Method, quals: &[], recv: true, args, ptr_only: true }
}

const fn path(name: &'static str, quals: &'static [&'static str], args: &'static [usize]) -> Api {
    Api { name, kind: Kind::Path, quals, recv: false, args, ptr_only: false }
}

const APIS: &[Api] = &[
    // slice/str indexing and integer preconditions
    method("get_unchecked", false, &[0]),
    method("get_unchecked_mut", false, &[0]),
    method("split_at_unchecked", false, &[0]),
    method("unchecked_add", true, &[0]),
    method("unchecked_sub", true, &[0]),
    method("unchecked_mul", true, &[0]),
    method("set_len", false, &[0]),
    method("unwrap_unchecked", true, &[]),
    // raw pointer methods
    ptr_method("add", &[0]),
    ptr_method("sub", &[0]),
    ptr_method("offset", &[0]),
    ptr_method("offset_from", &[0]),
    ptr_method("read", &[]),
    ptr_method("write", &[]),
    ptr_method("as_ref", &[]),
    ptr_method("as_mut", &[]),
    ptr_method("drop_in_place", &[]),
    ptr_method("write_bytes", &[1]),
    ptr_method("copy_from_nonoverlapping", &[0, 1]),
    ptr_method("copy_to_nonoverlapping", &[0, 1]),
    // free functions and associated functions; "" admits a bare imported name, "X*" a prefix
    path("from_utf8_unchecked", &["", "str", "String"], &[0]),
    path("from_utf8_unchecked_mut", &["", "str"], &[0]),
    path("from_raw_parts", &["", "slice", "Vec", "String"], &[0, 1]),
    path("from_raw_parts_mut", &["", "slice"], &[0, 1]),
    path("copy_nonoverlapping", &["", "ptr"], &[0, 1, 2]),
    path("copy", &["ptr"], &[0, 1, 2]),
    path("write_bytes", &["ptr"], &[0, 2]),
    path("read", &["ptr"], &[0]),
    path("write", &["ptr"], &[0]),
    path("drop_in_place", &["ptr"], &[0]),
    path("from_u32_unchecked", &["", "char"], &[0]),
    path("new_unchecked", &["NonNull", "Pin", "NonZero*"], &[0]),
    path("from_size_align_unchecked", &["", "Layout"], &[0, 1]),
    path("alloc", &["alloc"], &[0]),
    path("alloc_zeroed", &["alloc"], &[0]),
    path("dealloc", &["alloc"], &[0, 1]),
    path("realloc", &["alloc"], &[0, 1, 2]),
    path("from_raw", &["Box", "Rc", "Arc", "CString"], &[0]),
    path("from_ptr", &["CStr"], &[0]),
];

#[derive(Default, Clone)]
pub(crate) struct StructInfo {
    pub(crate) fields: BTreeSet<String>,
    pub(crate) pub_fields: BTreeSet<String>,
    ptr_fields: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Finding {
    pub(crate) file: String,
    pub(crate) line: usize,
    pub(crate) caller: String,
    pub(crate) callee: String,
    /// DSL rule tags of the `ast` family; empty for the `mem` family.
    pub(crate) tags: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Mem,
    Ast,
    All,
}

#[derive(Debug, Clone)]
struct Config {
    root: PathBuf,
    threads: usize,
    blacklist: Vec<String>,
    family: Family,
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

    let family = config.family;
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
                process_file(file, family, sender).await
            }));
        }
        drop(sender);
        let mut parse_errors = 0usize;
        for task in tasks {
            parse_errors += task.await??;
        }
        Ok::<_, anyhow::Error>(parse_errors)
    });

    let mut findings = BTreeSet::new();
    while let Some(finding) = receiver.recv().await {
        findings.insert(finding);
    }
    collector.await?;
    let parse_errors = scheduler.await??;

    for finding in &findings {
        print!(
            "{}:{}\t{}\t{}",
            finding.file, finding.line, finding.caller, finding.callee
        );
        if !finding.tags.is_empty() {
            print!("\t{}", finding.tags);
        }
        println!();
    }
    eprintln!("{} callsites, {} files failed to parse", findings.len(), parse_errors);
    Ok(())
}

async fn process_file(
    path: PathBuf,
    family: Family,
    sender: mpsc::UnboundedSender<Finding>,
) -> Result<usize> {
    let Ok(source) = tokio::fs::read_to_string(&path).await else {
        return Ok(1);
    };
    let Ok(parsed) = syn::parse_file(&source) else {
        return Ok(1);
    };
    for finding in analyze_file_family(&path, &parsed, family) {
        if sender.send(finding).is_err() {
            break;
        }
    }
    Ok(0)
}

#[cfg(test)]
fn analyze_file(path: &Path, file: &syn::File) -> Vec<Finding> {
    analyze_file_family(path, file, Family::Mem)
}

fn analyze_file_family(path: &Path, file: &syn::File, family: Family) -> Vec<Finding> {
    let mut structs = BTreeMap::new();
    collect_structs(&file.items, &mut structs);
    let ctx = ast_family::Ctx {
        idx_struct: structs.contains_key("Handle")
            || structs.contains_key("NodeRef")
            || structs.values().any(|info| info.fields.contains("idx")),
    };
    let mut findings = Vec::new();
    let env = Env { family, ctx: &ctx };
    analyze_items(&path.to_string_lossy(), &file.items, &structs, &env, &mut findings);
    findings
}

struct Env<'a> {
    family: Family,
    ctx: &'a ast_family::Ctx,
}

fn collect_structs(items: &[Item], structs: &mut BTreeMap<String, StructInfo>) {
    for item in items {
        match item {
            Item::Struct(item_struct) => {
                structs.insert(item_struct.ident.to_string(), struct_info(item_struct));
            }
            Item::Mod(module) => {
                if let Some((_, items)) = &module.content {
                    collect_structs(items, structs);
                }
            }
            _ => {}
        }
    }
}

fn struct_info(item: &ItemStruct) -> StructInfo {
    let mut info = StructInfo::default();
    for (index, field) in item.fields.iter().enumerate() {
        let name = field
            .ident
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| index.to_string());
        info.fields.insert(name.clone());
        if is_public(&field.vis) {
            info.pub_fields.insert(name.clone());
        }
        if is_pointer_type(&field.ty) {
            info.ptr_fields.insert(name);
        }
    }
    info
}

fn analyze_items(
    file: &str,
    items: &[Item],
    structs: &BTreeMap<String, StructInfo>,
    env: &Env,
    findings: &mut Vec<Finding>,
) {
    for item in items {
        match item {
            Item::Fn(function) if is_public(&function.vis) && !is_test(&function.attrs) => {
                if function.sig.unsafety.is_none() {
                    analyze_fn(
                        file,
                        function.sig.ident.to_string(),
                        &function.sig.inputs,
                        generic_names(&function.sig.generics, None),
                        &function.block,
                        &StructInfo::default(),
                        env,
                        findings,
                    );
                }
            }
            Item::Impl(item_impl) if !is_test(&item_impl.attrs) => {
                analyze_impl(file, item_impl, structs, env, findings);
            }
            Item::Mod(module) if !is_test(&module.attrs) => {
                if let Some((_, items)) = &module.content {
                    analyze_items(file, items, structs, env, findings);
                }
            }
            _ => {}
        }
    }
}

fn analyze_impl(
    file: &str,
    item_impl: &ItemImpl,
    structs: &BTreeMap<String, StructInfo>,
    env: &Env,
    findings: &mut Vec<Finding>,
) {
    let Type::Path(self_type) = item_impl.self_ty.as_ref() else {
        return;
    };
    let Some(segment) = self_type.path.segments.last() else {
        return;
    };
    let type_name = segment.ident.to_string();
    let info = structs.get(&type_name).cloned().unwrap_or_default();
    let trait_impl = item_impl.trait_.is_some();

    for item in &item_impl.items {
        let ImplItem::Fn(method) = item else { continue };
        if method.sig.unsafety.is_some()
            || is_test(&method.attrs)
            || !(trait_impl || is_public(&method.vis))
        {
            continue;
        }
        analyze_fn(
            file,
            format!("{type_name}::{}", method.sig.ident),
            &method.sig.inputs,
            generic_names(&method.sig.generics, Some(&item_impl.generics)),
            &method.block,
            &info,
            env,
            findings,
        );
    }
}

fn generic_names(own: &syn::Generics, outer: Option<&syn::Generics>) -> HashSet<String> {
    outer
        .into_iter()
        .chain(Some(own))
        .flat_map(|generics| generics.params.iter())
        .filter_map(|param| match param {
            syn::GenericParam::Type(ty) => Some(ty.ident.to_string()),
            _ => None,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn analyze_fn(
    file: &str,
    caller: String,
    inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
    generics: HashSet<String>,
    block: &syn::Block,
    info: &StructInfo,
    env: &Env,
    findings: &mut Vec<Finding>,
) {
    if env.family != Family::Mem {
        ast_family::analyze_fn(file, &caller, inputs, generics, block, info, env.ctx, findings);
    }
    if env.family == Family::Ast {
        return;
    }
    let mut checker = Checker {
        info,
        params: HashMap::new(),
        tainted: HashSet::new(),
        guarded: HashSet::new(),
        unsafe_depth: 0,
        hits: Vec::new(),
    };
    for input in inputs {
        let FnArg::Typed(typed) = input else { continue };
        let pointer = is_pointer_type(&typed.ty);
        let mut names = PatNames::default();
        names.visit_pat(&typed.pat);
        for name in names.0 {
            checker.params.insert(name.clone(), pointer);
            checker.tainted.insert(name);
        }
    }
    checker.visit_block(block);
    for (line, callee) in checker.hits {
        findings.push(Finding {
            file: file.to_string(),
            line,
            caller: caller.clone(),
            callee,
            tags: String::new(),
        });
    }
}

struct Checker<'a> {
    info: &'a StructInfo,
    params: HashMap<String, bool>,
    tainted: HashSet<String>,
    guarded: HashSet<String>,
    unsafe_depth: usize,
    hits: Vec<(usize, String)>,
}

impl Checker<'_> {
    fn idents(&self, expr: &Expr) -> Vec<String> {
        let mut collector = IdentCollector { info: self.info, out: Vec::new() };
        collector.visit_expr(expr);
        collector.out
    }

    /// Identifiers of an expression made only of names, casts and arithmetic with constants;
    /// `None` for anything computed (calls, indexing, ...), which the screen does not follow.
    fn direct_ids(&self, expr: &Expr) -> Option<Vec<String>> {
        match expr {
            Expr::Lit(_) => Some(Vec::new()),
            Expr::Paren(inner) => self.direct_ids(&inner.expr),
            Expr::Group(inner) => self.direct_ids(&inner.expr),
            Expr::Cast(cast) => self.direct_ids(&cast.expr),
            Expr::Reference(reference) => self.direct_ids(&reference.expr),
            Expr::Unary(unary) => self.direct_ids(&unary.expr),
            Expr::Path(path) if path.path.segments.len() > 1 => Some(Vec::new()),
            Expr::Path(_) | Expr::Field(_) => {
                let ids = self.idents(expr);
                (!ids.is_empty()).then_some(ids)
            }
            Expr::Binary(binary) => {
                let mut ids = self.direct_ids(&binary.left)?;
                ids.extend(self.direct_ids(&binary.right)?);
                Some(ids)
            }
            _ => None,
        }
    }

    fn is_tainted(&self, id: &str) -> bool {
        self.tainted.contains(id) || id.starts_with("self.")
    }

    fn unguarded_taint(&self, ids: &[String]) -> bool {
        ids.iter().any(|id| self.is_tainted(id) && !self.guarded.contains(id))
    }

    fn guard(&mut self, ids: Vec<String>) {
        self.guarded.extend(ids);
    }

    fn pointer_receiver(&self, ids: &[String]) -> bool {
        ids.iter().any(|id| match id.strip_prefix("self.") {
            Some(field) => self.info.ptr_fields.contains(field),
            None => self.params.get(id).copied().unwrap_or(true),
        })
    }

    fn check(&mut self, api: &Api, line: usize, callee: String, recv: Option<&Expr>, args: &[&Expr]) {
        let mut ids = Vec::new();
        if api.recv {
            if let Some(recv) = recv {
                let recv_ids = self.direct_ids(recv).unwrap_or_default();
                if api.ptr_only && !self.pointer_receiver(&recv_ids) {
                    return;
                }
                ids.extend(recv_ids);
            }
        }
        for &index in api.args {
            if let Some(arg) = args.get(index) {
                ids.extend(self.direct_ids(arg).unwrap_or_default());
            }
        }
        if self.unguarded_taint(&ids) {
            self.hits.push((line, callee));
        }
    }
}

impl<'ast> Visit<'ast> for Checker<'_> {
    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        self.unsafe_depth += 1;
        visit::visit_expr_unsafe(self, node);
        self.unsafe_depth -= 1;
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        visit::visit_local(self, node);
        let Some(init) = &node.init else { return };
        let all_ids = self.idents(&init.expr);
        let direct = self.direct_ids(&init.expr);
        let mut names = PatNames::default();
        names.visit_pat(&node.pat);
        if init.diverge.is_some() {
            self.guard(all_ids.clone());
        }
        if !all_ids.iter().any(|id| self.is_tainted(id)) {
            return;
        }
        let text = init.expr.to_token_stream().to_string();
        let sanitized = direct.is_none()
            || text.contains('%')
            || SANITIZERS.iter().any(|word| text.contains(word));
        for name in names.0 {
            if sanitized {
                self.guarded.insert(name);
            } else {
                self.tainted.insert(name);
            }
        }
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        let ids = self.idents(&node.cond);
        self.guard(ids);
        visit::visit_expr_if(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        let ids = self.idents(&node.cond);
        self.guard(ids);
        visit::visit_expr_while(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        let ids = self.idents(&node.expr);
        self.guard(ids);
        visit::visit_expr_match(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let name = node.path.segments.last().map(|s| s.ident.to_string());
        if name.as_deref().is_some_and(is_guard_name) {
            let mut ids = Vec::new();
            token_idents(node.tokens.clone(), self.info, &mut ids);
            self.guard(ids);
        }
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        if is_guard_name(&node.method.to_string()) {
            let ids = node.args.iter().flat_map(|arg| self.idents(arg)).collect();
            self.guard(ids);
        }
        if self.unsafe_depth > 0 {
            let name = node.method.to_string();
            let args: Vec<&Expr> = node.args.iter().collect();
            for api in APIS.iter().filter(|api| api.kind == Kind::Method && api.name == name) {
                self.check(
                    api,
                    node.method.span().start().line,
                    format!(".{name}"),
                    Some(&node.receiver),
                    &args,
                );
            }
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(func) = node.func.as_ref() {
            if func.path.segments.last().is_some_and(|s| is_guard_name(&s.ident.to_string())) {
                let ids = node.args.iter().flat_map(|arg| self.idents(arg)).collect();
                self.guard(ids);
            }
        }
        if self.unsafe_depth > 0 {
            if let Expr::Path(func) = node.func.as_ref() {
                let segments: Vec<String> =
                    func.path.segments.iter().map(|s| s.ident.to_string()).collect();
                if let Some((name, prefix)) = segments.split_last() {
                    let qualifier = prefix.last().map(String::as_str);
                    let args: Vec<&Expr> = node.args.iter().collect();
                    for api in APIS.iter().filter(|api| api.kind == Kind::Path && api.name == name) {
                        let qualified = api.quals.iter().any(|&q| match qualifier {
                            None => q.is_empty(),
                            Some(actual) => match q.strip_suffix('*') {
                                Some(prefix) => actual.starts_with(prefix),
                                None => actual == q,
                            },
                        });
                        if qualified {
                            self.check(
                                api,
                                func.span().start().line,
                                segments.join("::"),
                                None,
                                &args,
                            );
                        }
                    }
                }
            }
        }
        visit::visit_expr_call(self, node);
    }
}

pub(crate) struct IdentCollector<'a> {
    pub(crate) info: &'a StructInfo,
    pub(crate) out: Vec<String>,
}

impl<'ast> Visit<'ast> for IdentCollector<'_> {
    fn visit_expr_path(&mut self, node: &'ast ExprPath) {
        if let Some(ident) = node.path.get_ident() {
            let name = ident.to_string();
            if name != "self" && !name.starts_with(|c: char| c.is_uppercase()) {
                self.out.push(name);
            }
        }
    }

    fn visit_expr_field(&mut self, node: &'ast ExprField) {
        if let (Expr::Path(base), Member::Named(field)) = (node.base.as_ref(), &node.member) {
            if base.path.is_ident("self") {
                let field = field.to_string();
                if self.info.pub_fields.contains(&field) {
                    self.out.push(format!("self.{field}"));
                }
                return;
            }
        }
        visit::visit_expr_field(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        // `v.as_ptr()` / `v.len()` of one slice are consistent by construction.
        if matches!(
            node.method.to_string().as_str(),
            "as_ptr" | "as_mut_ptr" | "len" | "capacity" | "is_empty"
        ) && node.args.is_empty()
        {
            return;
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        token_idents(node.mac.tokens.clone(), self.info, &mut self.out);
    }
}

pub(crate) fn token_idents(tokens: TokenStream, info: &StructInfo, out: &mut Vec<String>) {
    fn flatten(tokens: TokenStream, flat: &mut Vec<TokenTree>) {
        for tree in tokens {
            match tree {
                TokenTree::Group(group) => flatten(group.stream(), flat),
                other => flat.push(other),
            }
        }
    }
    let mut flat = Vec::new();
    flatten(tokens, &mut flat);
    let mut index = 0;
    while index < flat.len() {
        if let TokenTree::Ident(ident) = &flat[index] {
            let name = ident.to_string();
            let self_field = match (flat.get(index + 1), flat.get(index + 2)) {
                (Some(TokenTree::Punct(dot)), Some(TokenTree::Ident(field)))
                    if name == "self" && dot.as_char() == '.' =>
                {
                    Some(field.to_string())
                }
                _ => None,
            };
            match self_field {
                Some(field) => {
                    if info.pub_fields.contains(&field) {
                        out.push(format!("self.{field}"));
                    }
                    index += 2;
                }
                None => out.push(name),
            }
        }
        index += 1;
    }
}

#[derive(Default)]
pub(crate) struct PatNames(pub(crate) Vec<String>);

impl<'ast> Visit<'ast> for PatNames {
    fn visit_pat_ident(&mut self, node: &'ast PatIdent) {
        self.0.push(node.ident.to_string());
        visit::visit_pat_ident(self, node);
    }

    fn visit_pat(&mut self, node: &'ast Pat) {
        visit::visit_pat(self, node);
    }
}

/// Names of macros and functions that validate their arguments before the unsafe call.
pub(crate) fn is_guard_name(name: &str) -> bool {
    !name.starts_with("debug_")
        && !name.contains("unchecked")
        && ["assert", "check", "valid", "bound", "ensure", "require", "verify", "panic_if"]
            .iter()
            .any(|word| name.contains(word))
}

pub(crate) fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

pub(crate) fn is_pointer_type(ty: &Type) -> bool {
    match ty {
        Type::Ptr(_) => true,
        Type::Path(path) => path.path.segments.last().is_some_and(|s| s.ident == "NonNull"),
        _ => false,
    }
}

fn is_test(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("test")
            || (attr.path().is_ident("cfg") && attr.to_token_stream().to_string().contains("test"))
    })
}

fn collect_rust_files(
    root: PathBuf,
    blacklist: Vec<String>,
    sender: mpsc::UnboundedSender<Result<PathBuf, String>>,
) {
    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| !is_skipped(entry.path(), &blacklist))
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if entry.file_type().is_file()
            && entry.path().extension().is_some_and(|ext| ext == "rs")
            && sender.send(Ok(entry.path().to_path_buf())).is_err()
        {
            return;
        }
    }
}

fn is_skipped(path: &Path, blacklist: &[String]) -> bool {
    let skipped_dir = path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| SKIPPED_DIRS.contains(&name));
    let path = path.to_string_lossy();
    skipped_dir || blacklist.iter().any(|pattern| path.contains(pattern.as_str()))
}

fn parse_args() -> Result<Config> {
    let mut root = None;
    let mut threads = DEFAULT_THREADS;
    let mut family = Family::Mem;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        let value = match arg.as_str() {
            "-j" | "--threads" => Some(
                args.next()
                    .ok_or_else(|| anyhow!("{arg} requires a value"))?,
            ),
            "--family" => Some(
                args.next()
                    .ok_or_else(|| anyhow!("{arg} requires a value"))?,
            ),
            "-h" | "--help" => {
                eprintln!("usage: fastcheck [--threads N|-j N] [--family mem|ast|all] [directory]");
                eprintln!("default threads: {DEFAULT_THREADS}");
                std::process::exit(0);
            }
            _ => arg
                .strip_prefix("--threads=")
                .or_else(|| arg.strip_prefix("--family="))
                .map(str::to_string),
        };
        if arg == "--family" || arg.starts_with("--family=") {
            let value = value.unwrap_or_default();
            family = match value.as_str() {
                "mem" => Family::Mem,
                "ast" => Family::Ast,
                "all" => Family::All,
                _ => return Err(anyhow!("unknown family: {value}")),
            };
        } else if let Some(value) = value {
            threads = value
                .parse()
                .with_context(|| format!("invalid thread count: {value}"))?;
        } else if arg.starts_with('-') {
            return Err(anyhow!("unknown option: {arg}"));
        } else if root.replace(PathBuf::from(&arg)).is_some() {
            return Err(anyhow!("only one directory argument is supported"));
        }
    }

    if threads == 0 {
        return Err(anyhow!("thread count must be greater than zero"));
    }
    Ok(Config {
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        threads,
        blacklist: vec!["windows".to_string()],
        family,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> Vec<(String, String)> {
        let parsed = syn::parse_file(source).unwrap();
        analyze_file(Path::new("sample.rs"), &parsed)
            .into_iter()
            .map(|f| (f.caller, f.callee))
            .collect()
    }

    #[test]
    fn flags_unchecked_parameter_and_skips_guarded_one() {
        let hits = scan(
            r#"
pub fn bad(v: &[u8], i: usize) -> u8 { unsafe { *v.get_unchecked(i) } }
pub fn good(v: &[u8], i: usize) -> u8 { if i < v.len() { unsafe { *v.get_unchecked(i) } } else { 0 } }
pub fn asserted(p: *const u8, n: usize) -> &'static [u8] {
    assert!(n < 10 && !p.is_null());
    unsafe { std::slice::from_raw_parts(p, n) }
}
"#,
        );
        assert_eq!(hits, vec![("bad".to_string(), ".get_unchecked".to_string())]);
    }

    #[test]
    fn tracks_pub_self_fields_pointer_receivers_and_derived_locals() {
        let hits = scan(
            r#"
pub struct S { pub len: usize, ptr: *mut u8, pub raw: *mut u8 }
impl S {
    pub fn a(&self) -> &[u8] { unsafe { std::slice::from_raw_parts(self.ptr, self.len) } }
    pub fn b(&self, n: usize) { let m = n + 1; unsafe { self.raw.add(m); } }
    pub fn c(&self, n: usize) { let m = n.min(3); unsafe { self.ptr.add(m); } }
    pub fn d(&self, n: usize) { unsafe { let _ = n.add(1); } }
}
"#,
        );
        assert_eq!(
            hits,
            vec![
                ("S::a".to_string(), "std::slice::from_raw_parts".to_string()),
                ("S::b".to_string(), ".add".to_string()),
            ]
        );
    }
}
