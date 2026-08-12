#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-488-22-rule-322-7332bd2ff1")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_488_22_rule_322_7332bd2ff1() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf([7u8]);
    v.length = __unsat_rerun_sym_000;
    let _ = v.pop();
}

