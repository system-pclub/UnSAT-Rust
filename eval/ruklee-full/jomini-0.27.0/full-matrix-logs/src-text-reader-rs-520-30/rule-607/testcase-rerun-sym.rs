#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-520-30-rule-607-95c339b98f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_520_30_rule_607_95c339b98f() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    use std::io::Cursor;

    let data = b"abcd";
    let mut reader = crate::text::TokenReader::new(Cursor::new(&data[..]));

    let _ = reader.read_bytes(__unsat_rerun_sym_000);
}

