#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-37-13-rule-302-rustc-1-87-line880-23dab1d7e3")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_37_13_rule_302_rustc_1_87_line880_23dab1d7e3() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let data = [__unsat_rerun_sym_000; 1];
    crate::prefetch(&data, __unsat_rerun_sym_001);
}

