use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_abi::FieldIdx;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{
    BasicBlock, Body, Location, Operand, Place, PlaceTy, ProjectionElem, Rvalue, StatementKind,
    TerminatorKind,
};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_middle::ty::TypeVisitableExt;
use rustc_span::Pos;
use rustc_span::Span;
use rustc_span::sym;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::vec;
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FnInfo {
    /// <mod>::<type>::<fn>
    pub name: String,
    pub path: String,
    pub line_start: usize, // signature line number
    pub line_end: usize,   // signature line number
    pub body_end: usize,   // line number of the closing brace of the function body
    #[serde(default)]
    pub require_template: bool,
    #[serde(default)]
    pub is_unsafe: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trait_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_ty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_match_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub call_chains: Vec<String>, // e.g. ["fn_a -> fn_b "], only for mutators across multiple struct functions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mut_ref_escape: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mut_ref_escape_fields: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub written_fields: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub return_self_fields: Vec<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructInfo {
    pub name: String,
    pub path: String,
    pub line_start: usize,
    pub body_end: usize,
    #[serde(default)]
    pub is_public: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldInfo {
    pub index: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldLayoutInfo {
    pub index: String,
    pub name: String,
    pub ty: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_ty: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub element_is_template: bool,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallsiteInfo {
    pub line: usize,
    pub col: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeLayoutControl {
    Fixed,
    External,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalleeTypeArgInfo {
    /// Zero-based index among the callee's type parameters (lifetimes and
    /// const parameters do not consume an index).
    pub index: usize,
    pub name: String,
    /// Whether this parameter is declared by an enclosing impl/type or by the
    /// function itself.
    pub owner: String,
    pub instantiated_ty: String,
    pub layout_control: TypeLayoutControl,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suspect {
    #[serde(alias = "target_fn_parent")]
    pub caller_parent: Option<StructInfo>,
    #[serde(alias = "target_fn")]
    pub caller: FnInfo,
    #[serde(alias = "unsafe_call")]
    pub callee: FnInfo,
    pub callsite: CallsiteInfo,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub callee_type_args: Vec<CalleeTypeArgInfo>,

    #[serde(skip)]
    pub unsafe_call_used_fields: Vec<String>,
    #[serde(skip)]
    pub unsafe_call_used_params: Vec<usize>, // parameter indices used in unsafe call
    #[serde(skip)]
    pub unsafe_call_used_globals: Vec<String>, // global variable names used in unsafe call
    #[serde(skip)]
    pub unsafe_call_control_fields: Vec<String>, // self fields that control whether unsafe call executes
    #[serde(skip)]
    pub unsafe_call_control_params: Vec<usize>, // params that control whether unsafe call executes
    #[serde(skip)]
    pub unsafe_call_control_globals: Vec<String>, // globals that control whether unsafe call executes
    #[serde(skip)]
    pub constructors: Vec<FnInfo>,
    #[serde(skip)]
    pub mutators: Vec<FnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInteractionInfo {
    #[serde(rename = "type")]
    pub ty: StructInfo,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub public_fields: Vec<FieldInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub field_layouts: Vec<FieldLayoutInfo>,
    pub constructors: Vec<FnInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observers: Vec<FnInfo>,
    pub mutators: Vec<FnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitMethodInfo {
    pub trait_name: String,
    pub method_name: String,
    pub implementor_type: String,
    pub return_ty: String,
    pub symbol_match_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscapedFieldInfo {
    pub source_path: String,
    pub wrapper_type: String,
    pub wrapper_field: String,
    pub target_type: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub target_is_template: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedFieldsInfo {
    pub name: String,
    pub path: String,
    pub line: usize,
    #[serde(default)]
    pub is_public: bool,
    #[serde(default)]
    pub is_unsafe: bool,
    pub root_types: Vec<String>,
    pub fields_written: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub escaped_fields: Vec<EscapedFieldInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub targets: Vec<Suspect>,
    #[serde(default)]
    pub types: Vec<TypeInteractionInfo>,
    #[serde(default)]
    pub trait_methods: Vec<TraitMethodInfo>,
    #[serde(default)]
    pub affected_fields: Vec<AffectedFieldsInfo>,
}

fn normalize_to_rust_relative(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    if let Some(index) = normalized.find("/rust/") {
        return normalized[index + 1..].to_string();
    }
    if normalized.starts_with("rust/") {
        return normalized;
    }
    normalized
}

fn source_relative_path(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    if let Ok(cwd) = std::env::current_dir() {
        let cwd = cwd.to_string_lossy().replace('\\', "/");
        if let Some(relative) = normalized.strip_prefix(&format!("{cwd}/")) {
            return relative.to_string();
        }
    }
    normalize_to_rust_relative(&normalized)
}

fn source_visible(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    let Some(local_def_id) = def_id.as_local() else {
        return false;
    };
    match tcx.hir_node_by_def_id(local_def_id) {
        rustc_hir::Node::Item(item) => source_span_is_visible(tcx, item.span),
        rustc_hir::Node::ImplItem(_) => {
            if source_span_is_visible(tcx, tcx.def_span(def_id)) {
                return true;
            }
            let trait_id =
                tcx.trait_id_of_impl(tcx.associated_item(def_id).container_id(tcx));
            trait_id.is_some_and(|trait_id| {
                let path = tcx.def_path_str(trait_id);
                path != "core::ops::drop::Drop" && !path.ends_with("::Drop")
            })
        }
        // Trait methods cannot spell a visibility and are part of the trait's
        // public interface.
        rustc_hir::Node::TraitItem(_) => true,
        _ => false,
    }
}

fn source_span_is_visible(tcx: TyCtxt<'_>, span: Span) -> bool {
    tcx.sess
        .source_map()
        .span_to_snippet(span)
        .ok()
        .is_some_and(|source| {
            // Item spans can include doc comments and attributes. Inspect
            // every line instead of requiring the snippet to begin with pub.
            source.lines().any(|line| {
                let line = line.trim_start();
                line.starts_with("pub ") || line.starts_with("pub(")
            })
        })
}

fn root_adt(ty: Ty<'_>) -> Option<(DefId, bool)> {
    match ty.kind() {
        ty::Adt(def, _) => Some((def.did(), false)),
        ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => match inner.kind() {
            ty::Adt(def, _) => Some((def.did(), true)),
            _ => None,
        },
        _ => None,
    }
}

fn contains_mut_ref(ty: Ty<'_>) -> bool {
    match ty.kind() {
        ty::Ref(_, _, rustc_middle::mir::Mutability::Mut)
        | ty::RawPtr(_, rustc_middle::mir::Mutability::Mut) => true,
        ty::Adt(_, args) => args
            .iter()
            .filter_map(|arg| arg.as_type())
            .any(contains_mut_ref),
        ty::Tuple(types) => types.iter().any(contains_mut_ref),
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AffectedPath {
    root: String,
    suffix: String,
}

impl AffectedPath {
    fn rendered(&self) -> String {
        format!("{}{}", self.root, self.suffix)
    }
}

struct AffectedFieldsVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'tcx Body<'tcx>,
    roots: HashMap<rustc_middle::mir::Local, AffectedPath>,
    aliases: HashMap<rustc_middle::mir::Local, AffectedPath>,
    written: HashSet<String>,
}

impl<'tcx> AffectedFieldsVisitor<'tcx> {
    fn path_for_place(&self, place: Place<'tcx>) -> Option<AffectedPath> {
        let mut path = self
            .aliases
            .get(&place.local)
            .or_else(|| self.roots.get(&place.local))?
            .clone();
        let mut place_ty = PlaceTy::from_ty(self.body.local_decls[place.local].ty);

        for elem in place.projection {
            match elem {
                ProjectionElem::Deref => {}
                ProjectionElem::Field(field, _) => {
                    let name = match place_ty.ty.kind() {
                        ty::Adt(def, _) => {
                            if let Some(index) = place_ty.variant_index {
                                def.variant(index).fields[field].name.to_string()
                            } else if !def.is_enum() {
                                def.non_enum_variant().fields[field].name.to_string()
                            } else {
                                field.index().to_string()
                            }
                        }
                        ty::Tuple(_) => field.index().to_string(),
                        _ => field.index().to_string(),
                    };
                    path.suffix.push('.');
                    path.suffix.push_str(&name);
                }
                ProjectionElem::Index(_) | ProjectionElem::ConstantIndex { .. } => {
                    path.suffix.push_str("[*]");
                }
                ProjectionElem::Subslice { .. } => path.suffix.push_str("[*]"),
                ProjectionElem::Downcast(_, _) => {}
                ProjectionElem::OpaqueCast(_)
                | ProjectionElem::Subtype(_)
                | ProjectionElem::UnwrapUnsafeBinder(_) => {}
            }
            place_ty = place_ty.projection_ty(self.tcx, elem);
        }
        Some(path)
    }

    fn path_for_operand(&self, operand: &Operand<'tcx>) -> Option<AffectedPath> {
        operand.place().and_then(|place| self.path_for_place(place))
    }

    fn alias_from_rvalue(&self, rvalue: &Rvalue<'tcx>) -> Option<AffectedPath> {
        match rvalue {
            Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) => self.path_for_place(*place),
            Rvalue::Use(operand) | Rvalue::Cast(_, operand, _) => self.path_for_operand(operand),
            Rvalue::CopyForDeref(place) => self.path_for_place(*place),
            _ => None,
        }
    }

    fn record(&mut self, place: Place<'tcx>) {
        if let Some(path) = self.path_for_place(place) {
            self.written.insert(path.rendered());
        }
    }

    fn collect_escaped_fields(&self) -> (Vec<EscapedFieldInfo>, HashSet<String>) {
        let mut aggregate_sources: HashMap<
            rustc_middle::mir::Local,
            Vec<(usize, AffectedPath)>,
        > = HashMap::new();

        for _ in 0..16 {
            let mut changed = false;
            for block in self.body.basic_blocks.iter() {
                for statement in &block.statements {
                    let StatementKind::Assign(assign) = &statement.kind else {
                        continue;
                    };
                    let (destination, rvalue) = &**assign;
                    if !destination.projection.is_empty() {
                        continue;
                    }
                    let sources = match rvalue {
                        Rvalue::Aggregate(_, operands) => {
                            let mut sources = Vec::new();
                            for (index, operand) in operands.iter().enumerate() {
                                let Some(place) = operand.place() else {
                                    continue;
                                };
                                let is_mut_ref = matches!(
                                    self.body.local_decls[place.local].ty.kind(),
                                    ty::Ref(_, _, rustc_middle::mir::Mutability::Mut)
                                        | ty::RawPtr(_, rustc_middle::mir::Mutability::Mut)
                                );
                                if !is_mut_ref {
                                    continue;
                                }
                                if let Some(path) = self.path_for_operand(operand) {
                                    sources.push((index, path));
                                }
                            }
                            sources
                        }
                        Rvalue::Use(operand) => {
                            let direct = operand.place().and_then(|place| {
                                let is_mut_ref = matches!(
                                    self.body.local_decls[place.local].ty.kind(),
                                    ty::Ref(_, _, rustc_middle::mir::Mutability::Mut)
                                        | ty::RawPtr(_, rustc_middle::mir::Mutability::Mut)
                                );
                                is_mut_ref
                                    .then(|| self.path_for_operand(operand))
                                    .flatten()
                                    .map(|path| vec![(0, path)])
                            });
                            direct
                                .or_else(|| {
                                    operand.place().and_then(|place| {
                                        aggregate_sources.get(&place.local).cloned()
                                    })
                                })
                                .unwrap_or_default()
                        }
                        Rvalue::Ref(
                            _,
                            rustc_middle::mir::BorrowKind::Mut { .. },
                            place,
                        )
                        | Rvalue::RawPtr(
                            rustc_middle::mir::RawPtrKind::Mut,
                            place,
                        ) => self
                            .path_for_place(*place)
                            .map(|path| vec![(0, path)])
                            .unwrap_or_default(),
                        _ => Vec::new(),
                    };
                    if !sources.is_empty()
                        && aggregate_sources.get(&destination.local) != Some(&sources)
                    {
                        aggregate_sources.insert(destination.local, sources);
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let return_local = rustc_middle::mir::Local::from_usize(0);
        let mut sources = aggregate_sources
            .get(&return_local)
            .cloned()
            .unwrap_or_default();
        if sources.is_empty()
            && contains_mut_ref(self.body.local_decls[return_local].ty)
        {
            if let Some(path) = self.aliases.get(&return_local) {
                sources.push((0, path.clone()));
            }
        }
        if sources.is_empty() {
            return (Vec::new(), HashSet::new());
        }
        let mut escaped_writes = HashSet::new();
        for (_, source) in &sources {
            let rendered = source.rendered();
            let wildcard = rendered
                .rfind("[*]")
                .map(|end| format!("{}.*", &rendered[..end + 3]))
                .unwrap_or_else(|| format!("{rendered}.*"));
            escaped_writes.insert(wildcard);
        }
        let return_ty = self.body.local_decls[return_local].ty;
        if let ty::Ref(_, inner, rustc_middle::mir::Mutability::Mut)
        | ty::RawPtr(inner, rustc_middle::mir::Mutability::Mut) = return_ty.kind()
        {
            let escaped = sources
                .into_iter()
                .map(|(_, source)| EscapedFieldInfo {
                    source_path: source.rendered(),
                    wrapper_type: String::new(),
                    wrapper_field: String::new(),
                    target_type: inner.to_string(),
                    target_is_template: inner.has_param(),
                })
                .collect();
            return (escaped, HashSet::new());
        }
        let ty::Adt(wrapper_def, wrapper_args) = return_ty.kind() else {
            return (Vec::new(), escaped_writes);
        };
        if !wrapper_def.is_struct() && !wrapper_def.is_enum() {
            return (Vec::new(), escaped_writes);
        }
        let wrapper_name = self.tcx.item_name(wrapper_def.did()).to_string();
        let mut escaped = Vec::new();
        for (index, source) in &sources {
            for variant in wrapper_def.variants() {
                if *index >= variant.fields.len() {
                    continue;
                }
                let field = &variant.fields[FieldIdx::from_usize(*index)];
                let field_ty = field.ty(self.tcx, wrapper_args);
                let target_ty = match field_ty.kind() {
                    ty::Ref(_, inner, rustc_middle::mir::Mutability::Mut)
                    | ty::RawPtr(inner, rustc_middle::mir::Mutability::Mut)
                        if matches!(inner.kind(), ty::Adt(_, _)) =>
                    {
                        Some(*inner)
                    }
                    _ => None,
                };
                let Some(target_ty) = target_ty else {
                    continue;
                };
                escaped.push(EscapedFieldInfo {
                    source_path: source.rendered(),
                    wrapper_type: wrapper_name.clone(),
                    wrapper_field: field.name.to_string(),
                    target_type: target_ty.to_string(),
                    target_is_template: target_ty.has_param(),
                });
            }
        }
        escaped.sort_by(|left, right| {
            (
                &left.source_path,
                &left.wrapper_type,
                &left.wrapper_field,
                &left.target_type,
            )
                .cmp(&(
                    &right.source_path,
                    &right.wrapper_type,
                    &right.wrapper_field,
                    &right.target_type,
                ))
        });
        escaped.dedup_by(|left, right| {
            left.source_path == right.source_path
                && left.wrapper_type == right.wrapper_type
                && left.wrapper_field == right.wrapper_field
                && left.target_type == right.target_type
        });
        (escaped, HashSet::new())
    }

    fn analyze(mut self) -> (HashSet<String>, Vec<EscapedFieldInfo>) {
        // Build simple local aliases to a fixed point before collecting writes,
        // since MIR block order need not be execution order.
        for _ in 0..16 {
            let mut changed = false;
            for block in self.body.basic_blocks.iter() {
                for statement in &block.statements {
                    if let StatementKind::Assign(assign) = &statement.kind {
                        let (destination, rvalue) = &**assign;
                        if destination.projection.is_empty() {
                            if let Some(path) = self.alias_from_rvalue(rvalue) {
                                if self.aliases.get(&destination.local) != Some(&path) {
                                    self.aliases.insert(destination.local, path);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
                if let TerminatorKind::Call {
                    func,
                    args,
                    destination,
                    ..
                } = &block.terminator().kind
                {
                    let Some((callee, _)) = func.const_fn_def() else {
                        continue;
                    };
                    let callee_name = self.tcx.def_path_str(callee);
                    let returns_element =
                        callee_name.contains("IndexMut") || callee_name.ends_with("::get_mut");
                    if returns_element {
                        if let Some(mut path) = args
                            .first()
                            .and_then(|arg| self.path_for_operand(&arg.node))
                        {
                            path.suffix.push_str("[*]");
                            if self.aliases.get(&destination.local) != Some(&path) {
                                self.aliases.insert(destination.local, path);
                                changed = true;
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }

        for block in self.body.basic_blocks.iter() {
            for statement in &block.statements {
                match &statement.kind {
                    StatementKind::Assign(assign)
                        if !(assign.0.projection.is_empty()
                            && self.alias_from_rvalue(&assign.1).is_some()) =>
                    {
                        self.record(assign.0)
                    }
                    StatementKind::Deinit(place) => self.record(**place),
                    StatementKind::SetDiscriminant { place, .. } => self.record(**place),
                    _ => {}
                }
            }
            if let TerminatorKind::Call { func, args, .. } = &block.terminator().kind {
                let Some((callee, _)) = func.const_fn_def() else {
                    continue;
                };
                let inputs = self.tcx.fn_sig(callee).skip_binder().skip_binder().inputs();
                if matches!(
                    inputs.first().map(|ty| ty.kind()),
                    Some(ty::Ref(_, _, rustc_middle::mir::Mutability::Mut))
                        | Some(ty::RawPtr(_, rustc_middle::mir::Mutability::Mut))
                ) {
                    if let Some(receiver) = args
                        .first()
                        .and_then(|arg| self.path_for_operand(&arg.node))
                    {
                        self.written.insert(receiver.rendered());
                    }
                }
            }
        }
        let (escaped, escaped_writes) = self.collect_escaped_fields();
        self.written.extend(escaped_writes);
        (self.written, escaped)
    }
}

fn collect_affected_fields<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<AffectedFieldsInfo> {
    let mut result = Vec::new();
    // Iterate item definitions rather than hir_body_owners. On recent rustc,
    // an associated function's body owner can map back to an expression HIR
    // node, which makes source visibility checks discard every public method.
    for local_def_id in tcx.hir_crate_items(()).definitions() {
        let def_id = local_def_id.to_def_id();
        let debug_affected = std::env::var_os("MIRSCAN_DEBUG_AFFECTED").is_some();
        let debug_name = tcx.def_path_str(def_id);
        let debug_this = debug_affected
            && (debug_name.ends_with("World::spawn_batch")
                || debug_name.ends_with("Entities::get_mut"));
        if !matches!(
            tcx.def_kind(def_id),
            rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn
        ) {
            continue;
        }
        if !source_visible(tcx, def_id) {
            if debug_this {
                eprintln!("affected-debug: {debug_name}: not source-visible");
            }
            continue;
        }
        let Some(body) = optimized_mir_if_available(tcx, def_id) else {
            if debug_this {
                eprintln!("affected-debug: {debug_name}: no MIR body");
            }
            continue;
        };
        let sig = tcx.fn_sig(def_id).skip_binder().skip_binder();
        let returned_adt = root_adt(sig.output()).map(|(def_id, _)| def_id);
        let mut roots = HashMap::new();
        let mut root_types = Vec::new();
        for (index, input) in sig.inputs().iter().enumerate() {
            let Some((adt_def_id, indirect)) = root_adt(*input) else {
                continue;
            };
            if !indirect && returned_adt != Some(adt_def_id) {
                continue;
            }
            let name = tcx.item_name(adt_def_id).to_string();
            root_types.push(name.clone());
            roots.insert(
                rustc_middle::mir::Local::from_usize(index + 1),
                AffectedPath {
                    root: name,
                    suffix: String::new(),
                },
            );
        }
        root_types.sort();
        root_types.dedup();
        if roots.is_empty() {
            if debug_this {
                eprintln!("affected-debug: {debug_name}: no root ADTs");
            }
            continue;
        }
        let (written, escaped_fields) = AffectedFieldsVisitor {
            tcx,
            body,
            roots,
            aliases: HashMap::new(),
            written: HashSet::new(),
        }
        .analyze();
        let mut fields_written: Vec<_> = written.into_iter().collect();
        fields_written.sort();
        if fields_written.is_empty() && escaped_fields.is_empty() {
            if debug_this {
                eprintln!("affected-debug: {debug_name}: no writes or escapes");
            }
            continue;
        }
        if debug_this {
            eprintln!(
                "affected-debug: {debug_name}: writes={fields_written:?} escapes={escaped_fields:?}"
            );
        }
        let span = tcx.def_span(def_id);
        let loc = tcx.sess.source_map().lookup_char_pos(span.lo());
        let name = tcx.def_path_str(def_id);
        result.push(AffectedFieldsInfo {
            name,
            path: source_relative_path(&loc.file.name.prefer_local().to_string()),
            line: loc.line,
            is_public: tcx.visibility(def_id).is_public(),
            is_unsafe: is_fn_unsafe(tcx, def_id),
            root_types,
            fields_written,
            escaped_fields,
        });
    }
    result.sort_by(|left, right| left.name.cmp(&right.name));
    result
}

// Visitor to find all unsafe function calls in a function body
struct UnsafeCallVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    unsafe_calls:
        Vec<(DefId, ty::GenericArgsRef<'tcx>, Span, Location, Vec<Place<'tcx>>)>,
}

#[derive(Clone)]
struct UnsafeCallSite<'tcx> {
    callee_def_id: DefId,
    callee_args: ty::GenericArgsRef<'tcx>,
    callsite_span: Span,
    location: Location,
    arg_places: Vec<Place<'tcx>>,
    depth: usize,
}

impl<'tcx> UnsafeCallVisitor<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            unsafe_calls: Vec::new(),
        }
    }
}

fn is_fn_unsafe(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    let sig = tcx.fn_sig(def_id).skip_binder().skip_binder();
    sig.safety == rustc_hir::Safety::Unsafe
}

fn is_core_or_std_fn(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    let path = tcx.def_path_str(def_id);
    path.starts_with("std::") || path.starts_with("core::")
}

fn configured_max_call_depth() -> usize {
    std::env::var("MIRSCAN_MAX_CALL_DEPTH")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1)
}

fn local_body_owner<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<LocalDefId> {
    // Some required trait methods have a HIR node that appears body-like due
    // to desugared signature constructs (for example `impl Trait` arguments),
    // but rustc deliberately has no MIR body for the method itself. Asking
    // `optimized_mir` for such a definition triggers an ICE in rustc.
    if !tcx.is_mir_available(def_id) {
        return None;
    }
    let local_def_id = def_id.as_local()?;
    tcx.hir_maybe_body_owned_by(local_def_id)?;
    Some(local_def_id)
}

fn optimized_mir_if_available<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<&'tcx Body<'tcx>> {
    local_body_owner(tcx, def_id).map(|local_def_id| tcx.optimized_mir(local_def_id))
}

fn span_with_body_if_available<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId, span: Span) -> Span {
    local_body_owner(tcx, def_id)
        .map(|local_def_id| {
            tcx.hir()
                .span_with_body(tcx.local_def_id_to_hir_id(local_def_id))
        })
        .unwrap_or(span)
}

fn collect_reachable_unsafe_calls<'tcx>(
    tcx: TyCtxt<'tcx>,
    root_def_id: DefId,
    max_call_depth: usize,
) -> Vec<UnsafeCallSite<'tcx>> {
    let mut results = Vec::new();
    let root_args = ty::GenericArgs::identity_for_item(tcx, root_def_id);
    let mut queue = VecDeque::from([(root_def_id, root_args, 0usize)]);
    let mut visited = HashSet::new();

    while let Some((current_def_id, current_args, depth)) = queue.pop_front() {
        let args_key = current_args
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>()
            .join(",");
        if !visited.insert((current_def_id, args_key, depth)) {
            continue;
        }

        let Some(body) = optimized_mir_if_available(tcx, current_def_id) else {
            continue;
        };

        let mut unsafe_visitor = UnsafeCallVisitor::new(tcx);
        unsafe_visitor.visit_body(body);
        for (callee_def_id, callee_args, callsite_span, location, arg_places) in
            unsafe_visitor.unsafe_calls
        {
            let callee_args =
                ty::EarlyBinder::bind(callee_args).instantiate(tcx, current_args);
            if is_core_or_std_fn(tcx, callee_def_id) {
                results.push(UnsafeCallSite {
                    callee_def_id,
                    callee_args,
                    callsite_span,
                    location,
                    arg_places,
                    depth,
                });
            } else if depth < max_call_depth && callee_def_id.as_local().is_some() {
                queue.push_back((callee_def_id, callee_args, depth + 1));
            }
        }
    }

    results
}

impl<'tcx> Visitor<'tcx> for UnsafeCallVisitor<'tcx> {
    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: Location,
    ) {
        if let TerminatorKind::Call { func, args, .. } = &terminator.kind {
            // Extract the called function's DefId
            if let Some((def_id, substs)) = func.const_fn_def() {
                // Check if the function is unsafe
                if is_fn_unsafe(self.tcx, def_id) {
                    // Extract all argument places (including receiver for method calls)
                    // In MIR, for method calls like "receiver.method(a, b)",
                    // args = [receiver, a, b], so receiver is already included
                    let mut arg_places: Vec<Place<'tcx>> = Vec::new();

                    for (idx, arg) in args.iter().enumerate() {
                        if let Some(place) = arg.node.place() {
                            arg_places.push(place);
                        } else if let Some(constant) = arg.node.constant() {
                            // Constants don't have places, skip them
                            continue;
                        }
                    }

                    self.unsafe_calls.push((
                        def_id,
                        substs,
                        terminator.source_info.span,
                        location,
                        arg_places,
                    ));
                }
            }
        }
        self.super_terminator(terminator, location);
    }
}

// Visitor to analyze control dependencies for unsafe calls
struct ControlDependencyVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'tcx Body<'tcx>,
    target_location: Location,
    self_local: rustc_middle::mir::Local,
    // Locals that appear in conditions controlling the unsafe call
    control_locals: HashSet<rustc_middle::mir::Local>,
    // Results
    pub control_self_fields: HashSet<String>,
    pub control_params: HashSet<usize>, // parameter indices
    pub control_globals: HashSet<DefId>,
}

impl<'tcx> ControlDependencyVisitor<'tcx> {
    fn new(
        tcx: TyCtxt<'tcx>,
        body: &'tcx Body<'tcx>,
        target_location: Location,
        self_local: rustc_middle::mir::Local,
    ) -> Self {
        Self {
            tcx,
            body,
            target_location,
            self_local,
            control_locals: HashSet::new(),
            control_self_fields: HashSet::new(),
            control_params: HashSet::new(),
            control_globals: HashSet::new(),
        }
    }

    fn analyze(&mut self) {
        // Find all basic blocks that could control whether we reach the target block
        let target_bb = self.target_location.block;

        // Simple approach: walk through all basic blocks up to target
        for (bb, bb_data) in self.body.basic_blocks.iter_enumerated() {
            if bb.index() >= target_bb.index() {
                break;
            }

            // Check if this block has a conditional terminator
            if let TerminatorKind::SwitchInt { discr, .. } = &bb_data.terminator().kind {
                // This is a conditional branch
                // Extract the discriminant (the value being tested)
                if let Some(place) = discr.place() {
                    self.control_locals.insert(place.local);
                }
            }
        }

        // Trace control locals back to their sources
        self.trace_locals_to_sources();
    }

    fn trace_locals_to_sources(&mut self) {
        let mut worklist: Vec<rustc_middle::mir::Local> =
            self.control_locals.iter().cloned().collect();
        let mut visited: HashSet<rustc_middle::mir::Local> = HashSet::new();

        while let Some(local) = worklist.pop() {
            if visited.contains(&local) {
                continue;
            }
            visited.insert(local);

            // Check if this is self
            if local == self.self_local {
                continue;
            }

            // Check if this is a parameter (parameters are locals 1..=n_args)
            let n_args = self.body.arg_count;
            if local.as_usize() > 0 && local.as_usize() <= n_args {
                self.control_params.insert(local.as_usize() - 1);
                continue;
            }

            // Find where this local is assigned
            for (_bb, bb_data) in self.body.basic_blocks.iter_enumerated() {
                for statement in &bb_data.statements {
                    if let StatementKind::Assign(assign) = &statement.kind {
                        let (place, rvalue) = &**assign;
                        if place.local == local {
                            // Found assignment to this local
                            self.extract_sources_from_rvalue(rvalue, &mut worklist);
                        }
                    }
                }
            }
        }
    }

    fn extract_sources_from_rvalue(
        &mut self,
        rvalue: &Rvalue<'tcx>,
        worklist: &mut Vec<rustc_middle::mir::Local>,
    ) {
        match rvalue {
            Rvalue::Use(operand)
            | Rvalue::Repeat(operand, _)
            | Rvalue::Cast(_, operand, _)
            | Rvalue::UnaryOp(_, operand) => {
                self.extract_sources_from_operand(operand, worklist);
            }
            Rvalue::Ref(_, _, place)
            | Rvalue::RawPtr(_, place)
            | Rvalue::Len(place)
            | Rvalue::CopyForDeref(place) => {
                self.extract_sources_from_place(*place, worklist);
            }
            Rvalue::BinaryOp(_, operands) => {
                self.extract_sources_from_operand(&operands.0, worklist);
                self.extract_sources_from_operand(&operands.1, worklist);
            }
            Rvalue::Aggregate(_, operands) => {
                for operand in operands.iter() {
                    self.extract_sources_from_operand(operand, worklist);
                }
            }
            _ => {}
        }
    }

    fn extract_sources_from_operand(
        &mut self,
        operand: &rustc_middle::mir::Operand<'tcx>,
        worklist: &mut Vec<rustc_middle::mir::Local>,
    ) {
        match operand {
            rustc_middle::mir::Operand::Move(place) | rustc_middle::mir::Operand::Copy(place) => {
                self.extract_sources_from_place(*place, worklist);
            }
            rustc_middle::mir::Operand::Constant(_constant) => {
                // Constants can reference static items, but we'll skip for now
            }
        }
    }

    fn extract_sources_from_place(
        &mut self,
        place: Place<'tcx>,
        worklist: &mut Vec<rustc_middle::mir::Local>,
    ) {
        // Check if from self with field projection
        if place.local == self.self_local {
            for elem in place.projection.iter() {
                if let rustc_middle::mir::ProjectionElem::Field(field, _) = elem {
                    self.control_self_fields
                        .insert(format!("{}", field.index()));
                }
            }
        } else {
            // Add to worklist for further tracing
            worklist.push(place.local);
        }
    }
}

// Visitor to extract data dependencies (self fields, parameters, globals) used in function arguments
struct DataDependencyVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'tcx Body<'tcx>,
    self_local: rustc_middle::mir::Local,
    // Map of locals to the sources they're derived from
    derived_from_self: HashMap<rustc_middle::mir::Local, HashSet<String>>,
    derived_from_params: HashMap<rustc_middle::mir::Local, HashSet<usize>>,
    derived_from_globals: HashMap<rustc_middle::mir::Local, HashSet<DefId>>,
    // Results
    pub self_fields: HashSet<String>,
    pub params: HashSet<usize>, // parameter indices
    pub globals: HashSet<DefId>,
}

fn extend_local_sources<T: Eq + Hash>(
    map: &mut HashMap<rustc_middle::mir::Local, HashSet<T>>,
    local: rustc_middle::mir::Local,
    values: HashSet<T>,
) -> bool {
    if values.is_empty() {
        return false;
    }
    let entry = map.entry(local).or_default();
    let before = entry.len();
    entry.extend(values);
    entry.len() != before
}

impl<'tcx> DataDependencyVisitor<'tcx> {
    fn new(
        tcx: TyCtxt<'tcx>,
        self_local: rustc_middle::mir::Local,
        body: &'tcx Body<'tcx>,
    ) -> Self {
        let mut visitor = Self {
            tcx,
            body,
            self_local,
            derived_from_self: HashMap::new(),
            derived_from_params: HashMap::new(),
            derived_from_globals: HashMap::new(),
            self_fields: HashSet::new(),
            params: HashSet::new(),
            globals: HashSet::new(),
        };

        // Build dataflow: which locals are derived from what sources
        visitor.analyze_dataflow();
        visitor
    }

    fn analyze_dataflow(&mut self) {
        // MIR commonly routes a returned field through several call
        // destinations (Index::index, HashMap::get, Try::branch, ...). Iterate
        // to a fixed point so those locals retain their originating self field.
        for _ in 0..16 {
            let mut changed = false;
            for (_bb, bb_data) in self.body.basic_blocks.iter_enumerated() {
                for statement in &bb_data.statements {
                    if let StatementKind::Assign(assign) = &statement.kind {
                        let (place, rvalue) = &**assign;
                        let mut self_fields = HashSet::new();
                        let mut params = HashSet::new();
                        let mut globals = HashSet::new();
                        self.collect_sources_from_rvalue(
                            rvalue,
                            &mut self_fields,
                            &mut params,
                            &mut globals,
                        );
                        changed |= extend_local_sources(
                            &mut self.derived_from_self,
                            place.local,
                            self_fields,
                        );
                        changed |= extend_local_sources(
                            &mut self.derived_from_params,
                            place.local,
                            params,
                        );
                        changed |= extend_local_sources(
                            &mut self.derived_from_globals,
                            place.local,
                            globals,
                        );
                    }
                }

                if let TerminatorKind::Call {
                    args, destination, ..
                } = &bb_data.terminator().kind
                {
                    let mut self_fields = HashSet::new();
                    let mut params = HashSet::new();
                    let mut globals = HashSet::new();
                    for arg in args {
                        self.collect_sources_from_operand(
                            &arg.node,
                            &mut self_fields,
                            &mut params,
                            &mut globals,
                        );
                    }
                    changed |= extend_local_sources(
                        &mut self.derived_from_self,
                        destination.local,
                        self_fields,
                    );
                    changed |= extend_local_sources(
                        &mut self.derived_from_params,
                        destination.local,
                        params,
                    );
                    changed |= extend_local_sources(
                        &mut self.derived_from_globals,
                        destination.local,
                        globals,
                    );
                }
            }
            if !changed {
                break;
            }
        }
    }

    fn collect_sources_from_rvalue(
        &self,
        rvalue: &Rvalue<'tcx>,
        self_fields: &mut HashSet<String>,
        params: &mut HashSet<usize>,
        globals: &mut HashSet<DefId>,
    ) {
        println!("  collect_sources_from_rvalue: {:?}", rvalue);
        match rvalue {
            Rvalue::Use(operand)
            | Rvalue::Repeat(operand, _)
            | Rvalue::Cast(_, operand, _)
            | Rvalue::UnaryOp(_, operand) => {
                println!("1");
                self.collect_sources_from_operand(operand, self_fields, params, globals);
            }
            Rvalue::Ref(_, _, place)
            | Rvalue::RawPtr(_, place)
            | Rvalue::Len(place)
            | Rvalue::CopyForDeref(place) => {
                println!("2");
                self.collect_sources_from_place(*place, self_fields, params, globals);
            }
            Rvalue::BinaryOp(_, operands) => {
                println!("3");
                self.collect_sources_from_operand(&operands.0, self_fields, params, globals);
                self.collect_sources_from_operand(&operands.1, self_fields, params, globals);
            }
            Rvalue::Aggregate(_, operands) => {
                println!("4");
                for operand in operands.iter() {
                    self.collect_sources_from_operand(operand, self_fields, params, globals);
                }
            }
            _ => {
                println!("5 - unhandled Rvalue kind");
            }
        }
    }

    fn collect_sources_from_operand(
        &self,
        operand: &rustc_middle::mir::Operand<'tcx>,
        self_fields: &mut HashSet<String>,
        params: &mut HashSet<usize>,
        globals: &mut HashSet<DefId>,
    ) {
        match operand {
            rustc_middle::mir::Operand::Move(place) | rustc_middle::mir::Operand::Copy(place) => {
                self.collect_sources_from_place(*place, self_fields, params, globals);
            }
            rustc_middle::mir::Operand::Constant(_constant) => {
                // Constants can reference static items, but we'll skip for now
            }
        }
    }

    fn collect_sources_from_place(
        &self,
        place: Place<'tcx>,
        self_fields: &mut HashSet<String>,
        params: &mut HashSet<usize>,
        globals: &mut HashSet<DefId>,
    ) {
        println!(
            "    collect_sources_from_place: {:?}, local={}",
            place,
            place.local.as_usize()
        );

        let local = place.local;

        // Check if this place is from self
        if local == self.self_local {
            println!("      From self!");
            let mut path = Vec::new();
            for elem in place.projection.iter() {
                if let rustc_middle::mir::ProjectionElem::Field(field, _) = elem {
                    let field_idx = format!("{}", field.index());
                    println!("      Found field: {}", field_idx);
                    path.push(field_idx);
                }
            }
            if !path.is_empty() {
                self_fields.insert(path.join("."));
            }
        }
        // Check if from parameter (parameters are locals 1..=n_args)
        else if local.as_usize() > 0 && local.as_usize() <= self.body.arg_count {
            let param_idx = local.as_usize() - 1; // 0-indexed parameter
            println!("      From parameter {}!", param_idx);
            params.insert(param_idx);
        }
        // Check if derived from tracked sources
        else {
            if let Some(derived_fields) = self.derived_from_self.get(&local) {
                println!(
                    "      From derived local (self), fields: {:?}",
                    derived_fields
                );
                self_fields.extend(derived_fields.clone());
            }
            if let Some(derived_params) = self.derived_from_params.get(&local) {
                println!("      From derived local (params): {:?}", derived_params);
                params.extend(derived_params.clone());
            }
            if let Some(derived_globals) = self.derived_from_globals.get(&local) {
                println!("      From derived local (globals): {:?}", derived_globals);
                globals.extend(derived_globals.clone());
            }
        }

        // Also check for derefs - if we're dereferencing a pointer/reference that came from tracked sources
        if !place.projection.is_empty() {
            if let Some(base_fields) = self.derived_from_self.get(&local) {
                println!(
                    "      Also inheriting from projections (self): {:?}",
                    base_fields
                );
                self_fields.extend(base_fields.clone());
            }
            if let Some(base_params) = self.derived_from_params.get(&local) {
                println!(
                    "      Also inheriting from projections (params): {:?}",
                    base_params
                );
                params.extend(base_params.clone());
            }
            if let Some(base_globals) = self.derived_from_globals.get(&local) {
                println!(
                    "      Also inheriting from projections (globals): {:?}",
                    base_globals
                );
                globals.extend(base_globals.clone());
            }
        }
    }

    pub fn extract_dependencies_from_place(&mut self, place: Place<'tcx>) {
        // Create temporary sets to collect results
        let mut temp_fields = HashSet::new();
        let mut temp_params = HashSet::new();
        let mut temp_globals = HashSet::new();

        self.collect_sources_from_place(
            place,
            &mut temp_fields,
            &mut temp_params,
            &mut temp_globals,
        );

        // Merge results into self
        self.self_fields.extend(temp_fields);
        self.params.extend(temp_params);
        self.globals.extend(temp_globals);
    }
}

// Visitor 1: Check if function writes to selected fields
struct FieldSetterVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    target_fields: HashSet<String>,
    self_local: rustc_middle::mir::Local,
    is_setter: bool,
    written_fields: HashSet<String>,
}

impl<'tcx> FieldSetterVisitor<'tcx> {
    fn new(
        tcx: TyCtxt<'tcx>,
        target_fields: HashSet<String>,
        self_local: rustc_middle::mir::Local,
    ) -> Self {
        Self {
            tcx,
            target_fields,
            self_local,
            is_setter: false,
            written_fields: HashSet::new(),
        }
    }
}

impl<'tcx> Visitor<'tcx> for FieldSetterVisitor<'tcx> {
    fn visit_statement(
        &mut self,
        statement: &rustc_middle::mir::Statement<'tcx>,
        location: Location,
    ) {
        if let StatementKind::Assign(place_and_rvalue) = &statement.kind {
            let (place, _rvalue) = &**place_and_rvalue;

            // Check if assignment writes to a target field from self
            if place.local == self.self_local {
                let mut path = Vec::new();
                for elem in place.projection.iter() {
                    if let rustc_middle::mir::ProjectionElem::Field(field, _) = elem {
                        let field_name = format!("{}", field.index());
                        path.push(field_name.clone());
                        if self.target_fields.contains(&field_name) {
                            self.is_setter = true;
                        }
                    }
                }
                if !path.is_empty() {
                    self.written_fields.insert(path.join("."));
                }
            }
        }
        self.super_statement(statement, location);
    }
}

// Visitor 2: Check if function returns &mut self or &mut self.field
struct MutRefReturnVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    target_fields: HashSet<String>,
    self_local: rustc_middle::mir::Local,
    returns_mut_ref: bool,
    body: &'tcx Body<'tcx>,
}

impl<'tcx> MutRefReturnVisitor<'tcx> {
    fn new(
        tcx: TyCtxt<'tcx>,
        target_fields: HashSet<String>,
        self_local: rustc_middle::mir::Local,
        body: &'tcx Body<'tcx>,
    ) -> Self {
        Self {
            tcx,
            target_fields,
            self_local,
            returns_mut_ref: false,
            body,
        }
    }

    fn is_return_type_mut_ref(&self) -> bool {
        // Check if _0 (return place) has a mutable reference type
        let return_ty = self.body.local_decls[rustc_middle::mir::Local::from_usize(0)].ty;
        matches!(
            return_ty.kind(),
            rustc_middle::ty::TyKind::Ref(_, _, rustc_middle::mir::Mutability::Mut)
        )
    }
}

impl<'tcx> Visitor<'tcx> for MutRefReturnVisitor<'tcx> {
    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: Location,
    ) {
        if let TerminatorKind::Return = &terminator.kind {
            // Check what _0 (return value) is assigned from
            // We need to look at the statements before return to see if _0 = &mut self.field
        }
        self.super_terminator(terminator, location);
    }

    fn visit_statement(
        &mut self,
        statement: &rustc_middle::mir::Statement<'tcx>,
        location: Location,
    ) {
        if let StatementKind::Assign(place_and_rvalue) = &statement.kind {
            let (place, rvalue) = &**place_and_rvalue;

            // Check if assigning to _0 (return value)
            if place.local.as_usize() == 0 {
                // First check if the return type is actually a mutable reference
                if !self.is_return_type_mut_ref() {
                    self.super_statement(statement, location);
                    return;
                }

                // Helper to check if a place from self returns a target field
                let check_place = |ret_place: Place<'tcx>| -> bool {
                    if ret_place.local == self.self_local {
                        // Check if it's returning a target field
                        for elem in ret_place.projection.iter() {
                            if let rustc_middle::mir::ProjectionElem::Field(field, _) = elem {
                                let field_name = format!("{}", field.index());
                                println!(
                                    "  MutRefReturnVisitor: Checking field {} against target_fields {:?}",
                                    field_name, self.target_fields
                                );
                                if self.target_fields.contains(&field_name) {
                                    return true;
                                }
                            }
                        }
                    }
                    false
                };

                match rvalue {
                    // Case 1: Creating a new mutable reference (&mut self.field)
                    Rvalue::Ref(_, rustc_middle::mir::BorrowKind::Mut { .. }, ref_place) => {
                        if ref_place.local == self.self_local {
                            // It's &mut self or &mut self.something
                            if ref_place.projection.is_empty() {
                                // &mut self - this can mutate all fields
                                self.returns_mut_ref = true;
                                return;
                            }

                            // Check if it references a target field
                            if check_place(*ref_place) {
                                self.returns_mut_ref = true;
                                return;
                            }
                        }
                    }
                    // Case 2: Returning an existing reference (self.field where field is already a reference)
                    Rvalue::Use(operand) => {
                        if let Some(ret_place) = operand.place() {
                            if check_place(ret_place) {
                                self.returns_mut_ref = true;
                                return;
                            }
                        }
                    }
                    // Case 3: CopyForDeref - used for (*self).field patterns
                    Rvalue::CopyForDeref(ret_place) => {
                        if check_place(*ret_place) {
                            self.returns_mut_ref = true;
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.super_statement(statement, location);
    }
}

// Visitor 3: Check if function returns aggregate containing &mut self or &mut self.field
struct AggregateWithMutRefVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    target_fields: HashSet<String>,
    self_local: rustc_middle::mir::Local,
    returns_aggregate_with_mut_ref: bool,
    // Track which locals contain &mut references to target fields
    mut_ref_locals: HashMap<rustc_middle::mir::Local, HashSet<String>>,
    // Track which fields in the returned aggregate contain &mut refs (field index -> original fields)
    pub aggregate_fields_with_mut_refs: HashSet<String>,
}

impl<'tcx> AggregateWithMutRefVisitor<'tcx> {
    fn new(
        tcx: TyCtxt<'tcx>,
        target_fields: HashSet<String>,
        self_local: rustc_middle::mir::Local,
    ) -> Self {
        Self {
            tcx,
            target_fields,
            self_local,
            returns_aggregate_with_mut_ref: false,
            mut_ref_locals: HashMap::new(),
            aggregate_fields_with_mut_refs: HashSet::new(),
        }
    }
}

impl<'tcx> Visitor<'tcx> for AggregateWithMutRefVisitor<'tcx> {
    fn visit_statement(
        &mut self,
        statement: &rustc_middle::mir::Statement<'tcx>,
        location: Location,
    ) {
        if let StatementKind::Assign(place_and_rvalue) = &statement.kind {
            let (place, rvalue) = &**place_and_rvalue;

            // Track &mut self.field assignments
            if let Rvalue::Ref(_, rustc_middle::mir::BorrowKind::Mut { .. }, ref_place) = rvalue {
                if ref_place.local == self.self_local {
                    let mut fields = HashSet::new();

                    if ref_place.projection.is_empty() {
                        // &mut self - all fields
                        fields = self.target_fields.clone();
                    } else {
                        // Check specific fields
                        for elem in ref_place.projection.iter() {
                            if let rustc_middle::mir::ProjectionElem::Field(field, _) = elem {
                                let field_name = format!("{}", field.index());
                                if self.target_fields.contains(&field_name) {
                                    fields.insert(field_name);
                                }
                            }
                        }
                    }

                    if !fields.is_empty() {
                        self.mut_ref_locals.insert(place.local, fields);
                    }
                }
            }

            // Check if _0 (return value) is assigned an aggregate containing tracked locals
            if place.local.as_usize() == 0 {
                if let Rvalue::Aggregate(_, operands) = rvalue {
                    for (field_idx, operand) in operands.iter().enumerate() {
                        if let rustc_middle::mir::Operand::Move(op_place)
                        | rustc_middle::mir::Operand::Copy(op_place) = operand
                        {
                            if self.mut_ref_locals.contains_key(&op_place.local) {
                                self.returns_aggregate_with_mut_ref = true;
                                // Track which field in the aggregate contains the mut ref
                                self.aggregate_fields_with_mut_refs
                                    .insert(format!("{}", field_idx));
                            }
                        }
                    }
                }
            }
        }
        self.super_statement(statement, location);
    }
}

// Helper to extract function info from DefId
fn get_doc_line_start<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<usize> {
    let source_map = tcx.sess.source_map();
    let mut earliest: Option<usize> = None;

    for attr in tcx.get_attrs(def_id, sym::doc) {
        let loc = source_map.lookup_char_pos(attr.span().lo());
        earliest = Some(match earliest {
            Some(current) => current.min(loc.line),
            None => loc.line,
        });
    }

    earliest
}

fn is_doc_attr_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("#[doc") || trimmed.starts_with("# [doc")
}

fn is_doc_comment_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("///") || trimmed.starts_with("//!")
}

fn get_doc_line_start_from_source(file_path: &str, signature_line: usize) -> Option<usize> {
    if signature_line <= 1 {
        return None;
    }

    let content = std::fs::read_to_string(file_path).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let mut idx = (signature_line.saturating_sub(2)) as isize;
    if idx as usize >= lines.len() {
        idx = (lines.len().saturating_sub(1)) as isize;
    }

    // Skip non-doc attributes and blank lines immediately above fn signature.
    while idx >= 0 {
        let trimmed = lines[idx as usize].trim();
        if trimmed.is_empty() {
            idx -= 1;
            continue;
        }
        if trimmed.starts_with("#[") && !is_doc_attr_line(trimmed) {
            idx -= 1;
            continue;
        }
        break;
    }

    if idx < 0 {
        return None;
    }

    let current = lines[idx as usize];
    if !(is_doc_comment_line(current) || is_doc_attr_line(current)) {
        return None;
    }

    let mut start = idx as usize;
    while start > 0 {
        let prev = lines[start - 1];
        if is_doc_comment_line(prev) || is_doc_attr_line(prev) {
            start -= 1;
            continue;
        }
        break;
    }

    Some(start + 1)
}

fn get_fn_info<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> FnInfo {
    get_fn_info_with_template_flag(tcx, def_id, false)
}

fn trait_name_for_assoc_fn<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<String> {
    let item = tcx.opt_associated_item(def_id)?;
    if item.kind != ty::AssocKind::Fn {
        return None;
    }

    match item.container {
        ty::AssocItemContainer::Trait => Some(tcx.def_path_str(item.container_id(tcx))),
        ty::AssocItemContainer::Impl => item
            .trait_item_def_id
            .map(|trait_item_def_id| tcx.def_path_str(tcx.parent(trait_item_def_id)))
            .or_else(|| {
                tcx.trait_id_of_impl(item.container_id(tcx))
                    .map(|trait_def_id| tcx.def_path_str(trait_def_id))
            }),
    }
}

fn return_ty_for_fn<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<String> {
    if !matches!(
        tcx.def_kind(def_id),
        rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn
    ) {
        return None;
    }
    let fn_sig = tcx.fn_sig(def_id).skip_binder();
    Some(fn_sig.output().skip_binder().to_string())
}

fn symbol_match_hint(trait_name: &str, method_name: &str) -> String {
    let trait_leaf = trait_name.rsplit("::").next().unwrap_or(trait_name);
    format!("{trait_leaf}::{method_name}")
}

fn collect_trait_methods<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<TraitMethodInfo> {
    let mut seen = HashSet::new();
    let mut methods = Vec::new();

    for local_def_id in tcx.hir_crate_items(()).definitions() {
        let impl_def_id = local_def_id.to_def_id();
        if !matches!(tcx.def_kind(impl_def_id), rustc_hir::def::DefKind::Impl { .. }) {
            continue;
        }
        let Some(trait_def_id) = tcx.trait_id_of_impl(impl_def_id) else {
            continue;
        };
        let trait_name = tcx.def_path_str(trait_def_id);
        let implementor_type = tcx.type_of(impl_def_id).instantiate_identity().to_string();
        let implementors = tcx.impl_item_implementor_ids(impl_def_id);

        // Iterate the trait rather than only explicit impl items. This also
        // reports default methods inherited by the implementor.
        for item in tcx.associated_items(trait_def_id).in_definition_order() {
            if item.kind != ty::AssocKind::Fn || !item.fn_has_self_parameter {
                continue;
            }
            let method_name = item.name.to_string();
            let method_def_id = implementors
                .get(&item.def_id)
                .copied()
                .unwrap_or(item.def_id);
            let return_ty =
                return_ty_for_fn(tcx, method_def_id).unwrap_or_else(|| "()".to_string());
            let symbol_match_hint = symbol_match_hint(&trait_name, &method_name);
            if !seen.insert((
                trait_name.clone(),
                method_name.clone(),
                implementor_type.clone(),
            )) {
                continue;
            }

            methods.push(TraitMethodInfo {
                trait_name: trait_name.clone(),
                method_name,
                implementor_type: implementor_type.clone(),
                return_ty,
                symbol_match_hint,
            });
        }
    }

    methods
}

fn collect_callee_type_args<'tcx>(
    tcx: TyCtxt<'tcx>,
    callee_def_id: DefId,
    callee_args: ty::GenericArgsRef<'tcx>,
) -> Vec<CalleeTypeArgInfo> {
    fn append_params<'tcx>(
        tcx: TyCtxt<'tcx>,
        owner: DefId,
        out: &mut Vec<(DefId, ty::GenericParamDef)>,
    ) {
        let generics = tcx.generics_of(owner);
        if let Some(parent) = generics.parent {
            append_params(tcx, parent, out);
        }
        out.extend(
            generics
                .own_params
                .iter()
                .cloned()
                .map(|param| (owner, param)),
        );
    }

    let mut params = Vec::new();
    append_params(tcx, callee_def_id, &mut params);
    let mut result = Vec::new();

    for (owner_def_id, param) in params {
        if !matches!(param.kind, ty::GenericParamDefKind::Type { .. }) {
            continue;
        }

        let index = result.len();
        let owner = if owner_def_id == callee_def_id {
            "function"
        } else {
            "impl"
        };
        let Some(arg) = callee_args.get(param.index as usize) else {
            result.push(CalleeTypeArgInfo {
                index,
                name: param.name.to_string(),
                owner: owner.to_string(),
                instantiated_ty: "<unknown>".to_string(),
                layout_control: TypeLayoutControl::Unknown,
                external_sources: Vec::new(),
            });
            continue;
        };
        let Some(instantiated_ty) = arg.as_type() else {
            result.push(CalleeTypeArgInfo {
                index,
                name: param.name.to_string(),
                owner: owner.to_string(),
                instantiated_ty: "<non-type argument>".to_string(),
                layout_control: TypeLayoutControl::Unknown,
                external_sources: Vec::new(),
            });
            continue;
        };

        let rendered = instantiated_ty.to_string();
        let is_external = instantiated_ty.has_param();
        result.push(CalleeTypeArgInfo {
            index,
            name: param.name.to_string(),
            owner: owner.to_string(),
            instantiated_ty: rendered.clone(),
            layout_control: if is_external {
                TypeLayoutControl::External
            } else {
                TypeLayoutControl::Fixed
            },
            external_sources: if is_external {
                vec![rendered]
            } else {
                Vec::new()
            },
        });
    }

    result
}

fn get_fn_info_with_template_flag<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    require_template: bool,
) -> FnInfo {
    let span = tcx.def_span(def_id);
    let source_map = tcx.sess.source_map();
    let loc = source_map.lookup_char_pos(span.lo());
    let end_loc = source_map.lookup_char_pos(span.hi());
    let body_span = span_with_body_if_available(tcx, def_id, span);
    let body_end_loc = source_map.lookup_char_pos(body_span.hi());
    let name = tcx.def_path_str(def_id);
    let method_name = name.rsplit("::").next().unwrap_or(&name).to_string();
    let trait_name = trait_name_for_assoc_fn(tcx, def_id);
    let return_ty = return_ty_for_fn(tcx, def_id);
    let symbol_match_hint = trait_name
        .as_ref()
        .map(|trait_name| symbol_match_hint(trait_name, &method_name));
    let file_path = loc.file.name.prefer_local().to_string();
    let path = normalize_to_rust_relative(&file_path);
    let line_start = get_doc_line_start(tcx, def_id)
        .or_else(|| get_doc_line_start_from_source(&file_path, loc.line))
        .unwrap_or(loc.line);

    FnInfo {
        name: name.clone(),
        path,
        line_start,
        line_end: end_loc.line,
        body_end: body_end_loc.line,
        require_template,
        is_unsafe: is_fn_unsafe(tcx, def_id),
        trait_name,
        return_ty,
        symbol_match_hint,
        call_chains: vec![],
        mut_ref_escape: None,
        mut_ref_escape_fields: vec![],
        written_fields: vec![],
        return_self_fields: vec![],
    }
}

fn requires_template<'tcx>(
    tcx: TyCtxt<'tcx>,
    caller_def_id: DefId,
    caller_parent_def_id: Option<DefId>,
) -> bool {
    if !tcx.generics_of(caller_def_id).own_params.is_empty() {
        return true;
    }
    if let Some(parent_def_id) = caller_parent_def_id {
        if !tcx.generics_of(parent_def_id).own_params.is_empty() {
            return true;
        }
    }
    false
}

fn get_struct_info<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> StructInfo {
    let span = tcx.def_span(def_id);
    let source_map = tcx.sess.source_map();
    let loc = source_map.lookup_char_pos(span.lo());
    let body_span = span_with_body_if_available(tcx, def_id, span);
    let end_loc = source_map.lookup_char_pos(body_span.hi());
    let name = tcx.def_path_str(def_id);
    let path = normalize_to_rust_relative(&loc.file.name.prefer_local().to_string());

    StructInfo {
        name: name.clone(),
        path,
        line_start: loc.line,
        body_end: end_loc.line,
        is_public: tcx.visibility(def_id).is_public(),
    }
}

// Helper to check if function is a constructor
fn is_constructor<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId, parent_def_id: Option<DefId>) -> bool {
    // Get function signature
    let fn_sig = tcx.fn_sig(def_id).skip_binder();

    // Check if function takes &self - if so, it's not a constructor
    // Constructors should be associated functions without self parameter
    let inputs = fn_sig.inputs().skip_binder();
    if !inputs.is_empty() {
        // Check if first parameter is self/&self/&mut self
        if let Some(first_input) = inputs.get(0) {
            // If the first input references the parent type, it's likely a method with self
            if let Some(parent) = parent_def_id {
                let parent_ty = tcx.type_of(parent).skip_binder();
                // Check for Self, &Self, &mut Self
                if first_input == &parent_ty
                    || matches!(first_input.kind(), rustc_middle::ty::TyKind::Ref(_, ty, _) if ty == &parent_ty)
                {
                    return false;
                }
            }
        }
    }

    // Check if function name is "new" or similar
    let binding = tcx.item_name(def_id);
    let fn_name = binding.as_str();
    // Check if return type matches the parent struct
    if let Some(parent) = parent_def_id {
        let output = fn_sig.output().skip_binder();
        let parent_ty = tcx.type_of(parent).skip_binder();

        // Direct match
        if output == parent_ty {
            return true;
        }

        // Check if it returns Self wrapped in Result, Option, etc.
        // For now, just check the outermost type
        if let rustc_middle::ty::TyKind::Adt(adt_def, substs) = output.kind() {
            // Check substs for the parent type
            for subst in substs.iter() {
                if let Some(ty) = subst.as_type() {
                    if ty == parent_ty {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn collect_constructors<'tcx>(tcx: TyCtxt<'tcx>, struct_def_id: DefId) -> Vec<FnInfo> {
    let mut constructors = Vec::new();
    let impl_def_ids = tcx.inherent_impls(struct_def_id);

    for &impl_def_id in impl_def_ids.iter() {
        let impl_items = tcx.associated_items(impl_def_id);

        for item in impl_items.in_definition_order() {
            if item.kind != rustc_middle::ty::AssocKind::Fn {
                continue;
            }

            let fn_def_id = item.def_id;
            if is_fn_unsafe(tcx, fn_def_id) || !tcx.visibility(fn_def_id).is_public() {
                continue;
            }

            if is_constructor(tcx, fn_def_id, Some(struct_def_id)) {
                constructors.push(get_fn_info(tcx, fn_def_id));
            }
        }
    }
    constructors
}

fn has_mut_self_receiver<'tcx>(tcx: TyCtxt<'tcx>, fn_def_id: DefId, struct_def_id: DefId) -> bool {
    let fn_sig = tcx.fn_sig(fn_def_id).skip_binder();
    let inputs = fn_sig.inputs().skip_binder();
    let Some(first_input) = inputs.get(0) else {
        return false;
    };

    let struct_ty = tcx.type_of(struct_def_id).skip_binder();
    matches!(
        first_input.kind(),
        rustc_middle::ty::TyKind::Ref(_, ty, rustc_middle::mir::Mutability::Mut)
            if ty == &struct_ty
    )
}

fn collect_all_field_indices<'tcx>(tcx: TyCtxt<'tcx>, struct_def_id: DefId) -> HashSet<String> {
    let mut fields = HashSet::new();
    if let Some(adt_def) = tcx.type_of(struct_def_id).skip_binder().ty_adt_def() {
        for variant in adt_def.variants() {
            for (index, _field) in variant.fields.iter().enumerate() {
                fields.insert(index.to_string());
            }
        }
    }
    fields
}

fn collect_public_fields<'tcx>(tcx: TyCtxt<'tcx>, struct_def_id: DefId) -> Vec<FieldInfo> {
    let mut fields = Vec::new();
    if let Some(adt_def) = tcx.type_of(struct_def_id).skip_binder().ty_adt_def() {
        for variant in adt_def.variants() {
            for (index, field) in variant.fields.iter().enumerate() {
                if !field.vis.is_public() {
                    continue;
                }
                let name = field.name.to_string();
                fields.push(FieldInfo {
                    index: index.to_string(),
                    path: name.clone(),
                    name,
                });
            }
        }
    }
    fields
}

fn collect_field_layouts<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_def_id: DefId,
) -> Vec<FieldLayoutInfo> {
    let struct_ty = tcx.type_of(struct_def_id).skip_binder();
    let typing_env = ty::TypingEnv::fully_monomorphized();
    let Ok(layout) = tcx.layout_of(typing_env.as_query_input(struct_ty)) else {
        return Vec::new();
    };
    let Some(adt_def) = struct_ty.ty_adt_def() else {
        return Vec::new();
    };
    let Some(variant) = adt_def.variants().iter().next() else {
        return Vec::new();
    };

    let mut fields = Vec::new();
    for (index, field) in variant.fields.iter().enumerate() {
        let field_ty = tcx.type_of(field.did).skip_binder();
        let Ok(field_layout) =
            tcx.layout_of(typing_env.as_query_input(field_ty))
        else {
            continue;
        };
        let element_ty = match field_ty.kind() {
            ty::Adt(def, args) => {
                let name = tcx.item_name(def.did()).to_string();
                let type_index = if name == "Vec" {
                    Some(0)
                } else if name == "HashMap" || name == "BTreeMap" {
                    Some(1)
                } else {
                    None
                };
                type_index.and_then(|i| args.get(i)).and_then(|arg| arg.as_type())
            }
            _ => None,
        };
        fields.push(FieldLayoutInfo {
            index: index.to_string(),
            name: field.name.to_string(),
            ty: field_ty.to_string(),
            element_ty: element_ty.map(|ty| ty.to_string()),
            element_is_template: element_ty.is_some_and(|ty| ty.has_param()),
            offset: layout.fields.offset(index).bytes(),
            size: field_layout.size.bytes(),
        });
    }
    fields
}

fn adt_def_id_behind_ref<'tcx>(ty: Ty<'tcx>) -> Option<DefId> {
    match ty.kind() {
        rustc_middle::ty::TyKind::Ref(_, inner, _) => adt_def_id_behind_ref(*inner),
        rustc_middle::ty::TyKind::Adt(adt_def, _) => Some(adt_def.did()),
        _ => None,
    }
}

fn field_type_def_ids_for_indices<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_def_id: DefId,
    field_indices: &HashSet<String>,
) -> Vec<DefId> {
    let mut out = Vec::new();
    let Some(adt_def) = tcx.type_of(struct_def_id).skip_binder().ty_adt_def() else {
        return out;
    };

    for variant in adt_def.variants() {
        for (index, field) in variant.fields.iter().enumerate() {
            if !field_indices.contains(&index.to_string()) {
                continue;
            }
            let field_ty = tcx.type_of(field.did).skip_binder();
            if let Some(def_id) = adt_def_id_behind_ref(field_ty) {
                out.push(def_id);
            }
        }
    }
    out
}

