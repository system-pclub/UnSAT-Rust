#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-61-48-rule-290-1e52d35637")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_61_48_rule_290_1e52d35637() {
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
    let mut __unsat_rerun_sym_005 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x01u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    use crate::{gas::Gas, instructions::stack, interpreter::Contract, DummyHost, Interpreter};
    use revm_primitives::{Bytecode, Bytes};

    let code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001]);
    let contract = Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code.clone()),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        revm_primitives::U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_002, __unsat_rerun_sym_003);
    interpreter.is_eof = __unsat_rerun_sym_004;
    interpreter.gas = Gas::new(__unsat_rerun_sym_005);
    interpreter.instruction_result = crate::InstructionResult::Continue;

    let backing = [__unsat_rerun_sym_006, __unsat_rerun_sym_007];
    interpreter.instruction_pointer = backing.as_ptr();

    let mut host = DummyHost::default();
    stack::dupn(&mut interpreter, &mut host);

    let _ = interpreter.instruction_pointer;
}

