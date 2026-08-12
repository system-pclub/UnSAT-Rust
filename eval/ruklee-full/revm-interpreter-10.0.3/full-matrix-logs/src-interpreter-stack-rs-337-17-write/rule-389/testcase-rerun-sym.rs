#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-337-17-write-rule-389-a95b13870c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_337_17_write_rule_389_a95b13870c() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0xAB;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    use crate::{Contract, DummyHost, Gas, Interpreter, InterpreterAction, InstructionResult};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut stack = crate::Stack::new();

    let mut backing = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    backing[__unsat_rerun_sym_002] = __unsat_rerun_sym_003;

    let bytecode = Bytecode::LegacyRaw(Bytes::from(vec![__unsat_rerun_sym_004]));
    let contract = Contract::new(
        Bytes::new(),
        bytecode,
        None,
        Default::default(),
        None,
        Default::default(),
        U256::ZERO,
    );

    let mut interp = Interpreter {
        instruction_pointer: backing.as_ptr(),
        gas: Gas::new(__unsat_rerun_sym_005),
        contract,
        instruction_result: InstructionResult::Continue,
        bytecode: Bytes::from(vec![__unsat_rerun_sym_006]),
        is_eof: __unsat_rerun_sym_007,
        is_eof_init: __unsat_rerun_sym_008,
        shared_memory: crate::SharedMemory::new(),
        stack: {
            stack.data_mut().push(U256::from(__unsat_rerun_sym_009));
            stack
        },
        function_stack: crate::FunctionStack::new(),
        return_data_buffer: Bytes::new(),
        is_static: __unsat_rerun_sym_010,
        next_action: InterpreterAction::None,
    };

    let mut host = DummyHost::default();

    let slice = &backing[__unsat_rerun_sym_011..33];
    let _ = interp.stack.push_slice(slice);

    let _ = &mut host;
}

