#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-75-27-rule-610-da93ba21b4")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_75_27_rule_610_da93ba21b4() {
    let mut __unsat_rerun_sym_000 = 11u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 33u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 44u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let backing = [__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003];
    let window = BufferWindow::from_slice(&backing);

    let start = backing.as_ptr();
    let end = start.wrapping_add(__unsat_rerun_sym_004);

    let range = start..end;
    let _ = window.get(range);
}

