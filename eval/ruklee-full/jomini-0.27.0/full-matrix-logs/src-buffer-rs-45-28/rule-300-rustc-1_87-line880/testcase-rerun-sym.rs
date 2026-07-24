#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-45-28-rule-300-rustc-1-87-line880-9ad4bc0c35")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_45_28_rule_300_rustc_1_87_line880_9ad4bc0c35() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = vec![11u8, 22u8].into_boxed_slice();
    let base = backing.as_ptr();

    let mut window = BufferWindow {
        buf: backing,
        start_buf: base,
        start: base,
        end: base.wrapping_add(__unsat_rerun_sym_000),
        prior_reads: __unsat_rerun_sym_001,
    };

    window.advance(__unsat_rerun_sym_002);

    let _ = window.start;
}

