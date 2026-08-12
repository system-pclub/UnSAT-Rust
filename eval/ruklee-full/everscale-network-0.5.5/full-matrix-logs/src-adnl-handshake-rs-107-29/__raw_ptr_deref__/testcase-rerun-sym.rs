#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-adnl-handshake-rs-107-29-raw-ptr-deref-102c91dc5f")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_adnl_handshake_rs_107_29_raw_ptr_deref_102c91dc5f() {
    let mut __unsat_rerun_sym_000 = 7u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = 0u8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 96;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    use std::sync::Arc;

    let key_bytes = [__unsat_rerun_sym_000; 32];
    let key = Arc::new(Key::from_bytes(key_bytes));

    let local_id = *key.id();
    let mut keys: FastHashMap<NodeIdShort, Arc<Key>> = FastHashMap::default();
    keys.insert(local_id, key);

    let mut packet = vec![__unsat_rerun_sym_001; __unsat_rerun_sym_002];
    packet[..32].copy_from_slice(local_id.as_slice());

    let mut view = PacketView::from(packet.as_mut_slice());
    let _ = parse_handshake_packet(&keys, &mut view);
}

