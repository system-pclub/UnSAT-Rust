#[cfg(feature = "unsat-poc-src-buffer-rs-75-27-rule-603-f0429206f7")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_75_27_rule_603_f0429206f7() {
    let mut __unsat_rerun_sym_000 = b'a';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = b'b';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = b'c';
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let data = [__unsat_rerun_sym_000, __unsat_rerun_sym_001, __unsat_rerun_sym_002];
    let mut window = crate::buffer::BufferWindow::from_slice(&data);
    let start = window.start;
    let end = start.wrapping_add(usize::MAX);
    window.end = end;
    let _ = window.get(start..end);
}

