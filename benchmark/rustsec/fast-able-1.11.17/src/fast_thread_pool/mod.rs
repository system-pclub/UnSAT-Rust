#[cfg(test)]
pub mod test;

pub mod pool;
pub use pool::*;

pub mod lite;
pub use lite::*;

pub mod const_num;
pub use const_num::*;

#[path = "./task_executor_crossbeam.rs"]
pub mod task_executor;
pub use task_executor::*;

#[test]
fn test_thread() {
    // fast_able::fast_thread_pool::_test_get_core_ids();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // 线程开启平均耗时: 1761 ns
    test::_main_loop(30, 100);

    // 线程开启平均耗时: 956.6 ns
    // fast_able::fast_thread_pool::thread_mod::_test_ThreadPoolConstNum(1000);

    // thread_lite 测试结果: 线程开启平均耗时: 168.1962 micros
    // 因为是串行同步开启线程, 所以耗时会比较长
    // fast_able::fast_thread_pool::thread_mod::_test_thread_lite(1000);

    std::thread::sleep(std::time::Duration::from_secs(3));
}

/*
[2024-07-01T08:11:32Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456000, 所有任务耗时(微秒): 1245146, 平均耗时: 2, 耗时任务数(100微秒): 255, 耗时任务数占比: 6/10000
[2024-07-01T08:11:35Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456300, 所有任务耗时(微秒): 1181427, 平均耗时: 2, 耗时任务数(100微秒): 223, 耗时任务数占比: 5/10000
[2024-07-01T08:11:38Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456700, 所有任务耗时(微秒): 1176375, 平均耗时: 2, 耗时任务数(100微秒): 183, 耗时任务数占比: 4/10000
[2024-07-01T08:11:41Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 457200, 所有任务耗时(微秒): 1218507, 平均耗时: 2, 耗时任务数(100微秒): 236, 耗时任务数占比: 5/10000
[2024-07-01T08:11:44Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 457100, 所有任务耗时(微秒): 1159028, 平均耗时: 2, 耗时任务数(100微秒): 113, 耗时任务数占比: 2/10000
[2024-07-01T08:11:47Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456808, 所有任务耗时(微秒): 1170696, 平均耗时: 2, 耗时任务数(100微秒): 218, 耗时任务数占比: 5/10000
[2024-07-01T08:11:50Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456992, 所有任务耗时(微秒): 1254381, 平均耗时: 2, 耗时任务数(100微秒): 242, 耗时任务数占比: 5/10000
[2024-07-01T08:11:53Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 458001, 所有任务耗时(微秒): 1181228, 平均耗时: 2, 耗时任务数(100微秒): 165, 耗时任务数占比: 4/10000
[2024-07-01T08:11:56Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 454999, 所有任务耗时(微秒): 1220606, 平均耗时: 2, 耗时任务数(100微秒): 230, 耗时任务数占比: 5/10000
[2024-07-01T08:11:59Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 457400, 所有任务耗时(微秒): 1203115, 平均耗时: 2, 耗时任务数(100微秒): 155, 耗时任务数占比: 3/10000
[2024-07-01T08:12:02Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456400, 所有任务耗时(微秒): 1316857, 平均耗时: 2, 耗时任务数(100微秒): 474, 耗时任务数占比: 10/10000
[2024-07-01T08:12:05Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456100, 所有任务耗时(微秒): 1189001, 平均耗时: 2, 耗时任务数(100微秒): 113, 耗时任务数占比: 2/10000
[2024-07-01T08:12:08Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 457000, 所有任务耗时(微秒): 1273741, 平均耗时: 2, 耗时任务数(100微秒): 309, 耗时任务数占比: 7/10000
[2024-07-01T08:12:11Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456700, 所有任务耗时(微秒): 1175973, 平均耗时: 2, 耗时任务数(100微秒): 326, 耗时任务数占比: 7/10000
[2024-07-01T08:12:14Z INFO  fast_able::fast_thread_pool] 3秒钟执行任务数: 456400, 所有任务耗时(微秒): 1178047, 平均耗时: 2, 耗时任务数(100微秒): 221, 耗时任务数占比: 5/10000
*/
#[test]
fn _test_task_executor() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    init();
    let pool = TaskExecutor::new(core_affinity::CoreId { id: 21 });
    std::thread::sleep(std::time::Duration::from_millis(200));

    let count = std::sync::Arc::new(crossbeam::atomic::AtomicCell::new(0_i64));
    let elapsed_total = std::sync::Arc::new(crossbeam::atomic::AtomicCell::new(0_i64));
    let elapsed_exp = std::sync::Arc::new(crossbeam::atomic::AtomicCell::new(0_i64));

    let count_c = count.clone();
    let elapsed_total_c = elapsed_total.clone();
    let elapsed_exp_c = elapsed_exp.clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let count = count_c.fetch_and(0);
        let elapsed_total = elapsed_total_c.fetch_and(0);
        let elapsed_exp = elapsed_exp_c.fetch_and(0);
        info!(
            "3秒钟执行任务数: {}, 所有任务耗时(微秒): {}, 平均耗时: {}, 耗时任务数(100微秒): {}, 耗时任务数占比: {:.0}/10000",
            count,
            elapsed_total,
            elapsed_total / count,
            elapsed_exp,
            elapsed_exp as f64 / count as f64 * 10000.0,
        );
    });

    loop {
        for i in 0..100 {
            let time_hs = std::time::Instant::now();
            let count = count.clone();
            let elapsed_total = elapsed_total.clone();
            let elapsed_exp = elapsed_exp.clone();
            // spin::Barrier::new(i % 10).wait();
            // spin::relax::Loop::(Duration::from_micros(i % 50));
            pool.spawn(move |_| {
                let micros = time_hs.elapsed().as_micros();
                count.fetch_add(1);
                elapsed_total.fetch_add(micros as i64);
                if micros > 100 {
                    elapsed_exp.fetch_add(1);
                }
            });
        }
        std::thread::sleep(std::time::Duration::from_micros(110));
    }
    std::thread::sleep(std::time::Duration::from_secs(9999));
}

#[test]
fn _test_tokio() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    init();
    let pool = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));

    static statis_v: once_cell::sync::Lazy<crate::statis::Statis> =
        once_cell::sync::Lazy::new(|| crate::statis::Statis::new(|v| debug!("一秒并发: {v}")));

    static thread_lite: once_cell::sync::Lazy<crate::fast_thread_pool::ThreadPoolLite> =
        once_cell::sync::Lazy::new(|| crate::fast_thread_pool::ThreadPoolLite::new());
    thread_lite.spawn(move || {
        warn!("thread_lite init");
    });
    std::thread::sleep(std::time::Duration::from_millis(600));

    loop {
        for _ in 0..500 {
            let time_hs = std::time::Instant::now();
            pool.spawn_blocking(move || {
                // println!("run _test_thread_lite i: {}", i);
                let micros = time_hs.elapsed().as_micros();
                if micros > 1000 {
                    thread_lite.spawn(move || {
                        warn!("任务耗时过长: {} micros", micros);
                    });
                }
                statis_v.add();
            });
        }
        // std::thread::sleep(std::time::Duration::from_micros(550));
    }
    std::thread::sleep(std::time::Duration::from_secs(9999));
}
