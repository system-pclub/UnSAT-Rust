#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-52-18-rule-603-2351855dec")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_52_18_rule_603_2351855dec() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
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

    let _ = window.window();
}

