#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-xffi-xtr-rs-10-9-ruklee-intrinsics-copy-nonoverlapping-disjoint-2704776644")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_xffi_xtr_rs_10_9_ruklee_intrinsics_copy_nonoverlapping_disjoint_2704776644() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let s = "A";
    let mut buf = [__unsat_rerun_sym_000; 2];
    crate::xffi::xtr::string_to_buffer(s, buf.as_mut_ptr(), __unsat_rerun_sym_001);
}

