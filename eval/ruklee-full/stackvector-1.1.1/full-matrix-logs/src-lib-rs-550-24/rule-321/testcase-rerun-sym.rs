#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-550-24-rule-321-36557b04ee")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_550_24_rule_321_36557b04ee() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let buf = [__unsat_rerun_sym_000; 1];
    let mut sv: StackVec<[u8; 1]> = StackVec::from_buf_and_len(buf, __unsat_rerun_sym_001);
    sv.length = __unsat_rerun_sym_002;
    let _ = sv.remove(__unsat_rerun_sym_003);
}

