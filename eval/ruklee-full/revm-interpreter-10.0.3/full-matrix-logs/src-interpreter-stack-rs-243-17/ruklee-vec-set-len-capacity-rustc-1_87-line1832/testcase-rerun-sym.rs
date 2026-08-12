#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-243-17-ruklee-vec-set-len-capacity-rustc-1-87-line1832-ded236af04")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_243_17_ruklee_vec_set_len_capacity_rustc_1_87_line1832_ded236af04() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter, InterpreterAction};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut interp = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from([0x00])),
            None,
            revm_primitives::Address::ZERO,
            None,
            revm_primitives::Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );

    interp.instruction_result = InstructionResult::Continue;
    interp.is_eof = __unsat_rerun_sym_002;
    interp.is_eof_init = __unsat_rerun_sym_003;
    interp.next_action = InterpreterAction::None;
    interp.gas = Gas::new(__unsat_rerun_sym_004);

    let mut host = DummyHost::default();

    let data = interp.stack.data_mut();
    data.clear();
    data.reserve_exact(__unsat_rerun_sym_005);
    data.push(U256::from(__unsat_rerun_sym_006));
    data.push(U256::from(__unsat_rerun_sym_007));

    let _ = interp.stack.dup(__unsat_rerun_sym_008);

    let _ = &mut host;
}

