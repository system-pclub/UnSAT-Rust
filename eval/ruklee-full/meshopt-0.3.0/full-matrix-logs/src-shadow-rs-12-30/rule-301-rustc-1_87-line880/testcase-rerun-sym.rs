#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-shadow-rs-12-30-rule-301-rustc-1-87-line880-1abb13fe04")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_shadow_rs_12_30_rule_301_rustc_1_87_line880_1abb13fe04() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let indices: [u32; 1] = [__unsat_rerun_sym_000];

    let vertex_bytes: [u8; 4] = [__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004];
    let vertices = VertexDataAdapter {
        reader: std::io::Cursor::new(&vertex_bytes),
        vertex_count: __unsat_rerun_sym_005,
        vertex_stride: __unsat_rerun_sym_006,
        position_offset: __unsat_rerun_sym_007,
    };

    let _ = generate_shadow_indices(&indices, &vertices);
}

