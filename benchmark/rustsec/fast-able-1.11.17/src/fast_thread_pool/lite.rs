use std::{fs, io::Write, sync::Arc};

use crossbeam::atomic::AtomicCell;

use crate::fast_thread_pool::FILE_CORE_AFFINITY;

use super::{get_core_skip, TaskExecutor};


/// 简易线程池
/// 用于快速提交任务
/// 只有一个线程, 只绑定一个核心
pub struct ThreadPoolLite {
    pub thread: TaskExecutor,
}

impl ThreadPoolLite {
    pub fn new() -> ThreadPoolLite {
        let core = core_affinity::get_core_ids().unwrap_or_else(|| {
            warn!("获取cpu核心数失败");
            vec![]
        });

        // 获得之前已经绑定的核心
        _ = fs::File::create_new(FILE_CORE_AFFINITY);
        let old_cpu_num = fs::read_to_string(FILE_CORE_AFFINITY)
            .expect("open core_affinity file read_to_string error");
        let old_cpu_num = old_cpu_num
            .replace("\n", ",")
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let old_last = old_cpu_num.last().map(|x| *x).unwrap_or_else(|| 0);
        let skip = get_core_skip();
        let use_core = if old_last > 1 {
            old_cpu_num
                .last()
                .map(|x| core_affinity::CoreId { id: *x - skip })
                .unwrap_or_else(|| {
                    warn!("获取cpu核心数失败");
                    core_affinity::CoreId { id: 0 }
                })
        } else {
            core.last().map(|x| x.clone()).unwrap_or_else(|| {
                warn!("获取cpu核心数失败");
                core_affinity::CoreId { id: 0 }
            })
        };

        let r = ThreadPoolLite {
            thread: TaskExecutor::new(use_core),
        };

        {
            warn!("绑核 {use_core:?}");
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(FILE_CORE_AFFINITY)
                .unwrap();
            // let _ = writeln!(file, "aaa");
            if !old_cpu_num.is_empty() {
                let _ = file.write_all("\n".as_bytes());
            }
            let _ = file.write_all(use_core.id.to_string().as_bytes());
            file.flush().expect("ThreadPoolLite flush error");
        }

        r
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        self.thread.spawn(|_| f());
    }
}

pub fn _test_thread_lite(test_count: u128) {
    let pool = ThreadPoolLite::new();
    std::thread::sleep(std::time::Duration::from_millis(200));
    let com_time = Arc::new(AtomicCell::new(0_u128));
    for _ in 0..test_count {
        let now = std::time::Instant::now();
        let com_time = com_time.clone();
        pool.spawn(move || {
            // println!("run _test_thread_lite i: {}", i);
            let el = now.elapsed().as_nanos();
            com_time.fetch_add(el);
        });
    }
    println!("------------------------thread_lite 任务提交完成------------------------");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!(
        "thread_lite 测试结果: 线程开启平均耗时: {:.3} micros",
        com_time.load() as f64 / test_count as f64 / 1000.0
    );
}