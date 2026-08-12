#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-268-14-rule-449-9421d3a7f2")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_268_14_rule_449_9421d3a7f2() {
    let mut __unsat_rerun_sym_000 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 3.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut row0_real = vec![__unsat_rerun_sym_000];
    let mut row1_real = vec![__unsat_rerun_sym_001];
    let mut real_rows: Vec<*mut f64> = vec![row0_real.as_mut_ptr(), row1_real.as_mut_ptr()];

    let mut row0_imag = vec![__unsat_rerun_sym_002];
    let mut row1_imag = vec![__unsat_rerun_sym_003];
    let mut imag_rows: Vec<*mut f64> = vec![row0_imag.as_mut_ptr(), row1_imag.as_mut_ptr()];

    let complex_matrix = quest_sys::ComplexMatrixN {
        real: real_rows.as_mut_ptr(),
        imag: imag_rows.as_mut_ptr(),
        numQubits: 1,
    };

    let mut matrix = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_004,
    };

    let _ = matrix.set(__unsat_rerun_sym_005, __unsat_rerun_sym_006, Complex64::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));
}

