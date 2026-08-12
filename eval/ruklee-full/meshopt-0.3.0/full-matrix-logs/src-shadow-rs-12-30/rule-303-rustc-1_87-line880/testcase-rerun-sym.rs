#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-shadow-rs-12-30-rule-303-rustc-1-87-line880-f48f52c517")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_shadow_rs_12_30_rule_303_rustc_1_87_line880_f48f52c517() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let vertex_bytes: &'static [u8] = Box::leak(vec![0u8; 8].into_boxed_slice());
    let vertices = VertexDataAdapter {
        reader: std::io::Cursor::new(vertex_bytes),
        vertex_count: __unsat_rerun_sym_000,
        vertex_stride: __unsat_rerun_sym_001,
        position_offset: __unsat_rerun_sym_002,
    };

    let indices: &[u32] = &[__unsat_rerun_sym_003];
    let _ = generate_shadow_indices(indices, &vertices);
}

