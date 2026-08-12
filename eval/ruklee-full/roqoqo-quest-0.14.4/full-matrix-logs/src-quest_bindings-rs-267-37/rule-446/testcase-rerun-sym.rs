#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-267-37-rule-446-272b960e6b")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_267_37_rule_446_272b960e6b() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut real_rows: Vec<Box<[f64]>> = vec![vec![0.0f64, 0.0f64].into_boxed_slice()];
    let mut imag_rows: Vec<Box<[f64]>> = vec![vec![0.0f64, 0.0f64].into_boxed_slice()];

    let mut real_ptrs: Vec<*mut f64> = real_rows
        .iter_mut()
        .map(|row| row.as_mut_ptr())
        .collect();
    let mut imag_ptrs: Vec<*mut f64> = imag_rows
        .iter_mut()
        .map(|row| row.as_mut_ptr())
        .collect();

    let complex_matrix = quest_sys::ComplexMatrixN {
        real: real_ptrs.as_mut_ptr(),
        imag: imag_ptrs.as_mut_ptr(),
        numQubits: 1,
    };

    let mut m = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_000,
    };

    let _ = m.set(__unsat_rerun_sym_001, __unsat_rerun_sym_002, Complex64::new(__unsat_rerun_sym_003, __unsat_rerun_sym_004));
}

