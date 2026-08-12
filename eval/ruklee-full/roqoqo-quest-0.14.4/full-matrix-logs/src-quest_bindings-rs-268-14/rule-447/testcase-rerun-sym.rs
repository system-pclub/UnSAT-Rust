#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-268-14-rule-447-7bf02914d9")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_268_14_rule_447_7bf02914d9() {
    let mut __unsat_rerun_sym_000 = 0.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0.0_f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 2.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut real_rows = vec![vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001], vec![__unsat_rerun_sym_002; __unsat_rerun_sym_003]];
    let mut imag_rows = vec![vec![__unsat_rerun_sym_004; __unsat_rerun_sym_005], vec![__unsat_rerun_sym_006; __unsat_rerun_sym_007]];

    let real_ptrs: Vec<*mut f64> = real_rows.iter_mut().map(|row| row.as_mut_ptr()).collect();
    let imag_ptrs: Vec<*mut f64> = imag_rows.iter_mut().map(|row| row.as_mut_ptr()).collect();

    let complex_matrix = quest_sys::ComplexMatrixN {
        numQubits: 1,
        real: real_ptrs.as_ptr() as *mut *mut f64,
        imag: imag_ptrs.as_ptr() as *mut *mut f64,
    };

    let mut receiver = ComplexMatrixN {
        complex_matrix,
        dimension: __unsat_rerun_sym_008,
    };

    let _ = receiver.set(__unsat_rerun_sym_009, __unsat_rerun_sym_010, Complex64::new(__unsat_rerun_sym_011, __unsat_rerun_sym_012));
}

