#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-system-rs-164-5-rule-576-fc9dd83878")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_system_rs_164_5_rule_576_fc9dd83878() {
    let mut __unsat_rerun_sym_000 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    use crate::{
        gas::Gas,
        host::DummyHost,
        instruction_result::InstructionResult,
        interpreter::{Contract, Interpreter},
        primitives::{Bytecode, Bytes, U256},
    };
    use std::sync::Arc;

    let mut host = DummyHost::default();

    let mut interp = Interpreter::new(
        Contract::new(
            Bytes::from([0x00]),
            Bytecode::LegacyRaw(Bytes::from([0x5b])),
            None,
            Default::default(),
            None,
            Default::default(),
            U256::from(__unsat_rerun_sym_000),
        ),
        __unsat_rerun_sym_001,
        __unsat_rerun_sym_002,
    );

    interp.is_eof = __unsat_rerun_sym_003;
    interp.gas = Gas::new(__unsat_rerun_sym_004);
    interp.instruction_result = InstructionResult::Continue;

    interp.stack.data_mut().clear();
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_005));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_006));

    interp.return_data_buffer = Bytes::from([0x11, 0x22, 0x33, 0x44]);

    crate::instructions::system::returndataload(&mut interp, &mut host);

    let _ = Arc::new(());
}

