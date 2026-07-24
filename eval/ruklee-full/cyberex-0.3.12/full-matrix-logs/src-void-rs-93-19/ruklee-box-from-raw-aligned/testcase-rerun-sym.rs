#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-void-rs-93-19-ruklee-box-from-raw-aligned-83ce0e604b")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_void_rs_93_19_ruklee_box_from_raw_aligned_83ce0e604b() {
    let mut __unsat_rerun_sym_000 = 11_u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22_u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut a = Box::new(__unsat_rerun_sym_000);
    let mut b = Box::new(__unsat_rerun_sym_001);

    let ctx = crate::void::mut_to_opacue(&mut *a);
    let other = crate::void::mut_to_opacue(&mut *b);

    let _keep_prefix_valid = crate::void::opacue_to_mut(other as *mut _ as *mut u64);
    crate::void::delete::<u8>(ctx);
}

