#[cfg(feature = "unsat-poc-src-lib-rs-475-13-rule-326-4dc7f00af0")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_475_13_rule_326_4dc7f00af0() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 32);
    let mut __unsat_rerun_sym_002 = 7;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 112);
    let mut v: StackVec<[u8; __unsat_rerun_sym_000]> = StackVec::new();
    v.length = __unsat_rerun_sym_001;
    v.push(__unsat_rerun_sym_002);
}

