#[cfg(feature = "unsat-poc-src-rank-select-mod-rs-328-21-rule-572-eace8f05f3")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_rank_select_mod_rs_328_21_rule_572_eace8f05f3() {
    let mut __unsat_rerun_sym_000 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = 0u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 16);
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 16);
    let mut __unsat_rerun_sym_004 = 512;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    klee_ext_bind::assume!(__unsat_rerun_sym_004 <= 8192);
    let content = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001].into_boxed_slice();
    let ranks = vec![__unsat_rerun_sym_002; __unsat_rerun_sym_003].into_boxed_slice();

    let rs = RankSimple { content, ranks };

    let _ = rs.try_rank(__unsat_rerun_sym_004);
}

