#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-267-37-rule-449-0bf63d018f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_267_37_rule_449_0bf63d018f() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 3.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 4.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let real_rows: Vec<Box<[f64]>> = vec![vec![1.0f64].into_boxed_slice()];
    let imag_rows: Vec<Box<[f64]>> = vec![vec![2.0f64].into_boxed_slice()];

    let real_ptrs: Vec<*mut f64> = real_rows
        .iter()
        .map(|row| row.as_ptr() as *mut f64)
        .collect();
    let imag_ptrs: Vec<*mut f64> = imag_rows
        .iter()
        .map(|row| row.as_ptr() as *mut f64)
        .collect();

    let complex_matrix = quest_sys::ComplexMatrixN {
        numQubits: __unsat_rerun_sym_000,
        real: real_ptrs.as_ptr() as *mut *mut f64,
        imag: imag_ptrs.as_ptr() as *mut *mut f64,
    };

    let mut receiver = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_001,
    };

    let _ = receiver.set(__unsat_rerun_sym_002, __unsat_rerun_sym_003, Complex64::new(__unsat_rerun_sym_004, __unsat_rerun_sym_005));
}

