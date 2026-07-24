#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-100-29-rule-300-rustc-1-87-line880-4e9b60a2c5")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_100_29_rule_300_rustc_1_87_line880_4e9b60a2c5() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let buf = vec![0u8, 1u8].into_boxed_slice();
    let mut window = BufferWindow {
        buf,
        start_buf: std::ptr::null(),
        start: std::ptr::null(),
        end: std::ptr::null(),
        prior_reads: __unsat_rerun_sym_000,
    };

    let base = window.buf.as_ptr();
    window.start_buf = base;
    window.start = base;
    window.end = base.wrapping_add(__unsat_rerun_sym_001);

    let _ = window.fill_buf(std::io::Cursor::new(&b""[..]));
}

