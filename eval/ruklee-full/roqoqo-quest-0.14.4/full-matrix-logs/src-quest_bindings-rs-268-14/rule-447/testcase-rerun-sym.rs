#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-268-14-rule-447-7bf02914d9")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_268_14_rule_447_7bf02914d9() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 3.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut matrix = ComplexMatrixN::new(__unsat_rerun_sym_000);
    matrix.dimension = __unsat_rerun_sym_001;

    let real_rows = vec![vec![__unsat_rerun_sym_002]];
    let imag_rows = vec![vec![__unsat_rerun_sym_003]];
    let real_ptrs: Vec<*mut f64> = real_rows
        .into_iter()
        .map(|mut row| row.as_mut_ptr())
        .collect();
    let imag_ptrs: Vec<*mut f64> = imag_rows
        .into_iter()
        .map(|mut row| row.as_mut_ptr())
        .collect();

    matrix.complex_matrix.real = real_ptrs.as_ptr() as *mut *mut f64;
    matrix.complex_matrix.imag = imag_ptrs.as_ptr() as *mut *mut f64;

    let _ = matrix.set(__unsat_rerun_sym_004, __unsat_rerun_sym_005, Complex64::new(__unsat_rerun_sym_006, __unsat_rerun_sym_007));
}

