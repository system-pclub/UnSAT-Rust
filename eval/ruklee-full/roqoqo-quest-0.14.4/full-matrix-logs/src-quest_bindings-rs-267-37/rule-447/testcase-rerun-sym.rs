#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-267-37-rule-447-85578bd325")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_267_37_rule_447_85578bd325() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 3.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 4.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let real_row0 = Box::new([1.0_f64]);
    let imag_row0 = Box::new([2.0_f64]);

    let real_rows = vec![real_row0.as_ptr() as *mut f64];
    let imag_rows = vec![imag_row0.as_ptr() as *mut f64];

    let complex_matrix = quest_sys::ComplexMatrixN {
        numQubits: 0,
        real: real_rows.as_ptr() as *mut *mut f64,
        imag: imag_rows.as_ptr() as *mut *mut f64,
    };

    let mut receiver = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_000,
    };

    let _ = receiver.set(__unsat_rerun_sym_001, __unsat_rerun_sym_002, Complex64::new(__unsat_rerun_sym_003, __unsat_rerun_sym_004));
}

