#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-37-48-rule-301-rustc-1-87-line880-2f021b16f0")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_37_48_rule_301_rustc_1_87_line880_2f021b16f0() {
    let mut __unsat_rerun_sym_000 = 0x60;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    use crate::{instructions::stack, Contract, DummyHost, Gas, Interpreter, InstructionResult};
    use revm_primitives::{Bytecode, Bytes};

    let code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002]);
    let contract = Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        revm_primitives::U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_003, __unsat_rerun_sym_004);
    interpreter.instruction_pointer = interpreter.bytecode.as_ptr().wrapping_add(__unsat_rerun_sym_005);
    interpreter.stack.push(revm_primitives::U256::from(__unsat_rerun_sym_006)).ok();
    interpreter.stack.push(revm_primitives::U256::from(__unsat_rerun_sym_007)).ok();
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.gas = Gas::new(__unsat_rerun_sym_008);

    let mut host = DummyHost::default();
    stack::push::<__unsat_rerun_sym_009, _>(&mut interpreter, &mut host);

    let _ = interpreter.instruction_pointer;
}

