#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-75-27-rule-608-ccd75971ae")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_75_27_rule_608_ccd75971ae() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let backing = vec![11u8, 22u8].into_boxed_slice();
    let window = BufferWindow {
        buf: backing,
        start_buf: std::ptr::null(),
        start: std::ptr::null(),
        end: std::ptr::null(),
        prior_reads: __unsat_rerun_sym_000,
    };

    let range = std::ops::Range {
        start: window.buf.as_ptr(),
        end: window.buf.as_ptr().wrapping_add(__unsat_rerun_sym_001),
    };

    let _ = window.get(range);
}

