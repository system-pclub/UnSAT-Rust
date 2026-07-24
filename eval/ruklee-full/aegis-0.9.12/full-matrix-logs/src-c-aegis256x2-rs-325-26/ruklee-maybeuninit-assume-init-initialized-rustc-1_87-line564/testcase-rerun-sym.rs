#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-c-aegis256x2-rs-325-26-ruklee-maybeuninit-assume-init-initialized-rustc-1-87-fe54c1fed8")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_c_aegis256x2_rs_325_26_ruklee_maybeuninit_assume_init_initialized_rustc_1_87_fe54c1fed8() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let key: crate::c::aegis256x2::Key = [__unsat_rerun_sym_000; 32];
    let nonce: crate::c::aegis256x2::Nonce = [__unsat_rerun_sym_001; 32];
    let _mac = crate::c::aegis256x2::Aegis256X2Mac::<__unsat_rerun_sym_002>::new_with_nonce(&key, &nonce);
}

