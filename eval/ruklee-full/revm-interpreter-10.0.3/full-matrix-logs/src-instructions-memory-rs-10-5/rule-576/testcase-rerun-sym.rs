#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-memory-rs-10-5-rule-576-5980bb48ff")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_memory_rs_10_5_rule_576_5980bb48ff() {
    let mut __unsat_rerun_sym_000 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 2u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    use crate::{
        gas::Gas,
        instructions::memory,
        interpreter::{Contract, Interpreter},
        primitives::{Bytecode, Bytes, U256},
        DummyHost,
    };

    let mut stack_bytes = Vec::with_capacity(__unsat_rerun_sym_000);
    stack_bytes.push(U256::from(__unsat_rerun_sym_001));
    stack_bytes.push(U256::from(__unsat_rerun_sym_002));

    let bytecode = Bytecode::LegacyRaw(Bytes::from([0x51u8]));
    let contract = Contract::new(
        Bytes::new(),
        bytecode,
        None,
        crate::primitives::Address::ZERO,
        None,
        crate::primitives::Address::ZERO,
        U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_003, __unsat_rerun_sym_004);
    interpreter.gas = Gas::new(__unsat_rerun_sym_005);
    interpreter.is_eof = __unsat_rerun_sym_006;
    interpreter.stack.data_mut().extend(stack_bytes);
    interpreter.shared_memory.resize(__unsat_rerun_sym_007);

    let mut host = DummyHost::default();
    memory::mload(&mut interpreter, &mut host);
}

