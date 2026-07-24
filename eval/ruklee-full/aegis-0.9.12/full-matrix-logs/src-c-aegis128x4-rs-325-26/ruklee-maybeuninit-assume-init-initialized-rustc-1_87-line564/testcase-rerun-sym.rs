#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-c-aegis128x4-rs-325-26-ruklee-maybeuninit-assume-init-initialized-rustc-1-87-1efaaf4d1d")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_c_aegis128x4_rs_325_26_ruklee_maybeuninit_assume_init_initialized_rustc_1_87_1efaaf4d1d() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let key: crate::c::aegis128x4::Key = [__unsat_rerun_sym_000; 16];
    let npub: crate::c::aegis128x4::Nonce = [__unsat_rerun_sym_001; 16];
    let _ = crate::c::aegis128x4::Aegis128X4Mac::<__unsat_rerun_sym_002>::new_with_nonce(&key, &npub);
}

