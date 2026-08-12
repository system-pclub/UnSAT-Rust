#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-util-rs-11-24-rule-303-rustc-1-87-line880-ffd1ba1ecb")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_util_rs_11_24_rule_303_rustc_1_87_line880_ffd1ba1ecb() {
    let mut __unsat_rerun_sym_000 = 10u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 20u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let s: &[u64] = &backing[..1];
    util::prefetch_index(s, __unsat_rerun_sym_002);
}

