#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-593-23-rule-448-5904f599cb")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_593_23_rule_448_5904f599cb() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf([0]);
    v.length = __unsat_rerun_sym_000;
    v.insert_many(__unsat_rerun_sym_001, core::iter::empty::<u8>());
}

