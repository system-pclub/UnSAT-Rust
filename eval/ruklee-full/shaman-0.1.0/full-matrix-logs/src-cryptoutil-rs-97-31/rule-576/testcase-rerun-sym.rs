#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-cryptoutil-rs-97-31-rule-576-97440ca20a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_cryptoutil_rs_97_31_rule_576_97440ca20a() {
    let mut __unsat_rerun_sym_000 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut dst = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    let input = vec![__unsat_rerun_sym_002; __unsat_rerun_sym_003];
    crate::cryptoutil::read_u64v_be(&mut dst[..], &input[..]);
}

