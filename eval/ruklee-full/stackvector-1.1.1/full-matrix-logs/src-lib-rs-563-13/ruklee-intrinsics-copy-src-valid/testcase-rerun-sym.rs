#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-563-13-ruklee-intrinsics-copy-src-valid-b4788c01a1")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_563_13_ruklee_intrinsics_copy_src_valid_b4788c01a1() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 7;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let buf = [__unsat_rerun_sym_000; 1];
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf_and_len(buf, __unsat_rerun_sym_001);
    v.insert(__unsat_rerun_sym_002, __unsat_rerun_sym_003);
}

