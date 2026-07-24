#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-100-29-rule-303-rustc-1-87-line880-8c7742c570")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_100_29_rule_303_rustc_1_87_line880_8c7742c570() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    use std::io::Cursor;

    let backing = vec![0u8; 2].into_boxed_slice();
    let mut window = BufferWindow {
        buf: backing,
        start_buf: std::ptr::null(),
        start: std::ptr::null(),
        end: std::ptr::null(),
        prior_reads: __unsat_rerun_sym_000,
    };

    let ptr = window.buf.as_ptr();
    window.start_buf = ptr;
    window.start = ptr;
    window.end = ptr.wrapping_add(__unsat_rerun_sym_001);

    let _ = window.fill_buf(Cursor::new(&b""[..]));
}

