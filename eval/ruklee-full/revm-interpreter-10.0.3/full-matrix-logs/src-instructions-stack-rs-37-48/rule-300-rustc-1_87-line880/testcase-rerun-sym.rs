#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-37-48-rule-300-rustc-1-87-line880-e2e1cd33b6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_37_48_rule_300_rustc_1_87_line880_e2e1cd33b6() {
    let mut __unsat_rerun_sym_000 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0xAAu8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0xBB;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0xCC;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0xDD;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, Contract};
    use revm_primitives::{Bytecode, Bytes, U256};

    let code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003]);
    let contract = Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code.clone()),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_004, __unsat_rerun_sym_005);

    let backing = vec![__unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009];
    interpreter.instruction_pointer = backing.as_ptr();

    interpreter.stack = crate::Stack::new();
    interpreter.gas = Gas::new(__unsat_rerun_sym_010);
    interpreter.instruction_result = InstructionResult::Continue;

    let mut host = DummyHost::default();

    crate::instructions::stack::push::<__unsat_rerun_sym_011, DummyHost>(&mut interpreter, &mut host);

    let _ = interpreter.instruction_pointer;
}

