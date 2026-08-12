#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-39-30-rule-300-rustc-1-87-line880-7b307b747a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_39_30_rule_300_rustc_1_87_line880_7b307b747a() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let indices: [u32; 1] = [__unsat_rerun_sym_000];
    let vertex_bytes: [u8; 4] = [__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003, __unsat_rerun_sym_004];
    let vertices = VertexDataAdapter::new(&vertex_bytes, __unsat_rerun_sym_005, __unsat_rerun_sym_006).unwrap();
    let _ = simplify(&indices, &vertices, __unsat_rerun_sym_007, __unsat_rerun_sym_008, SimplifyOptions::empty(), None);
}

