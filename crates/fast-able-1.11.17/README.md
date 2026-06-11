# sync-data
sync-data is a high-performance synchronization library

* SyncHashMap     (sync HashMap)
* SyncBtreeMap    (sync BtreeMap)
* SyncVec         (sync Vec)
* WaitGroup       (async/blocking all support WaitGroup)

for example:
```rust
    pub static RECKON_BY_SEC: once_cell::sync::Lazy<Statis> =
    once_cell::sync::Lazy::new(|| Statis::new(|v| println!("one sec run sum: {v}")));

// one sec run sum: 2347872
#[test]
fn bench_insert_mul_thread() {
    // common_uu::log4rs_mod::init().unwrap();
    let rw = Arc::new(SyncHashMap::new(Some(10)));
    rw.insert(1, 1);
    assert_eq!(rw.len(), 1);

    let rw2 = rw.clone();
    let rt1 = std::thread::spawn(move || {
        for i in 0..5_0000_0000_u64 {
            rw2.insert(i, i + 1);
            RECKON_BY_SEC.add();
        }
    });

    let rw2 = rw.clone();
    let rt2 = std::thread::spawn(move || {
        for i in 5_0000_0000..10_0000_0000_u64 {
            rw2.insert(i, i + 1);
            RECKON_BY_SEC.add();
        }
    });

    let rw2 = rw.clone();
    let rt3 = std::thread::spawn(move || {
        for i in 10_0000_0000_u64..50_0000_0000_u64 {
            rw2.insert(i, i + 1);
            RECKON_BY_SEC.add();
        }
    });


    rt1.join();
}
```


wait group:
```rust
use std::time::Duration;
use tokio::time::sleep;
use fast_able::wg::WaitGroup;
#[tokio::test]
async fn test_wg() {
    let wg = WaitGroup::new();
    let wg2 = wg.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        drop(wg2);
    });
    let wg2 = wg.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        drop(wg2);
    });
    wg.wait_async().await;
    println!("all done");
}
```