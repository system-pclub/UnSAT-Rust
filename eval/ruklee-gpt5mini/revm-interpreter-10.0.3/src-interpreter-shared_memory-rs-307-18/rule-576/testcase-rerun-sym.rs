#[cfg(feature = "unsat-poc-src-interpreter-shared-memory-rs-307-18-rule-576-46aaf2392f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_shared_memory_rs_307_18_rule_576_46aaf2392f() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 32);
    let mut mem = SharedMemory::new();
    mem.new_context();
    mem.resize(__unsat_rerun_sym_000);
    mem.last_checkpoint = __unsat_rerun_sym_001;
    let _ = mem.context_memory_mut();
}

