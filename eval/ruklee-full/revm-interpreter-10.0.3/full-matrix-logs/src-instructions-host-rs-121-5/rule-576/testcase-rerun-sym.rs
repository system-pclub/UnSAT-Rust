#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-host-rs-121-5-rule-576-be65e526ba")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_host_rs_121_5_rule_576_be65e526ba() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0xdead_beefu64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    use crate::{
        instructions::host,
        interpreter::{Contract, Interpreter},
        primitives::{Address, Bytecode, Bytes, U256},
        DummyHost, Gas, InstructionResult,
    };

    let mut interp = Interpreter {
        instruction_pointer: core::ptr::null(),
        gas: Gas::new(__unsat_rerun_sym_000),
        contract: Contract {
            input: Bytes::new(),
            bytecode: Bytecode::LegacyRaw(Bytes::from([0x00])),
            hash: None,
            target_address: Address::ZERO,
            bytecode_address: None,
            caller: Address::ZERO,
            call_value: U256::ZERO,
        },
        instruction_result: InstructionResult::Continue,
        bytecode: Bytes::from([0x00]),
        is_eof: __unsat_rerun_sym_001,
        is_eof_init: __unsat_rerun_sym_002,
        shared_memory: crate::SharedMemory::new(),
        stack: crate::Stack::new(),
        function_stack: crate::FunctionStack::new(),
        return_data_buffer: Bytes::new(),
        is_static: __unsat_rerun_sym_003,
        next_action: crate::InterpreterAction::None,
    };

    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_004));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_005));

    let mut host = DummyHost::default();
    host.storage.insert(U256::from(__unsat_rerun_sym_006), U256::from(__unsat_rerun_sym_007));

    host::sload::<DummyHost, crate::primitives::BerlinSpec>(&mut interp, &mut host);
}

