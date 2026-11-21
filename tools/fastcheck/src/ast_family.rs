//! Second checkpoint family (`--family ast`): callsites of unsafe APIs whose DSL preconditions
//! (`eval/rule/*/step2.json`, expressions without `$mem`) are plain field / AST comparisons.
//!
//! Instead of "argument derived from a parameter and never mentioned in a condition", each
//! checkpoint approximates the DSL comparison: a layout size that may be zero, an old/new layout
//! pair that is never compared, `start <= end` without a relational guard between both values,
//! an index that is only compared with a literal or a public field, and so on.
//! Output lines carry the DSL rule tags (`step2-<corpus>-rule-<n>`) as a fourth column.
use crate::{
    is_guard_name, is_pointer_type, token_idents, Finding, IdentCollector, PatNames, StructInfo,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{BinOp, Expr, ExprCall, ExprMethodCall, FnArg, Member};

pub(crate) struct Ctx {
    /// The file defines btree-like `Handle`/`NodeRef` types or a struct with an `idx` field.
    pub(crate) idx_struct: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Ck {
    Alloc,
    Realloc,
    Grow,
    Shrink,
    RangeNew,
    IntoIterNew,
    NextRange,
    NextBack,
    NextBtree,
    PushUnchecked,
    CStrBuf,
    GetWrong,
    HoleIdx,
    HoleNew,
    SiftUp,
    SiftOne,
    SiftRange,
    NodeCast,
    FromNewInternal,
    SliceHelper,
    SwapUnchecked,
    AsChunks,
    AssertUnchecked,
    ToInt,
    CharU32,
    CodePoint,
    VecParts,
    BorrowRaw,
    SliceU,
    SplitAt,
    U8to64,
    SmallEq,
}

/// One checkpoint. `names` is `|`-separated, `quals` empty for methods, `"*"` for any qualifier
/// and `""` for a bare imported name. `tags[i]` is the DSL rule a firing sub-check `i` refers to.
struct Cp {
    names: &'static str,
    method: bool,
    quals: &'static [&'static str],
    nargs: Option<usize>,
    ck: Ck,
    tags: &'static [&'static str],
}

const fn m(names: &'static str, nargs: usize, ck: Ck, tags: &'static [&'static str]) -> Cp {
    Cp { names, method: true, quals: &[], nargs: Some(nargs), ck, tags }
}

const fn p(
    names: &'static str,
    quals: &'static [&'static str],
    nargs: Option<usize>,
    ck: Ck,
    tags: &'static [&'static str],
) -> Cp {
    Cp { names, method: false, quals, nargs, ck, tags }
}

const ALLOC_QUALS: &[&str] = &["alloc", ""];

const CPS: &[Cp] = &[
    // allocation: layout size != 0, realloc new_size > 0, grow/shrink layout order
    p("alloc", ALLOC_QUALS, Some(1), Ck::Alloc, &["step2-alloc-rule-1106"]),
    p("alloc_zeroed", ALLOC_QUALS, Some(1), Ck::Alloc, &["step2-alloc-rule-1120"]),
    m("alloc", 1, Ck::Alloc, &["step2-core-rule-218"]),
    m("alloc_zeroed", 1, Ck::Alloc, &["step2-core-rule-222"]),
    p("realloc", ALLOC_QUALS, Some(3), Ck::Realloc, &["step2-alloc-rule-1112"]),
    m("realloc", 3, Ck::Realloc, &["step2-core-rule-226"]),
    m("grow", 3, Ck::Grow, &["step2-core-rule-254"]),
    m("grow_zeroed", 3, Ck::Grow, &["step2-core-rule-264"]),
    m("shrink", 3, Ck::Shrink, &["step2-core-rule-273"]),
    // ranges, iterators and unchecked cursors
    p("new_unchecked", &["IndexRange"], Some(2), Ck::RangeNew, &["step2-core-rule-498"]),
    p(
        "new_unchecked",
        &["IntoIter"],
        Some(2),
        Ck::IntoIterNew,
        &["step2-core-rule-288", "step2-core-rule-289"],
    ),
    m("next_unchecked", 0, Ck::NextRange, &["step2-core-rule-499"]),
    m(
        "next_back_unchecked",
        0,
        Ck::NextBack,
        &[
            "step2-core-rule-500",
            "step2-alloc-rule-1181",
            "step2-alloc-rule-1183",
            "step2-alloc-rule-1187",
        ],
    ),
    m("deallocating_next_back_unchecked", 1, Ck::NextBtree, &["step2-alloc-rule-1187"]),
    m("push_unchecked", 1, Ck::PushUnchecked, &["step2-core-rule-291"]),
    // NUL-terminated buffers
    p("from_ptr", &["CStr"], Some(1), Ck::CStrBuf, &["step2-core-rule-321"]),
    p("strlen", &["", "libc"], Some(1), Ck::CStrBuf, &["step2-core-rule-324"]),
    // index guards that do not bound the index by a length
    m(
        "get_unchecked|get_unchecked_mut",
        1,
        Ck::GetWrong,
        &["step2-alloc-rule-1155", "step2-alloc-rule-1157"],
    ),
    m(
        "get_unchecked|get_unchecked_mut|add|offset",
        1,
        Ck::HoleIdx,
        &[
            "step2-alloc-rule-1155",
            "step2-alloc-rule-1156",
            "step2-alloc-rule-1157",
            "step2-alloc-rule-1158",
        ],
    ),
    p("new", &["Hole"], Some(2), Ck::HoleNew, &["step2-alloc-rule-1154"]),
    m("sift_up", 2, Ck::SiftUp, &["step2-alloc-rule-1150"]),
    m("sift_down_range", 2, Ck::SiftRange, &["step2-alloc-rule-1151"]),
    m("sift_down", 1, Ck::SiftOne, &["step2-alloc-rule-1152"]),
    m("sift_down_to_bottom", 1, Ck::SiftOne, &["step2-alloc-rule-1153"]),
    // btree nodes and slice helpers
    m("cast_to_leaf_unchecked", 0, Ck::NodeCast, &["step2-alloc-rule-1203"]),
    m("cast_to_internal_unchecked", 0, Ck::NodeCast, &["step2-alloc-rule-1204"]),
    p("from_new_internal", &["NodeRef", "Self"], Some(2), Ck::FromNewInternal, &["step2-alloc-rule-1191"]),
    p("slice_insert", &["*"], Some(3), Ck::SliceHelper, &["step2-alloc-rule-1212"]),
    p("slice_remove", &["*"], Some(2), Ck::SliceHelper, &["step2-alloc-rule-1214"]),
    p("slice_shl", &["*"], Some(2), Ck::SliceHelper, &["step2-alloc-rule-1215"]),
    p("slice_shr", &["*"], Some(2), Ck::SliceHelper, &["step2-alloc-rule-1216"]),
    m("swap_unchecked", 2, Ck::SwapUnchecked, &["step2-core-rule-972", "step2-core-rule-973"]),
    m("as_chunks_unchecked", 0, Ck::AsChunks, &["step2-core-rule-974", "step2-core-rule-975"]),
    m("as_chunks_unchecked_mut", 0, Ck::AsChunks, &["step2-core-rule-976", "step2-core-rule-977"]),
    // scalar preconditions that are comparisons
    p("assert_unchecked", &["hint", ""], Some(1), Ck::AssertUnchecked, &["step2-core-rule-344"]),
    m("to_int_unchecked", 0, Ck::ToInt, &["step2-core-rule-483", "step2-core-rule-486"]),
    p("from_u32_unchecked", &["char", ""], Some(1), Ck::CharU32, &["step2-core-rule-302"]),
    p("from_u32_unchecked", &["CodePoint"], Some(1), Ck::CodePoint, &["step2-std-rule-206"]),
    p("from_raw_parts", &["Vec"], Some(3), Ck::VecParts, &["step2-alloc-rule-1390"]),
    p("from_parts", &["Vec"], Some(3), Ck::VecParts, &["step2-alloc-rule-1399"]),
    p("from_raw_parts_in", &["Vec"], Some(4), Ck::VecParts, &["step2-alloc-rule-1408"]),
    p("from_parts_in", &["Vec"], Some(4), Ck::VecParts, &["step2-alloc-rule-1417"]),
    p("borrow_raw", &["BorrowedFd"], Some(1), Ck::BorrowRaw, &["step2-std-rule-22"]),
    m("slice_unchecked", 2, Ck::SliceU, &["step2-core-rule-1083", "step2-core-rule-1084"]),
    m("slice_mut_unchecked", 2, Ck::SliceU, &["step2-core-rule-1086", "step2-core-rule-1087"]),
    m("split_at_unchecked", 1, Ck::SplitAt, &["step2-core-rule-978"]),
    m("split_at_mut_unchecked", 1, Ck::SplitAt, &["step2-core-rule-979"]),
    p("u8to64_le", &["*"], Some(3), Ck::U8to64, &["step2-core-rule-342"]),
    p("small_slice_eq", &["*"], Some(2), Ck::SmallEq, &["step2-core-rule-1092"]),
];

