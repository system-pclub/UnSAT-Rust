#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-502-17-ruklee-drop-in-place-aligned-rustc-1-87-line441-8ba256e137")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_502_17_ruklee_drop_in_place_aligned_rustc_1_87_line441_8ba256e137() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf([7]);
    v.truncate(__unsat_rerun_sym_000);
}

