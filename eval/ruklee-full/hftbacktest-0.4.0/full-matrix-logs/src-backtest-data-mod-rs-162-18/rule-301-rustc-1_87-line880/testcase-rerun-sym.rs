#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-backtest-data-mod-rs-162-18-rule-301-rustc-1-87-line880-ac08490491")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_backtest_data_mod_rs_162_18_rule_301_rustc_1_87_line880_ac08490491() {
    let mut __unsat_rerun_sym_000 = 11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 7u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut backing = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let ptr = crate::backtest::data::DataPtr::new(backing.len());
    let _ = ptr.at(__unsat_rerun_sym_002);

    let mut short = vec![__unsat_rerun_sym_003];
    let short_ptr = crate::backtest::data::DataPtr::new(short.len());
    let _ = short_ptr.at(__unsat_rerun_sym_004);

    backing[__unsat_rerun_sym_005] = __unsat_rerun_sym_006;
    short[0] = 44;
}

