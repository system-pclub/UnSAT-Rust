#[cfg(feature = "unsat-poc-src-buffer-rs-57-18-rule-294-rustc-1-87-line637-278c38ae21")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_57_18_rule_294_rustc_1_87_line637_278c38ae21() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let data = [__unsat_rerun_sym_000; 1];
    let mut window = crate::buffer::BufferWindow::from_slice(&data);

    window.start = data.as_ptr().wrapping_add(__unsat_rerun_sym_001);
    window.end = data.as_ptr();

    let _ = window.window_len();
}

