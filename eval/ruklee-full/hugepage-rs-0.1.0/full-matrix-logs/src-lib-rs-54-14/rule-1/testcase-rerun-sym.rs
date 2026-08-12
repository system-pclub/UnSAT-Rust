#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-54-14-rule-1-884e0e21dc")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_54_14_rule_1_884e0e21dc() {
    let mut __unsat_rerun_sym_000 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let layout = Layout::from_size_align(__unsat_rerun_sym_000, __unsat_rerun_sym_001).unwrap();
    let _ = alloc(layout);
}

