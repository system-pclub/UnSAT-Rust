#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-interpreter-stack-rs-283-44-rule-349-3d9fbc4a78")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_interpreter_stack_rs_283_44_rule_349_3d9fbc4a78() {
    let mut __unsat_rerun_sym_000 = 11u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 22u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 33u64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 1_000_000;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 2;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    let mut __unsat_rerun_sym_009 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_009, "__unsat_rerun_sym_009");
    let mut __unsat_rerun_sym_010 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_010, "__unsat_rerun_sym_010");
    use crate::{DummyHost, Gas, Interpreter, InstructionResult, Stack};
    use revm_primitives::{Bytecode, Bytes, U256};

    let mut stack = Stack::new();
    stack.data_mut().push(U256::from(__unsat_rerun_sym_000));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_001));
    stack.data_mut().push(U256::from(__unsat_rerun_sym_002));

    let bytecode = Bytecode::LegacyRaw(Bytes::from([0x00]));
    let mut interp = Interpreter::new(crate::Contract::new(
        Bytes::new(),
        bytecode,
        None,
        revm_primitives::Address::ZERO,
        None,
        revm_primitives::Address::ZERO,
        U256::ZERO,
    ), 1_000_000, false);

    interp.stack = stack;
    interp.is_eof = __unsat_rerun_sym_003;
    interp.gas = Gas::new(__unsat_rerun_sym_004);
    interp.instruction_result = InstructionResult::Continue;

    let mut host = DummyHost::default();

    let _ = interp.stack.exchange(__unsat_rerun_sym_005, __unsat_rerun_sym_006);
    let _ = interp.stack.exchange(__unsat_rerun_sym_007, __unsat_rerun_sym_008);
    let _ = interp.stack.exchange(__unsat_rerun_sym_009, __unsat_rerun_sym_010);

    let _ = &mut host;
}

