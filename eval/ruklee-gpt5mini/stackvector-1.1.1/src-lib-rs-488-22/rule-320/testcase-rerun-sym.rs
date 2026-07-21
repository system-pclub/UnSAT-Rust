#[cfg(feature = "unsat-poc-src-lib-rs-488-22-rule-320-5e94ab32d9")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_488_22_rule_320_5e94ab32d9() {
    let mut __unsat_rerun_sym_000 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 32);
    let mut __unsat_rerun_sym_001 = 10;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 160);
    let mut __unsat_rerun_sym_002 = 20;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 320);
    let mut __unsat_rerun_sym_003 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 48);
    let mut v: StackVec<[u8; __unsat_rerun_sym_000]> = StackVec::from_buf([__unsat_rerun_sym_001, __unsat_rerun_sym_002]);
    v.length = __unsat_rerun_sym_003;
    let _ = v.pop();
}

