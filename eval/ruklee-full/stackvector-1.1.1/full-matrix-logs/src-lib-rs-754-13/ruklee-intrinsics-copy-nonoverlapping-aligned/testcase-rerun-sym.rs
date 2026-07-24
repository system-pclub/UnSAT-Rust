#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-754-13-ruklee-intrinsics-copy-nonoverlapping-aligned-4d995169b4")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_754_13_ruklee_intrinsics_copy_nonoverlapping_aligned_4d995169b4() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 7u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut backing = [__unsat_rerun_sym_000; 1];
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf(backing);
    let src = [__unsat_rerun_sym_001];
    v.insert_from_slice(__unsat_rerun_sym_002, &src);
}

