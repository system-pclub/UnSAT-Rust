#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-337-17-write-rule-389-a95b13870c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_337_17_write_rule_389_a95b13870c() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x1111u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x2222u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x3333u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x4444u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0xAAu8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0xBB;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0xCC;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0xDD;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 0xEE;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 0xFF;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 0x11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 0x33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    use crate::{DummyHost, Gas, Interpreter, InterpreterAction, InstructionResult};
    use revm_primitives::{Address, Bytecode, Bytes, U256};

    let mut interp = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from([0x00])),
            None,
            Address::ZERO,
            None,
            Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );

    interp.stack.data_mut().clear();
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_002));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_003));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_004));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_005));

    let mut host = DummyHost::default();

    let slice = [__unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009, __unsat_rerun_sym_010, __unsat_rerun_sym_011, __unsat_rerun_sym_012, __unsat_rerun_sym_013, __unsat_rerun_sym_014];
    let _ = interp.stack.push_slice(&slice);

    let _ = (&mut interp, &mut host);

    interp.instruction_result = InstructionResult::Continue;
    interp.next_action = InterpreterAction::None;
    let _ = Gas::new(__unsat_rerun_sym_015);
}

