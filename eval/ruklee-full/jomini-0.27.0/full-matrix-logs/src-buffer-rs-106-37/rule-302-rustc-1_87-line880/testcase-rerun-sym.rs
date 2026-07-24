#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-106-37-rule-302-rustc-1-87-line880-ef3ea00b22")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_106_37_rule_302_rustc_1_87_line880_ef3ea00b22() {
    let mut __unsat_rerun_sym_000 = b'a';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = b'b';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = b'c';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = b'd';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = b'Z';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    use std::io::Cursor;

    let backing = vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003];
    let mut window = BufferWindow {
        buf: vec![0u8; 8].into_boxed_slice(),
        start_buf: backing.as_ptr(),
        start: backing.as_ptr(),
        end: backing.as_ptr().wrapping_add(__unsat_rerun_sym_004),
        prior_reads: __unsat_rerun_sym_005,
    };

    let reader = Cursor::new(vec![__unsat_rerun_sym_006]);
    let _ = window.fill_buf(reader);
}

