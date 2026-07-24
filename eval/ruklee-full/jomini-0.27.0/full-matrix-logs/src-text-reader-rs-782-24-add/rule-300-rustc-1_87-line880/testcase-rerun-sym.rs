#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-782-24-add-rule-300-rustc-1-87-line880-0f3efc6645")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_782_24_add_rule_300_rustc_1_87_line880_0f3efc6645() {
    use std::io::Cursor;

    let data = b"        {";
    let cursor = Cursor::new(&data[..]);
    let mut reader = crate::text::TokenReader::new(cursor);

    let _ = reader.next();
}

