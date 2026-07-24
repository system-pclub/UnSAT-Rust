#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-596-13-ruklee-intrinsics-copy-src-valid-288da223a1")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_596_13_ruklee_intrinsics_copy_src_valid_288da223a1() {
    let mut __unsat_rerun_sym_000 = 10u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 20u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 30u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let buf = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let mut v: StackVec<[u32; 2]> = StackVec::from_buf(buf);
    let extra = [__unsat_rerun_sym_002];
    v.insert_many(__unsat_rerun_sym_003, extra);
}

