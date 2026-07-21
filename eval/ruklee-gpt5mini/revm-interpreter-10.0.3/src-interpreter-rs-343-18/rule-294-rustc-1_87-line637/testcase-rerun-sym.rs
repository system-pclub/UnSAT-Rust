#[cfg(feature = "unsat-poc-src-interpreter-rs-343-18-rule-294-rustc-1-87-line637-558d5721ba")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_rs_343_18_rule_294_rustc_1_87_line637_558d5721ba() {
    let mut __unsat_rerun_sym_000 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 16);
    let mut __unsat_rerun_sym_004 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    klee_ext_bind::assume!(__unsat_rerun_sym_004 <= 16);
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 16);
    use crate::{Contract, DummyHost, Gas, Interpreter};
    use revm_primitives::{Bytecode, Bytes, U256};

    let bytecode = Bytes::from([__unsat_rerun_sym_000, __unsat_rerun_sym_001]);
    let contract = Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(bytecode.clone()),
        None,
        Default::default(),
        None,
        Default::default(),
        U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, u64::MAX, __unsat_rerun_sym_002);
    let mut host = DummyHost::default();

    interpreter.instruction_pointer = bytecode.as_ptr();
    interpreter.bytecode = Bytes::from([__unsat_rerun_sym_003, __unsat_rerun_sym_004]);

    let _ = interpreter.program_counter();

    let _ = (&mut host, Gas::new(__unsat_rerun_sym_005));
}

