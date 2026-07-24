#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-267-37-rule-448-b31012694f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_267_37_rule_448_b31012694f() {
    let mut __unsat_rerun_sym_000 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut real_rows = vec![vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001]];
    let mut imag_rows = vec![vec![__unsat_rerun_sym_002; __unsat_rerun_sym_003]];

    let mut real_ptrs: Vec<*mut f64> = real_rows.iter_mut().map(|row| row.as_mut_ptr()).collect();
    let mut imag_ptrs: Vec<*mut f64> = imag_rows.iter_mut().map(|row| row.as_mut_ptr()).collect();

    let matrix = quest_sys::ComplexMatrixN {
        real: real_ptrs.as_mut_ptr(),
        imag: imag_ptrs.as_mut_ptr(),
        numQubits: 0,
    };

    let mut receiver = ComplexMatrixN {
        complex_matrix: matrix,
        dimension: __unsat_rerun_sym_004,
    };

    let _ = receiver.set(__unsat_rerun_sym_005, __unsat_rerun_sym_006, Complex64::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));
}

