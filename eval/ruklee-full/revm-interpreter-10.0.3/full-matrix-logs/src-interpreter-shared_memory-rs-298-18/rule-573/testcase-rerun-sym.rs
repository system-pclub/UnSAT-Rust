#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-shared-memory-rs-298-18-rule-573-6d635a5040")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_shared_memory_rs_298_18_rule_573_6d635a5040() {
    let mut __unsat_rerun_sym_000 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut mem = crate::SharedMemory::with_capacity(__unsat_rerun_sym_000);
    mem.resize(__unsat_rerun_sym_001);
    mem.new_context();
    mem.resize(__unsat_rerun_sym_002);
    mem.free_context();

    let mut mem2 = crate::SharedMemory::with_capacity(__unsat_rerun_sym_003);
    mem2.resize(__unsat_rerun_sym_004);
    mem2.new_context();
    mem2.resize(__unsat_rerun_sym_005);

    let _ = mem.context_memory();
    let _ = mem2.context_memory();
}

