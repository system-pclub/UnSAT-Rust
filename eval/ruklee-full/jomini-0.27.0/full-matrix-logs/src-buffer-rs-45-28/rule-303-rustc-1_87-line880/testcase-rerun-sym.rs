#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-45-28-rule-303-rustc-1-87-line880-ab785a354f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_45_28_rule_303_rustc_1_87_line880_ab785a354f() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = [__unsat_rerun_sym_000; 1];
    let mut window = BufferWindow {
        buf: vec![0u8; 1].into_boxed_slice(),
        start_buf: backing.as_ptr(),
        start: backing.as_ptr(),
        end: backing.as_ptr(),
        prior_reads: __unsat_rerun_sym_001,
    };

    window.advance(__unsat_rerun_sym_002);

    let _ = window.start;
}

