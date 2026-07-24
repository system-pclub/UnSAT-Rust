#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-25-17-rule-289-33338729f6")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_25_17_rule_289_33338729f6() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0xBB;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut buf = crate::buffer::NetworkBuffer::new();

    buf.offset = __unsat_rerun_sym_000;
    buf.data[__unsat_rerun_sym_001] = __unsat_rerun_sym_002;
    buf.data[__unsat_rerun_sym_003] = __unsat_rerun_sym_004;

    buf.drain(__unsat_rerun_sym_005);
}

