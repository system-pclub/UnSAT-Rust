#[cfg(feature = "unsat-poc-src-copyless-rs-18-13-rule-326-4a963687e7")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_copyless_rs_18_13_rule_326_4a963687e7() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 1u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut v: Vec<u8> = Vec::new();
    v.push(__unsat_rerun_sym_000);

    let alloc = crate::copyless::VecHelper::alloc(&mut v);
    let _ = crate::copyless::VecAllocation::init(alloc, __unsat_rerun_sym_001);
}

