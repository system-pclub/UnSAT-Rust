#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-optimize-rs-166-30-rule-301-rustc-1-87-line880-aab7c6ae15")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_optimize_rs_166_30_rule_301_rustc_1_87_line880_aab7c6ae15() {
    let mut __unsat_rerun_sym_000 = 7;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 13;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0xBB;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0xCC;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0xDD;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let indices: &mut [u32] = &mut [__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002];

    let vertex_bytes: Vec<u8> = vec![__unsat_rerun_sym_003, __unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006];
    let vertices = VertexDataAdapter {
        reader: std::io::Cursor::new(vertex_bytes.as_slice()),
        vertex_count: __unsat_rerun_sym_007,
        vertex_stride: __unsat_rerun_sym_008,
        position_offset: __unsat_rerun_sym_009,
    };

    optimize_overdraw_in_place(indices, &vertices, __unsat_rerun_sym_010);
}

