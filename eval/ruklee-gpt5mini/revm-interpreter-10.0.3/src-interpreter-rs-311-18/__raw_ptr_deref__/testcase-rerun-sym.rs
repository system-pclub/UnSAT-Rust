#[cfg(feature = "unsat-poc-src-interpreter-rs-311-18-raw-ptr-deref-34f2a3a6fe")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_rs_311_18_raw_ptr_deref_34f2a3a6fe() {
    let mut __unsat_rerun_sym_000 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    use crate::{Contract, DummyHost, Gas, Interpreter};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut interp = Interpreter::new(
        Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from([__unsat_rerun_sym_000])),
            None,
            Default::default(),
            None,
            Default::default(),
            U256::ZERO,
        ),
        u64::MAX,
        __unsat_rerun_sym_001,
    );

    let _host = DummyHost::default();

    interp.bytecode = Bytes::new();
    interp.instruction_pointer = interp.bytecode.as_ptr();

    let _ = interp.current_opcode();
}

