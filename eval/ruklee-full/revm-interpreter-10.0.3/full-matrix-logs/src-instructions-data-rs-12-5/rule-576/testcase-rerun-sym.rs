#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-data-rs-12-5-rule-576-51bb62392a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_data_rs_12_5_rule_576_51bb62392a() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    use crate::{instructions::data, Contract, DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{bytes, Bytecode, Eof, U256};
    use std::sync::Arc;

    let eof_bytes = bytes!("ef000101000402000100010400000000800000fe");
    let mut eof = Eof::decode(eof_bytes).unwrap();
    eof.body.data_section = bytes!("01020304");
    eof.header.data_size = eof.body.data_section.len() as u16;
    eof.header.code_sizes[__unsat_rerun_sym_000] = __unsat_rerun_sym_001;
    eof.body.code_section[0] = bytes!("00");

    let contract = Contract::new(
        bytes!(""),
        Bytecode::Eof(Arc::new(eof)),
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_002, __unsat_rerun_sym_003);
    interpreter.gas = Gas::new(__unsat_rerun_sym_004);
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.stack.push(U256::from(__unsat_rerun_sym_005)).unwrap();

    let mut host = DummyHost::default();
    data::data_load(&mut interpreter, &mut host);
}

