#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-37-48-rule-300-rustc-1-87-line880-e2e1cd33b6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_37_48_rule_300_rustc_1_87_line880_e2e1cd33b6() {
    let mut __unsat_rerun_sym_000 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0xAAu64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{Bytecode, Bytes, U256};

    let bytecode = Bytecode::LegacyRaw(Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003]));
    let mut interpreter = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            bytecode,
            None,
            revm_primitives::Address::ZERO,
            None,
            revm_primitives::Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_004,
        __unsat_rerun_sym_005,
    );

    let backing = vec![__unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008];
    interpreter.instruction_pointer = backing.as_ptr();

    interpreter.stack.data_mut().clear();
    interpreter.stack.data_mut().push(U256::from(__unsat_rerun_sym_009));

    interpreter.gas = Gas::new(__unsat_rerun_sym_010);
    interpreter.instruction_result = InstructionResult::Continue;

    let mut host = DummyHost::default();
    crate::instructions::stack::push::<__unsat_rerun_sym_011, _>(&mut interpreter, &mut host);

    core::mem::forget(backing);
}

