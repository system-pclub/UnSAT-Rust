#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-71-48-rule-291-61d7a0667c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_71_48_rule_291_61d7a0667c() {
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
    let mut __unsat_rerun_sym_005 = 0x01u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{Bytecode, Bytes};

    let code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001]);
    let contract = crate::Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        revm_primitives::U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_002, __unsat_rerun_sym_003);
    interpreter.is_eof = __unsat_rerun_sym_004;
    interpreter.instruction_result = InstructionResult::Continue;

    let backing = [__unsat_rerun_sym_005, __unsat_rerun_sym_006];
    interpreter.instruction_pointer = backing.as_ptr();

    interpreter.stack.data_mut().clear();
    interpreter.stack.data_mut().push(revm_primitives::U256::from(__unsat_rerun_sym_007));
    interpreter.stack.data_mut().push(revm_primitives::U256::from(__unsat_rerun_sym_008));

    let mut host = DummyHost::default();
    crate::instructions::stack::swapn::<DummyHost>(&mut interpreter, &mut host);

    let _ = Gas::new(__unsat_rerun_sym_009);
}

