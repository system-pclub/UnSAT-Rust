#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-57-18-rule-294-rustc-1-87-line637-278c38ae21")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_57_18_rule_294_rustc_1_87_line637_278c38ae21() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = [__unsat_rerun_sym_000; 2];
    let mut window = BufferWindow::from_slice(&backing);
    window.start = backing.as_ptr();
    window.end = backing.as_ptr().wrapping_add(__unsat_rerun_sym_001);
    window.prior_reads = __unsat_rerun_sym_002;

    let _ = window.window_len();
}

