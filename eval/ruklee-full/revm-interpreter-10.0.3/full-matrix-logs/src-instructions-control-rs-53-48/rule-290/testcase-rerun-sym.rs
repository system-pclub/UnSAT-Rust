#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-53-48-rule-290-62a1cdf353")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_53_48_rule_290_62a1cdf353() {
    let mut __unsat_rerun_sym_000 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterAction};
    use revm_primitives::{Bytecode, Bytes};

    let code = Bytes::from(vec![
        __unsat_rerun_sym_000, // max_index = 2
        __unsat_rerun_sym_001, __unsat_rerun_sym_002, // case 0 offset
        __unsat_rerun_sym_003, __unsat_rerun_sym_004, // case 1 offset
        __unsat_rerun_sym_005, __unsat_rerun_sym_006, // case 2 offset
        __unsat_rerun_sym_007, // padding for the read at offset 5
    ]);

    let contract = crate::Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code.clone()),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        revm_primitives::U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_008, __unsat_rerun_sym_009);
    interpreter.is_eof = __unsat_rerun_sym_010;
    interpreter.gas = Gas::new(__unsat_rerun_sym_011);
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.next_action = InterpreterAction::None;

    interpreter.stack.push(revm_primitives::U256::from(__unsat_rerun_sym_012)).unwrap();

    let mut host = DummyHost::default();
    crate::instructions::control::rjumpv(&mut interpreter, &mut host);

    let _ = interpreter.program_counter();
}

