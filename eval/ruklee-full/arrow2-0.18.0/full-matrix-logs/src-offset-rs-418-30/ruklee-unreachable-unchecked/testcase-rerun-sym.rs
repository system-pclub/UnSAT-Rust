#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-offset-rs-418-30-ruklee-unreachable-unchecked-cc7b6e447b")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_offset_rs_418_30_ruklee_unreachable_unchecked_cc7b6e447b() {
    let mut __unsat_rerun_sym_000 = 0i32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1i32;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let offsets: crate::offset::OffsetsBuffer<i32> =
        crate::offset::OffsetsBuffer::try_from(vec![__unsat_rerun_sym_000, __unsat_rerun_sym_001]).unwrap();
    let _ = offsets.last();
}