fn returns_mut_ref_type<'tcx>(tcx: TyCtxt<'tcx>, fn_def_id: DefId) -> bool {
    let fn_sig = tcx.fn_sig(fn_def_id).skip_binder();
    let return_ty = fn_sig.output().skip_binder();
    match return_ty.kind() {
        rustc_middle::ty::TyKind::Ref(_, _, rustc_middle::mir::Mutability::Mut) => true,
        rustc_middle::ty::TyKind::Adt(adt_def, args) => {
            let name = tcx.def_path_str(adt_def.did());
            if !(name == "core::result::Result"
                || name == "std::result::Result"
                || name == "core::option::Option"
                || name == "std::option::Option")
            {
                return false;
            }
            args.iter().any(|arg| {
                arg.as_type().is_some_and(|ty| {
                    matches!(
                        ty.kind(),
                        rustc_middle::ty::TyKind::Ref(_, _, rustc_middle::mir::Mutability::Mut)
                    )
                })
            })
        }
        _ => false,
    }
}

fn collect_written_fields_for_fn<'tcx>(
    tcx: TyCtxt<'tcx>,
    fn_def_id: DefId,
    fields: HashSet<String>,
) -> Vec<String> {
    let Some(body) = optimized_mir_if_available(tcx, fn_def_id) else {
        return Vec::new();
    };
    let self_local = rustc_middle::mir::Local::from_usize(1);
    let mut visitor = FieldSetterVisitor::new(tcx, fields, self_local);
    visitor.visit_body(body);
    let mut written: Vec<String> = visitor.written_fields.into_iter().collect();
    written.sort();
    written
}

