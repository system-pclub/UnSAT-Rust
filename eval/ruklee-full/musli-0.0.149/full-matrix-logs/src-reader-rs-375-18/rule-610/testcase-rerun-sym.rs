#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-reader-rs-375-18-rule-610-eb64587726")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_reader_rs_375_18_rule_610_eb64587726() {
    let mut __unsat_rerun_sym_000 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let backing = [__unsat_rerun_sym_000; 1];
    let reader = crate::reader::SliceReader::new(&backing[..0]);
    let _ = reader.as_slice();
}

