#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-xffi-xtr-rs-11-9-rule-446-cb383e7255")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_xffi_xtr_rs_11_9_rule_446_cb383e7255() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let s = "A";
    let mut prefix = [__unsat_rerun_sym_000; 2];
    crate::xffi::xtr::string_to_buffer(s, prefix.as_mut_ptr(), __unsat_rerun_sym_001);

    let mut target = [__unsat_rerun_sym_002; 1];
    crate::xffi::xtr::string_to_buffer(s, target.as_mut_ptr(), __unsat_rerun_sym_003);
}

