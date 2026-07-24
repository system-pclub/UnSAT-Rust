#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-utilities-rs-188-34-rule-302-rustc-1-87-line880-d1377d1788")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_utilities_rs_188_34_rule_302_rustc_1_87_line880_d1377d1788() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let backing = [__unsat_rerun_sym_000; 1];
    let adapter = crate::utilities::VertexDataAdapter::new(&backing, __unsat_rerun_sym_001, __unsat_rerun_sym_002).unwrap();
    let _ = adapter.pos_ptr();

    let backing2 = [__unsat_rerun_sym_003; 2];
    let adapter2 = crate::utilities::VertexDataAdapter::new(&backing2, __unsat_rerun_sym_004, __unsat_rerun_sym_005).unwrap();
    let _ = adapter2.pos_ptr();
}

