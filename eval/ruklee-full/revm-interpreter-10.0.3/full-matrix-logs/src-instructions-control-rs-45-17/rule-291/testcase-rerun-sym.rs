#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-45-17-rule-291-a78dcc60ba")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_45_17_rule_291_a78dcc60ba() {
    let mut __unsat_rerun_sym_000 = 0xE2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    use crate::{
        gas::Gas,
        instructions::control,
        interpreter::{Contract, Interpreter},
        interpreter_action::InterpreterAction,
        primitives::{Address, Bytecode, Bytes, U256},
        DummyHost,
    };
    use std::sync::Arc;

    let code: Bytes = Bytes::from(vec![
        __unsat_rerun_sym_000, // RJUMPV
        __unsat_rerun_sym_001, // max_index = 1
        __unsat_rerun_sym_002, __unsat_rerun_sym_003, // case 0 offset
        __unsat_rerun_sym_004, __unsat_rerun_sym_005, // case 1 offset
        __unsat_rerun_sym_006, // padding / target byte for safe prefix reads
    ]);

    let bytecode = Bytecode::LegacyRaw(code.clone());
    let contract = Contract::new(
        Bytes::new(),
        bytecode,
        None,
        Address::ZERO,
        None,
        Address::ZERO,
        U256::ZERO,
    );

    let mut interp = Interpreter::new(contract, __unsat_rerun_sym_007, __unsat_rerun_sym_008);
    interp.is_eof = __unsat_rerun_sym_009;
    interp.gas = Gas::new(__unsat_rerun_sym_010);
    interp.instruction_result = crate::InstructionResult::Continue;
    interp.next_action = InterpreterAction::None;

    let mut host = DummyHost::default();

    interp.stack.push(U256::from(__unsat_rerun_sym_011)).unwrap();

    interp.instruction_pointer = code.as_ptr();

    control::rjumpv::<DummyHost>(&mut interp, &mut host);

    let _ = Arc::new(());
}

