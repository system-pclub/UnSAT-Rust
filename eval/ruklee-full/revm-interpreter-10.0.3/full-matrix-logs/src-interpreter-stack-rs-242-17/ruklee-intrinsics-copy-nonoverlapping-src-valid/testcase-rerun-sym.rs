#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-242-17-ruklee-intrinsics-copy-nonoverlapping-src-valid-27ad901522")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_242_17_ruklee_intrinsics_copy_nonoverlapping_src_valid_27ad901522() {
    let mut __unsat_rerun_sym_000 = 0x1111u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x2222u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterAction, Contract};
    use revm_primitives::{Address, Bytecode, Bytes, U256};

    let mut stack = crate::Stack::new();
    stack.data_mut().push(U256::from(__unsat_rerun_sym_000));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_001));

    let contract = Contract::new(
        Bytes::new(),
        Bytecode::LegacyRaw(Bytes::from([0x00])),
        None,
        Address::ZERO,
        None,
        Address::ZERO,
        U256::ZERO,
    );

    let mut interp = Interpreter {
        instruction_pointer: core::ptr::null(),
        gas: Gas::new(__unsat_rerun_sym_002),
        contract,
        instruction_result: InstructionResult::Continue,
        bytecode: Bytes::from([0x00]),
        is_eof: __unsat_rerun_sym_003,
        is_eof_init: __unsat_rerun_sym_004,
        shared_memory: crate::EMPTY_SHARED_MEMORY,
        stack,
        function_stack: crate::FunctionStack::new(),
        return_data_buffer: Bytes::new(),
        is_static: __unsat_rerun_sym_005,
        next_action: InterpreterAction::None,
    };

    let mut host = DummyHost::default();
    let _ = crate::interpreter::stack::Stack::dup(&mut interp.stack, __unsat_rerun_sym_006);
    let _ = &mut host;
}

