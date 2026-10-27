use std::{sync::Arc, thread, time::Duration};

use once_cell::sync::Lazy;

use crate::fast_thread_pool::ThreadPoolLite;

use super::pool;

pub fn _test_get_core_ids() {
    let core_ids = num_cpus::get();
    let core_physical = num_cpus::get_physical();
    println!("core_ids: {core_ids}, core_physical: {core_physical}");
}

// #[test]
// #[test]
pub fn _main_test(test_count: usize, input_fast_count: usize) {
    println!("------------------------start------------------------");
    pool::init();
    // use once_cell::sync::Lazy;
    let pool = pool::ThreadPool::new(2, 4);
    // let POOL2: Arc<thread_mod::ThreadPool> = thread_mod::ThreadPool::new(2, 3);
    thread::sleep(std::time::Duration::from_millis(500));
    for i in 0..180 {
        let ii = i + 10_000_00;
        pool.spawn(ii, move || {
            print!("init spawn: {}, ", i);
        });
        pool.spawn_fast(ii, move || {
            print!("init spawn_fast: {}, ", i);
            std::thread::sleep(Duration::from_micros(2));
        });
    }
    // thread::sleep(std::time::Duration::from_millis(200));

    thread::sleep(std::time::Duration::from_millis(1000));
    let add_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    static statis_v: Lazy<crate::statis::Statis> =
        Lazy::new(|| crate::statis::Statis::new(|v| println!("statis_v: {v}")));

    for _i in 0..test_count {
        let time_hs = std::time::Instant::now();
        let add_count_c = add_count.clone();
        let ii = _i as i32 + 10_000_00;
        pool.spawn(ii, move || {
            add_count_c.fetch_add(
                time_hs.elapsed().as_nanos() as usize,
                std::sync::atomic::Ordering::Relaxed,
            );
            statis_v.add();
        });

        let time_hs = std::time::Instant::now();
        let add_count_c = add_count.clone();
        pool.spawn(ii, move || {
            add_count_c.fetch_add(
                time_hs.elapsed().as_nanos() as usize,
                std::sync::atomic::Ordering::Relaxed,
            );
            statis_v.add();
        });

        for _ in 0..input_fast_count {
            let time_hs = std::time::Instant::now();
            let add_count_c = add_count.clone();
            pool.spawn_fast(ii, move || {
                add_count_c.fetch_add(
                    time_hs.elapsed().as_nanos() as usize,
                    std::sync::atomic::Ordering::Relaxed,
                );
                // std::thread::sleep(Duration::from_micros(2));
            });
            statis_v.add();
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("\n------------------------任务提交完成------------------------");
    println!(
        "线程开启平均耗时: {:?} ns",
        add_count.load(std::sync::atomic::Ordering::Relaxed)
            / (test_count * (2 + input_fast_count))
    );
    std::thread::sleep(std::time::Duration::from_secs(2));
}

pub fn _main_loop(stock_count: usize, input_fast_count: usize) {
    println!("------------------------start------------------------");
    pool::init();
    // use once_cell::sync::Lazy;
    let pool = pool::ThreadPool::new(2, 4);
    let pool = Arc::new(pool);
    // let POOL2: Arc<thread_mod::ThreadPool> = thread_mod::ThreadPool::new(2, 3);
    thread::sleep(std::time::Duration::from_millis(500));

    let pool_c = pool.clone();
    // std::thread::spawn(move || {
    //     for i in 0..stock_count {
    //         let ii = i + 10_000_00;
    //         let ii = ii as i32;
    //         pool_c.spawn(ii, move || {
    //             print!("init spawn: {}, ", i);
    //         });
    //         pool_c.spawn_fast(ii, move || {
    //             print!("init spawn_fast: {}, ", i);
    //             std::thread::sleep(Duration::from_micros(2));
    //         });
    //     }
    // });
    // thread::sleep(std::time::Duration::from_millis(200));

    thread::sleep(std::time::Duration::from_millis(1000));
    let add_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    static statis_v: Lazy<crate::statis::Statis> =
        Lazy::new(|| crate::statis::Statis::new(|v| debug!("statis_v: {v}")));

    let thread_lite_run1: ThreadPoolLite = ThreadPoolLite::new();
    thread_lite_run1.spawn(|| warn!("thread_lite_run1 init"));

    let thread_lite_run2: ThreadPoolLite = ThreadPoolLite::new();
    thread_lite_run2.spawn(|| warn!("thread_lite_run2 init"));

    static thread_lite: Lazy<ThreadPoolLite> = Lazy::new(|| ThreadPoolLite::new());
    thread_lite.spawn(|| {
        debug!("thread_lite init");
    });

    std::thread::sleep(std::time::Duration::from_millis(800));

    loop {
        let pool_c = pool.clone();
        let add_count_c = add_count.clone();
        thread_lite_run1.spawn(move || {
            for _i in 0..stock_count {
                let time_hs = std::time::Instant::now();
                let add_count_cc = add_count_c.clone();
                let ii = _i as i32 + 10_000_00;
                pool_c.spawn(ii, move || {
                    add_count_cc.fetch_add(
                        time_hs.elapsed().as_nanos() as usize,
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    statis_v.add();
                });

                let add_count_cc = add_count_c.clone();
                let time_hs = std::time::Instant::now();
                pool_c.spawn(ii, move || {
                    add_count_cc.fetch_add(
                        time_hs.elapsed().as_nanos() as usize,
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    statis_v.add();
                });

                for _ in 0..input_fast_count {
                    let time_hs = std::time::Instant::now();
                    pool_c.spawn_fast(ii, move || {
                        let micros = time_hs.elapsed().as_micros();
                        if micros > 1000 {
                            // thread_lite.spawn(move || {
                            //     warn!("elapsed micros: {micros}");
                            // });
                        }
                        // std::thread::sleep(Duration::from_micros(2));
                    });
                    statis_v.add();
                }
                std::thread::sleep(std::time::Duration::from_micros(200));
            }
        });

        let pool_c = pool.clone();
        let add_count_c = add_count.clone();
        thread_lite_run2.spawn(move || {
            for _i in 0..stock_count {
                let time_hs = std::time::Instant::now();
                let add_count_cc = add_count_c.clone();
                let ii = _i as i32 + 10_000_00;
                pool_c.spawn(ii, move || {
                    add_count_cc.fetch_add(
                        time_hs.elapsed().as_nanos() as usize,
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    statis_v.add();
                });

                let add_count_cc = add_count_c.clone();
                let time_hs = std::time::Instant::now();
                pool_c.spawn(ii, move || {
                    add_count_cc.fetch_add(
                        time_hs.elapsed().as_nanos() as usize,
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    statis_v.add();
                });

                for _ in 0..input_fast_count {
                    let time_hs = std::time::Instant::now();
                    pool_c.spawn_fast(ii, move || {
                        let micros = time_hs.elapsed().as_micros();
                        if micros > 1000 {
                            // thread_lite.spawn(move || {
                            //     warn!("elapsed micros: {micros}");
                            // });
                        }
                        // std::thread::sleep(Duration::from_micros(2));
                    });
                    statis_v.add();
                }
                std::thread::sleep(std::time::Duration::from_micros(200));
            }
        });

        std::thread::sleep(std::time::Duration::from_micros(2000));
    }
}
