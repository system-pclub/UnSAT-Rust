#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-text-reader-rs-751-24-read-unaligned-ruklee-const-ptr-read-unaligned-initial-cf09d1f375")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_text_reader_rs_751_24_read_unaligned_ruklee_const_ptr_read_unaligned_initial_cf09d1f375() {
    let mut __unsat_rerun_sym_000 = b'a';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    use std::io::Cursor;

    let data = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    let mut reader = crate::text::TokenReader::new(Cursor::new(data));

    let _ = reader.read();
}

