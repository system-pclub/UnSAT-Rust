#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-100-29-rule-302-rustc-1-87-line880-97beec55f6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_100_29_rule_302_rustc_1_87_line880_97beec55f6() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    use std::io::Cursor;

    let backing = vec![0u8; 2].into_boxed_slice();
    let mut window = BufferWindow {
        buf: backing,
        start_buf: core::ptr::null(),
        start: core::ptr::null(),
        end: core::ptr::null(),
        prior_reads: __unsat_rerun_sym_000,
    };

    let ptr = window.buf.as_ptr();
    window.start_buf = ptr;
    window.start = ptr;
    window.end = ptr.wrapping_add(__unsat_rerun_sym_001);
    window.prior_reads = __unsat_rerun_sym_002;

    let reader = Cursor::new([0x41u8]);
    let _ = window.fill_buf(reader);
}

