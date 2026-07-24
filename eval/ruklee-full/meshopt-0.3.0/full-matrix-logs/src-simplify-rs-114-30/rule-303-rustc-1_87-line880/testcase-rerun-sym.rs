#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-114-30-rule-303-rustc-1-87-line880-cc511d965c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_114_30_rule_303_rustc_1_87_line880_cc511d965c() {
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

    let indices: [u32; 0] = [];
    let vertex_lock: [bool; 0] = [];
    let _ = simplify_with_locks(
        &indices,
        &vertices,
        &vertex_lock,
        __unsat_rerun_sym_003,
        0.0,
        SimplifyOptions::empty(),
    );
}