fn collect_return_self_fields_for_fn<'tcx>(
    tcx: TyCtxt<'tcx>,
    fn_def_id: DefId,
) -> Vec<String> {
    let Some(body) = optimized_mir_if_available(tcx, fn_def_id) else {
        return Vec::new();
    };
    if body.arg_count == 0 {
        return Vec::new();
    }
    let self_local = rustc_middle::mir::Local::from_usize(1);
    let mut visitor = DataDependencyVisitor::new(tcx, self_local, body);
    visitor.extract_dependencies_from_place(Place::from(
        rustc_middle::mir::Local::from_usize(0),
    ));
    let mut fields: Vec<String> = visitor.self_fields.into_iter().collect();
    fields.sort();
    fields.dedup();
    fields
}

fn dedup_fn_infos(items: Vec<FnInfo>) -> Vec<FnInfo> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for item in items {
        let key = (item.name.clone(), item.path.clone(), item.line_start);
        if seen.insert(key) {
            deduped.push(item);
        }
    }
    deduped
}

fn collect_public_type_mutators<'tcx>(tcx: TyCtxt<'tcx>, struct_def_id: DefId) -> Vec<FnInfo> {
    let mut mutators = Vec::new();
    let all_fields = collect_all_field_indices(tcx, struct_def_id);
    let impl_def_ids = tcx.inherent_impls(struct_def_id);

    for &impl_def_id in impl_def_ids.iter() {
        let impl_items = tcx.associated_items(impl_def_id);

        for item in impl_items.in_definition_order() {
            if item.kind != rustc_middle::ty::AssocKind::Fn {
                continue;
            }

            let fn_def_id = item.def_id;
            if is_fn_unsafe(tcx, fn_def_id) {
                continue;
            }

            if has_mut_self_receiver(tcx, fn_def_id, struct_def_id) {
                let mut info = get_fn_info(tcx, fn_def_id);
                info.return_self_fields =
                    collect_return_self_fields_for_fn(tcx, fn_def_id);
                info.written_fields =
                    collect_written_fields_for_fn(tcx, fn_def_id, all_fields.clone());
                if returns_mut_ref_type(tcx, fn_def_id) {
                    info.mut_ref_escape = Some("returns_mut_ref_from_mut_self".to_string());
                    info.mut_ref_escape_fields =
                        if info.return_self_fields.is_empty() {
                            all_fields.iter().cloned().collect()
                        } else {
                            info.return_self_fields.clone()
                        };
                } else if let Some(body) = optimized_mir_if_available(tcx, fn_def_id) {
                    let self_local = rustc_middle::mir::Local::from_usize(1);
                    let mut aggregate_visitor =
                        AggregateWithMutRefVisitor::new(tcx, all_fields.clone(), self_local);
                    aggregate_visitor.visit_body(body);
                    if aggregate_visitor.returns_aggregate_with_mut_ref {
                        info.mut_ref_escape = Some("returns_aggregate_with_mut_ref".to_string());
                        info.mut_ref_escape_fields = aggregate_visitor
                            .aggregate_fields_with_mut_refs
                            .iter()
                            .cloned()
                            .collect();
                    }
                }
                mutators.push(info);
            }
        }
    }

    if !all_fields.is_empty() {
        mutators.extend(collect_fields_setters(
            tcx,
            struct_def_id,
            all_fields.clone(),
        ));
        mutators.extend(collect_escaped_mut_refs(
            tcx,
            struct_def_id,
            all_fields.clone(),
        ));
        mutators.extend(collect_escaped_mut_refs_in_aggregates(
            tcx,
            struct_def_id,
            all_fields,
            vec![],
        ));
    }

    dedup_fn_infos(mutators)
}

