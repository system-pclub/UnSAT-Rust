#[cfg(feature = "unsat-poc-src-buffer-rs-25-17-rule-290-be921ead89")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_buffer_rs_25_17_rule_290_be921ead89() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 16);
    let mut __unsat_rerun_sym_002 = 0xAA;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    klee_ext_bind::assume!(__unsat_rerun_sym_002 <= 2720);
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    klee_ext_bind::assume!(__unsat_rerun_sym_003 <= 16);
    let mut buf = crate::buffer::NetworkBuffer::new();

    buf.offset = __unsat_rerun_sym_000;
    buf.data[__unsat_rerun_sym_001] = __unsat_rerun_sym_002;

    buf.drain(__unsat_rerun_sym_003);
}

