#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-106-37-rule-301-rustc-1-87-line880-cc6b706faa")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_106_37_rule_301_rustc_1_87_line880_cc6b706faa() {
    let mut __unsat_rerun_sym_000 = 7;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    use std::io::Cursor;

    let backing = vec![11u8, 22, 33, 44].into_boxed_slice();
    let mut window = BufferWindow {
        buf: backing,
        start_buf: std::ptr::null(),
        start: std::ptr::null(),
        end: std::ptr::null(),
        prior_reads: __unsat_rerun_sym_000,
    };

    let base = window.buf.as_ptr();
    window.start_buf = base;
    window.start = base;
    window.end = base.wrapping_add(__unsat_rerun_sym_001);

    let reader = Cursor::new(&b"Z"[..]);
    let _ = window.fill_buf(reader);
}

