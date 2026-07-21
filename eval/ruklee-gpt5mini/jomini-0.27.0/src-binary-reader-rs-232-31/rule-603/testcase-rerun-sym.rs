#[cfg(feature = "unsat-poc-src-binary-reader-rs-232-31-rule-603-3197bf2d02")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_binary_reader_rs_232_31_rule_603_3197bf2d02() {
    let mut __unsat_rerun_sym_000 = 0x82;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 2080);
    let mut __unsat_rerun_sym_001 = 0x2d;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 720);
    let mut __unsat_rerun_sym_002 = 0x01;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 16);
    let mut __unsat_rerun_sym_003 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 16);
    let mut __unsat_rerun_sym_004 = 0x0f;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    klee_ext_bind::assume!(__unsat_rerun_sym_004 <= 240);
    let mut __unsat_rerun_sym_005 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    klee_ext_bind::assume!(__unsat_rerun_sym_005 <= 16);
    let mut __unsat_rerun_sym_006 = 0x03;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    klee_ext_bind::assume!(__unsat_rerun_sym_006 <= 48);
    let mut __unsat_rerun_sym_007 = 0x00;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    klee_ext_bind::assume!(__unsat_rerun_sym_007 <= 16);
    let mut __unsat_rerun_sym_008 = 0x45;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    klee_ext_bind::assume!(__unsat_rerun_sym_008 <= 1104);
    let mut __unsat_rerun_sym_009 = 0x4e;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    klee_ext_bind::assume!(__unsat_rerun_sym_009 <= 1248);
    let mut __unsat_rerun_sym_010 = 0x47;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    klee_ext_bind::assume!(__unsat_rerun_sym_010 <= 1136);
    use std::io::Cursor;

    let data = [
        __unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009, __unsat_rerun_sym_010,
    ];

    let mut reader = crate::binary::TokenReader::new(Cursor::new(&data[..]));

    let _ = reader.next();
    let _ = reader.next();
    let _ = reader.next();
    let _ = reader.next();
}

