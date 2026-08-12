#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-unix-rs-140-17-rule-431-15602de587")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_unix_rs_140_17_rule_431_15602de587() {
    let mut __unsat_rerun_sym_000 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = true;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let mut __unsat_rerun_sym_003 = 4096;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_003, "__unsat_rerun_sym_003");
    let mut __unsat_rerun_sym_004 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_004, "__unsat_rerun_sym_004");
    let mut __unsat_rerun_sym_005 = 4096;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_005, "__unsat_rerun_sym_005");
    let mut __unsat_rerun_sym_006 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_006, "__unsat_rerun_sym_006");
    let mut __unsat_rerun_sym_007 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_007, "__unsat_rerun_sym_007");
    let mut __unsat_rerun_sym_008 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_008, "__unsat_rerun_sym_008");
    use std::fs::OpenOptions;
    use std::io::Write;

    let path = std::env::temp_dir().join("memmap_unsat_poc_src_unix_rs_140_17");
    let mut file = OpenOptions::new()
        .read(__unsat_rerun_sym_000)
        .write(__unsat_rerun_sym_001)
        .create(__unsat_rerun_sym_002)
        .open(&path)
        .expect("open temp file");
    file.set_len(__unsat_rerun_sym_003).expect("set_len");
    file.write_all(&[__unsat_rerun_sym_004; 4096]).expect("write");

    let mut inner = MmapInner::map_mut(__unsat_rerun_sym_005, &file, __unsat_rerun_sym_006).expect("map_mut");
    let _ = inner.mut_ptr();
    let _ = inner.flush_async(__unsat_rerun_sym_007, __unsat_rerun_sym_008);
}

