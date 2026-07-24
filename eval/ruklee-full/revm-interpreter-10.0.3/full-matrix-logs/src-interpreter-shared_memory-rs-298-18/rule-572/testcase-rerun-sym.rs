#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-shared-memory-rs-298-18-rule-572-42acedf051")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_shared_memory_rs_298_18_rule_572_42acedf051() {
    let mut __unsat_rerun_sym_000 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 44;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 55;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 66;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut mem = crate::SharedMemory::with_capacity(__unsat_rerun_sym_000);
    mem.buffer = vec![__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006];
    mem.checkpoints = vec![__unsat_rerun_sym_007];
    mem.last_checkpoint = __unsat_rerun_sym_008;

    let slice = mem.context_memory();
    let _ = slice.len();
}

