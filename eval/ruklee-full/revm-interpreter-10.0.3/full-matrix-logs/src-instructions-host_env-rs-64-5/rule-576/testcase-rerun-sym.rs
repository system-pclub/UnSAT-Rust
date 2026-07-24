#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-host-env-rs-64-5-rule-576-1a87ce64ae")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_host_env_rs_64_5_rule_576_1a87ce64ae() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterResult, Stack};
    use revm_primitives::{Address, Bytes, U256};

    let mut host = DummyHost::default();
    host.env.tx.blob_hashes = vec![
        revm_primitives::B256::from([0x11u8; 32]),
        revm_primitives::B256::from([0x22u8; 32]),
    ];

    let mut interpreter = Interpreter::new(
        crate::Contract::new(
            Bytes::from([0x00u8]),
            revm_primitives::Bytecode::LegacyRaw(Bytes::from([0x00u8])),
            None,
            Address::ZERO,
            None,
            Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );

    interpreter.is_eof = __unsat_rerun_sym_002;
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.gas = Gas::new(__unsat_rerun_sym_003);
    interpreter.stack = Stack::new();
    let _ = interpreter.stack.push(U256::from(__unsat_rerun_sym_004));
    let _ = interpreter.stack.push(U256::from(__unsat_rerun_sym_005));

    interpreter.stack.data_mut().clear();
    interpreter.stack.data_mut().push(U256::from(__unsat_rerun_sym_006));
    interpreter.stack.data_mut().push(U256::from(__unsat_rerun_sym_007));

    crate::instructions::host_env::blob_hash::<DummyHost, crate::primitives::BerlinSpec>(
        &mut interpreter,
        &mut host,
    );

    let _ = InterpreterResult::new(InstructionResult::Continue, Bytes::new(), Gas::new(__unsat_rerun_sym_008));
}

