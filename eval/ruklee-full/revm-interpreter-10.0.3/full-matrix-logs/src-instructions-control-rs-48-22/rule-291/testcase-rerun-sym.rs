#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-48-22-rule-291-7a6d1dc057")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_48_22_rule_291_7a6d1dc057() {
    let mut __unsat_rerun_sym_000 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0xE2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut interp = Interpreter::new(
        crate::Contract::new(
            Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004]),
            Bytecode::LegacyRaw(Bytes::from(vec![__unsat_rerun_sym_005, __unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009])),
            None,
            revm_primitives::Address::ZERO,
            None,
            revm_primitives::Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_010,
        __unsat_rerun_sym_011,
    );

    interp.is_eof = __unsat_rerun_sym_012;
    interp.gas = Gas::new(__unsat_rerun_sym_013);
    interp.stack.push(U256::from(__unsat_rerun_sym_014)).unwrap();

    let mut host = DummyHost::default();
    crate::instructions::control::rjumpv(&mut interp, &mut host);

    let _ = interp.instruction_result == InstructionResult::Continue;
}

