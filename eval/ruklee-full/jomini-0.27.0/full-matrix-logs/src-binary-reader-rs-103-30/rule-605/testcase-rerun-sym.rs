#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-103-30-rule-605-61835641d2")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_103_30_rule_605_61835641d2() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x43;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    use std::io::Cursor;

    struct OneByteRead {
        data: Cursor<Vec<u8>>,
    }

    impl std::io::Read for OneByteRead {
        fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
            let mut tmp = [__unsat_rerun_sym_000; 1];
            let n = self.data.read(&mut tmp)?;
            if n == 0 || out.is_empty() {
                return Ok(n);
            }
            out[__unsat_rerun_sym_001] = tmp[__unsat_rerun_sym_002];
            Ok(__unsat_rerun_sym_003)
        }
    }

    let mut reader = crate::binary::TokenReader::builder()
        .buffer_len(__unsat_rerun_sym_004)
        .build(OneByteRead {
            data: Cursor::new(vec![__unsat_rerun_sym_005, __unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009]),
        });

    let _ = reader.read_bytes(__unsat_rerun_sym_010);
}

