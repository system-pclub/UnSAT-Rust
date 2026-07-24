#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-198-30-rule-303-rustc-1-87-line880-b013fa8560")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_198_30_rule_303_rustc_1_87_line880_b013fa8560() {
    let mut __unsat_rerun_sym_000 = 0x10;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0x11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0x12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0x13;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0x20;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0x21;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0x23;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x30;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0x31;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 0x32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 0x33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 0x40;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 0x41;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 0x42;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 0x43;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    let mut __unsat_rerun_sym_016 = 8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_016, "__unsat_rerun_sym_016");
    let mut __unsat_rerun_sym_017 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_017, "__unsat_rerun_sym_017");
    let mut __unsat_rerun_sym_018 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_018, "__unsat_rerun_sym_018");
    let mut __unsat_rerun_sym_019 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_019, "__unsat_rerun_sym_019");
    let mut __unsat_rerun_sym_020 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_020, "__unsat_rerun_sym_020");
    let mut __unsat_rerun_sym_021 = 0.0f32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_021, "__unsat_rerun_sym_021");
    let mut __unsat_rerun_sym_022 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_022, "__unsat_rerun_sym_022");
    let vertex_data: [u8; 16] = [
        __unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003,
        __unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006, __unsat_rerun_sym_007,
        __unsat_rerun_sym_008, __unsat_rerun_sym_009, __unsat_rerun_sym_010, __unsat_rerun_sym_011,
        __unsat_rerun_sym_012, __unsat_rerun_sym_013, __unsat_rerun_sym_014, __unsat_rerun_sym_015,
    ];

    let vertices = crate::VertexDataAdapter::new(&vertex_data, __unsat_rerun_sym_016, __unsat_rerun_sym_017).unwrap();

    let indices: [u32; 3] = [__unsat_rerun_sym_018, __unsat_rerun_sym_019, __unsat_rerun_sym_020];
    let mut result_error = __unsat_rerun_sym_021;

    let _ = crate::simplify::simplify_sloppy(
        &indices,
        &vertices,
        __unsat_rerun_sym_022,
        0.0,
        Some(&mut result_error),
    );
}

