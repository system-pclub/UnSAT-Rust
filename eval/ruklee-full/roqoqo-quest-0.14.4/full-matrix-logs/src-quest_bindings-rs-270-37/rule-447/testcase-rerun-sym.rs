#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-270-37-rule-447-a0438bdd6f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_270_37_rule_447_a0438bdd6f() {
    let mut __unsat_rerun_sym_000 = 0.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let row_storage = vec![vec![__unsat_rerun_sym_000]];
    let imag_rows: Vec<*mut f64> = row_storage
        .iter()
        .map(|r| r.as_ptr() as *mut f64)
        .collect();

    let real_storage = vec![vec![__unsat_rerun_sym_001]];
    let real_rows: Vec<*mut f64> = real_storage
        .iter()
        .map(|r| r.as_ptr() as *mut f64)
        .collect();

    let complex_matrix = quest_sys::ComplexMatrixN {
        numQubits: 0,
        real: real_rows.as_ptr() as *mut *mut f64,
        imag: imag_rows.as_ptr() as *mut *mut f64,
    };

    let mut receiver = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_002,
    };

    let _ = receiver.set(__unsat_rerun_sym_003, __unsat_rerun_sym_004, Complex64::new(__unsat_rerun_sym_005, __unsat_rerun_sym_006));
}

