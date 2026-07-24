#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-551-23-rule-431-bfeafd7a93")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_551_23_rule_431_bfeafd7a93() {
    let mut __unsat_rerun_sym_000 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut backing = Vec::from([10u8, 20u8]);
    let mut v: StackVec<[u8; 2]> = StackVec::from_buf_and_len([10u8, 20u8], __unsat_rerun_sym_000);

    let _ = backing.as_mut_ptr();

    let _ = v.remove(__unsat_rerun_sym_001);
}