const IDX_BTREE_TAGS: &[&str] = &[
    "step2-alloc-rule-1181",
    "step2-alloc-rule-1183",
    "step2-alloc-rule-1187",
];

const WORD_FILTER: &[&str] = &[
    "as", "usize", "isize", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128",
    "f32", "f64", "true", "false", "mut", "let", "return", "if", "else", "in", "ref",
];

/// Sanitisers that make a size or length non-zero or bounded regardless of the parameter.
const SANITIZERS: &[&str] = &["max", "min", "clamp", "NonZero", "checked_", "saturating_", "try_from"];

#[derive(Clone)]
enum LayoutSrc {
    /// Layout whose size derives from these parameter-derived identifiers.
    Size(Vec<String>),
    /// `Layout::new::<T>()` of a generic type parameter (may be zero-sized).
    Generic,
    /// A `Layout` parameter.
    Param(String),
}

struct Cmp {
    op: &'static str,
    left: HashSet<String>,
    right: HashSet<String>,
}

struct Cond {
    text: String,
    words: HashSet<String>,
    cmps: Vec<Cmp>,
}

pub(crate) fn analyze_fn(
    file: &str,
    caller: &str,
    inputs: &Punctuated<FnArg, syn::token::Comma>,
    generics: HashSet<String>,
    block: &syn::Block,
    info: &StructInfo,
    ctx: &Ctx,
    findings: &mut Vec<Finding>,
) {
    let mut checker = AstChecker {
        info,
        ctx,
        generics,
        method: caller.rsplit("::").next().unwrap_or(caller).to_string(),
        params: HashMap::new(),
        tainted: HashSet::new(),
        guarded: HashSet::new(),
        layouts: HashMap::new(),
        aliases: HashMap::new(),
        ptr_bufs: HashSet::new(),
        conds: Vec::new(),
        body_text: block.to_token_stream().to_string().to_lowercase(),
        unsafe_depth: 0,
        hits: Vec::new(),
    };
    for input in inputs {
        let FnArg::Typed(typed) = input else { continue };
        let ty = typed.ty.to_token_stream().to_string();
        let mut names = PatNames::default();
        syn::visit::Visit::visit_pat(&mut names, &typed.pat);
        for name in names.0 {
            if ty.ends_with("Layout") && !is_pointer_type(&typed.ty) {
                checker.layouts.insert(name.clone(), LayoutSrc::Param(name.clone()));
            }
            checker.params.insert(name.clone(), ty.clone());
            if !ty.contains("NonZero") {
                checker.tainted.insert(name);
            }
        }
    }
    checker.visit_block(block);
    for (line, callee, tags) in checker.hits {
        findings.push(Finding {
            file: file.to_string(),
            line,
            caller: caller.to_string(),
            callee,
            tags,
        });
    }
}

struct AstChecker<'a> {
    info: &'a StructInfo,
    ctx: &'a Ctx,
    generics: HashSet<String>,
    method: String,
    params: HashMap<String, String>,
    tainted: HashSet<String>,
    guarded: HashSet<String>,
    layouts: HashMap<String, LayoutSrc>,
    aliases: HashMap<String, Vec<String>>,
    ptr_bufs: HashSet<String>,
    conds: Vec<Cond>,
    body_text: String,
    unsafe_depth: usize,
    hits: Vec<(usize, String, String)>,
}

