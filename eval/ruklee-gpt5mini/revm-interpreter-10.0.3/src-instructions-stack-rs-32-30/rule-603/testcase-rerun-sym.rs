#[cfg(feature = "unsat-poc-src-instructions-stack-rs-32-30-rule-603-6f64bff3bf")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_stack_rs_32_30_rule_603_6f64bff3bf() {
    let mut __unsat_rerun_sym_000 = 0x60;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 1536);
    let mut __unsat_rerun_sym_001 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 2720);
    let mut __unsat_rerun_sym_002 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 16);
    let mut __unsat_rerun_sym_003 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    klee_ext_bind::assume!(__unsat_rerun_sym_004 <= 16);
    use crate::{instructions::stack, Contract, DummyHost, Interpreter};
    use revm_primitives::{Bytecode, Bytes};

    let mut host = DummyHost::default();

    let mut interp = Interpreter::new(
        Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from([__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002])),
            None,
            Default::default(),
            None,
            Default::default(),
            Default::default(),
        ),
        u64::MAX,
        __unsat_rerun_sym_003,
    );

    interp.instruction_pointer = interp.bytecode.as_ptr();
    stack::push::<__unsat_rerun_sym_004, DummyHost>(&mut interp, &mut host);
}

