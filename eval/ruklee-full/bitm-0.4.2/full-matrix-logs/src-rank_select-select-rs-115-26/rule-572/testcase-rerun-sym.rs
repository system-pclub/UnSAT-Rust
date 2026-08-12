#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-rank-select-select-rs-115-26-rule-572-d0f6b60ca9")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_rank_select_select_rs_115_26_rule_572_d0f6b60ca9() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let n: u64 = __unsat_rerun_sym_000;
    let rank: u8 = __unsat_rerun_sym_001;

    let _ = crate::rank_select::select64(n, rank);
}

