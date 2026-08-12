#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-550-24-rule-322-dbbd054db5")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_550_24_rule_322_dbbd054db5() {
    let mut __unsat_rerun_sym_000 = 11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 7u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut backing = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let mut v: StackVec<[u8; 2]> = StackVec::from_buf(backing);
    v.length = __unsat_rerun_sym_002;
    let _ = v.remove(__unsat_rerun_sym_003);

    let mut short_backing = [__unsat_rerun_sym_004];
    let mut w: StackVec<[u8; 1]> = StackVec::from_buf(short_backing);
    w.length = __unsat_rerun_sym_005;
    let _ = w.remove(__unsat_rerun_sym_006);
}

