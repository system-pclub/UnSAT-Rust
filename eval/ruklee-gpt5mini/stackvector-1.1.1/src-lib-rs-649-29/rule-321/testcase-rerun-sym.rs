#[cfg(feature = "unsat-poc-src-lib-rs-649-29-rule-321-81f76cba51")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_649_29_rule_321_81f76cba51() {
    let mut __unsat_rerun_sym_000 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 64);
    let mut __unsat_rerun_sym_001 = 10;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 160);
    let mut __unsat_rerun_sym_002 = 20;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 320);
    let mut __unsat_rerun_sym_003 = 30;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 480);
    let mut __unsat_rerun_sym_004 = 40;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    klee_ext_bind::assume!(__unsat_rerun_sym_004 <= 640);
    let mut __unsat_rerun_sym_005 = 5;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 80);
    let mut v: StackVec<[u8; __unsat_rerun_sym_000]> = StackVec::from_buf([__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004]);
    v.length = __unsat_rerun_sym_005;
    let _ = v.into_inner();
}

