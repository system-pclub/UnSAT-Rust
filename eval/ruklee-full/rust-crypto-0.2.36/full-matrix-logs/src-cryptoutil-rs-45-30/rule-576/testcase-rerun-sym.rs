#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-cryptoutil-rs-45-30-rule-576-da5659eac6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_cryptoutil_rs_45_30_rule_576_da5659eac6() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut dst = [__unsat_rerun_sym_000; 16];
    let input = [__unsat_rerun_sym_001, __unsat_rerun_sym_002];

    dst[__unsat_rerun_sym_003] = __unsat_rerun_sym_004;
    dst[__unsat_rerun_sym_005] = __unsat_rerun_sym_006;

    crate::cryptoutil::write_u64v_le(&mut dst[..], &input[..]);
}

