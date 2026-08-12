#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-283-13-rule-309-c58dea6ee1")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_283_13_rule_309_c58dea6ee1() {
    let mut __unsat_rerun_sym_000 = 11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 33u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut stack = crate::Stack::new();
    let data = stack.data_mut();

    data.push(crate::primitives::U256::from(__unsat_rerun_sym_000));
    data.push(crate::primitives::U256::from(__unsat_rerun_sym_001));
    data.push(crate::primitives::U256::from(__unsat_rerun_sym_002));

    let _ = stack.exchange(__unsat_rerun_sym_003, __unsat_rerun_sym_004);
}

