#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-649-29-rule-321-81f76cba51")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_649_29_rule_321_81f76cba51() {
    let mut __unsat_rerun_sym_000 = 11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let buf = [__unsat_rerun_sym_000];
    let v: StackVec<[u8; 1]> = StackVec::from_buf_and_len(buf, __unsat_rerun_sym_001);
    let _ = v.into_inner();
}

