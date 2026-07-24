#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-delaunay-delaunay-rs-29-18-rule-448-2911061f8c")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_delaunay_delaunay_rs_29_18_rule_448_2911061f8c() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    use crate::delaunay::delaunay::Delaunay;
    use crate::delaunay::triangle::DTriangle;
    use crate::delaunay::vertex::DVertex;
    use i_overlay::i_float::point::IntPoint;

    let tri = DTriangle::abc(
        __unsat_rerun_sym_000,
        DVertex::new(__unsat_rerun_sym_001, IntPoint::new(__unsat_rerun_sym_002, __unsat_rerun_sym_003)),
        DVertex::new(__unsat_rerun_sym_004, IntPoint::new(__unsat_rerun_sym_005, __unsat_rerun_sym_006)),
        DVertex::new(__unsat_rerun_sym_007, IntPoint::new(__unsat_rerun_sym_008, __unsat_rerun_sym_009)),
    );

    let delaunay = Delaunay {
        triangles: vec![tri],
    };

    let _ = delaunay.to_triangulation(__unsat_rerun_sym_010);
}

