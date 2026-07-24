#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-45-17-rule-290-f87d2c0ceb")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_45_17_rule_290_f87d2c0ceb() {
    let mut __unsat_rerun_sym_000 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x03;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x5b;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    use crate::{
        instructions::control,
        primitives::{Bytecode, Bytes, U256},
        DummyHost, Gas, Interpreter,
    };
    use std::sync::Arc;

    let code = Bytes::from(vec![
        __unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008,
    ]);
    let eof = crate::primitives::Eof::default();
    let bytecode = Bytecode::LegacyRaw(code.clone());

    let mut interp = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            bytecode,
            None,
            crate::primitives::Address::ZERO,
            None,
            crate::primitives::Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_009,
        __unsat_rerun_sym_010,
    );

    interp.is_eof = __unsat_rerun_sym_011;
    interp.gas = Gas::new(__unsat_rerun_sym_012);
    interp.instruction_pointer = code.as_ptr();
    interp.stack.push(U256::from(__unsat_rerun_sym_013)).unwrap();

    let mut host = DummyHost::default();
    control::rjumpv::<DummyHost>(&mut interp, &mut host);

    let _ = eof;
}

