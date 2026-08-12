#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-103-30-rule-604-044be5aa9e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_103_30_rule_604_044be5aa9e() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x41;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    use crate::binary::TokenReader;

    struct OneByteReader {
        done: bool,
    }

    impl std::io::Read for OneByteReader {
        fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
            if self.done || out.is_empty() {
                return Ok(__unsat_rerun_sym_000);
            }
            out[__unsat_rerun_sym_001] = __unsat_rerun_sym_002;
            self.done = __unsat_rerun_sym_003;
            Ok(__unsat_rerun_sym_004)
        }
    }

    let mut reader = TokenReader::builder().buffer_len(__unsat_rerun_sym_005).build(OneByteReader { done: __unsat_rerun_sym_006 });

    let _ = reader.read_bytes(__unsat_rerun_sym_007);
}

