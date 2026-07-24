#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-488-22-rule-321-f0ec5de942")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_488_22_rule_321_f0ec5de942() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let buf = [__unsat_rerun_sym_000; 1];
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf_and_len(buf, __unsat_rerun_sym_001);

    v.length = __unsat_rerun_sym_002;
    v.data = core::mem::MaybeUninit::new([0u8; 1]);

    let _ = v.pop();
}

