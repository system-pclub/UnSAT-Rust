#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-39-30-rule-302-rustc-1-87-line880-67b3bbc6b4")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_39_30_rule_302_rustc_1_87_line880_67b3bbc6b4() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let vertex_bytes = [__unsat_rerun_sym_000; 12];
    let vertices = VertexDataAdapter::new(&vertex_bytes, __unsat_rerun_sym_001, __unsat_rerun_sym_002).unwrap();
    let indices: [u32; 0] = [];
    let _ = simplify(&indices, &vertices, __unsat_rerun_sym_003, __unsat_rerun_sym_004, SimplifyOptions::empty(), None);
}

