#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-utilities-rs-188-34-rule-300-rustc-1-87-line880-6692cd8ea2")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_utilities_rs_188_34_rule_300_rustc_1_87_line880_6692cd8ea2() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 4;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let backing: [u8; 8] = [__unsat_rerun_sym_000; 8];
    let adapter = crate::utilities::VertexDataAdapter {
        reader: std::io::Cursor::new(&backing[..]),
        vertex_count: __unsat_rerun_sym_001,
        vertex_stride: __unsat_rerun_sym_002,
        position_offset: __unsat_rerun_sym_003,
    };

    let _ = adapter.pos_ptr();
}

