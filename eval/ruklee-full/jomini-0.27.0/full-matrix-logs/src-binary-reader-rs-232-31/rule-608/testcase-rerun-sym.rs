#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-232-31-rule-608-5b8fc902e0")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_232_31_rule_608_5b8fc902e0() {
    let mut __unsat_rerun_sym_000 = 0x2d;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x28;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x04;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    use crate::binary::TokenReader;

    let data = [__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004, __unsat_rerun_sym_005];
    let mut reader = TokenReader::from_slice(&data[..]);

    let _ = reader.next();
    let _ = reader.next();
}

