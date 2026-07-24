#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-271-14-rule-447-530bf88924")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_271_14_rule_447_530bf88924() {
    let mut __unsat_rerun_sym_000 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 2.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 3.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 7.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 9.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut row0 = vec![__unsat_rerun_sym_000];
    let mut row1 = vec![__unsat_rerun_sym_001];
    let real_rows: Vec<*mut f64> = vec![row0.as_mut_ptr(), row1.as_mut_ptr()];

    let mut imag_row0 = vec![__unsat_rerun_sym_002];
    let mut imag_row1 = vec![__unsat_rerun_sym_003];
    let imag_rows: Vec<*mut f64> = vec![imag_row0.as_mut_ptr(), imag_row1.as_mut_ptr()];

    let complex_matrix = quest_sys::ComplexMatrixN {
        real: real_rows.as_ptr() as *mut *mut f64,
        imag: imag_rows.as_ptr() as *mut *mut f64,
        numQubits: 1,
    };

    let mut receiver = crate::quest_bindings::ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_004,
    };

    let _ = receiver.set(__unsat_rerun_sym_005, __unsat_rerun_sym_006, Complex64::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));
}

