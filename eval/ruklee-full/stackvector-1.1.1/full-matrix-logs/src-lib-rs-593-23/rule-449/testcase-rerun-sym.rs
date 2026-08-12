#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-593-23-rule-449-b4066b4a22")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_593_23_rule_449_b4066b4a22() {
    let mut __unsat_rerun_sym_000 = 10u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 20u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let buf = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let mut v: StackVec<[u32; 2]> = StackVec::from_buf_and_len(buf, __unsat_rerun_sym_002);

    let extra = Vec::from([30u32]);
    v.insert_many(__unsat_rerun_sym_003, extra);
}

