#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-adnl-packet-view-rs-28-53-rule-449-0445a03ddc")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_adnl_packet_view_rs_28_53_rule_449_0445a03ddc() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut backing = [__unsat_rerun_sym_000; 3];
    backing[__unsat_rerun_sym_001] = __unsat_rerun_sym_002;
    backing[__unsat_rerun_sym_003] = __unsat_rerun_sym_004;
    backing[__unsat_rerun_sym_005] = __unsat_rerun_sym_006;

    let mut view = crate::adnl::packet_view::PacketView::from(&mut backing[..]);
    view.remove_prefix(__unsat_rerun_sym_007);

    let _ = view.as_slice();
}

