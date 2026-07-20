// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

#![deny(missing_docs)]
#![warn(private_intra_doc_links)]
#![warn(missing_crate_level_docs)]
#![warn(missing_doc_code_examples)]
#![warn(private_doc_tests)]
#![deny(missing_debug_implementations)]

//! # roqoqo-quest
//!
//! [QuEST](https://github.com/QuEST-Kit/QuEST) simulator backend for the roqoqo quantum computing toolkit.
//!
//! roqoqo-quest provides a backend to simulate roqoqo quantum circuits with the QuEST simulator

mod interface;
pub use interface::{call_circuit, call_operation, call_operation_with_device};
mod backend;
pub use backend::Backend;
mod quest_bindings;
pub use quest_bindings::*;

#[cfg(test)]
mod unsat_ir_instantiations {
    use num_complex::Complex64;
    use roqoqo::operations::{DefinitionBit, Operation};
    use std::collections::HashMap;

    #[test]
    #[ignore = "compile-only LLVM IR instantiation"]
    fn instantiate_mirscan_callers() {
        let mut qureg = super::Qureg::new(1, false);
        let _ = qureg.probabilites();
        let _ = qureg.state_vector();
        let _ = qureg.density_matrix_flattened_row_major();

        let mut matrix = super::ComplexMatrixN::new(1);
        let _ = matrix.set(0, 0, Complex64::new(0.0, 0.0));

        let operation = Operation::from(DefinitionBit::new("ir".to_owned(), 1, true));
        let mut bit_registers = HashMap::<String, Vec<bool>>::new();
        let mut float_registers = HashMap::<String, Vec<f64>>::new();
        let mut complex_registers = HashMap::<String, Vec<Complex64>>::new();
        let mut bit_registers_output = HashMap::<String, Vec<Vec<bool>>>::new();
        let mut device = None;
        let _ = super::call_operation_with_device(
            &operation,
            &mut qureg,
            &mut bit_registers,
            &mut float_registers,
            &mut complex_registers,
            &mut bit_registers_output,
            &mut device,
        );
    }
}
