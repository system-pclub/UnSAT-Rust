#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-198-30-rule-301-rustc-1-87-line880-e055280a62")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_198_30_rule_301_rustc_1_87_line880_e055280a62() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0.0f32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let vertex_bytes = [__unsat_rerun_sym_000; 12];
    let vertices = crate::VertexDataAdapter::new(&vertex_bytes, __unsat_rerun_sym_001, __unsat_rerun_sym_002).unwrap();

    let indices: &[u32] = &[__unsat_rerun_sym_003];
    let target_count = __unsat_rerun_sym_004;
    let target_error = __unsat_rerun_sym_005;
    let result_error: Option<&mut f32> = None;

    let _ = crate::simplify::simplify_sloppy(
        indices,
        &vertices,
        target_count,
        target_error,
        result_error,
    );
}