fn flatten(tokens: TokenStream, flat: &mut Vec<TokenTree>) {
    for tree in tokens {
        match tree {
            TokenTree::Group(group) => flatten(group.stream(), flat),
            other => flat.push(other),
        }
    }
}

/// Identifiers of a token stream; `self.f` is kept as `self.f` (and `f`), numbers are dropped.
fn words_of(tokens: TokenStream) -> HashSet<String> {
    let mut flat = Vec::new();
    flatten(tokens, &mut flat);
    let mut out = HashSet::new();
    let mut index = 0;
    while index < flat.len() {
        if let TokenTree::Ident(ident) = &flat[index] {
            let name = ident.to_string();
            if let (true, Some(TokenTree::Punct(dot)), Some(TokenTree::Ident(field))) =
                (name == "self", flat.get(index + 1), flat.get(index + 2))
            {
                if dot.as_char() == '.' {
                    out.insert(format!("self.{field}"));
                    out.insert(field.to_string());
                    index += 3;
                    continue;
                }
            }
            if !WORD_FILTER.contains(&name.as_str()) {
                out.insert(name);
            }
        }
        index += 1;
    }
    out
}

struct CmpCollector(Vec<Cmp>);

impl<'ast> Visit<'ast> for CmpCollector {
    fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
        let op = match node.op {
            BinOp::Lt(_) => Some("<"),
            BinOp::Le(_) => Some("<="),
            BinOp::Gt(_) => Some(">"),
            BinOp::Ge(_) => Some(">="),
            BinOp::Eq(_) => Some("=="),
            BinOp::Ne(_) => Some("!="),
            _ => None,
        };
        if let Some(op) = op {
            self.0.push(Cmp {
                op,
                left: words_of(node.left.to_token_stream()),
                right: words_of(node.right.to_token_stream()),
            });
        }
        visit::visit_expr_binary(self, node);
    }
}

fn is_relational(op: &str) -> bool {
    matches!(op, "<" | "<=" | ">" | ">=")
}

fn intersects(set: &HashSet<String>, ids: &[String]) -> bool {
    ids.iter().any(|id| set.contains(id))
}

fn strip_wrappers(expr: &Expr) -> &Expr {
    match expr {
        Expr::Paren(inner) => strip_wrappers(&inner.expr),
        Expr::Group(inner) => strip_wrappers(&inner.expr),
        Expr::Reference(inner) => strip_wrappers(&inner.expr),
        Expr::Cast(inner) => strip_wrappers(&inner.expr),
        Expr::Try(inner) => strip_wrappers(&inner.expr),
        _ => expr,
    }
}

