#[cfg(feature = "unsat-poc-src-utilities-rs-11-14-rule-603-23988d9c46")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_utilities_rs_11_14_rule_603_23988d9c46() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 64);
    let data = [__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    let typed = &data[..];

    let _ = crate::utilities::typed_to_bytes(typed);
}

