#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-lib-rs-70-9-rule-3-e11e77db89")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_lib_rs_70_9_rule_3_e11e77db89() {
    let mut __unsat_rerun_sym_000 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    #[cfg(target_os = "linux")]
    {
        use std::alloc::Layout;

        let layout = Layout::from_size_align(__unsat_rerun_sym_000, __unsat_rerun_sym_001).unwrap();
        let p = std::ptr::NonNull::<u8>::dangling().as_ptr();
        dealloc(p, layout);
    }
}

