#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-461-25-ruklee-slice-from-raw-parts-mut-initialized-rustc-1-87-line142-b7c23f6f62")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_461_25_ruklee_slice_from_raw_parts_mut_initialized_rustc_1_87_line142_b7c23f6f62() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut prefix = StackVec::<[u8; 2]>::from_buf([11, 22]);
    let _ = prefix.drain();

    let mut target = StackVec::<[u8; 1]>::from_buf([7]);
    target.length = __unsat_rerun_sym_000;
    let _ = target.drain();
}

