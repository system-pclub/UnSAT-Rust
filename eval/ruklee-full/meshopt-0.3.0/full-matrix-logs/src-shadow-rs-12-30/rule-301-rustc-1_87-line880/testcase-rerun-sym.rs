#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-shadow-rs-12-30-rule-301-rustc-1-87-line880-1abb13fe04")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_shadow_rs_12_30_rule_301_rustc_1_87_line880_1abb13fe04() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let vertex_bytes = [__unsat_rerun_sym_000; 12];
    let vertices = VertexDataAdapter::new(&vertex_bytes, __unsat_rerun_sym_001, __unsat_rerun_sym_002).unwrap();
    let indices: [u32; 1] = [__unsat_rerun_sym_003];
    let _ = generate_shadow_indices(&indices, &vertices);
}

