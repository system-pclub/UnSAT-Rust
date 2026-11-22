use fast_able::statis::Statis;

use fast_able::SyncVec;
use std::sync::RwLock;

pub static RECKON_BY_SEC: once_cell::sync::Lazy<Statis> =
    once_cell::sync::Lazy::new(|| Statis::new(|v| println!("one sec run sum: {v}")));

//test bench_rw_vec        ... bench:           4 ns/iter (+/- 0)
#[test]
fn bench_rw_vec() {
    let rw = RwLock::new(vec![1]);
    rw.write().unwrap().push(1);
    for i in 0..10000_0000{
        let _a = rw.read().unwrap().get(1);
        RECKON_BY_SEC.add();
    };
}

//test bench_sync_vec      ... bench:           0 ns/iter (+/- 0)
#[test]
fn bench_sync_vec() {
    let rw = SyncVec::new();
    rw.push(1);
    assert_eq!(rw.len(), 1);
    for i in 0..10000_0000{
        let _a = rw.get(i);
        RECKON_BY_SEC.add();
    }
}

//test bench_sync_vec_push ... bench:           17 ns/iter (+/- 2)
#[test]
fn bench_vec_push() {
    let rw = std::sync::Mutex::new(vec![1]);
    for i in 0..10000_0000{
        rw.lock().unwrap().push(i);
        RECKON_BY_SEC.add();
    };
}


//test bench_sync_vec_push ... bench:           17 ns/iter (+/- 7)
#[test]
fn bench_sync_vec_push() {
    let rw = SyncVec::new();
    let mut i = 0;
    for i in 0..10000_0000{
        rw.push(i);
        RECKON_BY_SEC.add();
    };
}
