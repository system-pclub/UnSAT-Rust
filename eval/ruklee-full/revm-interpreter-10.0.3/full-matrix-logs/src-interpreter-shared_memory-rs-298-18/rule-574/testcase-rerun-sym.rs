#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-shared-memory-rs-298-18-rule-574-16596c762f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_shared_memory_rs_298_18_rule_574_16596c762f() {
    let mut __unsat_rerun_sym_000 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 33u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    use crate::interpreter::SharedMemory;

    let mut mem = SharedMemory::with_capacity(__unsat_rerun_sym_000);
    mem.new_context();

    let buf = [__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003];
    mem.resize(buf.len());
    mem.slice_mut(__unsat_rerun_sym_004, buf.len()).copy_from_slice(&buf);

    let _ = mem.context_memory();

    mem.free_context();

    let mut mem2 = SharedMemory::with_capacity(__unsat_rerun_sym_005);
    mem2.new_context();
    mem2.resize(__unsat_rerun_sym_006);
    mem2.slice_mut(__unsat_rerun_sym_007, __unsat_rerun_sym_008).copy_from_slice(&[7u8]);

    let _ = mem2.context_memory();
}

