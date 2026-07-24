#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-45-28-rule-302-rustc-1-87-line880-faf94d8517")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_45_28_rule_302_rustc_1_87_line880_faf94d8517() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = vec![11u8, 22u8, 33u8].into_boxed_slice();
    let start = backing.as_ptr();
    let end = start.wrapping_add(__unsat_rerun_sym_000);

    let mut window = BufferWindow {
        buf: backing,
        start_buf: start,
        start,
        end,
        prior_reads: __unsat_rerun_sym_001,
    };

    window.advance(__unsat_rerun_sym_002);

    let _ = window.start;
}

