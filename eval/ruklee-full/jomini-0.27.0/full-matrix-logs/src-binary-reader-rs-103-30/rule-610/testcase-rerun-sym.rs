#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-103-30-rule-610-e82f6823ab")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_103_30_rule_610_e82f6823ab() {
    let mut __unsat_rerun_sym_000 = 0x43u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x44;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x45;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    use crate::binary::TokenReader;

    let backing = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002];
    let mut reader = TokenReader::from_slice(&backing);

    let _ = reader.read_bytes(__unsat_rerun_sym_003);
}

