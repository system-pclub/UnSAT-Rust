#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-595-35-rule-302-rustc-1-87-line880-94a1f9a973")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_595_35_rule_302_rustc_1_87_line880_94a1f9a973() {
    use std::io::Cursor;

    let data = b"{aaaaaaaaaaaaaaaa}";
    let cursor = Cursor::new(&data[..]);
    let mut reader = crate::text::TokenReader::new(cursor);

    let _ = reader.read();
    let _ = reader.skip_container();
}

