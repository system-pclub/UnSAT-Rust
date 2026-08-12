#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-551-23-rule-432-8110f75845")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_551_23_rule_432_8110f75845() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut v: StackVec<[u8; 2]> = StackVec::from_buf([11, 22]);
    v.length = __unsat_rerun_sym_000;
    let _ = v.remove(__unsat_rerun_sym_001);
}

