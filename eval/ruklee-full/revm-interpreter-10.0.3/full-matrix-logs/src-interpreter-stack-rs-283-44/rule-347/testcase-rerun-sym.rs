#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-283-44-rule-347-141947b6cf")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_283_44_rule_347_141947b6cf() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 33u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterAction};
    use revm_primitives::{Address, Bytecode, Bytes, U256};

    let bytecode = Bytecode::LegacyRaw(Bytes::from([0x00]));
    let mut interp = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            bytecode,
            None,
            Address::ZERO,
            None,
            Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );

    interp.is_eof = __unsat_rerun_sym_002;
    interp.instruction_result = InstructionResult::Continue;
    interp.gas = Gas::new(__unsat_rerun_sym_003);
    interp.next_action = InterpreterAction::None;

    interp.stack.data_mut().clear();
    interp.stack.data_mut().extend_from_slice(&[
        U256::from(__unsat_rerun_sym_004),
        U256::from(__unsat_rerun_sym_005),
        U256::from(__unsat_rerun_sym_006),
    ]);

    let mut host = DummyHost::default();

    let _ = interp.stack.exchange(__unsat_rerun_sym_007, __unsat_rerun_sym_008);
    let _ = interp.stack.exchange(__unsat_rerun_sym_009, __unsat_rerun_sym_010);
    let _ = interp.stack.exchange(__unsat_rerun_sym_011, __unsat_rerun_sym_012);
    let _ = interp.stack.exchange(__unsat_rerun_sym_013, __unsat_rerun_sym_014);
}