fn collect_public_type_observers<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_def_id: DefId,
) -> Vec<FnInfo> {
    let mut observers = Vec::new();
    let struct_ty = tcx.type_of(struct_def_id).skip_binder();
    for &impl_def_id in tcx.inherent_impls(struct_def_id).iter() {
        for item in tcx
            .associated_items(impl_def_id)
            .in_definition_order()
        {
            if item.kind != rustc_middle::ty::AssocKind::Fn {
                continue;
            }
            let fn_def_id = item.def_id;
            if is_fn_unsafe(tcx, fn_def_id) {
                continue;
            }
            let fn_sig = tcx.fn_sig(fn_def_id).skip_binder();
            let inputs = fn_sig.inputs().skip_binder();
            let Some(first_input) = inputs.get(0) else {
                continue;
            };
            let is_shared_self = matches!(
                first_input.kind(),
                rustc_middle::ty::TyKind::Ref(
                    _,
                    ty,
                    rustc_middle::mir::Mutability::Not
                ) if ty == &struct_ty
            );
            if !is_shared_self {
                continue;
            }
            let mut info = get_fn_info(tcx, fn_def_id);
            info.return_self_fields =
                collect_return_self_fields_for_fn(tcx, fn_def_id);
            observers.push(info);
        }
    }
    dedup_fn_infos(observers)
}

