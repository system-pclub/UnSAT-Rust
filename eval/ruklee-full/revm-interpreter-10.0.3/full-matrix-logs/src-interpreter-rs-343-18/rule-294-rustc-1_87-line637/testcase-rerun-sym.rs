#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-rs-343-18-rule-294-rustc-1-87-line637-558d5721ba")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_rs_343_18_rule_294_rustc_1_87_line637_558d5721ba() {
    let mut __unsat_rerun_sym_000 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x03;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x44;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    use crate::{Contract, DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{Bytecode, Bytes, U256};

    let bytecode = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003]);
    let contract = Contract::new(
        Bytes::new(),
        Bytecode::new_legacy(bytecode.clone()),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        U256::ZERO,
    );

    let mut interp = Interpreter::new(contract, __unsat_rerun_sym_004, __unsat_rerun_sym_005);

    let backing = vec![__unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009];
    interp.bytecode = Bytes::from(backing.clone());

    let ip = interp.bytecode.as_ptr();
    interp.instruction_pointer = ip.wrapping_add(__unsat_rerun_sym_010);

    interp.gas = Gas::new(__unsat_rerun_sym_011);
    interp.instruction_result = InstructionResult::Continue;

    let _ = interp.program_counter();

    let mut host = DummyHost::default();
    let _ = &mut host;
}

