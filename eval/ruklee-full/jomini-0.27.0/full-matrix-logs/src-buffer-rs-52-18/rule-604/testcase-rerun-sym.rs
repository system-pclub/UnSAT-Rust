#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-52-18-rule-604-70e03e5bf1")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_52_18_rule_604_70e03e5bf1() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let backing = vec![0x41u8, 0x42u8].into_boxed_slice();
    let ptr = backing.as_ptr();

    let window = BufferWindow {
        buf: backing,
        start_buf: ptr,
        start: ptr,
        end: ptr.wrapping_add(__unsat_rerun_sym_000),
        prior_reads: __unsat_rerun_sym_001,
    };

    let _ = window.window();
}