fn collect_public_type_infos<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<TypeInteractionInfo> {
    let mut types = Vec::new();

    for local_def_id in tcx.hir_crate_items(()).definitions() {
        let def_id = local_def_id.to_def_id();

        if !matches!(tcx.def_kind(def_id), rustc_hir::def::DefKind::Struct) {
            continue;
        }
        let Some(adt_def) = tcx.type_of(def_id).skip_binder().ty_adt_def() else {
            continue;
        };
        if !adt_def.is_struct() {
            continue;
        }

        types.push(TypeInteractionInfo {
            ty: get_struct_info(tcx, def_id),
            public_fields: collect_public_fields(tcx, def_id),
            field_layouts: collect_field_layouts(tcx, def_id),
            constructors: collect_constructors(tcx, def_id),
            observers: collect_public_type_observers(tcx, def_id),
            mutators: collect_public_type_mutators(tcx, def_id),
        });
    }

    types
}

fn collect_fields_setters<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_def_id: DefId,
    fields: HashSet<String>,
) -> Vec<FnInfo> {
    let mut setters = Vec::new();
    let impl_def_ids = tcx.inherent_impls(struct_def_id);

    for &impl_def_id in impl_def_ids.iter() {
        let impl_items = tcx.associated_items(impl_def_id);

        for item in impl_items.in_definition_order() {
            if item.kind != rustc_middle::ty::AssocKind::Fn {
                continue;
            }

            let fn_def_id = item.def_id;

            // Check if function is public and safe
            if is_fn_unsafe(tcx, fn_def_id) {
                continue;
            }

            // Analyze function body to see if it writes to any of the target fields
            if let Some(body) = optimized_mir_if_available(tcx, fn_def_id) {
                let self_local = rustc_middle::mir::Local::from_usize(1);
                let mut setter_visitor = FieldSetterVisitor::new(tcx, fields.clone(), self_local);
                setter_visitor.visit_body(body);

                if setter_visitor.is_setter {
                    let mut info = get_fn_info(tcx, fn_def_id);
                    let mut written: Vec<String> =
                        setter_visitor.written_fields.into_iter().collect();
                    written.sort();
                    info.written_fields = written;
                    setters.push(info);
                }
            }
        }
    }

    setters
}

