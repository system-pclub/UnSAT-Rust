#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-232-31-rule-604-4b696fb29b")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_232_31_rule_604_4b696fb29b() {
    let mut __unsat_rerun_sym_000 = 0x03u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x04;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    use crate::binary::TokenReader;

    let backing = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003];
    let mut reader = TokenReader::from_slice(&backing);

    let _ = reader.next();
}

