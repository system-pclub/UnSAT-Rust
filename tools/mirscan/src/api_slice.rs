use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Operand, TerminatorKind};
use rustc_middle::ty::{self, Ty, TyCtxt, TypeVisitableExt};
use rustc_span::Span;
use serde::Serialize;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Serialize)]
pub struct ApiSlice {
    pub schema_version: u32,
    pub requested_functions: Vec<String>,
    pub unresolved_functions: Vec<String>,
    pub functions: Vec<FunctionItem>,
    pub types: Vec<TypeItem>,
}

#[derive(Debug, Serialize)]
pub struct FunctionItem {
    pub name: String,
    pub role: &'static str,
    pub path: String,
    pub line: usize,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub local_callees: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TypeItem {
    pub name: String,
    pub path: String,
    pub line: usize,
    pub definition: String,
}

fn local_definitions(tcx: TyCtxt<'_>, kinds: &[DefKind]) -> Vec<DefId> {
    tcx.hir_crate_items(())
        .definitions()
        .map(|id| id.to_def_id())
        .filter(|id| kinds.contains(&tcx.def_kind(*id)))
        .collect()
}

fn normalized_name(value: &str) -> String {
    let mut depth = 0usize;
    let mut stripped = String::new();
    for ch in value.chars() {
        match ch {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => stripped.push(ch),
            _ => {}
        }
    }
    stripped
        .replace("::::", "::")
        .trim()
        .trim_start_matches("crate::")
        .to_string()
}

fn function_matches(path: &str, requested: &str) -> bool {
    let path = normalized_name(path);
    let requested = normalized_name(requested);
    path == requested
        || path.ends_with(&format!("::{requested}"))
        || requested.ends_with(&format!("::{path}"))
}

fn source_location(tcx: TyCtxt<'_>, span: Span) -> (String, usize) {
    let location = tcx
        .sess
        .source_map()
        .lookup_char_pos(span.source_callsite().lo());
    let absolute = location.file.name.prefer_local().to_string().replace('\\', "/");
    let path = std::env::current_dir()
        .ok()
        .and_then(|cwd| {
            let cwd = cwd.to_string_lossy().replace('\\', "/");
            absolute
                .strip_prefix(&format!("{cwd}/"))
                .map(ToOwned::to_owned)
        })
        .unwrap_or(absolute);
    (path, location.line)
}

fn item_span(tcx: TyCtxt<'_>, def_id: DefId, with_body: bool) -> Span {
    let Some(local) = def_id.as_local() else {
        return tcx.def_span(def_id);
    };
    if with_body && tcx.hir_maybe_body_owned_by(local).is_some() {
        tcx.hir()
            .span_with_body(tcx.local_def_id_to_hir_id(local))
    } else {
        match tcx.hir_node_by_def_id(local) {
            rustc_hir::Node::Item(item) => item.span,
            rustc_hir::Node::ImplItem(item) => item.span,
            rustc_hir::Node::TraitItem(item) => item.span,
            _ => tcx.def_span(def_id),
        }
    }
}

fn snippet(tcx: TyCtxt<'_>, span: Span) -> String {
    tcx.sess
        .source_map()
        .span_to_snippet(span)
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn signature_from_source(source: &str) -> String {
    let mut depth = 0usize;
    for (offset, ch) in source.char_indices() {
        match ch {
            '(' | '[' | '<' => depth += 1,
            ')' | ']' | '>' => depth = depth.saturating_sub(1),
            '{' if depth == 0 => return source[..offset].trim().to_string(),
            _ => {}
        }
    }
    source.lines().next().unwrap_or(source).trim().to_string()
}

fn local_callees(tcx: TyCtxt<'_>, def_id: DefId) -> Vec<DefId> {
    if !tcx.is_mir_available(def_id) {
        return Vec::new();
    }
    let Some(local) = def_id.as_local() else {
        return Vec::new();
    };
    if tcx.hir_maybe_body_owned_by(local).is_none() {
        return Vec::new();
    }
    let body = tcx.optimized_mir(local);
    let mut result = HashSet::new();
    for block in body.basic_blocks.iter() {
        let TerminatorKind::Call { func, .. } = &block.terminator().kind else {
            continue;
        };
        let ty = match func {
            Operand::Copy(place) | Operand::Move(place) => place.ty(body, tcx).ty,
            Operand::Constant(value) => value.const_.ty(),
        };
        if let ty::FnDef(callee, _) = *ty.kind()
            && callee.as_local().is_some()
        {
            result.insert(callee);
        }
    }
    let mut result: Vec<_> = result.into_iter().collect();
    result.sort_by_key(|id| tcx.def_path_str(*id));
    result
}

fn add_local_types_from_ty(tcx: TyCtxt<'_>, ty: Ty<'_>, out: &mut HashSet<DefId>) {
    for arg in ty.walk() {
        let Some(ty) = arg.as_type() else {
            continue;
        };
        if let ty::Adt(def, _) = ty.kind()
            && def.did().as_local().is_some()
        {
            out.insert(def.did());
        }
    }
}

fn add_function_types(tcx: TyCtxt<'_>, def_id: DefId, out: &mut HashSet<DefId>) {
    let signature = tcx.fn_sig(def_id).instantiate_identity().skip_binder();
    for ty in signature.inputs_and_output {
        add_local_types_from_ty(tcx, ty, out);
    }
    if tcx.is_mir_available(def_id)
        && let Some(local) = def_id.as_local()
        && tcx.hir_maybe_body_owned_by(local).is_some()
    {
        for declaration in tcx.optimized_mir(local).local_decls.iter() {
            add_local_types_from_ty(tcx, declaration.ty, out);
        }
    }
}

fn close_type_fields(tcx: TyCtxt<'_>, types: &mut HashSet<DefId>) {
    let mut queue: VecDeque<_> = types.iter().copied().collect();
    let mut expanded = HashSet::new();
    while let Some(def_id) = queue.pop_front() {
        if !expanded.insert(def_id) {
            continue;
        }
        let Some(adt) = tcx.type_of(def_id).instantiate_identity().ty_adt_def() else {
            continue;
        };
        let before: HashSet<_> = types.clone();
        for variant in adt.variants() {
            for field in &variant.fields {
                add_local_types_from_ty(
                    tcx,
                    tcx.type_of(field.did).instantiate_identity(),
                    types,
                );
            }
        }
        queue.extend(types.difference(&before).copied());
    }
}

fn associated_type(tcx: TyCtxt<'_>, function: DefId) -> Option<DefId> {
    let impl_id = tcx.impl_of_method(function)?;
    match tcx.type_of(impl_id).instantiate_identity().kind() {
        ty::Adt(def, _) if def.did().as_local().is_some() => Some(def.did()),
        _ => None,
    }
}

pub fn analyze(tcx: TyCtxt<'_>, requested_functions: Vec<String>) -> ApiSlice {
    let all_functions = local_definitions(tcx, &[DefKind::Fn, DefKind::AssocFn]);
    let mut seed_functions = HashSet::new();
    let mut unresolved_functions = Vec::new();
    for requested in &requested_functions {
        let matches: Vec<_> = all_functions
            .iter()
            .copied()
            .filter(|id| function_matches(&tcx.def_path_str(*id), requested))
            .collect();
        if matches.is_empty() {
            unresolved_functions.push(requested.clone());
        } else {
            seed_functions.extend(matches);
        }
    }

    let mut call_functions = seed_functions.clone();
    let mut call_edges: HashMap<DefId, Vec<DefId>> = HashMap::new();
    let mut queue: VecDeque<_> = seed_functions.iter().copied().collect();
    while let Some(function) = queue.pop_front() {
        let callees = local_callees(tcx, function);
        for callee in &callees {
            if call_functions.insert(*callee) {
                queue.push_back(*callee);
            }
        }
        call_edges.insert(function, callees);
    }

    let mut related_types = HashSet::new();
    for function in &call_functions {
        add_function_types(tcx, *function, &mut related_types);
    }
    close_type_fields(tcx, &mut related_types);

    let api_functions: HashSet<_> = all_functions
        .iter()
        .copied()
        .filter(|function| {
            associated_type(tcx, *function)
                .is_some_and(|ty| related_types.contains(&ty))
        })
        .collect();

    let mut included_functions = call_functions.clone();
    included_functions.extend(api_functions);
    let mut function_ids: Vec<_> = included_functions.into_iter().collect();
    function_ids.sort_by_key(|id| tcx.def_path_str(*id));
    let functions = function_ids
        .into_iter()
        .map(|def_id| {
            let full_source = snippet(tcx, item_span(tcx, def_id, true));
            let span = item_span(tcx, def_id, false);
            let (path, line) = source_location(tcx, span);
            let callees = call_edges.get(&def_id).cloned().unwrap_or_default();
            FunctionItem {
                name: tcx.def_path_str(def_id),
                role: if seed_functions.contains(&def_id) {
                    "requested"
                } else if call_functions.contains(&def_id) {
                    "call_chain"
                } else {
                    "related_type_api"
                },
                path,
                line,
                signature: signature_from_source(&full_source),
                body: call_functions.contains(&def_id).then_some(full_source),
                local_callees: callees
                    .into_iter()
                    .map(|id| tcx.def_path_str(id))
                    .collect(),
            }
        })
        .collect();

    let mut type_ids: Vec<_> = related_types.into_iter().collect();
    type_ids.sort_by_key(|id| tcx.def_path_str(*id));
    let types = type_ids
        .into_iter()
        .map(|def_id| {
            let span = item_span(tcx, def_id, false);
            let (path, line) = source_location(tcx, span);
            TypeItem {
                name: tcx.def_path_str(def_id),
                path,
                line,
                definition: snippet(tcx, span),
            }
        })
        .collect();

    ApiSlice {
        schema_version: 1,
        requested_functions,
        unresolved_functions,
        functions,
        types,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_matching_ignores_generics_and_crate_prefix() {
        assert!(function_matches("stack::Stack::<T>::dup", "Stack::dup"));
        assert!(function_matches("stack::Stack<T>::dup", "crate::stack::Stack::dup"));
        assert!(function_matches("stack::Stack::dup", "Stack::dup"));
        assert!(function_matches("stack::Stack::dup", "crate::stack::Stack::dup"));
        assert!(!function_matches("stack::Stack::push", "Stack::dup"));
    }

    #[test]
    fn signature_stops_at_body_brace_outside_generics() {
        let source = "pub fn f<T: Fn() -> [u8; 2]>(x: [u8; 1]) -> Option<T> where T: Copy { body }";
        assert_eq!(
            signature_from_source(source),
            "pub fn f<T: Fn() -> [u8; 2]>(x: [u8; 1]) -> Option<T> where T: Copy"
        );
        assert_eq!(signature_from_source("fn g();"), "fn g();");
    }
}
