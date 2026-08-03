#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-ffi-schema-rs-169-18-rule-447-21ab565e20")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_ffi_schema_rs_169_18_rule_447_21ab565e20() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut schema = ArrowSchema::empty();

    let child0 = Box::new(ArrowSchema::empty());
    let child1 = Box::new(ArrowSchema::empty());
    let children = vec![Box::into_raw(child0), Box::into_raw(child1)];

    schema.n_children = __unsat_rerun_sym_000;
    schema.children = Box::into_raw(children.into_boxed_slice()) as *mut *mut ArrowSchema;

    let _ = schema.child(__unsat_rerun_sym_001);
}

