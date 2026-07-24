#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-rank-select-mod-rs-328-21-rule-574-c92d39a18e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_rank_select_mod_rs_328_21_rule_574_c92d39a18e() {
    let mut __unsat_rerun_sym_000 = 64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let content: Box<[u64]> = vec![0u64; 9].into_boxed_slice();
    let ranks: Box<[u32]> = vec![0u32].into_boxed_slice();

    let r = crate::RankSimple { content, ranks };

    let _ = crate::RankSimple::try_rank(&r, __unsat_rerun_sym_000);
}

