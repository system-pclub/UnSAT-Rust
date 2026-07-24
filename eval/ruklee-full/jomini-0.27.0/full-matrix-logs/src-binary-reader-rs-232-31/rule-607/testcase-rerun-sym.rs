#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-232-31-rule-607-ea84892c7c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_232_31_rule_607_ea84892c7c() {
    let mut __unsat_rerun_sym_000 = 0x2d;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x28;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x03;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x04;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0xff;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0xff;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    use crate::binary::TokenReader;
    use std::io::Cursor;

    let data = vec![
        __unsat_rerun_sym_000, __unsat_rerun_sym_001, // token id
        __unsat_rerun_sym_002, __unsat_rerun_sym_003, // '='
        __unsat_rerun_sym_004, __unsat_rerun_sym_005, // '{'
        __unsat_rerun_sym_006, __unsat_rerun_sym_007, // '}'
        __unsat_rerun_sym_008, __unsat_rerun_sym_009, // trailing token
    ];

    let cursor = Cursor::new(data);
    let mut reader = TokenReader::builder().buffer_len(__unsat_rerun_sym_010).build(cursor);

    let _ = reader.next();
    let _ = reader.next();
    let _ = reader.next();
    let _ = reader.next();
}

