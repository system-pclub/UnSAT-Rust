#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-103-30-rule-604-044be5aa9e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_103_30_rule_604_044be5aa9e() {
    let mut __unsat_rerun_sym_000 = 0x41u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x42u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    use crate::binary::TokenReader;
    use std::io::Cursor;

    let backing = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let reader = Cursor::new(backing);
    let mut tr = TokenReader::builder().buffer_len(__unsat_rerun_sym_002).build(reader);

    let _ = tr.read_bytes(__unsat_rerun_sym_003);
}

