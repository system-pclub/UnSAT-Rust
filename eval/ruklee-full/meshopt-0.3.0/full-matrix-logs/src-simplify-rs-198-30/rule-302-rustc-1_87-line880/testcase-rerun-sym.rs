#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-198-30-rule-302-rustc-1-87-line880-a353eb179d")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_198_30_rule_302_rustc_1_87_line880_a353eb179d() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let vertex_bytes = [__unsat_rerun_sym_000; 12];
    let vertices = VertexDataAdapter {
        reader: std::io::Cursor::new(&vertex_bytes),
        vertex_count: __unsat_rerun_sym_001,
        vertex_stride: __unsat_rerun_sym_002,
        position_offset: __unsat_rerun_sym_003,
    };

    let indices: [u32; 1] = [__unsat_rerun_sym_004];
    let _ = simplify_sloppy(&indices, &vertices, __unsat_rerun_sym_005, __unsat_rerun_sym_006, None);
}

