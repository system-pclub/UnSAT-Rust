#[cfg(feature = "unsat-poc-src-copyless-rs-18-29-rule-447-7d9b55d9e0")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_copyless_rs_18_29_rule_447_7d9b55d9e0() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 1u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut v = vec![__unsat_rerun_sym_000];
    let alloc = VecHelper::alloc(&mut v);
    let _ = alloc.init(__unsat_rerun_sym_001);
}

