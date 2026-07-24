#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-502-17-ruklee-drop-in-place-valid-rustc-1-87-line441-5a285cf4cf")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_502_17_ruklee_drop_in_place_valid_rustc_1_87_line441_5a285cf4cf() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut v: StackVec<[u8; 1]> = StackVec::from_buf_and_len([1], __unsat_rerun_sym_000);
    v.truncate(__unsat_rerun_sym_001);
}

