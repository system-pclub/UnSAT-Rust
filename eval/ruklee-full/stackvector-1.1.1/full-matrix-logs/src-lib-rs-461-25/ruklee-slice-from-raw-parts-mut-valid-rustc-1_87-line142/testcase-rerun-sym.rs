#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-461-25-ruklee-slice-from-raw-parts-mut-valid-rustc-1-87-line142-80d413f223")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_461_25_ruklee_slice_from_raw_parts_mut_valid_rustc_1_87_line142_80d413f223() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut backing = Vec::from([11u8, 22u8]);
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf([7u8]);
    v.length = __unsat_rerun_sym_000;
    let _ = backing.as_mut_ptr();
    let _drain = v.drain();
}

