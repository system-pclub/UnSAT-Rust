#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-util-rs-50-24-rule-572-7c4a8494f4")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_util_rs_50_24_rule_572_7c4a8494f4() {
    let mut __unsat_rerun_sym_000 = 0x11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x33u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let lhs = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let rhs = [__unsat_rerun_sym_002, __unsat_rerun_sym_003];
    let _ = fixed_time_eq(&lhs[..], &rhs[..]);
}

