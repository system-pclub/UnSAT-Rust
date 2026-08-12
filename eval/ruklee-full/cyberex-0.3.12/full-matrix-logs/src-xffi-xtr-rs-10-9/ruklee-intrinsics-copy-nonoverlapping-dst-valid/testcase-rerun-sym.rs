#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-xffi-xtr-rs-10-9-ruklee-intrinsics-copy-nonoverlapping-dst-valid-52b084bd3a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_xffi_xtr_rs_10_9_ruklee_intrinsics_copy_nonoverlapping_dst_valid_52b084bd3a() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let s = "A";
    let mut dst = [__unsat_rerun_sym_000; 1];
    crate::xffi::xtr::string_to_buffer(s, dst.as_mut_ptr(), __unsat_rerun_sym_001);
}

