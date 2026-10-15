//! Concrete inputs for replaying the REVM verification obligations.
//! All receiver construction and mutations below use safe crate APIs.

use crate::{Contract, DummyHost, Interpreter, Stack};
use crate::primitives::{Address, Bytecode, Bytes, U256};

fn interpreter() -> Interpreter {
    Interpreter::new(
        Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from_static(&[0x00])),
            None,
            Address::ZERO,
            None,
            Address::ZERO,
            U256::ZERO,
        ),
        100_000,
        false,
    )
}

#[no_mangle]
pub extern "C" fn __revm_replay_dup() {
    let mut stack = Stack::new();
    let mut data = Vec::with_capacity(1);
    data.push(U256::from(11u64));
    *stack.data_mut() = data;
    let _ = stack.dup(1);
}

#[no_mangle]
pub extern "C" fn __revm_replay_push_slice() {
    let mut stack = Stack::new();
    let mut data = Vec::with_capacity(1);
    data.push(U256::from(11u64));
    *stack.data_mut() = data;
    let _ = stack.push_slice(&[0xabu8]);
}

#[no_mangle]
pub extern "C" fn __revm_replay_exchange() {
    let mut stack = Stack::new();
    let mut data = Vec::with_capacity(1);
    data.push(U256::from(11u64));
    *stack.data_mut() = data;
    let _ = stack.exchange(usize::MAX, 1);
}

#[no_mangle]
pub extern "C" fn __revm_replay_rjump() {
    let mut interp = interpreter();
    let immediate = [0x7fu8, 0xff];
    interp.instruction_pointer = immediate.as_ptr();
    interp.is_eof = true;
    crate::instructions::control::rjump(&mut interp, &mut DummyHost::default());
}

#[no_mangle]
pub extern "C" fn __revm_replay_rjumpi() {
    let mut interp = interpreter();
    let immediate = [0x7fu8, 0xff];
    interp.instruction_pointer = immediate.as_ptr();
    interp.is_eof = true;
    interp.stack.data_mut().push(U256::from(1u64));
    crate::instructions::control::rjumpi(&mut interp, &mut DummyHost::default());
}

#[no_mangle]
pub extern "C" fn __revm_replay_rjumpv() {
    let mut interp = interpreter();
    let table = [0u8, 0x7f, 0xff];
    interp.instruction_pointer = table.as_ptr();
    interp.is_eof = true;
    interp.stack.data_mut().push(U256::ZERO);
    crate::instructions::control::rjumpv(&mut interp, &mut DummyHost::default());
}

#[no_mangle]
pub extern "C" fn __revm_replay_program_counter() {
    let mut interp = interpreter();
    let other_allocation = [0u8; 8];
    interp.instruction_pointer = other_allocation.as_ptr();
    let _ = interp.program_counter();
}

#[no_mangle]
pub extern "C" fn __revm_replay_current_opcode() {
    let mut interp = interpreter();
    interp.instruction_pointer = core::ptr::null();
    let _ = interp.current_opcode();
}

#[no_mangle]
pub extern "C" fn __revm_replay_step() {
    let mut interp = interpreter();
    interp.instruction_pointer = core::ptr::null();
    let table: [fn(&mut Interpreter, &mut DummyHost); 256] = [|_, _| {}; 256];
    interp.step(&table, &mut DummyHost::default());
}

#[no_mangle]
pub extern "C" fn __revm_replay_load_eof_code() {
    let mut interp = interpreter();
    interp.contract.bytecode = Bytecode::Eof(std::sync::Arc::new(crate::primitives::Eof::default()));
    interp.load_eof_code(0, 1024);
}

#[no_mangle]
pub extern "C" fn __revm_replay_slice_range() {
    let memory = crate::SharedMemory::new();
    let _ = memory.slice_range(0..1);
}

#[no_mangle]
pub extern "C" fn __revm_replay_slice_mut() {
    let mut memory = crate::SharedMemory::new();
    let _ = memory.slice_mut(0, 1);
}

#[no_mangle]
pub extern "C" fn __revm_replay_stack_push() {
    let mut stack = Stack::new();
    *stack.data_mut() = Vec::new();
    let _ = stack.push(U256::ZERO);
}

#[no_mangle]
pub extern "C" fn __revm_replay_codesize() {
    let mut interp = interpreter();
    interp.contract.bytecode = Bytecode::Eof(std::sync::Arc::new(crate::primitives::Eof::default()));
    crate::instructions::system::codesize(&mut interp, &mut DummyHost::default());
}

#[no_mangle]
pub extern "C" fn __revm_replay_codecopy() {
    let mut interp = interpreter();
    interp.contract.bytecode = Bytecode::Eof(std::sync::Arc::new(crate::primitives::Eof::default()));
    interp.shared_memory = crate::SharedMemory::new();
    interp.stack.data_mut().extend_from_slice(&[U256::from(1u64), U256::ZERO, U256::ZERO]);
    crate::instructions::system::codecopy(&mut interp, &mut DummyHost::default());
}

#[no_mangle]
pub extern "C" fn __revm_replay_instruction_push() {
    let mut interp = interpreter();
    interp.instruction_pointer = core::ptr::null();
    crate::instructions::stack::push::<1, _>(&mut interp, &mut DummyHost::default());
}
