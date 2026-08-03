#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-reader-rs-375-18-rule-605-c2605fa373")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_reader_rs_375_18_rule_605_c2605fa373() {
    let mut __unsat_rerun_sym_000 = 0x41u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let data = [__unsat_rerun_sym_000];
    let reader = crate::reader::SliceReader::new(&data);
    let _ = reader.as_slice();
}

