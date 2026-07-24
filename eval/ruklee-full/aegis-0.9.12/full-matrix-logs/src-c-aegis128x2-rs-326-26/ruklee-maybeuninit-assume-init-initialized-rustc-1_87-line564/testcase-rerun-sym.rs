#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-c-aegis128x2-rs-326-26-ruklee-maybeuninit-assume-init-initialized-rustc-1-87-547ab51c2d")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_c_aegis128x2_rs_326_26_ruklee_maybeuninit_assume_init_initialized_rustc_1_87_547ab51c2d() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let key: crate::c::aegis128x2::Key = [__unsat_rerun_sym_000; 16];
    let nonce: crate::c::aegis128x2::Nonce = [__unsat_rerun_sym_001; 16];

    let _ = crate::c::aegis128x2::Aegis128X2Mac::<__unsat_rerun_sym_002>::new_with_nonce(&key, &nonce);
}

