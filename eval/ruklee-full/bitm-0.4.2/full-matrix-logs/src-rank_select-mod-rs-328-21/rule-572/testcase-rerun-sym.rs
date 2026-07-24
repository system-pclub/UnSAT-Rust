#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-rank-select-mod-rs-328-21-rule-572-eace8f05f3")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_rank_select_mod_rs_328_21_rule_572_eace8f05f3() {
    let mut __unsat_rerun_sym_000 = 64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let content = vec![0u64; 1].into_boxed_slice();
    let ranks = vec![1u32, 0u32].into_boxed_slice();
    let r = crate::RankSimple { content, ranks };
    let _ = crate::Rank::try_rank(&r, __unsat_rerun_sym_000);
}

