#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-contract-rs-82-48-rule-290-9a99bf7a3f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_contract_rs_82_48_rule_290_9a99bf7a3f() {
    let mut __unsat_rerun_sym_000 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    use crate::{
        gas::Gas,
        interpreter::{Contract, Interpreter},
        interpreter_action::{CallValue, EOFCreateKind, InterpreterAction},
        primitives::{Address, Bytes, Eof, U256},
        DummyHost, FunctionStack, InstructionResult, SharedMemory, Stack,
    };
    use std::sync::Arc;

    let eof_bytes = Bytes::from_static(b"\xef\x00\x01\x01\x00\x04\x02\x00\x01\x00\x01\x04\x00\x00\x00\x00\x80\x00\x00\xfe");
    let eof = Eof::decode(eof_bytes).unwrap();
    let eof = Arc::new(eof);

    let mut contract = Contract::default();
    contract.bytecode = crate::primitives::Bytecode::Eof(eof.clone());
    contract.target_address = Address::ZERO;
    contract.caller = Address::ZERO;
    contract.call_value = U256::ZERO;

    let mut interpreter = Interpreter {
        instruction_pointer: [1u8, 0u8].as_ptr(),
        gas: Gas::new(__unsat_rerun_sym_000),
        contract,
        instruction_result: InstructionResult::Continue,
        bytecode: Bytes::new(),
        is_eof: __unsat_rerun_sym_001,
        is_eof_init: __unsat_rerun_sym_002,
        shared_memory: SharedMemory::new(),
        stack: Stack::new(),
        function_stack: FunctionStack::new(),
        return_data_buffer: Bytes::new(),
        is_static: __unsat_rerun_sym_003,
        next_action: InterpreterAction::None,
    };

    interpreter.stack.push(U256::ZERO).unwrap();
    interpreter.stack.push(U256::ZERO).unwrap();
    interpreter.stack.push(U256::ZERO).unwrap();
    interpreter.stack.push(U256::ZERO).unwrap();

    interpreter.shared_memory.resize(__unsat_rerun_sym_004);

    let mut host = DummyHost::default();
    crate::instructions::contract::eofcreate(&mut interpreter, &mut host);

    let _ = EOFCreateKind::default();
    let _ = CallValue::default();
}

