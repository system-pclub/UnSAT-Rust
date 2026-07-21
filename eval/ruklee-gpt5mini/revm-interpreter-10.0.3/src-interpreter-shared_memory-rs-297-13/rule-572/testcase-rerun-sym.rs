#[cfg(feature = "unsat-poc-src-interpreter-shared-memory-rs-297-13-rule-572-a170f873fd")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_shared_memory_rs_297_13_rule_572_a170f873fd() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 2720);
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 16);
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    klee_ext_bind::assume!(__unsat_rerun_sym_004 <= 32);
    let mut mem = crate::SharedMemory::new();
    mem.new_context();

    mem.resize(__unsat_rerun_sym_000);
    mem.buffer[__unsat_rerun_sym_001] = __unsat_rerun_sym_002;

    mem.new_context();
    mem.resize(__unsat_rerun_sym_003);

    mem.last_checkpoint = __unsat_rerun_sym_004;

    let _ = mem.context_memory();
}

