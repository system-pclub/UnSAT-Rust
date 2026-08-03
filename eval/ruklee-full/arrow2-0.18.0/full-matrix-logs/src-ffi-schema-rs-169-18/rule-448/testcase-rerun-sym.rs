#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-ffi-schema-rs-169-18-rule-448-7180fc896f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_ffi_schema_rs_169_18_rule_448_7180fc896f() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let child0 = Box::new(ArrowSchema::empty());
    let child1 = Box::new(ArrowSchema::empty());

    let mut children = vec![Box::into_raw(child0), Box::into_raw(child1)];
    let mut schema = ArrowSchema::empty();

    schema.n_children = __unsat_rerun_sym_000;
    schema.children = children.as_mut_ptr();

    let _ = schema.child(__unsat_rerun_sym_001);

    let _ = children;
}

