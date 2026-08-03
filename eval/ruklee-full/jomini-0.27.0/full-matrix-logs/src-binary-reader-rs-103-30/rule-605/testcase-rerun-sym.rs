#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-103-30-rule-605-61835641d2")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_103_30_rule_605_61835641d2() {
    let mut __unsat_rerun_sym_000 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x43;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    use std::io::Cursor;

    struct OneByteReader {
        data: Cursor<Vec<u8>>,
    }

    impl std::io::Read for OneByteReader {
        fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
            self.data.read(out)
        }
    }

    let mut reader = crate::binary::TokenReader::builder()
        .buffer_len(__unsat_rerun_sym_000)
        .build(OneByteReader {
            data: Cursor::new(vec![__unsat_rerun_sym_001]),
        });

    let _ = reader.read_bytes(__unsat_rerun_sym_002);
}

