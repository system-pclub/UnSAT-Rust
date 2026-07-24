#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-268-14-rule-448-9d06e98951")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_268_14_rule_448_9d06e98951() {
    let mut __unsat_rerun_sym_000 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0.0f64;
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
    let mut row0 = vec![__unsat_rerun_sym_000];
    let mut row1 = vec![__unsat_rerun_sym_001];
    let mut real_rows: Vec<*mut f64> = vec![row0.as_mut_ptr(), row1.as_mut_ptr()];
    let mut imag_rows: Vec<*mut f64> = vec![row0.as_mut_ptr(), row1.as_mut_ptr()];

    let complex_matrix = quest_sys::ComplexMatrixN {
        numQubits: 1,
        real: real_rows.as_mut_ptr(),
        imag: imag_rows.as_mut_ptr(),
    };

    let mut matrix = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_002,
    };

    let _ = matrix.set(__unsat_rerun_sym_003, __unsat_rerun_sym_004, Complex64::new(__unsat_rerun_sym_005, __unsat_rerun_sym_006));
}

