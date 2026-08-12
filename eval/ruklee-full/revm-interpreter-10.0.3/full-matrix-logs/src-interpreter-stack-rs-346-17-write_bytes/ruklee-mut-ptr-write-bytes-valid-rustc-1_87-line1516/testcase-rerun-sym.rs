#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-346-17-write-bytes-ruklee-mut-ptr-write-bytes-valid-rus-9454a34345")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_346_17_write_bytes_ruklee_mut_ptr_write_bytes_valid_rus_9454a34345() {
    let mut __unsat_rerun_sym_000 = 0x11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x33u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x44u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x55u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 100_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterAction, Contract};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut stack = crate::Stack::new();
    stack.data_mut().push(U256::from(__unsat_rerun_sym_000));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_001));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_002));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_003));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_004));

    let mut interp = Interpreter {
        instruction_pointer: Bytes::from_static(&[0x00]).as_ptr(),
        gas: Gas::new(__unsat_rerun_sym_005),
        contract: Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from_static(&[0x00])),
            None,
            Default::default(),
            None,
            Default::default(),
            U256::ZERO,
        ),
        instruction_result: InstructionResult::Continue,
        bytecode: Bytes::from_static(&[0x00]),
        is_eof: __unsat_rerun_sym_006,
        is_eof_init: __unsat_rerun_sym_007,
        shared_memory: crate::SharedMemory::new(),
        stack,
        function_stack: crate::FunctionStack::new(),
        return_data_buffer: Bytes::new(),
        is_static: __unsat_rerun_sym_008,
        next_action: InterpreterAction::None,
    };

    let mut host = DummyHost::default();

    let slice = [__unsat_rerun_sym_009; 33];
    let _ = interp.stack.push_slice(&slice);
    let _ = &mut host;
}

