#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-host-rs-110-5-rule-576-4f29f1f3ea")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_host_rs_110_5_rule_576_4f29f1f3ea() {
    let mut __unsat_rerun_sym_000 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterAction};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut interpreter = Interpreter::new(
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

    interpreter.is_eof = __unsat_rerun_sym_002;
    interpreter.gas = Gas::new(__unsat_rerun_sym_003);
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.next_action = InterpreterAction::None;

    interpreter.stack.data_mut().clear();
    interpreter.stack.data_mut().push(U256::from(__unsat_rerun_sym_004));
    interpreter.stack.data_mut().push(U256::from(__unsat_rerun_sym_005));

    let mut host = DummyHost::default();
    crate::instructions::host::blockhash::<DummyHost, crate::primitives::PragueSpec>(
        &mut interpreter,
        &mut host,
    );
}

