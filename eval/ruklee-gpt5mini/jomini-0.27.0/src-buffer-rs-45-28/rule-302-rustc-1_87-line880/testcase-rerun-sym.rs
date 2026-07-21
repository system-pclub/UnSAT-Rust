#[cfg(feature = "unsat-poc-src-buffer-rs-45-28-rule-302-rustc-1-87-line880-faf94d8517")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_45_28_rule_302_rustc_1_87_line880_faf94d8517() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 32);
    let data = [__unsat_rerun_sym_000; 1];
    let mut bw = crate::buffer::BufferWindow::from_slice(&data[..]);
    bw.advance(__unsat_rerun_sym_001);
}

