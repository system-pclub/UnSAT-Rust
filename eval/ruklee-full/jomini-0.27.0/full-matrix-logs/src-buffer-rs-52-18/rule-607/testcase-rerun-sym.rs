#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-52-18-rule-607-bfa834d104")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_52_18_rule_607_bfa834d104() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 7;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = vec![11u8, 22u8].into_boxed_slice();
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
    window.prior_reads = __unsat_rerun_sym_002;

    let _ = window.window();
}

