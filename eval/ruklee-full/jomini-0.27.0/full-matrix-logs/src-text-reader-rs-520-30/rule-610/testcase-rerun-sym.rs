#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-520-30-rule-610-8a1a3b3310")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_520_30_rule_610_8a1a3b3310() {
    let mut __unsat_rerun_sym_000 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    use std::io::Cursor;

    let data = b"abcde";
    let cursor = Cursor::new(&data[..]);
    let mut reader = crate::text::TokenReader::new(cursor);

    let _ = reader.read_bytes(__unsat_rerun_sym_000).unwrap();
}

