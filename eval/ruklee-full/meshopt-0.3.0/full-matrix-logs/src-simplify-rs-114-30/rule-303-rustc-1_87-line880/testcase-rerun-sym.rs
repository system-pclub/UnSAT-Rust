#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-114-30-rule-303-rustc-1-87-line880-cc511d965c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_114_30_rule_303_rustc_1_87_line880_cc511d965c() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 0.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let vertices_bytes = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    let vertices = VertexDataAdapter::new(&vertices_bytes, __unsat_rerun_sym_002, __unsat_rerun_sym_003).unwrap();

    let indices: [u32; 3] = [__unsat_rerun_sym_004, __unsat_rerun_sym_005, __unsat_rerun_sym_006];
    let vertex_lock: [bool; 3] = [__unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009];
    let options = SimplifyOptions::empty();

    let _ = simplify_with_locks(&indices, &vertices, &vertex_lock, __unsat_rerun_sym_010, __unsat_rerun_sym_011, options);
}

