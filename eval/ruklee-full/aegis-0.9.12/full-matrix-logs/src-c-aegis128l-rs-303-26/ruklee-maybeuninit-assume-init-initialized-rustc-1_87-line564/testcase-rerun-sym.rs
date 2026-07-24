#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-c-aegis128l-rs-303-26-ruklee-maybeuninit-assume-init-initialized-rustc-1-87-f4d10f3514")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_c_aegis128l_rs_303_26_ruklee_maybeuninit_assume_init_initialized_rustc_1_87_f4d10f3514() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 16;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let key: crate::c::aegis128l::Key = [__unsat_rerun_sym_000; 16];
    let _ = crate::c::aegis128l::Aegis128LMac::<__unsat_rerun_sym_001>::new(&key);
}

