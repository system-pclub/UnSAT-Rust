#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-stack-rs-84-48-rule-291-d5719a0966")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_84_48_rule_291_d5719a0966() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    use crate::{instructions::stack, DummyHost, Gas, Interpreter, InstructionResult};
    use revm_primitives::{Bytecode, Bytes};

    let mut interpreter = Interpreter::new(
        crate::Contract::new(
            Bytes::from_static(&[]),
            Bytecode::new(),
            None,
            Default::default(),
            None,
            Default::default(),
            Default::default(),
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );

    interpreter.is_eof = __unsat_rerun_sym_002;
    interpreter.instruction_result = InstructionResult::Continue;
    interpreter.gas = Gas::new(__unsat_rerun_sym_003);

    let code = [__unsat_rerun_sym_004, __unsat_rerun_sym_005];
    interpreter.instruction_pointer = code.as_ptr();

    let mut host = DummyHost::default();
    stack::exchange(&mut interpreter, &mut host);
}

