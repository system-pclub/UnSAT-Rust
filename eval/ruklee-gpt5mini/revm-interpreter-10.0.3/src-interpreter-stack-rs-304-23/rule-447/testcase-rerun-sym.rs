#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-304-23-rule-447-a3b0ed2f40")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_304_23_rule_447_a3b0ed2f40() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    klee_ext_bind::assume!(__unsat_rerun_sym_000 <= 16);
    let mut __unsat_rerun_sym_001 = 33;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    klee_ext_bind::assume!(__unsat_rerun_sym_001 <= 528);
    let mut stack = crate::Stack::new();

    let bytes = [__unsat_rerun_sym_000; __unsat_rerun_sym_001];
    let slice = &bytes[..];

    let _ = stack.push_slice(slice);
}

