#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-52-18-rule-608-de786c6cc8")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_52_18_rule_608_de786c6cc8() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let backing = vec![11u8, 22u8].into_boxed_slice();
    let mut window = BufferWindow::from_slice(&backing);
    window.buf = vec![0u8; 1].into_boxed_slice();

    let base = window.buf.as_ptr();
    window.start = base;
    window.end = base.wrapping_add(__unsat_rerun_sym_000);
    window.prior_reads = __unsat_rerun_sym_001;

    let _ = window.window();
}

