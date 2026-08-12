#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-551-23-rule-431-bfeafd7a93")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_551_23_rule_431_bfeafd7a93() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut backing = [__unsat_rerun_sym_000; 2];
    let mut v: StackVec<[u8; 2]> = StackVec::from_buf(backing);
    v.length = __unsat_rerun_sym_001;
    let _ = v.remove(__unsat_rerun_sym_002);
}

