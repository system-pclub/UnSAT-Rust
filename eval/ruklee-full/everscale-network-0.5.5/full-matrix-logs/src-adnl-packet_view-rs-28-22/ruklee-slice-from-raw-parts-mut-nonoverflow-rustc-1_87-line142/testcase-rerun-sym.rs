#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-adnl-packet-view-rs-28-22-ruklee-slice-from-raw-parts-mut-nonoverflow-rustc-be9cf45e96")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_adnl_packet_view_rs_28_22_ruklee_slice_from_raw_parts_mut_nonoverflow_rustc_be9cf45e96() {
    let mut __unsat_rerun_sym_000 = 11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut backing = [__unsat_rerun_sym_000, __unsat_rerun_sym_001];
    let slice = &mut backing[..];
    let mut view = crate::adnl::packet_view::PacketView::from(slice);

    view.remove_prefix(__unsat_rerun_sym_002);

    let _ = view.as_slice();
}

