#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-114-30-rule-300-rustc-1-87-line880-1e8397e21e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_114_30_rule_300_rustc_1_87_line880_1e8397e21e() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let vertices_bytes = [__unsat_rerun_sym_000; 12];
    let vertices = VertexDataAdapter::new(&vertices_bytes, __unsat_rerun_sym_001, __unsat_rerun_sym_002).unwrap();

    let indices = [__unsat_rerun_sym_003; 1];
    let vertex_lock = [__unsat_rerun_sym_004; 1];
    let options = SimplifyOptions::empty();

    let _ = simplify_with_locks(&indices, &vertices, &vertex_lock, __unsat_rerun_sym_005, __unsat_rerun_sym_006, options);
}

