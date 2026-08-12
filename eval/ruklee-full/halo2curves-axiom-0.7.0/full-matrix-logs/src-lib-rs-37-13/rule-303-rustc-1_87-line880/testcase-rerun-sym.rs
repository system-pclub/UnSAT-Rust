#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-37-13-rule-303-rustc-1-87-line880-48d5fd0300")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_37_13_rule_303_rustc_1_87_line880_48d5fd0300() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let data: &[u8] = &[__unsat_rerun_sym_000];
    crate::prefetch(data, __unsat_rerun_sym_001);
}

