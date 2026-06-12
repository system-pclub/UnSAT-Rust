use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{BasicBlock, Body, Location, Place, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_span::Pos;
use rustc_span::Span;
use rustc_span::sym;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec;
use walkdir::WalkDir;
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
    pub has_template_in_test: bool,
    #[serde(skip)]
    pub call_chains: Vec<String>, // e.g. ["fn_a -> fn_b "], only for mutators across multiple struct functions
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructInfo {
    pub name: String,
    pub path: String,
    pub line_start: usize,
    pub body_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallsiteInfo {
    pub line: usize,
    pub col: usize,
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
    pub constructors: Vec<FnInfo>,
    pub mutators: Vec<FnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub targets: Vec<Suspect>,
    #[serde(default)]
    pub types: Vec<TypeInteractionInfo>,
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

// Visitor to find all unsafe function calls in a function body
struct UnsafeCallVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    unsafe_calls: Vec<(DefId, Span, Location, Vec<Place<'tcx>>)>, // (callee_def_id, span, mir_location, args)
}

#[derive(Clone)]
struct UnsafeCallSite<'tcx> {
    callee_def_id: DefId,
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

struct CallVisitor {
    calls: Vec<(DefId, Span)>,
}

impl CallVisitor {
    fn new() -> Self {
        Self { calls: Vec::new() }
    }
}

impl<'tcx> Visitor<'tcx> for CallVisitor {
    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: Location,
    ) {
        if let TerminatorKind::Call { func, .. } = &terminator.kind {
            if let Some((def_id, _substs)) = func.const_fn_def() {
                self.calls.push((def_id, terminator.source_info.span));
            }
        }
        self.super_terminator(terminator, location);
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

fn collect_reachable_unsafe_calls<'tcx>(
    tcx: TyCtxt<'tcx>,
    root_def_id: DefId,
    max_call_depth: usize,
) -> Vec<UnsafeCallSite<'tcx>> {
    let mut results = Vec::new();
    let mut queue = VecDeque::from([(root_def_id, 0usize)]);
    let mut visited = HashSet::new();

    while let Some((current_def_id, depth)) = queue.pop_front() {
        if !visited.insert((current_def_id, depth)) {
            continue;
        }

        let Some(local_def_id) = current_def_id.as_local() else {
            continue;
        };
        let body = tcx.optimized_mir(local_def_id);

        let mut unsafe_visitor = UnsafeCallVisitor::new(tcx);
        unsafe_visitor.visit_body(body);
        for (callee_def_id, callsite_span, location, arg_places) in unsafe_visitor.unsafe_calls {
            if is_core_or_std_fn(tcx, callee_def_id) {
                results.push(UnsafeCallSite {
                    callee_def_id,
                    callsite_span,
                    location,
                    arg_places,
                    depth,
                });
            }
        }

        if depth >= max_call_depth {
            continue;
        }

        let mut call_visitor = CallVisitor::new();
        call_visitor.visit_body(body);
        for (callee_def_id, _span) in call_visitor.calls {
            if callee_def_id.as_local().is_some() {
                queue.push_back((callee_def_id, depth + 1));
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
            if let Some((def_id, _substs)) = func.const_fn_def() {
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
        // Iterate through all statements to track which locals are derived from what sources
        for (bb, bb_data) in self.body.basic_blocks.iter_enumerated() {
            for statement in &bb_data.statements {
                if let StatementKind::Assign(assign) = &statement.kind {
                    println!("Analyzing statement in BB {:?}: {:?}", bb, statement);
                    let (place, rvalue) = &**assign;

                    let mut derived_self_fields = HashSet::new();
                    let mut derived_params = HashSet::new();
                    let mut derived_globals = HashSet::new();

                    self.collect_sources_from_rvalue(
                        rvalue,
                        &mut derived_self_fields,
                        &mut derived_params,
                        &mut derived_globals,
                    );

                    println!(
                        "Analyzing assignment to {:?} with derived fields: {:?}, params: {:?}, globals: {:?}",
                        place, derived_self_fields, derived_params, derived_globals
                    );

                    if !derived_self_fields.is_empty() {
                        self.derived_from_self
                            .insert(place.local, derived_self_fields);
                    }
                    if !derived_params.is_empty() {
                        self.derived_from_params.insert(place.local, derived_params);
                    }
                    if !derived_globals.is_empty() {
                        self.derived_from_globals
                            .insert(place.local, derived_globals);
                    }
                }
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
            for elem in place.projection.iter() {
                if let rustc_middle::mir::ProjectionElem::Field(field, _) = elem {
                    let field_idx = format!("{}", field.index());
                    println!("      Found field: {}", field_idx);
                    self_fields.insert(field_idx);
                }
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
                for elem in place.projection.iter() {
                    if let rustc_middle::mir::ProjectionElem::Field(field, _) = elem {
                        let field_name = format!("{}", field.index());
                        if self.target_fields.contains(&field_name) {
                            self.is_setter = true;
                            return;
                        }
                    }
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
    get_fn_info_with_template_flags(tcx, def_id, false, false)
}

fn get_fn_info_with_template_flags<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    require_template: bool,
    has_template_in_test: bool,
) -> FnInfo {
    let span = tcx.def_span(def_id);
    let source_map = tcx.sess.source_map();
    let loc = source_map.lookup_char_pos(span.lo());
    let end_loc = source_map.lookup_char_pos(span.hi());
    let body_span = def_id
        .as_local()
        .map(|local_id| {
            tcx.hir()
                .span_with_body(tcx.local_def_id_to_hir_id(local_id))
        })
        .unwrap_or(span);
    let body_end_loc = source_map.lookup_char_pos(body_span.hi());
    let name = tcx.def_path_str(def_id);
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
        has_template_in_test,
        call_chains: vec![],
    }
}

fn strip_generics(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut depth: i32 = 0;
    for ch in text.chars() {
        match ch {
            '<' => depth += 1,
            '>' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out.replace(":::", "::")
}

fn collect_test_sources() -> Vec<String> {
    let Ok(crate_root) = std::env::current_dir() else {
        return Vec::new();
    };

    let mut sources = Vec::new();
    for entry in WalkDir::new(&crate_root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !entry.file_type().is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let rel = match path.strip_prefix(&crate_root) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if rel.components().any(|c| c.as_os_str() == "target") {
            continue;
        }
        if !rel.starts_with("tests") && !rel.starts_with("src") {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(path) {
            sources.push(content);
        }
    }

    sources
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

fn has_template_instantiation_in_tests(
    caller_name: &str,
    caller_parent_name: Option<&str>,
    test_sources: &[String],
) -> bool {
    let normalized_caller = strip_generics(caller_name);
    let caller_parts: Vec<&str> = normalized_caller
        .split("::")
        .filter(|s| !s.is_empty())
        .collect();

    let mut symbols = Vec::new();
    if let Some(last) = caller_parts.last() {
        symbols.push((*last).to_string());
    }
    if caller_parts.len() >= 2 {
        symbols.push(caller_parts[caller_parts.len() - 2].to_string());
    }

    if let Some(parent) = caller_parent_name {
        let normalized_parent = strip_generics(parent);
        if let Some(seg) = normalized_parent
            .split("::")
            .filter(|s| !s.is_empty())
            .last()
        {
            symbols.push(seg.to_string());
        }
    }

    symbols.sort();
    symbols.dedup();

    for source in test_sources {
        for symbol in &symbols {
            if source.contains(&format!("{symbol}::<")) || source.contains(&format!("{symbol}<")) {
                return true;
            }
        }
    }
    false
}

fn get_struct_info<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> StructInfo {
    let span = tcx.def_span(def_id);
    let source_map = tcx.sess.source_map();
    let loc = source_map.lookup_char_pos(span.lo());
    let body_span = def_id
        .as_local()
        .map(|local_id| {
            tcx.hir()
                .span_with_body(tcx.local_def_id_to_hir_id(local_id))
        })
        .unwrap_or(span);
    let end_loc = source_map.lookup_char_pos(body_span.hi());
    let name = tcx.def_path_str(def_id);
    let path = normalize_to_rust_relative(&loc.file.name.prefer_local().to_string());

    StructInfo {
        name: name.clone(),
        path,
        line_start: loc.line,
        body_end: end_loc.line,
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
            if is_fn_unsafe(tcx, fn_def_id) || !tcx.visibility(fn_def_id).is_public() {
                continue;
            }

            if has_mut_self_receiver(tcx, fn_def_id, struct_def_id) {
                mutators.push(get_fn_info(tcx, fn_def_id));
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

fn collect_public_type_infos<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<TypeInteractionInfo> {
    let mut types = Vec::new();

    for local_def_id in tcx.hir_crate_items(()).definitions() {
        let def_id = local_def_id.to_def_id();

        if !matches!(tcx.def_kind(def_id), rustc_hir::def::DefKind::Struct) {
            continue;
        }
        if !tcx.visibility(def_id).is_public() {
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
            constructors: collect_constructors(tcx, def_id),
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
            if is_fn_unsafe(tcx, fn_def_id) || !tcx.visibility(fn_def_id).is_public() {
                continue;
            }

            // Analyze function body to see if it writes to any of the target fields
            if let Some(local_fn_def_id) = fn_def_id.as_local() {
                let body = tcx.optimized_mir(local_fn_def_id);
                let self_local = rustc_middle::mir::Local::from_usize(1);
                let mut setter_visitor = FieldSetterVisitor::new(tcx, fields.clone(), self_local);
                setter_visitor.visit_body(body);

                if setter_visitor.is_setter {
                    setters.push(get_fn_info(tcx, fn_def_id));
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
            if is_fn_unsafe(tcx, fn_def_id) || !tcx.visibility(fn_def_id).is_public() {
                continue;
            }

            // Analyze function body to see if it returns &mut to any of the target fields
            if let Some(local_fn_def_id) = fn_def_id.as_local() {
                let body = tcx.optimized_mir(local_fn_def_id);
                let self_local = rustc_middle::mir::Local::from_usize(1);
                let mut mutator_visitor =
                    MutRefReturnVisitor::new(tcx, fields.clone(), self_local, body);
                mutator_visitor.visit_body(body);

                if mutator_visitor.returns_mut_ref {
                    let fn_info = get_fn_info(tcx, fn_def_id);
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
                if is_fn_unsafe(tcx, fn_def_id) || !tcx.visibility(fn_def_id).is_public() {
                    continue;
                }

                println!(
                    "Analyzing function {} in type {:?} for aggregate mut ref returns",
                    tcx.def_path_str(fn_def_id),
                    tcx.def_path_str(current_type_def_id)
                );

                if let Some(local_fn_def_id) = fn_def_id.as_local() {
                    let body = tcx.optimized_mir(local_fn_def_id);
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
                            queue.push((return_type_def_id, new_chain.clone(), next_fields));
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
                            fn_info.call_chains = new_chain;
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
    let test_sources = collect_test_sources();
    let mut template_presence_cache: HashMap<(DefId, Option<DefId>), bool> = HashMap::new();
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
                    let has_template_in_test = if !require_template {
                        false
                    } else if let Some(cached) =
                        template_presence_cache.get(&(fn_def_id, Some(def_id)))
                    {
                        *cached
                    } else {
                        let found = has_template_instantiation_in_tests(
                            &tcx.def_path_str(fn_def_id),
                            Some(&tcx.def_path_str(def_id)),
                            &test_sources,
                        );
                        template_presence_cache.insert((fn_def_id, Some(def_id)), found);
                        found
                    };

                    let fn_info = get_fn_info_with_template_flags(
                        tcx,
                        fn_def_id,
                        require_template,
                        has_template_in_test,
                    );

                    // Analyze the function body for unsafe calls
                    if let Some(local_fn_def_id) = fn_def_id.as_local() {
                        let body = tcx.optimized_mir(local_fn_def_id);

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
        let has_template_in_test = if !require_template {
            false
        } else if let Some(cached) = template_presence_cache.get(&(def_id, None)) {
            *cached
        } else {
            let found =
                has_template_instantiation_in_tests(&tcx.def_path_str(def_id), None, &test_sources);
            template_presence_cache.insert((def_id, None), found);
            found
        };

        let fn_info =
            get_fn_info_with_template_flags(tcx, def_id, require_template, has_template_in_test);

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
                };

                let suspect = Suspect {
                    caller_parent: None,
                    caller: fn_info.clone(),
                    callsite: callsite_info,
                    callee: get_fn_info(tcx, unsafe_call.callee_def_id),
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

    Report { targets, types }
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
        let args = vec![
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
                    self.read_inner(index)
                }

                fn read_inner(&self, index: usize) -> u8 {
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
}
