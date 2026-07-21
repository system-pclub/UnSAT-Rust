#[cfg(feature = "unsat-poc-src-lib-rs-593-23-rule-447-1c417c214b")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_593_23_rule_447_1c417c214b() {
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
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 16);
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    klee_ext_bind::assume!(__unsat_rerun_sym_006 <= 16);
    let mut __unsat_rerun_sym_007 = 99u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    klee_ext_bind::assume!(__unsat_rerun_sym_007 <= 1584);
    let mut v: StackVec<[u8; __unsat_rerun_sym_000]> = StackVec::from_slice(&[__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004]);
    v.length = __unsat_rerun_sym_005;
    v.insert_many(__unsat_rerun_sym_006, [__unsat_rerun_sym_007].iter().cloned());
}

