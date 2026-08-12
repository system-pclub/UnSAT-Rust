#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-346-17-rule-447-485609ff7e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_346_17_rule_447_485609ff7e() {
    let mut __unsat_rerun_sym_000 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 2u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 3u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 4u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 5u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterResult};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut interp = Interpreter::new(
        crate::Contract::new(
            Bytes::from([0x00]),
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

    let mut host = DummyHost::default();

    let mut data = Vec::with_capacity(__unsat_rerun_sym_004);
    data.extend_from_slice(&[0u8; 32]);
    data.push(__unsat_rerun_sym_005);

    let _ = interp.stack.push(U256::from(__unsat_rerun_sym_006));
    let _ = interp.stack.push(U256::from(__unsat_rerun_sym_007));
    let _ = interp.stack.push(U256::from(__unsat_rerun_sym_008));
    let _ = interp.stack.push(U256::from(__unsat_rerun_sym_009));
    let _ = interp.stack.push(U256::from(__unsat_rerun_sym_010));

    let _ = interp.stack.push_slice(&data);

    let _ = (&mut interp, &mut host, InterpreterResult::new(InstructionResult::Continue, Bytes::new(), Gas::new(__unsat_rerun_sym_011)));
}