/// find out if there is any functions which return &mut self.xxx or &mut self, which can be used to mutate fields indirectly
fn collect_escaped_mut_refs<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_def_id: DefId,
    fields: HashSet<String>,
) -> Vec<FnInfo> {
    let mut mutators = Vec::new();
    let impl_def_ids = tcx.inherent_impls(struct_def_id);

    for &impl_def_id in impl_def_ids.iter() {
        let impl_items = tcx.associated_items(impl_def_id);

        for item in impl_items.in_definition_order() {
            if item.kind != rustc_middle::ty::AssocKind::Fn {
                continue;
            }

            let fn_def_id = item.def_id;

            // Check if function is public and safe
            if is_fn_unsafe(tcx, fn_def_id) {
                continue;
            }

            // Analyze function body to see if it returns &mut to any of the target fields
            if let Some(body) = optimized_mir_if_available(tcx, fn_def_id) {
                let self_local = rustc_middle::mir::Local::from_usize(1);
                let mut mutator_visitor =
                    MutRefReturnVisitor::new(tcx, fields.clone(), self_local, body);
                mutator_visitor.visit_body(body);

                if mutator_visitor.returns_mut_ref {
                    let mut fn_info = get_fn_info(tcx, fn_def_id);
                    fn_info.written_fields =
                        collect_written_fields_for_fn(tcx, fn_def_id, fields.clone());
                    fn_info.mut_ref_escape = Some("returns_mut_ref_to_self_field".to_string());
                    fn_info.mut_ref_escape_fields = fields.iter().cloned().collect();
                    mutators.push(fn_info);
                }
            }
        }
    }

    mutators
}

/// find out is there is any function which return A { xxx: &mut self.xxx, } or A { xxx: &mut self }
/// Uses a queue-based approach to track call chains across struct functions
fn collect_escaped_mut_refs_in_aggregates<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_def_id: DefId,
    fields: HashSet<String>,
    call_chains: Vec<String>,
) -> Vec<FnInfo> {
    let mut mutators = Vec::new();
    let mut visited_types: HashSet<DefId> = HashSet::new();

    // Queue entries: (type_def_id, call_chain, fields_to_check)
    let mut queue: Vec<(DefId, Vec<String>, HashSet<String>)> =
        vec![(struct_def_id, call_chains, fields)];

    while let Some((current_type_def_id, current_chain, current_fields)) = queue.pop() {
        // Avoid infinite loops
        if visited_types.contains(&current_type_def_id) {
            continue;
        }
        visited_types.insert(current_type_def_id);
        println!(
            "Exploring type {:?} with call chain: {:?}, fields: {:?}",
            tcx.def_path_str(current_type_def_id),
            current_chain,
            current_fields
        );

        let impl_def_ids = tcx.inherent_impls(current_type_def_id);

        for &impl_def_id in impl_def_ids.iter() {
            let impl_items = tcx.associated_items(impl_def_id);

            for item in impl_items.in_definition_order() {
                if item.kind != rustc_middle::ty::AssocKind::Fn {
                    continue;
                }

                let fn_def_id = item.def_id;

                // Check if function is public and safe
                if is_fn_unsafe(tcx, fn_def_id) {
                    continue;
                }

                println!(
                    "Analyzing function {} in type {:?} for aggregate mut ref returns",
                    tcx.def_path_str(fn_def_id),
                    tcx.def_path_str(current_type_def_id)
                );

                if let Some(body) = optimized_mir_if_available(tcx, fn_def_id) {
                    let self_local = rustc_middle::mir::Local::from_usize(1);

                    // Check if this function returns aggregate with &mut references
                    let mut aggregate_visitor =
                        AggregateWithMutRefVisitor::new(tcx, current_fields.clone(), self_local);
                    aggregate_visitor.visit_body(body);

                    if aggregate_visitor.returns_aggregate_with_mut_ref {
                        // This function returns an aggregate with &mut refs
                        let fn_name = tcx.def_path_str(fn_def_id);
                        let mut new_chain = current_chain.clone();
                        new_chain.push(fn_name.clone());
                        println!(
                            "Found aggregate mut ref return in function {}, call chain: {:?}",
                            fn_name, new_chain
                        );

                        // Get the return type and check if it's an ADT
                        let fn_sig = tcx.fn_sig(fn_def_id).skip_binder();
                        let return_ty = fn_sig.output().skip_binder();

                        if let rustc_middle::ty::TyKind::Adt(adt_def, _) = return_ty.kind() {
                            let return_type_def_id = adt_def.did();
                            // The fields to check in the returned type are the aggregate fields that contain mut refs
                            let next_fields =
                                aggregate_visitor.aggregate_fields_with_mut_refs.clone();
                            println!(
                                "Enqueued return type {:?} with fields {:?} for further exploration",
                                tcx.def_path_str(return_type_def_id),
                                next_fields
                            );
                            queue.push((
                                return_type_def_id,
                                new_chain.clone(),
                                next_fields.clone(),
                            ));
                            for field_type_def_id in field_type_def_ids_for_indices(
                                tcx,
                                return_type_def_id,
                                &next_fields,
                            ) {
                                queue.push((
                                    field_type_def_id,
                                    new_chain.clone(),
                                    collect_all_field_indices(tcx, field_type_def_id),
                                ));
                            }
                        }
                    } else {
                        println!(
                            "Function {} does not return aggregate with mut refs",
                            tcx.def_path_str(fn_def_id)
                        );
                        // Check if this function directly returns &mut reference
                        let mut mutref_visitor =
                            MutRefReturnVisitor::new(tcx, current_fields.clone(), self_local, body);
                        mutref_visitor.visit_body(body);

                        println!(
                            "Checking whether return &mut refs for fields {:?}: {}",
                            current_fields, mutref_visitor.returns_mut_ref
                        );

                        if mutref_visitor.returns_mut_ref {
                            println!(
                                "Function {} returns direct &mut reference",
                                tcx.def_path_str(fn_def_id)
                            );
                            // This function returns &mut directly
                            let fn_name = tcx.def_path_str(fn_def_id);
                            let mut new_chain = current_chain.clone();
                            new_chain.push(fn_name.clone());
                            println!(
                                "Found direct mut ref return in function {}, call chain: {:?}",
                                fn_name, new_chain
                            );

                            let mut fn_info = get_fn_info(tcx, fn_def_id);
                            fn_info.written_fields =
                                collect_written_fields_for_fn(tcx, fn_def_id, current_fields.clone());
                            fn_info.call_chains = new_chain;
                            fn_info.mut_ref_escape = Some("call_chain_returns_mut_ref".to_string());
                            fn_info.mut_ref_escape_fields =
                                current_fields.iter().cloned().collect();
                            mutators.push(fn_info);
                        }
                    }
                }
            }
        }
    }

    mutators
}

