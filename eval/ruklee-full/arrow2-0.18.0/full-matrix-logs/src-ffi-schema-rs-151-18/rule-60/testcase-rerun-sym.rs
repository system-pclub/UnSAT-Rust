#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-ffi-schema-rs-151-18-rule-60-9b0cf4fd74")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_ffi_schema_rs_151_18_rule_60_9b0cf4fd74() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let backing = CString::new("x").unwrap();
    let schema = ArrowSchema {
        format: backing.as_ptr(),
        name: ptr::null(),
        metadata: ptr::null(),
        flags: __unsat_rerun_sym_000,
        n_children: __unsat_rerun_sym_001,
        children: ptr::null_mut(),
        dictionary: ptr::null_mut(),
        release: None,
        private_data: ptr::null_mut(),
    };

    let s = schema.format();
    assert_eq!(s, "x");
}

