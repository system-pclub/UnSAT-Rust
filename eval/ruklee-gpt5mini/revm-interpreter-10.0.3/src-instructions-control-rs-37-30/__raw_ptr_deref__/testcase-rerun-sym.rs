#[cfg(feature = "unsat-poc-src-instructions-control-rs-37-30-raw-ptr-deref-5d761ddacd")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_control_rs_37_30_raw_ptr_deref_5d761ddacd() {
    let mut __unsat_rerun_sym_000 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 16);
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 16000);
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    klee_ext_bind::assume!(__unsat_rerun_sym_006 <= 16);
    use crate::{
        instructions::control::rjumpv,
        primitives::{Bytecode, Bytes, U256},
        Contract, DummyHost, Gas, Interpreter,
    };

    let mut host = DummyHost::default();

    let code = Bytes::from([
        crate::opcode::RJUMPV,
        __unsat_rerun_sym_000, // max_index = 0
        __unsat_rerun_sym_001, __unsat_rerun_sym_002, // single table entry
        crate::opcode::STOP,
    ]);

    let contract = Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(code),
        None,
        crate::primitives::Address::ZERO,
        None,
        crate::primitives::Address::ZERO,
        U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, u64::MAX, __unsat_rerun_sym_003);
    interpreter.is_eof = __unsat_rerun_sym_004;
    interpreter.gas = Gas::new(__unsat_rerun_sym_005);

    interpreter.stack.push(U256::from(__unsat_rerun_sym_006)).unwrap();

    rjumpv::<DummyHost>(&mut interpreter, &mut host);
}

