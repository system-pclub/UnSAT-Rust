#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-rs-360-45-rule-290-a2b8cad0c2")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_rs_360_45_rule_290_a2b8cad0c2() {
    let mut __unsat_rerun_sym_000 = 1_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x00u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x01u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    use crate::{DummyHost, Gas, InstructionResult, Interpreter};
    use revm_primitives::{Bytecode, Bytes};

    let code = Bytes::from_static(&[0x00, 0x00]);
    let contract = Interpreter::new(
        crate::Contract::new(
            Bytes::new(),
            Bytecode::new_legacy(code.clone()),
            None,
            Default::default(),
            None,
            Default::default(),
            Default::default(),
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );

    let mut interp = contract;
    let backing = [__unsat_rerun_sym_002, __unsat_rerun_sym_003];
    interp.instruction_pointer = backing.as_ptr();

    let mut host = DummyHost::default();
    let mut table: [fn(&mut Interpreter, &mut DummyHost); 256] = [|_, _| {}; 256];

    interp.step(&table, &mut host);

    let _ = Gas::new(__unsat_rerun_sym_004);
    let _ = InstructionResult::Continue;
}

