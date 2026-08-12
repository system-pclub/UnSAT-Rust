#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-563-13-ruklee-intrinsics-copy-aligned-4033e2e06c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_563_13_ruklee_intrinsics_copy_aligned_4033e2e06c() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 9;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut v = StackVec::<[u8; 1]>::from_buf([7]);
    v.length = __unsat_rerun_sym_000;
    v.insert(__unsat_rerun_sym_001, __unsat_rerun_sym_002);
}

