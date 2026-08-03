#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-util-rs-50-24-rule-574-b088f5c1bb")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_util_rs_50_24_rule_574_b088f5c1bb() {
    let mut __unsat_rerun_sym_000 = 7u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 9u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 7u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let lhs = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let rhs = [__unsat_rerun_sym_002, __unsat_rerun_sym_003];
    let _ = fixed_time_eq(&lhs[..], &rhs[..]);
}

