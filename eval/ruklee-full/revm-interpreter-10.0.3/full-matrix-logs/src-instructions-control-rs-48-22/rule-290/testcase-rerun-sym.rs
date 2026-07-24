#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-48-22-rule-290-a2e8c55a83")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_48_22_rule_290_a2e8c55a83() {
    let mut __unsat_rerun_sym_000 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x03;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x5b;
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
    use crate::{DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{Bytes, Bytecode};

    let code = Bytes::from(vec![
        __unsat_rerun_sym_000, // max_index = 2
        __unsat_rerun_sym_001, __unsat_rerun_sym_002, // case 0
        __unsat_rerun_sym_003, __unsat_rerun_sym_004, // case 1
        __unsat_rerun_sym_005, __unsat_rerun_sym_006, // case 2
        __unsat_rerun_sym_007, // padding
    ]);

    let contract = crate::Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code),
        None,
        Default::default(),
        None,
        Default::default(),
        Default::default(),
    );

    let mut interp = Interpreter::new(contract, __unsat_rerun_sym_008, __unsat_rerun_sym_009);
    interp.is_eof = __unsat_rerun_sym_010;
    interp.gas = Gas::new(__unsat_rerun_sym_011);
    interp.stack.push(crate::primitives::U256::from(__unsat_rerun_sym_012)).ok();

    let mut host = DummyHost::default();
    crate::instructions::control::rjumpv(&mut interp, &mut host);

    let _ = interp.instruction_result == InstructionResult::Continue;
}

