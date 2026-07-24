#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-system-rs-63-5-rule-576-957d0dc326")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_system_rs_63_5_rule_576_957d0dc326() {
    let mut __unsat_rerun_sym_000 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut interpreter = crate::Interpreter::new(
        crate::Contract::new(
            crate::primitives::Bytes::from(vec![__unsat_rerun_sym_000]),
            crate::primitives::Bytecode::LegacyRaw(crate::primitives::Bytes::from(vec![__unsat_rerun_sym_001])),
            None,
            crate::primitives::Address::ZERO,
            None,
            crate::primitives::Address::ZERO,
            crate::primitives::U256::ZERO,
        ),
        __unsat_rerun_sym_002,
        __unsat_rerun_sym_003,
    );

    interpreter.stack.push(crate::primitives::U256::from(__unsat_rerun_sym_004)).ok();
    interpreter.contract.input = crate::primitives::Bytes::from(vec![__unsat_rerun_sym_005]);

    let mut host = crate::DummyHost::default();
    crate::instructions::system::calldataload(&mut interpreter, &mut host);
}

