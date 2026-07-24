#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-520-30-rule-603-8fb291d340")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_520_30_rule_603_8fb291d340() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    use std::io::Cursor;

    let data = b"abc";
    let cursor = Cursor::new(&data[..]);
    let mut reader = crate::text::TokenReader::new(cursor);

    let buf = vec![0u8; 8].into_boxed_slice();
    reader = crate::text::TokenReader::builder().buffer(buf).build(reader.into_parts().1);

    let _ = reader.read_bytes(__unsat_rerun_sym_000);
}

