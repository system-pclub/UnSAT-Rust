#[cfg(feature = "unsat-poc-src-rank-select-select-rs-115-26-rule-572-d0f6b60ca9")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_rank_select_select_rs_115_26_rule_572_d0f6b60ca9() {
    let mut __unsat_rerun_sym_000 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = 0usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 16);
    let content = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001].into_boxed_slice();
    let r: crate::RankSelect101111 = content.into();

    let mut rank = __unsat_rerun_sym_002;
    let _ = r.select(rank);

    let _ = rank;
}

