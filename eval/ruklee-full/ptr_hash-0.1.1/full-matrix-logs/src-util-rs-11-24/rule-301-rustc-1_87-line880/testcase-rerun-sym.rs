#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-util-rs-11-24-rule-301-rustc-1-87-line880-b36a8fca45")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_util_rs_11_24_rule_301_rustc_1_87_line880_b36a8fca45() {
    let mut __unsat_rerun_sym_000 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let backing = [__unsat_rerun_sym_000; 2];
    let s: &[u64] = &backing[..1];
    crate::util::prefetch_index(s, __unsat_rerun_sym_001);
}

