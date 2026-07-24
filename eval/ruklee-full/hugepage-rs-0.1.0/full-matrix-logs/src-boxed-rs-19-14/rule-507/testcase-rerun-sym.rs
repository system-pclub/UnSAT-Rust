#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-boxed-rs-19-14-rule-507-521b61bcf9")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_boxed_rs_19_14_rule_507_521b61bcf9() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let _ = crate::Box::new(__unsat_rerun_sym_000);
}

