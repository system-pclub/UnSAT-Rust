#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-unix-rs-126-34-rule-433-e944602416")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_unix_rs_126_34_rule_433_e944602416() {
    let mut __unsat_rerun_sym_000 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 4096;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 4096;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    use std::fs::OpenOptions;
    use std::io::Write;

    let path = std::env::temp_dir().join("memmap_unsat_poc_src_unix_rs_126_34_rule_433");
    let mut file = OpenOptions::new()
        .read(__unsat_rerun_sym_000)
        .write(__unsat_rerun_sym_001)
        .create(__unsat_rerun_sym_002)
        .open(&path)
        .unwrap();
    file.set_len(__unsat_rerun_sym_003).unwrap();
    file.write_all(&[0u8; 1]).unwrap();

    let inner = MmapInner::map_mut(__unsat_rerun_sym_004, &file, __unsat_rerun_sym_005).unwrap();
    let _ = inner.flush(__unsat_rerun_sym_006, __unsat_rerun_sym_007);
}