pub fn audit<'tcx>(tcx: TyCtxt<'tcx>) -> Report {
    let mut targets = Vec::new();
    let types = collect_public_type_infos(tcx);
    let affected_fields = collect_affected_fields(tcx);
    let max_call_depth = configured_max_call_depth();

    // Find all ADTs (structs/enums)
    for local_def_id in tcx.hir_crate_items(()).definitions() {
        let def_id = local_def_id.to_def_id();
        // print!("Checking item {:?} (def_id={:?})... ", tcx.def_path_str(def_id), def_id);
        // Check if it's a struct, enum, or union using def_kind first
        use rustc_hir::def::DefKind;
        match tcx.def_kind(def_id) {
            DefKind::Struct | DefKind::Enum | DefKind::Union => {}
            _ => continue,
        }

        println!("Analyzing ADT: {}", tcx.def_path_str(def_id));

        // Check if it's a struct
        if let Some(adt_def) = tcx.type_of(def_id).skip_binder().ty_adt_def() {
            if !adt_def.is_struct() {
                continue;
            }

            let constructors = collect_constructors(tcx, def_id);

            // Find all inherent impl blocks for this struct
            let impl_def_ids = tcx.inherent_impls(def_id);

            // Iterate through all impl blocks
            for &impl_def_id in impl_def_ids.iter() {
                let impl_items = tcx.associated_items(impl_def_id);

                for item in impl_items.in_definition_order() {
                    if item.kind != rustc_middle::ty::AssocKind::Fn {
                        continue;
                    }

                    let fn_def_id = item.def_id;

                    // Skip unsafe functions
                    if is_fn_unsafe(tcx, fn_def_id) {
                        continue;
                    }

                    // Check if it's public
                    if !tcx.visibility(fn_def_id).is_public() {
                        continue;
                    }

                    let require_template = requires_template(tcx, fn_def_id, Some(def_id));
                    let fn_info =
                        get_fn_info_with_template_flag(tcx, fn_def_id, require_template);

                    // Analyze the function body for unsafe calls
                    if let Some(body) = optimized_mir_if_available(tcx, fn_def_id) {
                        // Get the self local (first argument, which is _1 in MIR)
                        // _0 is the return value, _1 is the first argument (self)
                        let self_local = rustc_middle::mir::Local::from_usize(1);

                        let unsafe_calls =
                            collect_reachable_unsafe_calls(tcx, fn_def_id, max_call_depth);

                        // skip if no unsafe calls found
                        // since it cannot be a target
                        if unsafe_calls.is_empty() {
                            continue;
                        }

                        // For each unsafe call, extract used fields
                        for unsafe_call in unsafe_calls {
                            let callsite_loc = tcx
                                .sess
                                .source_map()
                                .lookup_char_pos(unsafe_call.callsite_span.lo());
                            let callsite_info = CallsiteInfo {
                                line: callsite_loc.line,
                                col: callsite_loc.col.to_usize() + 1,
                                path: Some(normalize_to_rust_relative(
                                    &callsite_loc.file.name.prefer_local().to_string(),
                                )),
                            };

                            let callee_def_id = unsafe_call.callee_def_id;
                            let callee_path = tcx.def_path_str(callee_def_id);

                            println!(
                                "Found unsafe call to {} in function {}",
                                callee_path, fn_info.name
                            );

                            println!(
                                "  Number of argument places: {}",
                                unsafe_call.arg_places.len()
                            );

                            // If fn_def_id is constructor, add suspect and return (no mutators needed since it's already a constructor)
                            if constructors.iter().any(|ctor| ctor.name == fn_info.name)
                                || unsafe_call.depth > 0
                            {
                                let suspect = Suspect {
                                    caller_parent: Some(get_struct_info(tcx, def_id)),
                                    caller: fn_info.clone(),
                                    callsite: callsite_info.clone(),
                                    callee: get_fn_info(tcx, callee_def_id),
                                    callee_type_args: collect_callee_type_args(
                                        tcx,
                                        callee_def_id,
                                        unsafe_call.callee_args,
                                    ),
                                    unsafe_call_used_fields: vec![],
                                    unsafe_call_used_params: vec![],
                                    unsafe_call_used_globals: vec![],
                                    unsafe_call_control_fields: vec![],
                                    unsafe_call_control_params: vec![],
                                    unsafe_call_control_globals: vec![],
                                    constructors: vec![],
                                    mutators: vec![],
                                };
                                targets.push(suspect);
                                continue;
                            }

                            // Analyze data dependencies
                            let mut data_visitor =
                                DataDependencyVisitor::new(tcx, self_local, body);

                            for (i, place) in unsafe_call.arg_places.iter().enumerate() {
                                let from_self = place.local == self_local;
                                let derived_self =
                                    data_visitor.derived_from_self.contains_key(&place.local);
                                let derived_params =
                                    data_visitor.derived_from_params.contains_key(&place.local);
                                let derived_globals =
                                    data_visitor.derived_from_globals.contains_key(&place.local);
                                println!(
                                    "  Arg {} (from_self={}, derived_self={}, derived_params={}, derived_globals={}): {:?}",
                                    i,
                                    from_self,
                                    derived_self,
                                    derived_params,
                                    derived_globals,
                                    place
                                );
                                data_visitor.extract_dependencies_from_place(*place);
                            }

                            let used_fields: Vec<String> =
                                data_visitor.self_fields.iter().cloned().collect();
                            let used_params: Vec<usize> =
                                data_visitor.params.iter().cloned().collect();
                            let used_globals: Vec<String> = data_visitor
                                .globals
                                .iter()
                                .map(|def_id| tcx.def_path_str(*def_id))
                                .collect();
                            println!("  Used fields: {:?}", used_fields);
                            println!("  Used params: {:?}", used_params);
                            println!("  Used globals: {:?}", used_globals);

                            // Analyze control dependencies
                            let mut control_visitor = ControlDependencyVisitor::new(
                                tcx,
                                body,
                                unsafe_call.location,
                                self_local,
                            );
                            control_visitor.analyze();

                            let control_fields: Vec<String> = control_visitor
                                .control_self_fields
                                .iter()
                                .cloned()
                                .collect();
                            let control_params: Vec<usize> =
                                control_visitor.control_params.iter().cloned().collect();
                            let control_globals: Vec<String> = control_visitor
                                .control_globals
                                .iter()
                                .map(|def_id| tcx.def_path_str(*def_id))
                                .collect();
                            println!("  Control fields: {:?}", control_fields);
                            println!("  Control params: {:?}", control_params);
                            println!("  Control globals: {:?}", control_globals);

                            // Find mutators for the used fields
                            let mut mutators = Vec::new();
                            let target_fields: HashSet<String> =
                                used_fields.iter().cloned().collect();

                            // 1. setter: self.xxx = ...
                            let setters =
                                collect_fields_setters(tcx, def_id, target_fields.clone());
                            println!("  Found {} setters", setters.len());
                            for setter in &setters {
                                println!("    Setter: {}", setter.name);
                            }
                            mutators.extend(setters);

                            // 2. return &mut self.xxx or &mut self
                            let emr = collect_escaped_mut_refs(tcx, def_id, target_fields.clone());
                            println!("  Found {} mut ref returns", emr.len());
                            for m in &emr {
                                println!("    Mut ref return: {}", m.name);
                            }
                            mutators.extend(emr);

                            // 3. return A { xxx: &mut self.xxx, } or A { xxx: &mut self }
                            let mria = collect_escaped_mut_refs_in_aggregates(
                                tcx,
                                def_id,
                                target_fields.clone(),
                                vec![],
                            );
                            println!("  Found {} aggregate mut ref returns", mria.len());
                            for m in &mria {
                                println!("    Aggregate mut ref return: {}", m.name);
                            }
                            mutators.extend(mria);

                            // Create suspect
                            let suspect = Suspect {
                                caller_parent: Some(get_struct_info(tcx, def_id)),
                                caller: fn_info.clone(),
                                callsite: callsite_info,
                                callee: get_fn_info(tcx, callee_def_id),
                                callee_type_args: collect_callee_type_args(
                                    tcx,
                                    callee_def_id,
                                    unsafe_call.callee_args,
                                ),
                                unsafe_call_used_fields: used_fields,
                                unsafe_call_used_params: used_params,
                                unsafe_call_used_globals: used_globals,
                                unsafe_call_control_fields: control_fields,
                                unsafe_call_control_params: control_params,
                                unsafe_call_control_globals: control_globals,
                                constructors: constructors.clone(),
                                mutators,
                            };

                            targets.push(suspect);
                        }
                    }
                }
            }
        }
    }

    // Also scan public safe free functions in the crate (non-methods).
    for local_def_id in tcx.hir_crate_items(()).definitions() {
        let def_id = local_def_id.to_def_id();
        if !matches!(tcx.def_kind(def_id), rustc_hir::def::DefKind::Fn) {
            continue;
        }

        if is_fn_unsafe(tcx, def_id) || !tcx.visibility(def_id).is_public() {
            continue;
        }

        let require_template = requires_template(tcx, def_id, None);
        let fn_info = get_fn_info_with_template_flag(tcx, def_id, require_template);

        if def_id.as_local().is_some() {
            let unsafe_calls = collect_reachable_unsafe_calls(tcx, def_id, max_call_depth);

            if unsafe_calls.is_empty() {
                continue;
            }

            for unsafe_call in unsafe_calls {
                let callsite_loc = tcx
                    .sess
                    .source_map()
                    .lookup_char_pos(unsafe_call.callsite_span.lo());
                let callsite_info = CallsiteInfo {
                    line: callsite_loc.line,
                    col: callsite_loc.col.to_usize() + 1,
                    path: Some(normalize_to_rust_relative(
                        &callsite_loc.file.name.prefer_local().to_string(),
                    )),
                };

                let suspect = Suspect {
                    caller_parent: None,
                    caller: fn_info.clone(),
                    callsite: callsite_info,
                    callee: get_fn_info(tcx, unsafe_call.callee_def_id),
                    callee_type_args: collect_callee_type_args(
                        tcx,
                        unsafe_call.callee_def_id,
                        unsafe_call.callee_args,
                    ),
                    unsafe_call_used_fields: vec![],
                    unsafe_call_used_params: vec![],
                    unsafe_call_used_globals: vec![],
                    unsafe_call_control_fields: vec![],
                    unsafe_call_control_params: vec![],
                    unsafe_call_control_globals: vec![],
                    constructors: vec![],
                    mutators: vec![],
                };

                targets.push(suspect);
            }
        }
    }

    let trait_methods = collect_trait_methods(tcx);

    Report {
        targets,
        types,
        trait_methods,
        affected_fields,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_driver::{Callbacks, Compilation};
    use rustc_interface::interface::Config;
    use rustc_middle::ty::{Ty, TyCtxt};
    use rustc_session::config as sess_config;
    use rustc_span::FileName;

    struct TestAuditCallbacks {
        src: String,
        report: Option<Report>,
    }

    impl TestAuditCallbacks {
        fn new(src: String) -> Self {
            TestAuditCallbacks { src, report: None }
        }
    }

    impl Callbacks for TestAuditCallbacks {
        fn config(&mut self, cfg: &mut Config) {
            cfg.input = sess_config::Input::Str {
                name: FileName::Custom("test.rs".into()),
                input: self.src.clone(),
            };
        }

        fn after_analysis<'tcx>(
            &mut self,
            _: &rustc_interface::interface::Compiler,
            tcx: TyCtxt<'tcx>,
        ) -> Compilation {
            if tcx.sess.dcx().has_errors_or_delayed_bugs().is_some() {
                tcx.dcx()
                    .fatal("raudit cannot be run on programs that fail compilation");
            }

            let report = audit(tcx);
            self.report = Some(report);
            Compilation::Stop
        }
    }

    fn run_audit(src: &str) -> Report {
        let mut callbacks = TestAuditCallbacks::new(src.to_string());

        // Pass ordinary rustc args. Including a dummy input path keeps arg parsing happy;
        // `config()` overrides the real input with our string.
        let mut args = vec![
            "rustc".into(),
            "test.rs".into(),
            "--crate-name".into(),
            "under_test".into(),
            // compile to library to avoid warnings about missing main
            "--crate-type".into(),
            "lib".into(),
            "--edition=2021".into(),
            "--emit=metadata".into(),
        ];
        if let Ok(sysroot) = std::env::var("MIRSCAN_SYSROOT") {
            args.push("--sysroot".into());
            args.push(sysroot);
        }

        // Run the compiler with our callbacks.
        let exit = rustc_driver::catch_with_exit_code(|| {
            rustc_driver::run_compiler(&args, &mut callbacks);
        });

        assert_eq!(exit, 0);
        callbacks.report.expect("Report should be generated")
    }

    #[test]
    fn test_audit_basic() {
        // Test a struct with safe public method that calls unsafe functions
        let src = r#"
            pub struct MyStruct {
                data: *mut u8,
                len: usize,
            }
            
            impl MyStruct {
                pub fn new(capacity: usize) -> Self {
                    unsafe {
                        let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
                        let data = std::alloc::alloc(layout);
                        MyStruct { data, len: capacity }
                    }
                }
                
                pub fn get(&self, index: usize) -> u8 {
                    unsafe {
                        *self.data.add(index)
                    }
                }
                
                pub fn set(&mut self, index: usize, value: u8) {
                    unsafe {
                        *self.data.add(index) = value;
                    }
                }
            }
        "#;

        let report = run_audit(src);

        // Should find suspects in get() and set() methods
        assert!(
            !report.targets.is_empty(),
            "Should find at least one suspect"
        );

        // Check that we have the right structure
        for suspect in &report.targets {
            println!("Suspect: {}", suspect.caller.name);
            println!("  Unsafe call: {}", suspect.callee.name);
            println!("  Used fields: {:?}", suspect.unsafe_call_used_fields);
            println!(
                "  Constructors: {:?}",
                suspect
                    .constructors
                    .iter()
                    .map(|f| &f.name)
                    .collect::<Vec<_>>()
            );
            println!(
                "  Mutators: {:?}",
                suspect.mutators.iter().map(|f| &f.name).collect::<Vec<_>>()
            );
        }

        // Verify we found the new() constructor
        let has_constructor = report
            .targets
            .iter()
            .any(|s| s.constructors.iter().any(|c| c.name.contains("new")));
        assert!(has_constructor, "Should find new() as a constructor");
    }

    #[test]
    fn test_audit_no_unsafe() {
        // Test a struct with no unsafe calls
        let src = r#"
            pub struct SafeStruct {
                value: i32,
            }
            
            impl SafeStruct {
                pub fn new(value: i32) -> Self {
                    SafeStruct { value }
                }
                
                pub fn get(&self) -> i32 {
                    self.value
                }
                
                pub fn set(&mut self, value: i32) {
                    self.value = value;
                }
            }
        "#;

        let report = run_audit(src);

        // Should find no suspects since there are no unsafe calls
        assert!(
            report.targets.is_empty(),
            "Should find no suspects in safe code"
        );
    }

    #[test]
    fn test_audit_return_mut_ref() {
        // Test a struct with a method that returns &mut to a field, which should be flagged as a mutator
        let src = r#"
            pub struct MyStruct {
                value: i32,
            }

            pub struct MyIter<'a> {
                sss: i32,
                value: &'a mut i32,
            }

            impl MyStruct {
                pub fn new(value: i32) -> Self {
                    MyStruct { value }
                }

                pub fn get_mut(&mut self) -> MyIter {
                    MyIter { sss: 42, value: &mut self.value }
                }

                pub fn get(&self) -> i32 {
                    unsafe {
                        let a = &self.value as *const i32;
                        let b = a.add(0);
                    }
                    self.value
                }
            }

            impl<'a> MyIter<'a> {
                pub fn next(&mut self) -> &mut i32 {
                    self.value
                }
            }
        "#;

        let report = run_audit(src);

        for suspect in &report.targets {
            println!("Suspect: {}", suspect.caller.name);
            println!("  Unsafe call: {}", suspect.callee.name);
            println!("  Used fields: {:?}", suspect.unsafe_call_used_fields);
            println!(
                "  Constructors: {:?}",
                suspect
                    .constructors
                    .iter()
                    .map(|f| &f.name)
                    .collect::<Vec<_>>()
            );
            for mutator in &suspect.mutators {
                println!("  Mutator: {}", mutator.name);
                println!("    Call chains: {:?}", mutator.call_chains);
            }
        }

        // Should find a suspect for get_mut() and it should be flagged as a mutator
        assert!(
            !report.targets.is_empty(),
            "Should find at least one suspect"
        );
        let suspect = &report.targets[0];
        assert!(
            suspect.mutators.iter().any(|m| m.name.contains("next")),
            "next() should be identified as a mutator"
        );
    }

    #[test]
    fn test_audit_field_setters() {
        // Test that field setters are correctly identified as mutators
        let src = r#"
            pub struct Buffer {
                data: *mut u8,
                len: usize,
            }
            
            impl Buffer {
                pub fn new(capacity: usize) -> Self {
                    unsafe {
                        let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
                        let data = std::alloc::alloc(layout);
                        Buffer { data, len: capacity }
                    }
                }
                
                pub fn set_data(&mut self, new_data: *mut u8) {
                    self.data = new_data;
                }
                
                pub fn set_len(&mut self, new_len: usize) {
                    self.len = new_len;
                }
                
                pub fn read(&self, index: usize) -> u8 {
                    unsafe {
                        *self.data.add(index)
                    }
                }
            }
        "#;

        let report = run_audit(src);

        for suspect in &report.targets {
            println!("Suspect: {}", suspect.caller.name);
            println!("  Unsafe call: {}", suspect.callee.name);
            println!("  Used fields: {:?}", suspect.unsafe_call_used_fields);
            println!(
                "  Mutators: {:?}",
                suspect.mutators.iter().map(|f| &f.name).collect::<Vec<_>>()
            );
        }

        assert_eq!(report.targets.len(), 2, "suspect should be 3");

        let suspect = &report.targets[1];
        // Verify we found set_data as a mutator since it modifies 'data' field
        assert!(
            suspect.mutators.iter().any(|m| m.name.contains("set_data")),
            "set_data() should be identified as a mutator for the data field"
        );
    }

    #[test]
    fn test_audit_direct_mut_ref_return() {
        // Test that functions returning &mut self.field are correctly identified
        let src = r#"
            pub struct Config {
                buffer: *mut u8,
                size: usize,
            }
            
            impl Config {
                pub fn new(capacity: usize) -> Self {
                    unsafe {
                        let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
                        let buffer = std::alloc::alloc(layout);
                        Config { buffer, size: capacity }
                    }
                }
                
                pub fn buffer_mut(&mut self) -> &mut *mut u8 {
                    &mut self.buffer
                }
                
                pub fn size_mut(&mut self) -> &mut usize {
                    &mut self.size
                }
                
                pub fn access(&self, offset: usize) -> u8 {
                    unsafe {
                        *self.buffer.add(offset)
                    }
                }
            }
        "#;

        let report = run_audit(src);

        for suspect in &report.targets {
            println!("Suspect: {}", suspect.caller.name);
            println!("  Unsafe call: {}", suspect.callee.name);
            println!("  Used fields: {:?}", suspect.unsafe_call_used_fields);
            println!(
                "  Mutators: {:?}",
                suspect.mutators.iter().map(|f| &f.name).collect::<Vec<_>>()
            );
        }

        // Should find suspect in access() method
        assert_eq!(report.targets.len(), 2, "Should find 2 suspects");

        let suspect = &report.targets[1];
        // Verify we found buffer_mut as a mutator since it returns &mut to buffer field
        assert!(
            suspect
                .mutators
                .iter()
                .any(|m| m.name.contains("buffer_mut")),
            "buffer_mut() should be identified as a mutator returning &mut to buffer field"
        );
    }

    #[test]
    fn test_audit_multiple_mutators() {
        // Test detection of both setters and mut ref returns
        let src = r#"
            pub struct Memory {
                ptr: *mut u8,
                capacity: usize,
            }
            
            impl Memory {
                pub fn new(cap: usize) -> Self {
                    unsafe {
                        let layout = std::alloc::Layout::array::<u8>(cap).unwrap();
                        let ptr = std::alloc::alloc(layout);
                        Memory { ptr, capacity: cap }
                    }
                }
                
                // Setter for ptr
                pub fn update_ptr(&mut self, new_ptr: *mut u8) {
                    self.ptr = new_ptr;
                }
                
                // Returns &mut to ptr
                pub fn ptr_mut(&mut self) -> &mut *mut u8 {
                    &mut self.ptr
                }
                
                // Setter for capacity
                pub fn set_capacity(&mut self, cap: usize) {
                    self.capacity = cap;
                }
                
                pub fn write(&mut self, index: usize, value: u8) {
                    unsafe {
                        *self.ptr.add(index) = value;
                    }
                }
            }
        "#;

        let report = run_audit(src);

        assert!(
            !report.targets.is_empty(),
            "Should find at least one suspect"
        );

        for suspect in &report.targets {
            println!("Suspect: {}", suspect.caller.name);
            println!("  Unsafe call: {}", suspect.callee.name);
            println!("  Used fields: {:?}", suspect.unsafe_call_used_fields);
            println!("  Mutators ({} total):", suspect.mutators.len());
            for mutator in &suspect.mutators {
                println!("    - {}", mutator.name);
            }
        }

        let suspect = report
            .targets
            .iter()
            .find(|s| s.caller.name.contains("write"))
            .expect("Should find write() suspect");
        // Should find both the setter and the mut ref return for ptr field
        assert!(
            suspect
                .mutators
                .iter()
                .any(|m| m.name.contains("update_ptr")),
            "update_ptr() setter should be identified as a mutator"
        );
        assert!(
            suspect.mutators.iter().any(|m| m.name.contains("ptr_mut")),
            "ptr_mut() should be identified as a mutator"
        );

        // Verify we found at least 2 mutators for the ptr field
        let ptr_mutators = suspect
            .mutators
            .iter()
            .filter(|m| m.name.contains("update_ptr") || m.name.contains("ptr_mut"))
            .count();
        assert!(
            ptr_mutators >= 2,
            "Should find at least 2 mutators for ptr field"
        );
    }

    #[test]
    fn test_audit_one_layer_deeper_method() {
        let src = r#"
            pub struct Buffer {
                data: *const u8,
            }

            impl Buffer {
                pub fn read(&self, index: usize) -> u8 {
                    unsafe {
                        self.read_inner(index)
                    }
                }

                unsafe fn read_inner(&self, index: usize) -> u8 {
                    unsafe {
                        *self.data.add(index)
                    }
                }
            }
        "#;

        let report = run_audit(src);

        assert!(
            report.targets.iter().any(|s| s.caller.name.contains("read")
                && !s.caller.name.contains("read_inner")
                && s.callee
                    .name
                    .starts_with("std::ptr::const_ptr::<impl *const T>::add")),
            "Should report unsafe core/std call reached through a private method"
        );
    }

    #[test]
    fn test_audit_one_layer_deeper_free_function() {
        let src = r#"
            pub fn read(ptr: *const u8, index: usize) -> u8 {
                unsafe {
                    read_inner(ptr, index)
                }
            }

            unsafe fn read_inner(ptr: *const u8, index: usize) -> u8 {
                unsafe {
                    *ptr.add(index)
                }
            }
        "#;

        let report = run_audit(src);

        assert!(
            report
                .targets
                .iter()
                .any(|s| s.caller.name.ends_with("read")
                    && !s.caller.name.ends_with("read_inner")
                    && s.callee
                        .name
                        .starts_with("std::ptr::const_ptr::<impl *const T>::add")),
            "Should report unsafe core/std call reached through a private free function"
        );
    }

    #[test]
    fn test_audit_does_not_expand_safe_helper() {
        let src = r#"
            pub fn read(ptr: *const u8, index: usize) -> u8 {
                read_inner(ptr, index)
            }

            fn read_inner(ptr: *const u8, index: usize) -> u8 {
                unsafe {
                    *ptr.add(index)
                }
            }
        "#;

        let report = run_audit(src);

        assert!(
            report.targets.is_empty(),
            "Should not expand through safe helper functions"
        );
    }

    #[test]
    fn test_reports_general_trait_implementors() {
        let report = run_audit(
            r#"
                pub trait Source {
                    fn read(&self) -> usize;
                }

                pub struct Counter(pub usize);

                impl Source for Counter {
                    fn read(&self) -> usize {
                        self.0
                    }
                }
            "#,
        );

        let method = report
            .trait_methods
            .iter()
            .find(|method| method.method_name == "read")
            .expect("trait implementation should be reported");
        assert!(method.trait_name.ends_with("Source"));
        assert!(method.implementor_type.ends_with("Counter"));
        assert_eq!(method.return_ty, "usize");
    }

    #[test]
    fn test_reports_callee_type_argument_layout_control() {
        let report = run_audit(
            r#"
                pub fn generic<T>(ptr: *mut T) {
                    unsafe { let _ = ptr.add(1); }
                }

                pub fn fixed(ptr: *mut u64) {
                    unsafe { let _ = ptr.add(1); }
                }

                #[inline(never)]
                unsafe fn helper<T>(ptr: *mut T) -> *mut T {
                    ptr.add(1)
                }

                pub fn fixed_through_helper(ptr: *mut u32) -> *mut u32 {
                    unsafe { helper::<u32>(ptr) }
                }

                pub fn multiple<T>(left: *const T, right: *const u8) -> isize {
                    unsafe { left.byte_offset_from(right) }
                }
            "#,
        );

        let generic = report
            .targets
            .iter()
            .find(|target| target.caller.name.ends_with("generic"))
            .expect("generic pointer call should be reported");
        assert!(matches!(
            generic.callee_type_args[0].layout_control,
            TypeLayoutControl::External
        ));
        assert_eq!(generic.callee_type_args[0].name, "T");

        let fixed = report
            .targets
            .iter()
            .find(|target| target.caller.name.ends_with("fixed"))
            .expect("fixed pointer call should be reported");
        assert!(matches!(
            fixed.callee_type_args[0].layout_control,
            TypeLayoutControl::Fixed
        ));
        assert_eq!(fixed.callee_type_args[0].instantiated_ty, "u64");

        let through_helper = report
            .targets
            .iter()
            .find(|target| target.caller.name.ends_with("fixed_through_helper"))
            .expect("nested fixed pointer call should be reported");
        assert!(matches!(
            through_helper.callee_type_args[0].layout_control,
            TypeLayoutControl::Fixed
        ));
        assert_eq!(
            through_helper.callee_type_args[0].instantiated_ty,
            "u32"
        );

        let multiple = report
            .targets
            .iter()
            .find(|target| target.caller.name.ends_with("multiple"))
            .expect("multi-type pointer call should be reported");
        assert_eq!(multiple.callee_type_args.len(), 2);
        assert!(matches!(
            multiple.callee_type_args[0].layout_control,
            TypeLayoutControl::External
        ));
        assert!(matches!(
            multiple.callee_type_args[1].layout_control,
            TypeLayoutControl::Fixed
        ));
        assert_eq!(multiple.callee_type_args[1].instantiated_ty, "u8");
    }

    #[test]
    fn test_required_trait_method_without_mir_is_skipped() {
        let report = run_audit(
            r#"
                pub trait DynamicBundle {
                    unsafe fn put(
                        self,
                        callback: impl FnMut(*mut u8),
                    );
                }

                pub fn invoke<B: DynamicBundle>(bundle: B) {
                    unsafe { bundle.put(|_| {}); }
                }
            "#,
        );

        assert!(
            report.targets.is_empty(),
            "a required local trait method has no MIR body to traverse"
        );
    }

    #[test]
    fn test_affected_fields_nested_index_alias_and_collection_call() {
        let report = run_audit(
            r#"
                pub struct Leaf { pub x: usize }
                pub struct Root {
                    pub leaf: Leaf,
                    pub array: [Leaf; 2],
                    pub items: Vec<Leaf>,
                }

                impl Root {
                    pub(crate) fn mutate(&mut self) {
                        self.leaf.x = 1;
                        self.array[0].x = 2;
                        let alias = &mut self.leaf;
                        alias.x = 3;
                        self.items.push(Leaf { x: 4 });
                        self.items[0].x = 5;
                    }

                    fn private_mutate(&mut self) {
                        self.leaf.x = 5;
                    }
                }
            "#,
        );

        let affected = report
            .affected_fields
            .iter()
            .find(|item| item.name.ends_with("Root::mutate"))
            .expect("publicly visible mutator should be reported");
        assert_eq!(affected.root_types, ["Root"]);
        assert!(affected.fields_written.contains(&"Root.leaf.x".to_string()));
        assert!(
            affected
                .fields_written
                .contains(&"Root.array[*].x".to_string())
        );
        assert!(affected.fields_written.contains(&"Root.items".to_string()));
        assert!(
            affected
                .fields_written
                .contains(&"Root.items[*].x".to_string())
        );
        assert!(
            !report
                .affected_fields
                .iter()
                .any(|item| item.name.ends_with("private_mutate"))
        );
    }

    #[test]
    fn test_affected_fields_by_value_requires_same_return_type() {
        let report = run_audit(
            r#"
                pub struct Root { pub x: usize }

                pub fn returned(mut root: Root) -> Root {
                    root.x = 1;
                    root
                }

                pub fn consumed(mut root: Root) {
                    root.x = 2;
                }
            "#,
        );

        let returned = report
            .affected_fields
            .iter()
            .find(|item| item.name.ends_with("returned"))
            .expect("returned by-value ADT should be a root");
        assert_eq!(returned.fields_written, ["Root.x"]);
        assert!(
            !report
                .affected_fields
                .iter()
                .any(|item| item.name.ends_with("consumed"))
        );
    }

    #[test]
    fn test_affected_fields_reports_aggregate_mut_escape_and_trait_writes() {
        let report = run_audit(
            r#"
                pub struct Item { pub x: usize }
                pub struct Store { pub items: Vec<Item> }
                pub struct Root { store: Store }
                pub struct Wrapper<'a> { store: &'a mut Store }

                impl Root {
                    pub fn escape(&mut self) -> Wrapper<'_> {
                        Wrapper { store: &mut self.store }
                    }

                    pub fn store_mut(&mut self) -> &mut Store {
                        &mut self.store
                    }
                }

                impl Iterator for Wrapper<'_> {
                    type Item = ();

                    fn next(&mut self) -> Option<()> {
                        self.store.items[0].x = 1;
                        Some(())
                    }
                }
            "#,
        );

        let escape = report
            .affected_fields
            .iter()
            .find(|item| item.name.ends_with("Root::escape"))
            .expect("aggregate mutable-reference escape should be reported");
        assert_eq!(escape.escaped_fields.len(), 1);
        assert_eq!(escape.escaped_fields[0].source_path, "Root.store");
        assert_eq!(escape.escaped_fields[0].wrapper_type, "Wrapper");
        assert_eq!(escape.escaped_fields[0].wrapper_field, "store");
        assert_eq!(escape.escaped_fields[0].target_type, "Store");
        assert!(!escape.escaped_fields[0].target_is_template);

        let direct = report
            .affected_fields
            .iter()
            .find(|item| item.name.ends_with("Root::store_mut"))
            .expect("direct mutable-reference escape should be reported");
        assert!(direct.is_public);
        assert_eq!(direct.escaped_fields.len(), 1);
        assert_eq!(direct.escaped_fields[0].source_path, "Root.store");
        assert!(direct.escaped_fields[0].wrapper_type.is_empty());
        assert!(direct.escaped_fields[0].wrapper_field.is_empty());
        assert_eq!(direct.escaped_fields[0].target_type, "Store");

        let next = report
            .affected_fields
            .iter()
            .find(|item| item.name.ends_with("::next"))
            .unwrap_or_else(|| {
                panic!(
                    "public trait implementation method should be reported: {:?}",
                    report
                        .affected_fields
                        .iter()
                        .map(|item| &item.name)
                        .collect::<Vec<_>>()
                )
            });
        assert!(
            next.fields_written
                .contains(&"Wrapper.store.items[*].x".to_string())
        );
    }
}
