#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-delaunay-delaunay-rs-84-46-rule-574-2f1f045005")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_delaunay_delaunay_rs_84_46_rule_574_2f1f045005() {
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
    let mut __unsat_rerun_sym_014 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    let mut __unsat_rerun_sym_016 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_016, "__unsat_rerun_sym_016");
    let mut __unsat_rerun_sym_017 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_017, "__unsat_rerun_sym_017");
    let mut __unsat_rerun_sym_018 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_018, "__unsat_rerun_sym_018");
    let mut __unsat_rerun_sym_019 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_019, "__unsat_rerun_sym_019");
    let mut __unsat_rerun_sym_020 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_020, "__unsat_rerun_sym_020");
    let mut __unsat_rerun_sym_021 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_021, "__unsat_rerun_sym_021");
    let mut __unsat_rerun_sym_022 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_022, "__unsat_rerun_sym_022");
    let a = DVertex::new(__unsat_rerun_sym_000, IntPoint::new(__unsat_rerun_sym_001, __unsat_rerun_sym_002));
    let b = DVertex::new(__unsat_rerun_sym_003, IntPoint::new(__unsat_rerun_sym_004, __unsat_rerun_sym_005));
    let c = DVertex::new(__unsat_rerun_sym_006, IntPoint::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));

    let mut tri0 = DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_009, a, b, c, __unsat_rerun_sym_010, __unsat_rerun_sym_011, __unsat_rerun_sym_012);
    tri0.neighbors = [__unsat_rerun_sym_013, __unsat_rerun_sym_014, __unsat_rerun_sym_015];

    let mut tri1 = DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_016, a, b, c, __unsat_rerun_sym_017, __unsat_rerun_sym_018, __unsat_rerun_sym_019);
    tri1.neighbors = [__unsat_rerun_sym_020, __unsat_rerun_sym_021, __unsat_rerun_sym_022];

    let mut delaunay = crate::delaunay::delaunay::Delaunay {
        triangles: vec![tri0, tri1],
    };

    delaunay.build();
}

