#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-utilities-rs-188-34-rule-303-rustc-1-87-line880-8495d336be")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_utilities_rs_188_34_rule_303_rustc_1_87_line880_8495d336be() {
    let mut __unsat_rerun_sym_000 = 11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 44;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let backing: [u8; 4] = [__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003];
    let adapter = crate::utilities::VertexDataAdapter::new(&backing, __unsat_rerun_sym_004, __unsat_rerun_sym_005).unwrap();
    let p = adapter.pos_ptr();
    let _ = p;
}

