#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-520-30-rule-606-f79f7af9de")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_520_30_rule_606_f79f7af9de() {
    let mut __unsat_rerun_sym_000 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    use std::io::Cursor;

    let data = b"abc";
    let mut reader = crate::text::TokenReader::new(Cursor::new(&data[..]));

    let _ = reader.read_bytes(__unsat_rerun_sym_000).unwrap();

    let backing = [__unsat_rerun_sym_001; 1];
    let mut reader2 = crate::text::TokenReader::builder()
        .buffer_len(__unsat_rerun_sym_002)
        .build(Cursor::new(&backing[..]));
    let _ = reader2.read_bytes(__unsat_rerun_sym_003).unwrap();
}

