#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-14-48-rule-291-0808f8e5fc")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_14_48_rule_291_0808f8e5fc() {
    let mut __unsat_rerun_sym_000 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00;
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
    let mut __unsat_rerun_sym_008 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    use crate::{gas::Gas, instructions::control, primitives::Bytes, DummyHost, Interpreter};
    use std::sync::Arc;

    let code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003]);
    let contract = crate::Contract::new(
        Bytes::new(),
        crate::primitives::Bytecode::LegacyRaw(code.clone()),
        None,
        crate::primitives::Address::ZERO,
        None,
        crate::primitives::Address::ZERO,
        crate::primitives::U256::ZERO,
    );

    let mut interp = Interpreter::new(contract, __unsat_rerun_sym_004, __unsat_rerun_sym_005);
    interp.is_eof = __unsat_rerun_sym_006;
    interp.gas = Gas::new(__unsat_rerun_sym_007);

    let backing = vec![__unsat_rerun_sym_008, __unsat_rerun_sym_009, __unsat_rerun_sym_010];
    interp.instruction_pointer = backing.as_ptr();

    let mut host = DummyHost::default();

    control::rjump(&mut interp, &mut host);

    let _ = Arc::new(backing);
}

