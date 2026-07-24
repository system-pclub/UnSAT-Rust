#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-monotone-mslice-buffer-rs-91-44-rule-576-b26e888ede")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_monotone_mslice_buffer_rs_91_44_rule_576_b26e888ede() {
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
    let mut __unsat_rerun_sym_010 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    let mut __unsat_rerun_sym_011 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_011, "__unsat_rerun_sym_011");
    let mut __unsat_rerun_sym_012 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_012, "__unsat_rerun_sym_012");
    let mut __unsat_rerun_sym_013 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_013, "__unsat_rerun_sym_013");
    let mut __unsat_rerun_sym_014 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_014, "__unsat_rerun_sym_014");
    let mut __unsat_rerun_sym_015 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_015, "__unsat_rerun_sym_015");
    use crate::delaunay::triangle::DTriangle;
    use crate::delaunay::vertex::DVertex;
    use crate::monotone::mslice_buffer::{MSlice, MSliceBuffer};
    use i_overlay::i_float::point::IntPoint;

    let a = DVertex::new(__unsat_rerun_sym_000, IntPoint::new(__unsat_rerun_sym_001, __unsat_rerun_sym_002));
    let b = DVertex::new(__unsat_rerun_sym_003, IntPoint::new(__unsat_rerun_sym_004, __unsat_rerun_sym_005));
    let c = DVertex::new(__unsat_rerun_sym_006, IntPoint::new(__unsat_rerun_sym_007, __unsat_rerun_sym_008));

    let mut triangles = vec![DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_009, a, b, c, __unsat_rerun_sym_010, __unsat_rerun_sym_011, __unsat_rerun_sym_012)];

    let slices = [MSlice::new(__unsat_rerun_sym_013, __unsat_rerun_sym_014)];
    let mut buffer = MSliceBuffer::new(__unsat_rerun_sym_015, &slices);

    buffer.add_connections(&mut triangles);
}

