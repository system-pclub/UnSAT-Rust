#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-copyless-rs-18-13-rule-326-4a963687e7")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_copyless_rs_18_13_rule_326_4a963687e7() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x41u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x42u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    use crate::copyless::VecHelper;

    let mut backing = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    let alloc = backing.alloc();
    let _ = alloc.init(__unsat_rerun_sym_002);

    let mut backing2 = vec![__unsat_rerun_sym_003; __unsat_rerun_sym_004];
    let alloc2 = backing2.alloc();
    let _ = alloc2.init(__unsat_rerun_sym_005);
}

