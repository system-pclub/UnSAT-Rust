#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-quest-bindings-rs-268-14-rule-448-9d06e98951")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_quest_bindings_rs_268_14_rule_448_9d06e98951() {
    let mut __unsat_rerun_sym_000 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0.0f64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 3.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 4.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut row0 = [__unsat_rerun_sym_000];
    let mut row1 = [__unsat_rerun_sym_001];
    let mut imag0 = [__unsat_rerun_sym_002];
    let mut imag1 = [__unsat_rerun_sym_003];

    let real_rows = [row0.as_mut_ptr(), row1.as_mut_ptr()];
    let imag_rows = [imag0.as_mut_ptr(), imag1.as_mut_ptr()];

    let mut m = ComplexMatrixN {
        complex_matrix: quest_sys::ComplexMatrixN {
            numQubits: 1,
            real: real_rows.as_ptr() as *mut *mut f64,
            imag: imag_rows.as_ptr() as *mut *mut f64,
        },
        dimension: __unsat_rerun_sym_004,
    };

    let _ = m.set(__unsat_rerun_sym_005, __unsat_rerun_sym_006, Complex64::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));
}

