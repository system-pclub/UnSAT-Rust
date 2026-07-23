//! # revm-interpreter
//!
//! REVM Interpreter.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc as std;

#[macro_use]
mod macros;

// silence lint
#[cfg(test)]
use serde_json as _;

#[cfg(test)]
use walkdir as _;

mod function_stack;
pub mod gas;
mod host;
mod instruction_result;
pub mod instructions;
pub mod interpreter;
pub mod interpreter_action;
pub mod opcode;

// Reexport primary types.
pub use function_stack::{FunctionReturnFrame, FunctionStack};
pub use gas::Gas;
pub use host::{
    AccountLoad, DummyHost, Eip7702CodeLoad, Host, SStoreResult, SelfDestructResult, StateLoad,
};
pub use instruction_result::*;
pub use interpreter::{
    analysis, num_words, Contract, Interpreter, InterpreterResult, SharedMemory, Stack,
    EMPTY_SHARED_MEMORY, STACK_LIMIT,
};
pub use interpreter_action::{
    CallInputs, CallOutcome, CallScheme, CallValue, CreateInputs, CreateOutcome, CreateScheme,
    EOFCreateInputs, EOFCreateKind, InterpreterAction,
};
pub use opcode::{Instruction, OpCode, OPCODE_INFO_JUMPTABLE};
pub use primitives::{MAX_CODE_SIZE, MAX_INITCODE_SIZE};

#[doc(hidden)]
pub use revm_primitives as primitives;

#[cfg(test)]
mod unsat_ir_instantiations {
    use super::*;
    use crate::primitives::{Bytecode, Bytes};

    #[test]
    #[ignore = "compile-only LLVM IR instantiation"]
    fn instantiate_interpreter_outcome_insertion() {
        let mut interpreter = Interpreter::new_bytecode(Bytecode::LegacyRaw(Bytes::from([0x00])));
        let result = InterpreterResult::new(InstructionResult::Stop, Bytes::new(), Gas::new(0));

        interpreter.insert_create_outcome(CreateOutcome::new(result.clone(), None));
        interpreter.insert_eofcreate_outcome(CreateOutcome::new(result.clone(), None));

        let mut shared_memory = SharedMemory::new();
        interpreter.insert_call_outcome(
            &mut shared_memory,
            CallOutcome::new(result, 0..0),
        );
    }
}
