#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-242-17-ruklee-intrinsics-copy-nonoverlapping-dst-valid-cda16c51a0")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_242_17_ruklee_intrinsics_copy_nonoverlapping_dst_valid_cda16c51a0() {
    let mut __unsat_rerun_sym_000 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut stack = Stack::new();

    let mut backing = Vec::with_capacity(__unsat_rerun_sym_000);
    backing.push(revm_primitives::U256::from(__unsat_rerun_sym_001));
    backing.push(revm_primitives::U256::from(__unsat_rerun_sym_002));

    stack.data_mut().extend_from_slice(&backing);

    let _ = stack.dup(__unsat_rerun_sym_003);
}

