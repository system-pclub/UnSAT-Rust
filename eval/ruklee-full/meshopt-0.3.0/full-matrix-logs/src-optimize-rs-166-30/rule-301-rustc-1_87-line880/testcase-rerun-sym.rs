#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-optimize-rs-166-30-rule-301-rustc-1-87-line880-aab7c6ae15")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_optimize_rs_166_30_rule_301_rustc_1_87_line880_aab7c6ae15() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let vertex_bytes = [__unsat_rerun_sym_000; 4];
    let vertices = VertexDataAdapter {
        reader: std::io::Cursor::new(&vertex_bytes[..]),
        vertex_count: __unsat_rerun_sym_001,
        vertex_stride: __unsat_rerun_sym_002,
        position_offset: __unsat_rerun_sym_003,
    };

    let mut indices = [__unsat_rerun_sym_004; 1];
    optimize_overdraw_in_place(&mut indices, &vertices, __unsat_rerun_sym_005);
}

