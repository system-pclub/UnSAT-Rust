#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-346-17-rule-446-c5374a7906")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_346_17_rule_446_c5374a7906() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut stack = Stack::new();

    let slice = [__unsat_rerun_sym_000; 33];

    let _ = stack.push_slice(&slice);
}

