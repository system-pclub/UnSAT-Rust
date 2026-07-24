#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-buffer-rs-25-17-rule-291-035040be81")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_25_17_rule_291_035040be81() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0x11;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0x22;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut buf = crate::buffer::NetworkBuffer::new();

    buf.offset = __unsat_rerun_sym_000;
    buf.data[__unsat_rerun_sym_001] = __unsat_rerun_sym_002;

    buf.drain(__unsat_rerun_sym_003);

    let mut buf2 = crate::buffer::NetworkBuffer::new();
    buf2.offset = __unsat_rerun_sym_004;
    buf2.data[__unsat_rerun_sym_005] = __unsat_rerun_sym_006;
    buf2.data[__unsat_rerun_sym_007] = __unsat_rerun_sym_008;

    buf2.drain(__unsat_rerun_sym_009);
}

