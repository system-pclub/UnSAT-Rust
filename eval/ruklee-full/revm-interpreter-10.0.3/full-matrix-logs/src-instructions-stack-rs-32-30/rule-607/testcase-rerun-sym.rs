#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-32-30-rule-607-61d5da5cbe")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_32_30_rule_607_61d5da5cbe() {
    let mut __unsat_rerun_sym_000 = 0x60;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{Bytecode, Bytes};

    let code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003]);
    let contract = crate::Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        revm_primitives::U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_004, __unsat_rerun_sym_005);
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.gas = Gas::new(__unsat_rerun_sym_006);

    let backing = vec![__unsat_rerun_sym_007, __unsat_rerun_sym_008];
    interpreter.instruction_pointer = backing.as_ptr();

    let mut host = DummyHost::default();
    crate::instructions::stack::push::<__unsat_rerun_sym_009, _>(&mut interpreter, &mut host);

    let _ = interpreter;
}

