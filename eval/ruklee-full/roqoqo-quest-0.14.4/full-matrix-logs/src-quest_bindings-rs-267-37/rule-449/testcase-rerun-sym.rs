#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-267-37-rule-449-0bf63d018f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_267_37_rule_449_0bf63d018f() {
    let mut __unsat_rerun_sym_000 = 1usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let row = __unsat_rerun_sym_000;
    let column = __unsat_rerun_sym_001;
    let value = Complex64::new(__unsat_rerun_sym_002, __unsat_rerun_sym_003);

    let mut real_row0 = [__unsat_rerun_sym_004; 1];
    let mut real_row1 = [__unsat_rerun_sym_005; 1];
    let mut imag_row0 = [__unsat_rerun_sym_006; 1];
    let mut imag_row1 = [__unsat_rerun_sym_007; 1];

    let real_rows = vec![real_row0.as_mut_ptr(), real_row1.as_mut_ptr()];
    let imag_rows = vec![imag_row0.as_mut_ptr(), imag_row1.as_mut_ptr()];

    let complex_matrix = quest_sys::ComplexMatrixN {
        real: real_rows.as_ptr() as *mut *mut f64,
        imag: imag_rows.as_ptr() as *mut *mut f64,
        numQubits: 1,
    };

    let mut receiver = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_008,
    };

    let _ = receiver.set(row, column, value);
}

