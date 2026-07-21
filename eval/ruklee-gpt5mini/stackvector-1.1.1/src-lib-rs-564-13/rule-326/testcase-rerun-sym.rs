#[cfg(feature = "unsat-poc-src-lib-rs-564-13-rule-326-7e7d29a943")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_564_13_rule_326_7e7d29a943() {
    let mut __unsat_rerun_sym_000 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 32);
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 32);
    let mut __unsat_rerun_sym_003 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 48);
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    klee_ext_bind::assume!(__unsat_rerun_sym_004 <= 32);
    let mut __unsat_rerun_sym_005 = 9;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 144);
    let mut v: StackVec<[u8; __unsat_rerun_sym_000]> = StackVec::new();
    v.push(__unsat_rerun_sym_001);
    v.push(__unsat_rerun_sym_002);
    v.length = __unsat_rerun_sym_003;
    v.insert(__unsat_rerun_sym_004, __unsat_rerun_sym_005);
}

