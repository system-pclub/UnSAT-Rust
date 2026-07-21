#[cfg(feature = "unsat-poc-src-instructions-stack-rs-71-48-rule-288-ddd3325745")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_71_48_rule_288_ddd3325745() {
    let mut __unsat_rerun_sym_000 = 0x02;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 32);
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 16000);
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 16000);
    let mut __unsat_rerun_sym_006 = 11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    klee_ext_bind::assume!(__unsat_rerun_sym_006 <= 176);
    let mut __unsat_rerun_sym_007 = 22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    klee_ext_bind::assume!(__unsat_rerun_sym_007 <= 352);
    let mut __unsat_rerun_sym_008 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    klee_ext_bind::assume!(__unsat_rerun_sym_008 <= 528);
    use crate::{gas::Gas, instructions::stack::swapn, primitives::Bytes, Contract, DummyHost, Interpreter};

    let mut host = DummyHost::default();

    let bytecode = Bytes::from([crate::opcode::SWAPN, __unsat_rerun_sym_000, __unsat_rerun_sym_001]);
    let contract = Contract::new(
        Bytes::new(),
        crate::primitives::Bytecode::LegacyRaw(bytecode),
        None,
        crate::primitives::Address::default(),
        None,
        crate::primitives::Address::default(),
        crate::primitives::U256::ZERO,
    );

    let mut interpreter = Interpreter::new(contract, __unsat_rerun_sym_002, __unsat_rerun_sym_003);
    interpreter.is_eof = __unsat_rerun_sym_004;
    interpreter.gas = Gas::new(__unsat_rerun_sym_005);

    interpreter.stack.push(crate::primitives::U256::from(__unsat_rerun_sym_006)).unwrap();
    interpreter.stack.push(crate::primitives::U256::from(__unsat_rerun_sym_007)).unwrap();
    interpreter.stack.push(crate::primitives::U256::from(__unsat_rerun_sym_008)).unwrap();

    swapn(&mut interpreter, &mut host);

    let _ = interpreter.program_counter();
}

