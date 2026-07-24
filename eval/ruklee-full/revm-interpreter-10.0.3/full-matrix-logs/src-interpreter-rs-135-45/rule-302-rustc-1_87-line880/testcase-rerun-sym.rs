#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-rs-135-45-rule-302-rustc-1-87-line880-be5862ff48")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_rs_135_45_rule_302_rustc_1_87_line880_be5862ff48() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    use crate::{DummyHost, Interpreter};
    use revm_primitives::{Bytecode, Eof};
    use std::sync::Arc;

    let mut eof = Eof::decode(revm_primitives::bytes!(
        "ef000101000402000100010400000000800000fe"
    ))
    .unwrap();

    eof.body.code_section.clear();
    eof.header.code_sizes.clear();

    eof.body.code_section.push(revm_primitives::Bytes::from_static(&[0x00]));
    eof.header.code_sizes.push(__unsat_rerun_sym_000);

    let mut interp = Interpreter::new(
        crate::Contract::new(
            revm_primitives::Bytes::new(),
            Bytecode::Eof(Arc::new(eof)),
            None,
            revm_primitives::Address::ZERO,
            None,
            revm_primitives::Address::ZERO,
            revm_primitives::U256::ZERO,
        ),
        __unsat_rerun_sym_001,
        __unsat_rerun_sym_002,
    );

    interp.load_eof_code(__unsat_rerun_sym_003, __unsat_rerun_sym_004);
    let _ = DummyHost::default();
}

