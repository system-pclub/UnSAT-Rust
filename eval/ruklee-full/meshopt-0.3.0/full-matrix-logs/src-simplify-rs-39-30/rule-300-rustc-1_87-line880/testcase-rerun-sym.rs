#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-simplify-rs-39-30-rule-300-rustc-1-87-line880-7b307b747a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_simplify_rs_39_30_rule_300_rustc_1_87_line880_7b307b747a() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0u32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0.0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let vertex_bytes = [__unsat_rerun_sym_000; 12];
    let vertices = VertexDataAdapter::new(&vertex_bytes, __unsat_rerun_sym_001, __unsat_rerun_sym_002).unwrap();

    let indices = [__unsat_rerun_sym_003; 1];
    let _ = simplify(&indices, &vertices, __unsat_rerun_sym_004, __unsat_rerun_sym_005, SimplifyOptions::empty(), None);
}

