#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-270-37-rule-448-5d1c220a01")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_270_37_rule_448_5d1c220a01() {
    let mut __unsat_rerun_sym_000 = 0usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let row = __unsat_rerun_sym_000;
    let column = __unsat_rerun_sym_001;
    let value = Complex64::new(__unsat_rerun_sym_002, __unsat_rerun_sym_003);

    let real_row = vec![__unsat_rerun_sym_004];
    let imag_row = vec![__unsat_rerun_sym_005];
    let real_rows = vec![real_row.as_ptr() as *mut f64];
    let imag_rows = vec![imag_row.as_ptr() as *mut f64];

    let complex_matrix = quest_sys::ComplexMatrixN {
        numQubits: 0,
        real: real_rows.as_ptr() as *mut *mut f64,
        imag: imag_rows.as_ptr() as *mut *mut f64,
    };

    let mut matrix = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_006,
    };

    let _ = matrix.set(row, column, value);
}

