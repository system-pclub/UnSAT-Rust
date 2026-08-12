#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-283-44-rule-348-4ef8bb58ce")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_283_44_rule_348_4ef8bb58ce() {
    let mut __unsat_rerun_sym_000 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut stack = crate::Stack::new();
    let data = stack.data_mut();
    data.push(crate::primitives::U256::from(__unsat_rerun_sym_000));
    data.push(crate::primitives::U256::from(__unsat_rerun_sym_001));

    let _ = stack.exchange(__unsat_rerun_sym_002, __unsat_rerun_sym_003);
}

