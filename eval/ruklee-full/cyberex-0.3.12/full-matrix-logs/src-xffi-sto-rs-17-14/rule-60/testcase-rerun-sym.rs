#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-xffi-sto-rs-17-14-rule-60-93bb13e97e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_xffi_sto_rs_17_14_rule_60_93bb13e97e() {
    let mut __unsat_rerun_sym_000 = b'X';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let bytes = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let c_str = bytes.as_ptr().cast::<c_char>();
    let _ = crate::xffi::sto::cchar_to_string(c_str);
}

