#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-util-rs-11-24-rule-302-rustc-1-87-line880-a5b61e156c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_util_rs_11_24_rule_302_rustc_1_87_line880_a5b61e156c() {
    let mut __unsat_rerun_sym_000 = 7u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let s: &[u64] = &backing[..1];
    crate::util::prefetch_index(s, __unsat_rerun_sym_002);
}

