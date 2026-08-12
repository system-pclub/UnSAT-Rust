#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-backtest-data-mod-rs-162-18-rule-302-rustc-1-87-line880-1e85d90cde")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_backtest_data_mod_rs_162_18_rule_302_rustc_1_87_line880_1e85d90cde() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut backing = crate::utils::AlignedArray::<u8, { crate::utils::CACHE_LINE_SIZE }>::new(__unsat_rerun_sym_000);
    backing.as_mut_slice()[0] = 7;

    let data_ptr = crate::backtest::data::DataPtr::new(__unsat_rerun_sym_001);
    let _ = data_ptr.at(__unsat_rerun_sym_002);

    let _ = backing.as_ptr();
}

