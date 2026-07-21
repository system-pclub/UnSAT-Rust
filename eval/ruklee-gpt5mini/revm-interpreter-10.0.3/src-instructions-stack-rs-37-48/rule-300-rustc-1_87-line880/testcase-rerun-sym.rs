#[cfg(feature = "unsat-poc-src-instructions-stack-rs-37-48-rule-300-rustc-1-87-line880-e2e1cd33b6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_37_48_rule_300_rustc_1_87_line880_e2e1cd33b6() {
    let mut __unsat_rerun_sym_000 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 272);
    let mut __unsat_rerun_sym_002 = 0x22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 544);
    let mut __unsat_rerun_sym_003 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 32);
    let mut stack = crate::Stack::new();
    let mut interp = crate::Interpreter::new(crate::Contract::default(), u64::MAX, __unsat_rerun_sym_000);
    let mut host = crate::DummyHost::default();

    interp.stack = stack;
    interp.instruction_pointer = interp.bytecode.as_ptr();

    let _ = interp.stack.push(crate::primitives::U256::from(__unsat_rerun_sym_001));
    let _ = interp.stack.push(crate::primitives::U256::from(__unsat_rerun_sym_002));

    crate::instructions::stack::push::<__unsat_rerun_sym_003, _>(&mut interp, &mut host);

    stack = interp.stack;
    let _ = stack;
}