impl AstChecker<'_> {
    fn ids(&self, expr: &Expr) -> Vec<String> {
        let mut collector = IdentCollector { info: self.info, out: Vec::new() };
        collector.visit_expr(expr);
        collector.out
    }

    fn is_tainted(&self, id: &str) -> bool {
        self.tainted.contains(id) || id.starts_with("self.")
    }

    fn tainted_ids(&self, expr: &Expr) -> Vec<String> {
        let text = expr.to_token_stream().to_string();
        if SANITIZERS.iter().any(|word| text.contains(word)) || text.contains('%') {
            return Vec::new();
        }
        self.ids(expr).into_iter().filter(|id| self.is_tainted(id)).collect()
    }

    fn mention(&self, ids: &[String]) -> bool {
        ids.iter().any(|id| self.guarded.contains(id))
    }

    fn cond_has(&self, needles: &[&str]) -> bool {
        self.conds.iter().any(|cond| needles.iter().any(|needle| cond.text.contains(needle)))
    }

    /// Relational comparison of one of `ids` against a non-literal bound.
    fn bound_sides(&self, ids: &[String]) -> Vec<&HashSet<String>> {
        let mut out = Vec::new();
        for cond in &self.conds {
            for cmp in cond.cmps.iter().filter(|cmp| is_relational(cmp.op)) {
                if intersects(&cmp.left, ids) && !cmp.right.is_empty() {
                    out.push(&cmp.right);
                }
                if intersects(&cmp.right, ids) && !cmp.left.is_empty() {
                    out.push(&cmp.left);
                }
            }
        }
        out
    }

    /// Bounds of every relational comparison involving one of `ids` (empty set = literal). When the
    /// compared side is itself an expression (`len - i >= 2`) its other words are the bound.
    fn relational_sides(&self, ids: &[String]) -> Vec<HashSet<String>> {
        let mut out = Vec::new();
        for cond in &self.conds {
            for cmp in cond.cmps.iter().filter(|cmp| is_relational(cmp.op)) {
                for (own, other) in [(&cmp.left, &cmp.right), (&cmp.right, &cmp.left)] {
                    if !intersects(own, ids) {
                        continue;
                    }
                    let rest: HashSet<String> =
                        own.iter()
                            .filter(|word| {
                                !ids.contains(word) && !ids.contains(&format!("self.{word}"))
                            })
                            .cloned()
                            .collect();
                    out.push(if rest.is_empty() { other.clone() } else { rest });
                }
            }
        }
        out
    }

    fn bound_guard(&self, ids: &[String]) -> bool {
        !self.bound_sides(ids).is_empty()
    }

    /// A comparison (or a condition) relating a value of `a` with a value of `b`.
    fn pair_guard(&self, a: &[String], b: &[String]) -> bool {
        self.conds.iter().any(|cond| {
            cond.cmps.iter().any(|cmp| {
                (intersects(&cmp.left, a) && intersects(&cmp.right, b))
                    || (intersects(&cmp.left, b) && intersects(&cmp.right, a))
            }) || (intersects(&cond.words, a) && intersects(&cond.words, b))
        })
    }

    /// A bound made only of public fields of `self`, which safe code can set to any value.
    fn weak_bound(&self, side: &HashSet<String>) -> bool {
        side.iter().any(|word| word.starts_with("self."))
            && side.iter().all(|word| {
                let field = word.strip_prefix("self.").unwrap_or(word);
                self.info.pub_fields.contains(field)
            })
    }

    fn add_cond(&mut self, expr: &Expr) {
        let tokens = expr.to_token_stream();
        let mut collector = CmpCollector(Vec::new());
        collector.visit_expr(expr);
        let ids = self.ids(expr);
        self.guard(ids);
        self.conds.push(Cond {
            text: tokens.to_string().to_lowercase(),
            words: words_of(tokens),
            cmps: collector.0,
        });
    }

    /// Marks identifiers as guarded, including the identifiers local aliases were computed from.
    fn guard(&mut self, ids: Vec<String>) {
        let mut pending = ids;
        while let Some(id) = pending.pop() {
            if self.guarded.insert(id.clone()) {
                if let Some(sources) = self.aliases.get(&id) {
                    pending.extend(sources.iter().cloned());
                }
            }
        }
    }

    fn add_opaque(&mut self, args: &[&Expr]) {
        for arg in args {
            let ids = self.ids(arg);
            self.guard(ids);
        }
        let tokens: TokenStream = args.iter().map(|arg| arg.to_token_stream()).collect();
        let words = words_of(tokens.clone());
        // A validating helper (`assert_in_bounds(i, ..)`) is taken as a bound check on its arguments.
        let cmps = vec![Cmp { op: "<", left: words.clone(), right: HashSet::from(["len".to_string()]) }];
        self.conds.push(Cond { text: tokens.to_string().to_lowercase(), words, cmps });
    }

    fn size_checked(&self) -> bool {
        self.cond_has(&["size_of", "size_of_val", "is_zst", "size ()", "needs_drop"])
    }

    fn layout_class(&self, expr: &Expr) -> Option<LayoutSrc> {
        match expr {
            Expr::Paren(_) | Expr::Group(_) | Expr::Reference(_) | Expr::Try(_) => {
                self.layout_class(strip_wrappers(expr))
            }
            Expr::MethodCall(call)
                if matches!(
                    call.method.to_string().as_str(),
                    "unwrap" | "expect" | "unwrap_unchecked" | "unwrap_or_else" | "unwrap_or"
                        | "map_err" | "ok" | "ok_or" | "ok_or_else" | "pad_to_align"
                ) =>
            {
                self.layout_class(&call.receiver)
            }
            Expr::Call(call) => {
                let Expr::Path(func) = call.func.as_ref() else { return None };
                let segments: Vec<_> = func.path.segments.iter().collect();
                let (last, prefix) = segments.split_last()?;
                if prefix.last()?.ident != "Layout" {
                    return None;
                }
                match last.ident.to_string().as_str() {
                    "from_size_align_unchecked" | "from_size_align" | "from_size" | "array" => {
                        let ids = self.tainted_ids(call.args.first()?);
                        (!ids.is_empty()).then_some(LayoutSrc::Size(ids))
                    }
                    "new" => {
                        let syn::PathArguments::AngleBracketed(args) = &last.arguments else {
                            return None;
                        };
                        let syn::GenericArgument::Type(syn::Type::Path(ty)) = args.args.first()?
                        else {
                            return None;
                        };
                        let name = ty.path.get_ident()?.to_string();
                        self.generics.contains(&name).then_some(LayoutSrc::Generic)
                    }
                    _ => None,
                }
            }
            Expr::Path(path) => self.layouts.get(&path.path.get_ident()?.to_string()).cloned(),
            _ => None,
        }
    }

    fn layout_ids(&self, expr: &Expr) -> Vec<String> {
        let mut ids = self.ids(expr);
        if let Some(LayoutSrc::Size(size)) = self.layout_class(expr) {
            ids.extend(size);
        }
        if let Expr::Path(path) = strip_wrappers(expr) {
            if let Some(ident) = path.path.get_ident() {
                ids.push(ident.to_string());
            }
        }
        ids
    }

    fn layout_may_be_zero(&self, class: &LayoutSrc) -> bool {
        match class {
            LayoutSrc::Size(ids) => !self.mention(ids) && !self.size_checked(),
            LayoutSrc::Generic => !self.size_checked(),
            LayoutSrc::Param(name) => !self.mention(std::slice::from_ref(name)),
        }
    }

    /// Receiver is `self`, a field of `self`, or a parameter (not a local built in the function).
    fn rooted(&self, expr: &Expr) -> bool {
        match strip_wrappers(expr) {
            Expr::Path(path) => path.path.get_ident().is_some_and(|ident| {
                ident == "self" || self.params.contains_key(&ident.to_string())
            }),
            Expr::Field(field) => self.rooted(&field.base),
            Expr::MethodCall(call) => {
                matches!(call.method.to_string().as_str(), "borrow_mut" | "as_mut" | "lock" | "unwrap")
                    && self.rooted(&call.receiver)
            }
            _ => false,
        }
    }

    /// Whether `expr` takes a pointer from a byte buffer that is a parameter or pub field.
    fn buffer_pointer(&self, expr: &Expr) -> bool {
        struct Finder<'a, 'b>(&'a AstChecker<'b>, bool);
        impl<'ast> Visit<'ast> for Finder<'_, '_> {
            fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
                let name = node.method.to_string();
                if matches!(name.as_str(), "as_ptr" | "as_mut_ptr") {
                    let mut receiver = strip_wrappers(&node.receiver);
                    while let Expr::MethodCall(inner) = receiver {
                        if !matches!(
                            inner.method.to_string().as_str(),
                            "as_bytes" | "as_str" | "as_slice" | "to_bytes"
                        ) {
                            break;
                        }
                        receiver = strip_wrappers(&inner.receiver);
                    }
                    let param = matches!(receiver, Expr::Path(path) if path.path.get_ident().is_some_and(|ident| {
                        self.0.params.get(&ident.to_string()).is_some_and(|ty| {
                            !["CStr", "CString", "*", "NonNull", "Ptr"].iter().any(|bad| ty.contains(bad))
                        })
                    }));
                    let field = matches!(receiver, Expr::Field(field)
                        if matches!(field.base.as_ref(), Expr::Path(base) if base.path.is_ident("self"))
                            && matches!(&field.member, Member::Named(name) if self.0.info.pub_fields.contains(&name.to_string())));
                    if param || field {
                        self.1 = true;
                    }
                }
                visit::visit_expr_method_call(self, node);
            }

            fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
                if let Some(ident) = node.path.get_ident() {
                    if self.0.ptr_bufs.contains(&ident.to_string()) {
                        self.1 = true;
                    }
                }
            }
        }
        let mut finder = Finder(self, false);
        finder.visit_expr(expr);
        finder.1
    }

    fn eval(&self, cp: &Cp, recv: Option<&Expr>, args: &[&Expr], turbofish: &str) -> Vec<usize> {
        let tainted = |expr: &Expr| self.tainted_ids(expr);
        match cp.ck {
            Ck::Alloc => {
                let fires = self
                    .layout_class(args[0])
                    .is_some_and(|class| self.layout_may_be_zero(&class));
                if fires { vec![0] } else { vec![] }
            }
            Ck::Realloc => {
                let layout_like = self.layout_class(args[1]).is_some()
                    || args[1].to_token_stream().to_string().to_lowercase().contains("layout");
                let ids = tainted(args[2]);
                if layout_like && !ids.is_empty() && !self.mention(&ids) { vec![0] } else { vec![] }
            }
            Ck::Grow | Ck::Shrink => {
                let (old, new) = (args[1], args[2]);
                let Some(class) = self.layout_class(new) else { return vec![] };
                if matches!(class, LayoutSrc::Generic) {
                    return vec![];
                }
                let (old_ids, new_ids) = (self.layout_ids(old), self.layout_ids(new));
                let derived = cp.ck == Ck::Grow && intersects(&new_ids.iter().cloned().collect(), &old_ids);
                if derived || self.pair_guard(&old_ids, &new_ids) { vec![] } else { vec![0] }
            }
            Ck::RangeNew => {
                let (a, b) = (tainted(args[0]), tainted(args[1]));
                if (a.is_empty() && b.is_empty()) || self.pair_guard(&self.ids(args[0]), &self.ids(args[1])) {
                    vec![]
                } else {
                    vec![0]
                }
            }
            Ck::IntoIterNew => {
                let mut out = Vec::new();
                match strip_wrappers(args[1]) {
                    Expr::Range(range) => {
                        let (Some(start), Some(end)) = (&range.start, &range.end) else {
                            return out;
                        };
                        let (sa, sb) = (tainted(start), tainted(end));
                        if sa.is_empty() && sb.is_empty() {
                            return out;
                        }
                        if !self.pair_guard(&self.ids(start), &self.ids(end)) {
                            out.push(0);
                        }
                        if !sb.is_empty() && !self.bound_guard(&sb) {
                            out.push(1);
                        }
                    }
                    other => {
                        let ids = tainted(other);
                        if !ids.is_empty() && !self.mention(&ids) {
                            out = vec![0, 1];
                        }
                    }
                }
                out
            }
            Ck::NextRange | Ck::NextBack | Ck::NextBtree => {
                let words = ["start", "end", "len", "is_empty", "idx", "remaining", "empty", "exhaust"];
                if self.cond_has(&words) || self.conds.iter().any(|cond| !cond.cmps.is_empty()) {
                    return vec![];
                }
                if cp.ck == Ck::NextBack && self.ctx.idx_struct {
                    (0..cp.tags.len()).collect()
                } else {
                    vec![0]
                }
            }
            Ck::PushUnchecked => {
                if !recv.is_some_and(|r| self.rooted(r))
                    || self.cond_has(&["len", "cap", "full", "remaining", "space"])
                    || ["reserve", "with_capacity", "capacity"].iter().any(|w| self.body_text.contains(w))
                {
                    vec![]
                } else {
                    vec![0]
                }
            }
            Ck::CStrBuf => {
                let nul = ["\\0", "nul", "memchr", "cstring", "cstr::from_bytes", "with_nul", "push (0", "== 0", "!= 0", "(& 0)"];
                if self.buffer_pointer(args[0]) && !nul.iter().any(|w| self.body_text.contains(w)) {
                    vec![0]
                } else {
                    vec![]
                }
            }
            Ck::GetWrong => {
                let ids = tainted(args[0]);
                if ids.is_empty() {
                    return vec![];
                }
                let sides = self.relational_sides(&ids);
                if !sides.is_empty() && sides.iter().all(|side| side.is_empty() || self.weak_bound(side)) {
                    vec![0, 1]
                } else {
                    vec![]
                }
            }
            Ck::HoleIdx => {
                let fields = &self.info.fields;
                if !(fields.contains("pos") && fields.contains("elt")) {
                    return vec![];
                }
                let ids = tainted(args[0]);
                if ids.is_empty() {
                    return vec![];
                }
                let base = if self.method.contains("move") { 2 } else { 0 };
                let mut out = Vec::new();
                if !self.bound_guard(&ids) {
                    out.push(base);
                }
                if !self.pair_guard(&ids, &["pos".to_string(), "self.pos".to_string()]) {
                    out.push(base + 1);
                }
                out
            }
            Ck::HoleNew | Ck::SiftUp => {
                let ids = tainted(args[1]);
                if !ids.is_empty() && !self.bound_guard(&ids) { vec![0] } else { vec![] }
            }
            Ck::SiftOne => {
                let ids = tainted(args[0]);
                if !ids.is_empty() && !self.bound_guard(&ids) { vec![0] } else { vec![] }
            }
            Ck::SiftRange => {
                let (a, b) = (tainted(args[0]), tainted(args[1]));
                if a.is_empty() && b.is_empty() {
                    return vec![];
                }
                if !self.pair_guard(&self.ids(args[0]), &self.ids(args[1])) || !self.bound_guard(&b) {
                    vec![0]
                } else {
                    vec![]
                }
            }
            Ck::NodeCast => {
                if self.cond_has(&["height", "force", "is_leaf"]) { vec![] } else { vec![0] }
            }
            Ck::FromNewInternal => {
                let ids = tainted(args[1]);
                if !ids.is_empty() && !self.mention(&ids) { vec![0] } else { vec![] }
            }
            Ck::SliceHelper => {
                let ids = tainted(args[1]);
                if !ids.is_empty() && !self.bound_guard(&ids) { vec![0] } else { vec![] }
            }
            Ck::SwapUnchecked => (0..2)
                .filter(|&i| {
                    let ids = tainted(args[i]);
                    !ids.is_empty() && !self.bound_guard(&ids)
                })
                .collect(),
            Ck::AsChunks => {
                let mut out = Vec::new();
                if !self.cond_has(&["%"]) {
                    out.push(0);
                }
                let numeric = turbofish.chars().all(|c| c.is_ascii_digit() || c == ' ');
                let names = words_of(turbofish.parse::<TokenStream>().unwrap_or_default());
                if !numeric && !self.conds.iter().any(|cond| intersects(&cond.words, &names.iter().cloned().collect::<Vec<_>>())) {
                    out.push(1);
                }
                out
            }
            Ck::AssertUnchecked => {
                let ids = tainted(args[0]);
                if !ids.is_empty() && !self.mention(&ids) { vec![0] } else { vec![] }
            }
            Ck::ToInt => {
                let ids = recv.map(|r| tainted(r)).unwrap_or_default();
                if !ids.is_empty()
                    && !self.mention(&ids)
                    && !self.cond_has(&["is_nan", "is_finite", "clamp"])
                {
                    vec![0, 1]
                } else {
                    vec![]
                }
            }
            Ck::CharU32 | Ck::CodePoint => {
                let ids = tainted(args[0]);
                if ids.is_empty() || !self.mention(&ids) || self.cond_has(&["from_u32"]) {
                    return vec![];
                }
                let upper = self.cond_has(&["10ffff", "110000", "1114111", "1_114_111", "char :: max", "0x10_ffff"]);
                let surrogate = self.cond_has(&["d800", "dfff", "e000", "55296", "57343", "57344"]);
                if upper && (surrogate || cp.ck == Ck::CodePoint) { vec![] } else { vec![0] }
            }
            Ck::VecParts => {
                let (len, cap) = (self.ids(args[1]), self.ids(args[2]));
                let mixed: Vec<String> = len.iter().chain(cap.iter()).cloned().collect();
                let any_taint = mixed.iter().any(|id| self.is_tainted(id));
                let same = !len.is_empty() && len == cap;
                if len.is_empty() || cap.is_empty() || !any_taint || same || self.pair_guard(&len, &cap) {
                    vec![]
                } else {
                    vec![0]
                }
            }
            Ck::BorrowRaw => {
                let ids = tainted(args[0]);
                if !ids.is_empty() && !self.mention(&ids) { vec![0] } else { vec![] }
            }
            Ck::SliceU => {
                let (a, b) = (tainted(args[0]), tainted(args[1]));
                if a.is_empty() && b.is_empty() {
                    return vec![];
                }
                let mut out = Vec::new();
                if !self.pair_guard(&self.ids(args[0]), &self.ids(args[1])) {
                    out.push(0);
                }
                if !self.bound_guard(&b) {
                    out.push(1);
                }
                out
            }
            Ck::SplitAt => {
                let ids = tainted(args[0]);
                if !ids.is_empty() && !self.bound_guard(&ids) { vec![0] } else { vec![] }
            }
            Ck::U8to64 => {
                let mut ids = tainted(args[1]);
                ids.extend(tainted(args[2]));
                if !ids.is_empty() && !self.bound_guard(&ids) { vec![0] } else { vec![] }
            }
            Ck::SmallEq => {
                let any = args.iter().any(|arg| !tainted(arg).is_empty());
                if any && !self.cond_has(&["len"]) { vec![0] } else { vec![] }
            }
        }
    }

    fn matches(&self, cp: &Cp, name: &str, qualifier: Option<&str>, nargs: usize, method: bool) -> bool {
        if cp.method != method
            || !cp.names.split('|').any(|n| n == name)
            || cp.nargs.is_some_and(|n| n != nargs)
        {
            return false;
        }
        method
            || cp.quals.iter().any(|&q| match qualifier {
                None => q.is_empty() || q == "*",
                Some(actual) => q == "*" || q == actual,
            })
    }

    fn fire(&mut self, cp: &Cp, line: usize, callee: String, indices: Vec<usize>) {
        if indices.is_empty() {
            return;
        }
        let tags: Vec<&str> = indices.iter().filter_map(|&i| cp.tags.get(i).copied()).collect();
        self.hits.push((line, callee, tags.join(",")));
    }
}

