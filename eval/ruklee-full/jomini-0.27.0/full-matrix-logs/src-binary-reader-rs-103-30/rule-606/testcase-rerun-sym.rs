#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-103-30-rule-606-2c620ecd6a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_103_30_rule_606_2c620ecd6a() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    use crate::binary::TokenReader;

    let backing = [__unsat_rerun_sym_000; 1];
    let mut reader = TokenReader::from_slice(&backing);

    let _ = reader.read_bytes(__unsat_rerun_sym_001);
}

