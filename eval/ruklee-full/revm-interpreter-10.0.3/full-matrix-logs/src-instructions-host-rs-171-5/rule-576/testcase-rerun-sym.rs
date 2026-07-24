#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-instructions-host-rs-171-5-rule-576-ace0742c7a")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_instructions_host_rs_171_5_rule_576_ace0742c7a() {
    let mut __unsat_rerun_sym_000 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 10_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 7;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    use crate::{
        gas::Gas,
        host::DummyHost,
        interpreter::{Contract, Interpreter},
        primitives::{Address, Bytecode, Bytes, U256},
        instructions::host,
    };

    let mut interp = Interpreter::new(
        Contract::new(
            Bytes::new(),
            Bytecode::LegacyRaw(Bytes::from([0x5c])),
            None,
            Address::ZERO,
            None,
            Address::ZERO,
            U256::ZERO,
        ),
        __unsat_rerun_sym_000,
        __unsat_rerun_sym_001,
    );
    interp.is_eof = __unsat_rerun_sym_002;
    interp.gas = Gas::new(__unsat_rerun_sym_003);

    interp.stack.data_mut().clear();
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_004));
    interp.stack.data_mut().push(U256::from(__unsat_rerun_sym_005));

    let mut host = DummyHost::default();
    host.transient_storage.insert(U256::from(__unsat_rerun_sym_006), U256::from(__unsat_rerun_sym_007));

    host::tload::<DummyHost, crate::primitives::PragueSpec>(&mut interp, &mut host);
}

