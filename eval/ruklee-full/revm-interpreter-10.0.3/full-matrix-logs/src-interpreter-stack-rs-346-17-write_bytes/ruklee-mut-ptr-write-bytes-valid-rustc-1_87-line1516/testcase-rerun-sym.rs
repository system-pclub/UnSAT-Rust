#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-346-17-write-bytes-ruklee-mut-ptr-write-bytes-valid-rus-9454a34345")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_346_17_write_bytes_ruklee_mut_ptr_write_bytes_valid_rus_9454a34345() {
    let mut __unsat_rerun_sym_000 = 0x1111u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x2222u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x3333u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 5;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 6;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 7;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 9;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    use crate::{
        interpreter::{Contract, Interpreter},
        primitives::{Address, Bytecode, Bytes, U256},
        DummyHost,
    };

    let mut stack = crate::Stack::new();
    stack.data_mut().push(U256::from(__unsat_rerun_sym_000));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_001));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_002));

    let bytecode = Bytecode::LegacyRaw(Bytes::from([0x00]));
    let contract = Contract::new(
        Bytes::new(),
        bytecode,
        None,
        Address::ZERO,
        None,
        Address::ZERO,
        U256::ZERO,
    );

    let mut interp = Interpreter::new(contract, __unsat_rerun_sym_003, __unsat_rerun_sym_004);
    interp.stack = stack;
    interp.is_eof = __unsat_rerun_sym_005;
    interp.shared_memory = crate::SharedMemory::new();
    interp.instruction_pointer = interp.bytecode.as_ptr();

    let slice = [__unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009, __unsat_rerun_sym_010, __unsat_rerun_sym_011, __unsat_rerun_sym_012, __unsat_rerun_sym_013, __unsat_rerun_sym_014];
    let mut host = DummyHost::default();
    let _ = crate::interpreter::stack::Stack::push_slice(&mut interp.stack, &slice);
    let _ = &mut host;
}

