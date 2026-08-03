#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-ffi-schema-rs-164-18-rule-62-7c186f3955")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_ffi_schema_rs_164_18_rule_62_7c186f3955() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let name = CString::new("x").unwrap();
    let schema = ArrowSchema {
        format: ptr::null(),
        name: name.as_ptr(),
        metadata: ptr::null(),
        flags: __unsat_rerun_sym_000,
        n_children: __unsat_rerun_sym_001,
        children: ptr::null_mut(),
        dictionary: ptr::null_mut(),
        release: None,
        private_data: ptr::null_mut(),
    };

    let _ = schema.name();

    std::mem::forget(name);
}

