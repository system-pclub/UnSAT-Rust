#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-binary-reader-rs-103-30-rule-607-ecab9a0a2c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_103_30_rule_607_ecab9a0a2c() {
    let mut __unsat_rerun_sym_000 = 0x82;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x2d;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x0f;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x03;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x45;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x4e;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 0x47;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 0x82;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 0x2d;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    let mut __unsat_rerun_sym_016 = 0x0f;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_016, "__unsat_rerun_sym_016");
    let mut __unsat_rerun_sym_017 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_017, "__unsat_rerun_sym_017");
    let mut __unsat_rerun_sym_018 = 0x03;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_018, "__unsat_rerun_sym_018");
    let mut __unsat_rerun_sym_019 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_019, "__unsat_rerun_sym_019");
    let mut __unsat_rerun_sym_020 = 0x45;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_020, "__unsat_rerun_sym_020");
    let mut __unsat_rerun_sym_021 = 0x4e;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_021, "__unsat_rerun_sym_021");
    let mut __unsat_rerun_sym_022 = 0x47;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_022, "__unsat_rerun_sym_022");
    let mut __unsat_rerun_sym_023 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_023, "__unsat_rerun_sym_023");
    let mut __unsat_rerun_sym_024 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_024, "__unsat_rerun_sym_024");
    use crate::binary::TokenReader;
    use std::io::Cursor;

    let data = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009, __unsat_rerun_sym_010];
    let mut reader = TokenReader::new(Cursor::new(data));

    let _ = reader.read_bytes(__unsat_rerun_sym_011);

    let data2 = vec![__unsat_rerun_sym_012, __unsat_rerun_sym_013, __unsat_rerun_sym_014, __unsat_rerun_sym_015, __unsat_rerun_sym_016, __unsat_rerun_sym_017, __unsat_rerun_sym_018, __unsat_rerun_sym_019, __unsat_rerun_sym_020, __unsat_rerun_sym_021, __unsat_rerun_sym_022];
    let mut reader2 = TokenReader::builder().buffer_len(__unsat_rerun_sym_023).build(Cursor::new(data2));

    let _ = reader2.read_bytes(__unsat_rerun_sym_024);
}

