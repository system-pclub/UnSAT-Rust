#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-cryptoutil-rs-129-31-rule-576-6d4cc6ca4b")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_cryptoutil_rs_129_31_rule_576_6d4cc6ca4b() {
    let mut __unsat_rerun_sym_000 = 0u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut dst = [__unsat_rerun_sym_000; 1];
    let input = [__unsat_rerun_sym_001; 4];
    read_u32v_be(&mut dst[..], &input[..]);
}

