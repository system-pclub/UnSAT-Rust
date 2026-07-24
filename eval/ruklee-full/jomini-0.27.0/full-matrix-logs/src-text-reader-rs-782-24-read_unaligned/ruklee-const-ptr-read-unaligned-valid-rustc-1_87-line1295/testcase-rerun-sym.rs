#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-782-24-ruklee-const-ptr-read-unaligned-valid-rustc-1-87-line1-059f4abd4f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_782_24_ruklee_const_ptr_read_unaligned_valid_rustc_1_87_line1_059f4abd4f() {
    let mut __unsat_rerun_sym_000 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    use std::io::Cursor;

    let data = b"abc";
    let cursor = Cursor::new(&data[..]);

    let mut reader = crate::text::TokenReader::builder()
        .buffer_len(__unsat_rerun_sym_000)
        .build(cursor);

    let _ = reader.next();
}

