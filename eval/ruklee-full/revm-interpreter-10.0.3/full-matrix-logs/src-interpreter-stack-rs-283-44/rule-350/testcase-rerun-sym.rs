#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-283-44-rule-350-460b0d883a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_283_44_rule_350_460b0d883a() {
    let mut __unsat_rerun_sym_000 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 32u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterResult};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut interp = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from([0x00])),
            None,
            revm_primitives::Address::ZERO,
            None,
            revm_primitives::Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );

    interp.is_eof = __unsat_rerun_sym_002;
    interp.instruction_result = InstructionResult::Continue;
    interp.gas = Gas::new(__unsat_rerun_sym_003);

    interp.stack.data_mut().clear();
    interp.stack.data_mut().reserve(crate::STACK_LIMIT);
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_004));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_005));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_006));

    let mut host = DummyHost::default();

    let _ = interp.stack.exchange(__unsat_rerun_sym_007, __unsat_rerun_sym_008);

    let _ = InterpreterResult::new(InstructionResult::Stop, Bytes::new(), Gas::new(__unsat_rerun_sym_009));
    let _ = &mut host;
}

