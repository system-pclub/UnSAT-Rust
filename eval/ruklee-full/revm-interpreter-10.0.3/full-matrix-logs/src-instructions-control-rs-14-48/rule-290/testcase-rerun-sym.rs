#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-14-48-rule-290-69d054ad66")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_14_48_rule_290_69d054ad66() {
    let mut __unsat_rerun_sym_000 = 0xE0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    use crate::{gas::Gas, instructions::control, Contract, DummyHost, Interpreter};
    use revm_primitives::{Bytecode, Bytes};

    let mut host = DummyHost::default();

    let code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003]);
    let contract = Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        revm_primitives::U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_004, __unsat_rerun_sym_005);
    interpreter.is_eof = __unsat_rerun_sym_006;
    interpreter.gas = Gas::new(__unsat_rerun_sym_007);

    let ip_storage = vec![__unsat_rerun_sym_008; __unsat_rerun_sym_009];
    interpreter.instruction_pointer = ip_storage.as_ptr();

    control::rjump(&mut interpreter, &mut host);

    core::mem::forget(ip_storage);
}

