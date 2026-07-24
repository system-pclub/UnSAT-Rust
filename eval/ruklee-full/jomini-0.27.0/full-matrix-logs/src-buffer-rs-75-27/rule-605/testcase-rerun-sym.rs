#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-75-27-rule-605-4ea5aa20a7")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_75_27_rule_605_4ea5aa20a7() {
    let mut __unsat_rerun_sym_000 = 3;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let backing = vec![11u8, 22, 33, 44].into_boxed_slice();
    let start = backing.as_ptr();
    let end = start.wrapping_add(__unsat_rerun_sym_000);

    let window = BufferWindow {
        buf: backing,
        start_buf: start,
        start,
        end,
        prior_reads: __unsat_rerun_sym_001,
    };

    let range = start.wrapping_add(__unsat_rerun_sym_002)..end;
    let _ = window.get(range);
}

