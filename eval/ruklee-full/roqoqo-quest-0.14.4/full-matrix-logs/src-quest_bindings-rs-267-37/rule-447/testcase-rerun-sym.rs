#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-267-37-rule-447-85578bd325")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_267_37_rule_447_85578bd325() {
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
    let mut __unsat_rerun_sym_008 = 8.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut real_row0 = [__unsat_rerun_sym_000];
    let mut real_row1 = [__unsat_rerun_sym_001];
    let mut imag_row0 = [__unsat_rerun_sym_002];
    let mut imag_row1 = [__unsat_rerun_sym_003];

    let real_rows: [*mut f64; 2] = [real_row0.as_mut_ptr(), real_row1.as_mut_ptr()];
    let imag_rows: [*mut f64; 2] = [imag_row0.as_mut_ptr(), imag_row1.as_mut_ptr()];

    let complex_matrix = quest_sys::ComplexMatrixN {
        real: real_rows.as_ptr() as *mut *mut f64,
        imag: imag_rows.as_ptr() as *mut *mut f64,
        numQubits: 1,
    };

    let mut matrix = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_004,
    };

    let _ = matrix.set(__unsat_rerun_sym_005, __unsat_rerun_sym_006, Complex64::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));
}

