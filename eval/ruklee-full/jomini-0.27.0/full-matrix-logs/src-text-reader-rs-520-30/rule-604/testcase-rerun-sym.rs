#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-520-30-rule-604-d1b300e010")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_520_30_rule_604_d1b300e010() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    use std::io::Cursor;

    let data = b"abcde";
    let mut reader = crate::text::TokenReader::new(Cursor::new(&data[..]));

    let _ = reader.read_bytes(__unsat_rerun_sym_000);
}

