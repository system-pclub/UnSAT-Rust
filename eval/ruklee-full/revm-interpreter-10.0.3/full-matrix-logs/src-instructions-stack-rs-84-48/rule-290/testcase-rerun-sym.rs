#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-84-48-rule-290-05fd70ae9d")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_84_48_rule_290_05fd70ae9d() {
    let mut __unsat_rerun_sym_000 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 2u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 3u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 4u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    use crate::{
        gas::Gas,
        instructions::stack,
        primitives::{Bytecode, Bytes},
        DummyHost, Interpreter,
    };
    use std::sync::Arc;

    let mut code = Bytes::from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001]);
    let bytecode = Bytecode::new_legacy(code.clone());

    let mut interpreter = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            bytecode,
            None,
            crate::primitives::Address::ZERO,
            None,
            crate::primitives::Address::ZERO,
            crate::primitives::U256::ZERO,
        ),
        __unsat_rerun_sym_002,
        __unsat_rerun_sym_003,
    );

    interpreter.is_eof = __unsat_rerun_sym_004;
    interpreter.gas = Gas::new(__unsat_rerun_sym_005);

    let ip_storage = vec![__unsat_rerun_sym_006, __unsat_rerun_sym_007];
    interpreter.instruction_pointer = ip_storage.as_ptr();

    interpreter.stack.data_mut().clear();
    interpreter.stack.data_mut().extend_from_slice(&[
        crate::primitives::U256::from(__unsat_rerun_sym_008),
        crate::primitives::U256::from(__unsat_rerun_sym_009),
        crate::primitives::U256::from(__unsat_rerun_sym_010),
        crate::primitives::U256::from(__unsat_rerun_sym_011),
    ]);

    let mut host = DummyHost::default();
    stack::exchange(&mut interpreter, &mut host);

    let _ = Arc::new(code);
}

