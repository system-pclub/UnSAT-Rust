#[cfg(feature = "unsat-poc-src-lib-rs-563-28-rule-431-342db0100c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_563_28_rule_431_342db0100c() {
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
    let mut __unsat_rerun_sym_005 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 48);
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    klee_ext_bind::assume!(__unsat_rerun_sym_006 <= 16);
    let mut __unsat_rerun_sym_007 = 99;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    klee_ext_bind::assume!(__unsat_rerun_sym_007 <= 1584);
    let mut v: StackVec<[u8; __unsat_rerun_sym_000]> = StackVec::from_buf([__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004]);
    v.length = __unsat_rerun_sym_005;
    v.insert(__unsat_rerun_sym_006, __unsat_rerun_sym_007);
}

