#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-delaunay-triangle-rs-13-14-rule-574-87408aa44b")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_delaunay_triangle_rs_13_14_rule_574_87408aa44b() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 10;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 12;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let a = DVertex::new(__unsat_rerun_sym_000, i_overlay::i_float::point::IntPoint::new(__unsat_rerun_sym_001, __unsat_rerun_sym_002));
    let b = DVertex::new(__unsat_rerun_sym_003, i_overlay::i_float::point::IntPoint::new(__unsat_rerun_sym_004, __unsat_rerun_sym_005));
    let c = DVertex::new(__unsat_rerun_sym_006, i_overlay::i_float::point::IntPoint::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));

    let triangle = DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_009, a, b, c, __unsat_rerun_sym_010, __unsat_rerun_sym_011, __unsat_rerun_sym_012);
    let _ = triangle.neighbor_by_order(__unsat_rerun_sym_013);
}

