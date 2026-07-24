#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-354-18-copy-nonoverlapping-ruklee-intrinsics-copy-nonoverlapping-disj-82101408f4")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_354_18_copy_nonoverlapping_ruklee_intrinsics_copy_nonoverlapping_disj_82101408f4() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let vec: Vec<u8> = Vec::with_capacity(__unsat_rerun_sym_000);
    let _ = crate::StackVec::<[u8; 0]>::from_vec(vec);
}

