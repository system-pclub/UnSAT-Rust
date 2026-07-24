#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-71-48-rule-290-28810f1e82")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_71_48_rule_290_28810f1e82() {
    let mut __unsat_rerun_sym_000 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 3u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let code = crate::primitives::Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001]);
    let contract = crate::Contract::new(
        crate::primitives::Bytes::new(),
        crate::primitives::Bytecode::LegacyRaw(code),
        None,
        crate::primitives::Address::ZERO,
        None,
        crate::primitives::Address::ZERO,
        crate::primitives::U256::ZERO,
    );

    let mut interpreter = crate::Interpreter::new(contract, __unsat_rerun_sym_002, __unsat_rerun_sym_003);
    interpreter.is_eof = __unsat_rerun_sym_004;
    interpreter.instruction_result = crate::InstructionResult::Continue;
    interpreter.stack.data_mut().clear();
    interpreter.stack.data_mut().push(crate::primitives::U256::from(__unsat_rerun_sym_005));
    interpreter.stack.data_mut().push(crate::primitives::U256::from(__unsat_rerun_sym_006));
    interpreter.stack.data_mut().push(crate::primitives::U256::from(__unsat_rerun_sym_007));

    let ip = interpreter.bytecode.as_ptr();
    interpreter.instruction_pointer = ip;

    let mut host = crate::DummyHost::default();
    crate::instructions::stack::swapn(&mut interpreter, &mut host);
}

