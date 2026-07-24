#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-delaunay-delaunay-rs-30-18-rule-448-e47f774d33")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_delaunay_delaunay_rs_30_18_rule_448_e47f774d33() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 5;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    let mut __unsat_rerun_sym_016 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_016, "__unsat_rerun_sym_016");
    let mut __unsat_rerun_sym_017 = 6;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_017, "__unsat_rerun_sym_017");
    let mut __unsat_rerun_sym_018 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_018, "__unsat_rerun_sym_018");
    let mut __unsat_rerun_sym_019 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_019, "__unsat_rerun_sym_019");
    let mut __unsat_rerun_sym_020 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_020, "__unsat_rerun_sym_020");
    use crate::delaunay::delaunay::Delaunay;
    use crate::delaunay::triangle::DTriangle;
    use crate::delaunay::vertex::DVertex;
    use i_overlay::i_float::point::IntPoint;

    let tri0 = DTriangle::abc(
        __unsat_rerun_sym_000,
        DVertex::new(__unsat_rerun_sym_001, IntPoint::new(__unsat_rerun_sym_002, __unsat_rerun_sym_003)),
        DVertex::new(__unsat_rerun_sym_004, IntPoint::new(__unsat_rerun_sym_005, __unsat_rerun_sym_006)),
        DVertex::new(__unsat_rerun_sym_007, IntPoint::new(__unsat_rerun_sym_008, __unsat_rerun_sym_009)),
    );

    let tri1 = DTriangle::abc(
        __unsat_rerun_sym_010,
        DVertex::new(__unsat_rerun_sym_011, IntPoint::new(__unsat_rerun_sym_012, __unsat_rerun_sym_013)),
        DVertex::new(__unsat_rerun_sym_014, IntPoint::new(__unsat_rerun_sym_015, __unsat_rerun_sym_016)),
        DVertex::new(__unsat_rerun_sym_017, IntPoint::new(__unsat_rerun_sym_018, __unsat_rerun_sym_019)),
    );

    let delaunay = Delaunay {
        triangles: vec![tri0, tri1],
    };

    let _ = delaunay.to_triangulation(__unsat_rerun_sym_020);
}

