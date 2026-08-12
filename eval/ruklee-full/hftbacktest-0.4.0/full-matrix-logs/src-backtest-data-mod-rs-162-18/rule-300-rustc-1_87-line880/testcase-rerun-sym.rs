#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-backtest-data-mod-rs-162-18-rule-300-rustc-1-87-line880-0838fd92ba")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_backtest_data_mod_rs_162_18_rule_300_rustc_1_87_line880_0838fd92ba() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 7u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let backing = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    let ptr = crate::backtest::data::DataPtr::new(backing.len());
    let _ = ptr.at(__unsat_rerun_sym_002);

    let tiny = vec![__unsat_rerun_sym_003];
    let dp = crate::backtest::data::DataPtr::new(tiny.len());
    let _ = dp.at(__unsat_rerun_sym_004);
}

