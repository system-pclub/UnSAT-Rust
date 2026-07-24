#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-delaunay-delaunay-rs-84-46-rule-575-0f3b694b9a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_delaunay_delaunay_rs_84_46_rule_575_0f3b694b9a() {
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
    let mut __unsat_rerun_sym_010 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    let mut __unsat_rerun_sym_016 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_016, "__unsat_rerun_sym_016");
    use crate::delaunay::delaunay::Delaunay;
    use crate::delaunay::triangle::DTriangle;
    use crate::delaunay::vertex::DVertex;
    use i_overlay::i_float::point::IntPoint;

    let v0 = DVertex::new(__unsat_rerun_sym_000, IntPoint::new(__unsat_rerun_sym_001, __unsat_rerun_sym_002));
    let v1 = DVertex::new(__unsat_rerun_sym_003, IntPoint::new(__unsat_rerun_sym_004, __unsat_rerun_sym_005));
    let v2 = DVertex::new(__unsat_rerun_sym_006, IntPoint::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));

    let t0 = DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_009, v0, v1, v2, __unsat_rerun_sym_010, __unsat_rerun_sym_011, __unsat_rerun_sym_012);
    let t1 = DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_013, v0, v1, v2, __unsat_rerun_sym_014, __unsat_rerun_sym_015, __unsat_rerun_sym_016);

    let mut delaunay = Delaunay {
        triangles: vec![t0, t1],
    };

    delaunay.build();
}

