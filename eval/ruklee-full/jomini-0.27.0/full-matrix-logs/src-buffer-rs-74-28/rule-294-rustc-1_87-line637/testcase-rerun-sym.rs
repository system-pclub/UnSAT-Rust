#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-74-28-rule-294-rustc-1-87-line637-009f2fff0d")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_74_28_rule_294_rustc_1_87_line637_009f2fff0d() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = vec![11u8, 22u8].into_boxed_slice();
    let start = backing.as_ptr();
    let end = start.wrapping_add(__unsat_rerun_sym_000);

    let window = BufferWindow {
        buf: backing,
        start_buf: start,
        start,
        end,
        prior_reads: __unsat_rerun_sym_001,
    };

    let range = start.wrapping_add(__unsat_rerun_sym_002)..start;
    let _ = window.get(range);
}

