#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-267-37-rule-448-b31012694f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_267_37_rule_448_b31012694f() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let real_row = Box::new([0.0_f64]);
    let imag_row = Box::new([0.0_f64]);

    let real_rows: Box<[*mut f64; 1]> = Box::new([real_row.as_ptr() as *mut f64]);
    let imag_rows: Box<[*mut f64; 1]> = Box::new([imag_row.as_ptr() as *mut f64]);

    let mut m = ComplexMatrixN {
        complex_matrix: quest_sys::ComplexMatrixN {
            numQubits: 0,
            real: real_rows.as_ptr() as *mut *mut f64,
            imag: imag_rows.as_ptr() as *mut *mut f64,
        },
        dimension: __unsat_rerun_sym_000,
    };

    let _ = m.set(__unsat_rerun_sym_001, __unsat_rerun_sym_002, Complex64::new(__unsat_rerun_sym_003, __unsat_rerun_sym_004));
}

