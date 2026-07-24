#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-61-48-rule-291-d95e752c0e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_61_48_rule_291_d95e752c0e() {
    let mut __unsat_rerun_sym_000 = 0x01u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let code = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let bytecode = crate::primitives::Bytes::from(code);
    let contract = crate::Contract::new(
        crate::primitives::Bytes::new(),
        crate::primitives::Bytecode::LegacyRaw(bytecode),
        None,
        Default::default(),
        None,
        Default::default(),
        Default::default(),
    );

    let mut interpreter = crate::Interpreter::new(contract, __unsat_rerun_sym_002, __unsat_rerun_sym_003);
    interpreter.is_eof = __unsat_rerun_sym_004;
    interpreter.instruction_result = crate::InstructionResult::Continue;

    let imm = [__unsat_rerun_sym_005];
    interpreter.instruction_pointer = imm.as_ptr();

    interpreter.stack.data_mut().clear();
    interpreter.stack.data_mut().push(Default::default());

    let mut host = crate::DummyHost::default();
    crate::instructions::stack::dupn::<crate::DummyHost>(&mut interpreter, &mut host);
}

