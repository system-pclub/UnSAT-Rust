#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-25-17-rule-289-33338729f6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_25_17_rule_289_33338729f6() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut buf = crate::buffer::NetworkBuffer {
        data: [__unsat_rerun_sym_000; crate::buffer::MAX_BUFFER_SIZE],
        offset: __unsat_rerun_sym_001,
    };
    buf.drain(__unsat_rerun_sym_002);
}

