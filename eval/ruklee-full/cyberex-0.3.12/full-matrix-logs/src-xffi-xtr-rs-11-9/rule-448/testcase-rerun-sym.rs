#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-xffi-xtr-rs-11-9-rule-448-ec0f032a7d")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_xffi_xtr_rs_11_9_rule_448_ec0f032a7d() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let s = "A";
    let mut backing = [__unsat_rerun_sym_000; 2];
    let buf = backing.as_mut_ptr();
    let buf_max = __unsat_rerun_sym_001;
    crate::xffi::xtr::string_to_buffer(s, buf, buf_max);
}

