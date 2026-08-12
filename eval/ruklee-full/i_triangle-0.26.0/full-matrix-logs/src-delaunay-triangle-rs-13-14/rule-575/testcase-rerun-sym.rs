#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-delaunay-triangle-rs-13-14-rule-575-7a96e9bfe7")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_delaunay_triangle_rs_13_14_rule_575_7a96e9bfe7() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 10;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let p = i_overlay::i_float::point::IntPoint::new(__unsat_rerun_sym_000, __unsat_rerun_sym_001);
    let a = DVertex::new(__unsat_rerun_sym_002, p);
    let b = DVertex::new(__unsat_rerun_sym_003, p);
    let c = DVertex::new(__unsat_rerun_sym_004, p);

    let triangle = DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_005, a, b, c, __unsat_rerun_sym_006, __unsat_rerun_sym_007, __unsat_rerun_sym_008);
    let _ = triangle.neighbor_by_order(__unsat_rerun_sym_009);
}

