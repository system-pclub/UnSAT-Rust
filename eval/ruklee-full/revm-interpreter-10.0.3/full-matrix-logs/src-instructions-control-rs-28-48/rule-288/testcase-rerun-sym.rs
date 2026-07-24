#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-control-rs-28-48-rule-288-e9e0ec666a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_28_48_rule_288_e9e0ec666a() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    use crate::{gas::Gas, instructions::control, interpreter::Contract, DummyHost, Interpreter};
    use revm_primitives::{Address, Bytecode, Bytes, U256};

    let contract = Contract::new(
        Bytes::from([0x00]),
        Bytecode::LegacyRaw(Bytes::from([0x00])),
        None,
        Address::ZERO,
        None,
        Address::ZERO,
        U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_000, __unsat_rerun_sym_001);
    interpreter.is_eof = __unsat_rerun_sym_002;
    interpreter.gas = Gas::new(__unsat_rerun_sym_003);

    let code = vec![__unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006];
    interpreter.instruction_pointer = code.as_ptr();

    interpreter.stack.push(U256::from(__unsat_rerun_sym_007)).unwrap();

    let mut host = DummyHost::default();
    control::rjumpi(&mut interpreter, &mut host);

    core::mem::forget(code);
}

