// #![feature(test)]
// extern crate test;

use fast_able::statis::Statis;
use fast_able::SyncHashMap;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_iter() {
    let rw = SyncHashMap::new();
    rw.insert(1, 1);
    assert_eq!(rw.len(), 1);

    for i in 0..10000_0000 {
        rw.insert(i, i + 1);
        RECKON_BY_SEC.add();
    }

    for ele in rw.iter() {
        println!("{:?}", ele);
    }

    for ele in rw.iter().rev() {
        println!("{:?}", ele);
    }

    rt1.join();
    rt2.join();
}

//6 ns/iter (+/- 0)
/* #[bench]
fn bench_sync_map_get(b: &mut test::Bencher) {
    let rw = SyncHashMap::new();
    rw.insert(1, 1);
    assert_eq!(rw.len(), 1);
    b.iter(|| {
        rw.get(&1);
    });
} */

// //18 ns/iter (+/- 0)
// #[bench]
// fn bench_dash_map_get(b: &mut test::Bencher) {
//     let rw = dashmap::DashMap::new();
//     rw.insert(1,1);
//     b.iter(|| {
//         let _=rw.get(&1);
//     });
// }

//8 ns/iter (+/- 0)
/* #[bench]
fn bench_sync_map_insert(b: &mut test::Bencher) {
    let rw = SyncHashMap::new();
    b.iter(|| {
        rw.insert(1, 1);
    });
} */

// //17 ns/iter (+/- 0)
// #[bench]
// fn bench_dash_map_insert(b: &mut test::Bencher) {
//     let rw = dashmap::DashMap::new();
//     b.iter(|| {
//         rw.insert(1,1);
//     });
// }

pub static RECKON_BY_SEC: once_cell::sync::Lazy<Statis> =
    once_cell::sync::Lazy::new(|| Statis::new(|v| println!("one sec run sum: {v}")));

// one sec run sum: 1142993
#[test]
fn bench_insert_mul_thread() {
    let rw = Arc::new(SyncHashMap::new());
    rw.insert(1, 1);
    assert_eq!(rw.len(), 1);

    let rw2 = rw.clone();
    let rt1 = std::thread::spawn(move || {
        for i in 0..10000_0000 {
            rw2.insert(i, i + 1);
            RECKON_BY_SEC.add();
        }
    });

    let rw2 = rw.clone();
    let rt2 = std::thread::spawn(move || {
        for i in 10000_0000..20000_0000 {
            rw2.insert(i, i + 1);
            RECKON_BY_SEC.add();
        }
    });

    let rw2 = rw.clone();
    let rt3 = std::thread::spawn(move || {
        for i in 20000_0000..30000_0000 {
            rw2.insert(i, i + 1);
            RECKON_BY_SEC.add();
        }
    });

    rt1.join();
    rt2.join();
}

/// one sec run sum: 1335790
#[test]
fn bench_insert_one_thread() {
    let rw = Arc::new(SyncHashMap::new());
    rw.insert(1, 1);
    assert_eq!(rw.len(), 1);

    let rw2 = rw.clone();
    let rt1 = std::thread::spawn(move || {
        for i in 0..10000_0000 {
            rw2.insert(i, i + 1);
            RECKON_BY_SEC.add();
        }
    });

    rt1.join();
}

// //62 ns/iter (+/- 27)
// #[bench]
// fn bench_dash_map_insert_race(b: &mut test::Bencher) {
//     let rw = Arc::new(dashmap::DashMap::new());
//     let rw2=rw.clone();
//     std::thread::spawn(move ||{
//         loop{
//             rw2.insert(1,1);
//         }
//     });
//     b.iter(|| {
//         rw.insert(1,1);
//     });
// }
