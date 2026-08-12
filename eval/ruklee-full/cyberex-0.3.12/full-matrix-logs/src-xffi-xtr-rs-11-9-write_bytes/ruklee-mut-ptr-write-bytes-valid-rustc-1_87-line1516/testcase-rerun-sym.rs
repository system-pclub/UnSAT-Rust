#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-xffi-xtr-rs-11-9-write-bytes-ruklee-mut-ptr-write-bytes-valid-rustc-1-87-lin-a23bd764b6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_xffi_xtr_rs_11_9_write_bytes_ruklee_mut_ptr_write_bytes_valid_rustc_1_87_lin_a23bd764b6() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1usize;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let s = "A";
    let mut backing = [__unsat_rerun_sym_000; 2];
    let buf = backing.as_mut_ptr();
    let buf_max = __unsat_rerun_sym_001;
    crate::xffi::xtr::string_to_buffer(s, buf, buf_max);
}

