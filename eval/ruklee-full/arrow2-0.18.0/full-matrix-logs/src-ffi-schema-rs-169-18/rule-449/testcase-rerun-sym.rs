#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-ffi-schema-rs-169-18-rule-449-6588ca478d")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_ffi_schema_rs_169_18_rule_449_6588ca478d() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut child0 = ArrowSchema::empty();
    let mut child1 = ArrowSchema::empty();

    let mut children = vec![&mut child0 as *mut ArrowSchema, &mut child1 as *mut ArrowSchema];

    let mut schema = ArrowSchema::empty();
    schema.n_children = __unsat_rerun_sym_000;
    schema.children = children.as_mut_ptr();

    let _ = schema.child(__unsat_rerun_sym_001);
}

