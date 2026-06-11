use std::{fs, io::Write, sync::Arc};

use core_affinity::CoreId;
use crossbeam::atomic::AtomicCell;

use super::{get_core_skip, TaskExecutor, FILE_CORE_AFFINITY};


/// 简易线程池
/// 用于快速提交任务
/// 只有一个线程, 只绑定一个核心
pub struct ThreadPoolConstNum<const N: usize> {
    thread: [TaskExecutor; N],
    cur_run_core: AtomicCell<usize>,
}

impl<const N: usize> ThreadPoolConstNum<N> {
    pub fn new() -> Self {
        let cores = core_affinity::get_core_ids().unwrap_or_else(|| {
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
        let mut use_core = if !old_cpu_num.is_empty() {
            old_last
        } else {
            cores.last().map(|x| x.id).unwrap_or_else(|| 0)
        };

        let mut use_core_cur = vec![];
        let mut threads = vec![];

        let skip = get_core_skip();

        for _ in 0..N {
            if (use_core as i32) - (skip as i32) < 0 {
                use_core = cores.len() + 1;
            }
            if use_core == 0 {
                use_core = 2;
            }
            use_core -= skip;
            use_core_cur.push(use_core);
            threads.push(TaskExecutor::new(CoreId { id: use_core }));
        }

        let r = ThreadPoolConstNum {
            thread: threads
                .try_into()
                .expect("ThreadPoolLiteNum threads.try_into()"),
            cur_run_core: 0.into(),
        };

        if !use_core_cur.is_empty() {
            println!("old_cpu_num: {old_cpu_num:?}; use_core_cur {use_core_cur:?}");
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(FILE_CORE_AFFINITY)
                .unwrap();
            // let _ = writeln!(file, "aaa");
            if !old_cpu_num.is_empty() {
                let _ = file.write_all("\n".as_bytes());
            }
            let _ = file.write_all(
                use_core_cur
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_string()
                    .as_bytes(),
            );
            file.flush().expect("ThreadPoolLite flush error");
        }

        r
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let cur_index = self.cur_run_core.fetch_add(1);
        self.thread[cur_index % N].spawn(|_| f());
        if cur_index >= usize::MAX - N {
            self.cur_run_core.store(0);
        }
    }
}

pub fn _test_ThreadPoolConstNum(test_count: u128) {
    println!("------------------------start------------------------");
    // println!("1%3: {}", 1 % 3);
    // println!("2%3: {}", 2 % 3);
    // println!("3%3: {}", 3 % 3);
    // println!("4%3: {}", 4 % 3);
    // println!("5%3: {}", 5 % 3);
    // println!("6%3: {}", 6 % 3);
    // println!("7%3: {}", 7 % 3);
    // println!("8%3: {}", 8 % 3);

    let pool = ThreadPoolConstNum::<5>::new();
    std::thread::sleep(std::time::Duration::from_millis(200));
    let com_time = Arc::new(AtomicCell::new(0_u128));
    for _ in 0..test_count {
        let com_time = com_time.clone();
        let now = std::time::Instant::now();
        pool.spawn(move || {
            // println!("run _test_cur_index i: {}", i);
            com_time.fetch_add(now.elapsed().as_nanos());
        });
    }
    println!("------------------------ThreadPoolConstNum 任务提交完成------------------------");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!(
        "ThreadPoolConstNum 线程开启平均耗时: {} ns",
        com_time.load() as f64 / test_count as f64
    );
}
