#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-contract-rs-82-48-rule-291-c384f4c11f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_contract_rs_82_48_rule_291_c384f4c11f() {
    let mut __unsat_rerun_sym_000 = 0xef;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x04;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 0x04;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    let mut __unsat_rerun_sym_016 = 0x80;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_016, "__unsat_rerun_sym_016");
    let mut __unsat_rerun_sym_017 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_017, "__unsat_rerun_sym_017");
    let mut __unsat_rerun_sym_018 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_018, "__unsat_rerun_sym_018");
    let mut __unsat_rerun_sym_019 = 0xfe;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_019, "__unsat_rerun_sym_019");
    let mut __unsat_rerun_sym_020 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_020, "__unsat_rerun_sym_020");
    let mut __unsat_rerun_sym_021 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_021, "__unsat_rerun_sym_021");
    let mut __unsat_rerun_sym_022 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_022, "__unsat_rerun_sym_022");
    let mut __unsat_rerun_sym_023 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_023, "__unsat_rerun_sym_023");
    let mut __unsat_rerun_sym_024 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_024, "__unsat_rerun_sym_024");
    let mut __unsat_rerun_sym_025 = 1u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_025, "__unsat_rerun_sym_025");
    use crate::{
        gas::Gas,
        host::DummyHost,
        interpreter::{Contract, Interpreter},
        primitives::{Bytes, Eof},
        InstructionResult,
    };
    use std::sync::Arc;

    let eof_bytes = Bytes::copy_from_slice(&[
        __unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009, __unsat_rerun_sym_010, __unsat_rerun_sym_011, __unsat_rerun_sym_012, __unsat_rerun_sym_013,
        __unsat_rerun_sym_014, __unsat_rerun_sym_015, __unsat_rerun_sym_016, __unsat_rerun_sym_017, __unsat_rerun_sym_018, __unsat_rerun_sym_019,
    ]);
    let mut eof = Eof::decode(eof_bytes.clone()).unwrap();
    eof.body.container_section = vec![eof_bytes.clone()];
    eof.header.container_sizes = vec![eof_bytes.len() as u16];
    eof.header.sum_container_sizes = eof_bytes.len();

    let contract = Contract {
        input: Bytes::new(),
        bytecode: crate::primitives::Bytecode::Eof(Arc::new(eof)),
        hash: None,
        target_address: Default::default(),
        bytecode_address: None,
        caller: Default::default(),
        call_value: Default::default(),
    };

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_020, __unsat_rerun_sym_021);
    interpreter.is_eof = __unsat_rerun_sym_022;
    interpreter.is_static = __unsat_rerun_sym_023;
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.gas = Gas::new(__unsat_rerun_sym_024);

    let ip = [__unsat_rerun_sym_025];
    interpreter.instruction_pointer = ip.as_ptr();

    let mut host = DummyHost::default();
    crate::instructions::contract::eofcreate(&mut interpreter, &mut host);
}

