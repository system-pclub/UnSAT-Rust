#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-53-48-rule-291-b9068ffbc5")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_53_48_rule_291_b9068ffbc5() {
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
    let mut __unsat_rerun_sym_006 = 0x5B;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x5B;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    use crate::{gas::Gas, instructions::control, interpreter::Interpreter, DummyHost};
    use revm_primitives::{Bytecode, Bytes};

    let code = Bytes::from(vec![
        __unsat_rerun_sym_000, // RJUMPV
        __unsat_rerun_sym_001, // max_index = 1
        __unsat_rerun_sym_002, __unsat_rerun_sym_003, // case 0 offset = 1
        __unsat_rerun_sym_004, __unsat_rerun_sym_005, // case 1 offset = 2
        __unsat_rerun_sym_006, // JUMPDEST
        __unsat_rerun_sym_007, // JUMPDEST
        __unsat_rerun_sym_008, // STOP
    ]);

    let contract = crate::Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code.clone()),
        None,
        Default::default(),
        None,
        Default::default(),
        Default::default(),
    );

    let mut interp = Interpreter::new(contract, __unsat_rerun_sym_009, __unsat_rerun_sym_010);
    interp.is_eof = __unsat_rerun_sym_011;
    interp.gas = Gas::new(__unsat_rerun_sym_012);
    interp.stack.push(revm_primitives::U256::from(__unsat_rerun_sym_013)).unwrap();
    interp.instruction_pointer = interp.bytecode.as_ptr();

    let mut host = DummyHost::default();
    control::rjumpv(&mut interp, &mut host);
}