fn macro_exprs(tokens: TokenStream) -> Vec<Expr> {
    use syn::parse::Parser;
    Punctuated::<Expr, syn::Token![,]>::parse_terminated
        .parse2(tokens)
        .map(|list| list.into_iter().collect())
        .unwrap_or_default()
}

impl<'ast> Visit<'ast> for AstChecker<'_> {
    fn visit_expr_unsafe(&mut self, node: &'ast syn::ExprUnsafe) {
        self.unsafe_depth += 1;
        visit::visit_expr_unsafe(self, node);
        self.unsafe_depth -= 1;
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        visit::visit_local(self, node);
        let Some(init) = &node.init else { return };
        let mut names = PatNames::default();
        syn::visit::Visit::visit_pat(&mut names, &node.pat);
        if init.diverge.is_some() {
            self.add_cond(&init.expr);
        }
        let sources = self.ids(&init.expr);
        for name in &names.0 {
            self.aliases.insert(name.clone(), sources.clone());
        }
        if let Some(class) = self.layout_class(&init.expr) {
            for name in &names.0 {
                self.layouts.insert(name.clone(), class.clone());
            }
        }
        if self.buffer_pointer(&init.expr) {
            self.ptr_bufs.extend(names.0.iter().cloned());
        }
        let ids = self.ids(&init.expr);
        if !ids.iter().any(|id| self.is_tainted(id)) {
            return;
        }
        let text = init.expr.to_token_stream().to_string();
        let sanitized = text.contains('%') || SANITIZERS.iter().any(|word| text.contains(word));
        for name in names.0 {
            if sanitized {
                self.guarded.insert(name);
            } else {
                self.tainted.insert(name);
            }
        }
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.add_cond(&node.cond);
        visit::visit_expr_if(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.add_cond(&node.cond);
        visit::visit_expr_while(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.add_cond(&node.expr);
        visit::visit_expr_match(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let name = node.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
        if !is_guard_name(&name) {
            return;
        }
        let exprs = macro_exprs(node.tokens.clone());
        match (name.as_str(), exprs.as_slice()) {
            ("assert_eq" | "assert_ne", [a, b, ..]) => {
                let cond: Expr = syn::parse_quote!((#a) == (#b));
                self.add_cond(&cond);
            }
            ("assert", [first, ..]) => self.add_cond(first),
            _ => {
                let refs: Vec<&Expr> = exprs.iter().collect();
                if refs.is_empty() {
                    let mut ids = Vec::new();
                    token_idents(node.tokens.clone(), self.info, &mut ids);
                    self.guarded.extend(ids);
                } else {
                    self.add_opaque(&refs);
                }
            }
        }
    }

    fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
        if matches!(node.op, BinOp::And(_)) {
            self.add_cond(&node.left);
        }
        // `self.idx - 1` on a pub field without `idx > 0` (btree Handle::next_back_unchecked).
        if self.unsafe_depth > 0 && matches!(node.op, BinOp::Sub(_)) {
            if let (Expr::Field(field), Expr::Lit(lit)) = (node.left.as_ref(), node.right.as_ref()) {
                let is_self = matches!(field.base.as_ref(), Expr::Path(p) if p.path.is_ident("self"));
                if let (true, Member::Named(member), syn::Lit::Int(int)) = (is_self, &field.member, &lit.lit) {
                    let id = format!("self.{member}");
                    if member == "idx"
                        && int.base10_digits() == "1"
                        && self.info.pub_fields.contains("idx")
                        && !self.guarded.contains(&id)
                    {
                        self.hits.push((
                            node.span().start().line,
                            "self.idx - 1".to_string(),
                            IDX_BTREE_TAGS.join(","),
                        ));
                    }
                }
            }
        }
        visit::visit_expr_binary(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let name = node.method.to_string();
        let args: Vec<&Expr> = node.args.iter().collect();
        if is_guard_name(&name) {
            self.add_opaque(&args);
        } else if matches!(name.as_str(), "then" | "then_some") {
            self.add_cond(&node.receiver);
        } else if matches!(name.as_str(), "get" | "get_mut" | "contains" | "contains_key") {
            let ids: Vec<String> = args.iter().flat_map(|arg| self.ids(arg)).collect();
            self.guard(ids);
            let words: HashSet<String> = args.iter().flat_map(|arg| words_of(arg.to_token_stream())).collect();
            self.conds.push(Cond {
                text: String::new(),
                words: words.clone(),
                cmps: vec![Cmp { op: "<", left: words, right: HashSet::from(["len".to_string()]) }],
            });
        }
        if self.unsafe_depth > 0 {
            let line = node.method.span().start().line;
            let turbofish = node
                .turbofish
                .as_ref()
                .map(|t| t.args.to_token_stream().to_string())
                .unwrap_or_default();
            for cp in CPS {
                if self.matches(cp, &name, None, args.len(), true) {
                    let indices = self.eval(cp, Some(&node.receiver), &args, &turbofish);
                    self.fire(cp, line, format!(".{name}"), indices);
                }
            }
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        let args: Vec<&Expr> = node.args.iter().collect();
        if let Expr::Path(func) = node.func.as_ref() {
            let segments: Vec<String> =
                func.path.segments.iter().map(|s| s.ident.to_string()).collect();
            if let Some((name, prefix)) = segments.split_last() {
                if is_guard_name(name) {
                    self.add_opaque(&args);
                }
                if self.unsafe_depth > 0 {
                    let qualifier = prefix.last().map(String::as_str);
                    for cp in CPS {
                        if self.matches(cp, name, qualifier, args.len(), false) {
                            let indices = self.eval(cp, None, &args, "");
                            self.fire(cp, func.span().start().line, segments.join("::"), indices);
                        }
                    }
                }
            }
        }
        visit::visit_expr_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use crate::{analyze_file_family, Family};
    use std::path::Path;

    fn scan(source: &str) -> Vec<(String, String, String)> {
        let parsed = syn::parse_file(source).unwrap();
        analyze_file_family(Path::new("sample.rs"), &parsed, Family::Ast)
            .into_iter()
            .map(|f| (f.caller, f.callee, f.tags))
            .collect()
    }

    fn callers(source: &str) -> Vec<String> {
        scan(source).into_iter().map(|(caller, _, _)| caller).collect()
    }

    #[test]
    fn alloc_layout_size_may_be_zero() {
        let hits = scan(
            r#"
use std::alloc::{alloc, Layout};
pub fn bad(n: usize) -> *mut u8 { unsafe { alloc(Layout::from_size_align_unchecked(n, 8)) } }
pub fn arr<T>(n: usize) -> *mut u8 { let l = Layout::array::<T>(n).unwrap(); unsafe { alloc(l) } }
pub fn zst<T>() -> *mut u8 { unsafe { std::alloc::alloc(Layout::new::<T>()) } }
pub fn from_layout(l: Layout) -> *mut u8 { unsafe { std::alloc::alloc_zeroed(l) } }
pub fn guarded(n: usize) -> *mut u8 { if n == 0 { return std::ptr::null_mut(); } unsafe { alloc(Layout::from_size_align_unchecked(n, 8)) } }
pub fn sized<T>() -> *mut u8 { if std::mem::size_of::<T>() == 0 { return std::ptr::null_mut(); } unsafe { alloc(Layout::new::<T>()) } }
pub fn fixed() -> *mut u8 { unsafe { alloc(Layout::new::<u64>()) } }
pub fn layout_guard(l: Layout) -> *mut u8 { if l.size() == 0 { return std::ptr::null_mut(); } unsafe { alloc(l) } }
"#,
        );
        let tags: Vec<_> = hits.iter().map(|(c, _, t)| (c.as_str(), t.as_str())).collect();
        assert_eq!(
            tags,
            vec![
                ("bad", "step2-alloc-rule-1106"),
                ("arr", "step2-alloc-rule-1106"),
                ("zst", "step2-alloc-rule-1106"),
                ("from_layout", "step2-alloc-rule-1120"),
            ]
        );
    }

    #[test]
    fn grow_needs_new_layout_compared_with_old() {
        let hits = callers(
            r#"
pub fn bad(a: &A, p: NonNull<u8>, old: Layout, n: usize) {
    let new = Layout::from_size_align(n, 8).unwrap();
    unsafe { a.grow(p, old, new) };
}
pub fn good(a: &A, p: NonNull<u8>, old: Layout, n: usize) {
    let new = Layout::from_size_align(n, 8).unwrap();
    if new.size() < old.size() { return; }
    unsafe { a.grow(p, old, new) };
}
pub fn shrink_bad(a: &A, p: NonNull<u8>, old: Layout, new: Layout) { unsafe { a.shrink(p, old, new) }; }
"#,
        );
        assert_eq!(hits, vec!["bad", "shrink_bad"]);
    }

    #[test]
    fn range_order_and_cursor_guards() {
        let hits = callers(
            r#"
pub fn bad(a: usize, b: usize) { unsafe { IndexRange::new_unchecked(a, b) }; }
pub fn good(a: usize, b: usize) { assert!(a <= b); unsafe { IndexRange::new_unchecked(a, b) }; }
pub fn iter_bad(buf: [u8; 4], a: usize, b: usize) { unsafe { IntoIter::new_unchecked(buf, a..b) }; }
impl S {
    pub fn next_bad(&mut self) -> usize { unsafe { self.r.next_unchecked() } }
    pub fn next_good(&mut self) -> usize { if self.r.start == self.r.end { return 0; } unsafe { self.r.next_unchecked() } }
    pub fn push_bad(&mut self, x: u8) { unsafe { self.push_unchecked(x) } }
    pub fn push_good(&mut self, x: u8) { if self.len < 4 { unsafe { self.push_unchecked(x) } } }
}
"#,
        );
        assert_eq!(hits, vec!["bad", "iter_bad", "S::next_bad", "S::push_bad"]);
    }

    #[test]
    fn index_compared_with_literal_or_pub_field_is_not_a_bound() {
        let hits = callers(
            r#"
pub struct S { pub len: usize, data: Vec<u8> }
impl S {
    pub fn literal(&self, i: usize) -> u8 { if i > 0 { unsafe { *self.data.get_unchecked(i) } } else { 0 } }
    pub fn field(&self, i: usize) -> u8 { if i < self.len { unsafe { *self.data.get_unchecked(i) } } else { 0 } }
    pub fn good(&self, i: usize) -> u8 { if i < self.data.len() { unsafe { *self.data.get_unchecked(i) } } else { 0 } }
    pub fn unguarded(&self, i: usize) -> u8 { unsafe { *self.data.get_unchecked(i) } }
    pub fn arithmetic(&self, i: usize) -> u8 { if self.data.len() - i >= 2 { unsafe { *self.data.get_unchecked(i) } } else { 0 } }
    pub fn zero(&self, i: usize) -> u8 { if self.len - i >= 2 { unsafe { *self.data.get_unchecked(i) } } else { 0 } }
}
"#,
        );
        assert_eq!(hits, vec!["S::literal", "S::field", "S::zero"]);
    }

    #[test]
    fn hole_like_and_btree_helpers() {
        let hits = scan(
            r#"
pub struct Hole<'a, T> { data: &'a mut [T], elt: T, pos: usize }
impl<'a, T> Hole<'a, T> {
    pub fn get(&self, index: usize) -> &T { unsafe { self.data.get_unchecked(index) } }
    pub fn move_to(&mut self, index: usize) { assert!(index < self.data.len()); unsafe { self.data.get_unchecked(index); } }
}
pub fn insert(s: &mut [u8], idx: usize) { unsafe { slice_insert(s, idx, 1) } }
pub fn insert_ok(s: &mut [u8], idx: usize) { assert!(idx < s.len()); unsafe { slice_insert(s, idx, 1) } }
"#,
        );
        let tags: Vec<_> = hits.iter().map(|(c, _, t)| (c.as_str(), t.as_str())).collect();
        assert_eq!(
            tags,
            vec![
                ("Hole::get", "step2-alloc-rule-1155,step2-alloc-rule-1156"),
                ("Hole::move_to", "step2-alloc-rule-1158"),
                ("insert", "step2-alloc-rule-1212"),
            ]
        );
    }

    #[test]
    fn cstr_buffer_and_scalar_checks() {
        let hits = callers(
            r#"
pub fn bad(buf: &[u8]) { unsafe { CStr::from_ptr(buf.as_ptr() as *const i8) }; }
pub fn good(buf: &[u8]) { if buf.last() != Some(&0) { return; } unsafe { CStr::from_ptr(buf.as_ptr() as *const i8) }; }
pub fn cs(s: &CStr) { unsafe { CStr::from_ptr(s.as_ptr()) }; }
pub fn ch_partial(x: u32) { if x < 0x110000 { unsafe { char::from_u32_unchecked(x) }; } }
pub fn ch_full(x: u32) { if char::from_u32(x).is_some() { unsafe { char::from_u32_unchecked(x) }; } }
pub fn vec_bad(p: *mut u8, len: usize, cap: usize) { unsafe { Vec::from_raw_parts(p, len, cap) }; }
pub fn vec_ok(p: *mut u8, len: usize, cap: usize) { assert!(len <= cap); unsafe { Vec::from_raw_parts(p, len, cap) }; }
pub fn fd(raw: i32) { unsafe { BorrowedFd::borrow_raw(raw) }; }
pub fn fd_ok(raw: i32) { if raw < 0 { return; } unsafe { BorrowedFd::borrow_raw(raw) }; }
pub fn float(x: f32) -> i32 { unsafe { x.to_int_unchecked::<i32>() } }
pub fn assume(i: usize) { unsafe { core::hint::assert_unchecked(i < 4) } }
pub fn split(s: &[u8], mid: usize) { unsafe { s.split_at_unchecked(mid) }; }
pub fn split_helper(s: &Bits, mid: usize) { s.assert_in_bounds(mid, 0..=s.len()); unsafe { s.split_at_unchecked(mid) }; }
pub fn split_then(s: &[u8], mid: usize) { (mid <= s.len()).then(|| unsafe { s.split_at_unchecked(mid) }); }
pub fn split_and(s: &[u8], mid: usize) { let _ = mid <= s.len() && unsafe { s.split_at_unchecked(mid) }.0.is_empty(); }
"#,
        );
        assert_eq!(
            hits,
            vec!["bad", "ch_partial", "vec_bad", "fd", "float", "assume", "split"]
        );
    }

    #[test]
    fn mem_family_output_has_no_tags() {
        let parsed = syn::parse_file("pub fn f(v: &[u8], i: usize) -> u8 { unsafe { *v.get_unchecked(i) } }").unwrap();
        let findings = analyze_file_family(Path::new("s.rs"), &parsed, Family::Mem);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].tags.is_empty());
    }
}
