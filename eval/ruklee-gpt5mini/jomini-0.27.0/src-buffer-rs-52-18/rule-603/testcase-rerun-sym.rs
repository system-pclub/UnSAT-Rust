#[cfg(feature = "unsat-poc-src-buffer-rs-52-18-rule-603-2351855dec")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_52_18_rule_603_2351855dec() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let data = [__unsat_rerun_sym_000; 1];
    let mut bw = crate::buffer::BufferWindow::from_slice(&data[..]);

    bw.start = bw.end.wrapping_add(__unsat_rerun_sym_001);

    let _ = bw.window();
}

