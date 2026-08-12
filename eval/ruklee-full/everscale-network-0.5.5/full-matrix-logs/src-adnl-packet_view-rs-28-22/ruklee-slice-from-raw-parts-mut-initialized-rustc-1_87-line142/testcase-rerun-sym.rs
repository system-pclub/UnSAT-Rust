#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-adnl-packet-view-rs-28-22-ruklee-slice-from-raw-parts-mut-initialized-rustc-6ed2c1ea9e")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_adnl_packet_view_rs_28_22_ruklee_slice_from_raw_parts_mut_initialized_rustc_6ed2c1ea9e() {
    let mut __unsat_rerun_sym_000 = 11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 33u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut backing = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002];
    let mut view = crate::adnl::packet_view::PacketView::from(backing.as_mut_slice());

    view.remove_prefix(__unsat_rerun_sym_003);

    let _ = view.as_slice();
}

