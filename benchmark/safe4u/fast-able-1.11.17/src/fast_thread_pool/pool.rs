use crate::stock_pool::StockPool;
use core_affinity::CoreId;
use crossbeam::{atomic::AtomicCell, queue};
use spin::Mutex;
use std::{fmt::Debug, fs, io::Write, sync::Arc, thread};

use super::TaskExecutor;

pub const FILE_CORE_AFFINITY: &str = "./.core_affinity";

pub fn init() {
    _ = fs::remove_file(FILE_CORE_AFFINITY);
    warn!(
        "thread_mod init; remove_file core_affinity: {:?}",
        FILE_CORE_AFFINITY
    );
}

#[cfg(feature = "deal_physical_cpu")]
pub fn get_core_skip() -> usize {
    let core_ids = num_cpus::get();
    let core_physical = num_cpus::get_physical();
    if core_ids / core_physical == 2 {
        warn!("core_ids: {core_ids}, core_physical: {core_physical}; skip 2");
        2
    } else {
        1
    }
}

#[cfg(not(feature = "deal_physical_cpu"))]
pub fn get_core_skip() -> usize {
    1
}

/// 线程池
/// 通过线程池来管理线程
/// 此线程池的线程个数默认为cpu核心数的4分之1; 任务默认提交在默认线程池;
/// 通过一个api开启独享高性能模式; 独享高性能模式下,线程池的线程个数最多为cpu核心数的5分之1, 比如128核的cpu, 线程池的线程个数最多为25个;
/// theads_高性能模式: 通过查看当前线程的任务数, 如果任务数10毫秒内任务数是其它线程中最少的, 则将任务分配给该线程;
/// 使用 core_affinity 获得cpu核心数
/// 如果已经有一个股票的任务在一个线程中执行, 则将任务分配给该线程; 如果该股票的任务全部执行完毕, 则将任务分配给任务数最少的线程;
pub struct ThreadPool {
    pub threads_share: Vec<TaskExecutor>,
    pub threads_fast: crate::vec::SyncVec<TaskExecutor>,

    pub switch_thread_index: Mutex<i32>,

    // pub 高性能模式_任务数最少的线程: AtomicCell<usize>,
    /// 记录高性能模式下, 当前股票代码在哪个线程中执行
    pub threads_fast_idx: StockPool<AtomicCell<Option<usize>>>,

    /// 记录高性能模式下, 当前股票代码正在执行的任务数
    // 高性能模式_记录_任务数: StockPool<Arc<()>>,

    /// 防止同一支票并行运行
    pub stock_lock: StockPool<Arc<spin::Mutex<()>>>,
    // 多少个核心
    // core_num: Vec<core_affinity::CoreId>,
    // current_core: AtomicCell<i32>,
}

// pub struct Pto<T: Debug + Default>(AtomicCell<T>);
// impl<T: Debug + Default> Deref for Pto<T> {
//     type Target = AtomicCell<T>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl<T: Debug + Default + Copy> Debug for Pto<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&format!("{:?}", self.load()))
//     }
// }
// impl<T: Debug + Default> Default for Pto<T> {
//     fn default() -> Self {
//         Pto(AtomicCell::new(T::default()))
//     }
// }

impl ThreadPool {
    pub fn new(mut cpu_fraction_fast: usize, cpu_fraction_share: usize) -> ThreadPool {
        let cpu_num = core_affinity::get_core_ids()
            .unwrap_or_else(|| vec![])
            .len();

        let skip = get_core_skip();
        let cpu_num = cpu_num / skip;
        let num = cpu_num / cpu_fraction_share;

        let core_num = core_affinity::get_core_ids().unwrap_or_else(|| {
            warn!("获取cpu核心数失败");
            vec![]
        });
        let mut current_core: i32 = core_num.len() as i32;
        let max_core = current_core;

        // 读取之前的绑核信息
        _ = fs::File::create_new(FILE_CORE_AFFINITY);
        let old_cpu_num = fs::read_to_string(FILE_CORE_AFFINITY)
            .expect("open core_affinity file read_to_string error");
        println!("old_cpu_num: {}", old_cpu_num);
        if !old_cpu_num.is_empty() {
            let old_cpu_num = old_cpu_num.replace("\n", ",");
            let old_cpu_num = old_cpu_num.split(',').collect::<Vec<_>>();
            if let Some(old_cpu_num) = old_cpu_num.last() {
                if let Ok(old_cpu_num) = old_cpu_num.parse::<i32>() {
                    current_core = old_cpu_num;
                }
            }
        }
        let skip = get_core_skip() as i32;
        let mut bind_cores = vec![];

        debug!("threads_share cpu count: {}", num);
        let mut threads_共享 = Vec::with_capacity(num);
        for _ in 0..num {
            current_core -= skip;
            if current_core < 0 {
                current_core = max_core - 1;
            }
            let core = core_num
                .get(current_core as usize)
                .map(|x| x.clone())
                .unwrap_or_else(|| {
                    warn!("获取cpu核心数失败");
                    core_affinity::CoreId { id: 0 }
                });
            bind_cores.push(core.id.to_string());
            threads_共享.push(TaskExecutor::new(core));
        }

        if cpu_fraction_fast == 0 {
            cpu_fraction_fast = cpu_num;
        }
        let mut num = cpu_num / cpu_fraction_fast;
        if num == 0 {
            num = 1;
        }
        debug!("theads_fraction_fast cpu count: {}", num);
        let theads_高性能模式 = crate::vec::SyncVec::with_capacity(num);

        for _ in 0..num {
            current_core -= skip;
            if current_core < 0 {
                current_core = max_core;
            }

            let core = core_num
                .get(current_core as usize)
                .map(|x| x.clone())
                .unwrap_or_else(|| {
                    warn!("获取cpu核心数失败");
                    core_affinity::CoreId { id: 0 }
                });
            bind_cores.push(core.id.to_string());

            theads_高性能模式.push(TaskExecutor::new(core));
        }

        debug!("fast_thread_pool_bind_cores: {:?}", bind_cores);
        // fs::write(FILE_CORE_AFFINITY, bind_cores.join(","))
        //     .expect("write core_affinity file write_all error");

        use std::fs::OpenOptions;
        use std::io::Write;

        {
            let mut file = OpenOptions::new()
                .append(true)
                .open(FILE_CORE_AFFINITY)
                .unwrap();
            // let _ = writeln!(file, "aaa");
            if !old_cpu_num.is_empty() {
                let _ = file.write_all("\n".as_bytes());
            }
            let _ = file.write_all(bind_cores.join(",").as_bytes());
            file.flush().expect("ThreadPoolLite flush error");
        }

        // std::env::set_var("fast_thread_pool_bind_cores", bind_cores.join(","));

        let r = ThreadPool {
            threads_share: threads_共享,
            threads_fast: theads_高性能模式,
            // 高性能模式_任务数最少的线程: 0.into(),
            threads_fast_idx: StockPool::new(),
            // 高性能模式_记录_任务数: StockPool::new(),
            stock_lock: StockPool::new(),
            switch_thread_index: (-1).into(),
            // core_num,
            // current_core: -1.into(),
        };

        // let r1 = r.clone();
        // std::thread::spawn(move || loop {
        //     r1.loop_任务数最少的线程();
        //     std::thread::sleep(std::time::Duration::from_millis(10));
        // });
        r
    }

    /// 获取拥有最少任务的线程索引
    ///
    /// 本函数遍历快速线程池中的线程，寻找任务计数最少的线程。
    /// 如果找到一个任务计数为0的线程，它将立即返回该线程的索引，
    /// 因为这表示该线程目前没有任务。否则，函数将返回任务计数最少的线程索引。
    /// 这个信息用于调度新任务到拥有最少任务的线程，以平衡线程间的工作负载。
    /*
    pub fn count_task_min(&self) -> usize {
        // 初始化最小任务计数为第一个线程的任务计数，最小索引为0
        let mut min_count = self.theads_fast[0].count.load();
        let mut min_index = 0;

        // 遍历快速线程池中的线程
        for (i, thread) in self.theads_fast.iter().enumerate() {
            let count = thread.count.load();

            // 如果找到一个任务计数为0的线程，立即返回其索引
            if count == 0 {
                min_index = i;
                break;
            }

            // 如果当前线程的任务计数少于最小任务计数，更新最小任务计数和索引
            if count < min_count {
                min_count = count;
                min_index = i;
            }
        }

        // 返回任务计数最少的线程索引
        min_index
    }
    */

    /// 获取下一个要切换到的线程的索引，以实现线程之间的任务平衡。
    ///
    /// 这个方法通过循环的方式选择下一个线程索引，以确保任务能够均匀地分配给每个线程。
    /// 它使用了一个互斥锁来保证在多线程环境下对索引的访问是安全的。
    ///
    /// # 返回值
    /// 根据当前线程池的状态，计算并返回下一个应该执行任务的线程索引。
    /// 这个方法旨在平衡线程间的任务分配，避免某个线程过载而其他线程闲置的情况。
    ///
    /// 参数 `i7` 作为一个辅助的计算参数，用于在无法立即获得锁时决定返回哪个线程索引。
    ///
    /// 返回值是一个枚举 `IdxStatus`，它可以指示线程应该记住当前计算出的索引（`Remember`），
    /// 或者由于锁的竞争失败而丢弃当前的计算并使用另一个索引（`Discard`）。
    /// 返回当前选择的线程索引。
    pub fn count_task_min(&self, i7: i32) -> IdxStatus {
        // 获取线程池中线程的数量，用于后续计算下一个任务线程的索引。
        // 获取线程池中线程的数量
        let len = self.threads_fast.len();

        // 尝试获取用于控制任务分配的索引互斥锁，如果无法立即获得锁，则根据 `i7` 返回一个备选索引。
        // 获取用于控制线程切换的索引的互斥锁
        let mut min_index: spin::MutexGuard<i32> = match self.switch_thread_index.try_lock() {
            Some(mutex) => mutex,
            None => return IdxStatus::Discard(i7 as usize % len),
        };

        // 如果当前索引为0，将其设置为最大值，否则递减索引，以实现循环分配策略。
        // 这样做是为了实现循环访问，避免索引越界
        if *min_index == 0 || *min_index == -1 {
            *min_index = len as i32 - 1;
        } else {
            *min_index -= 1;
        }

        // 返回之前复制的索引值，指示线程应该记住这个索引以供下次使用。
        // 返回复制的索引值
        let r = *min_index as usize;
        self.threads_fast_idx[i7].store(Some(r));
        IdxStatus::Remember(r)
    }

    #[inline(always)]
    pub fn spawn<F>(&self, i7: i32, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let _lock = self.stock_lock[i7].clone();
        let index = i7 % self.threads_share.len() as i32;
        self.threads_share[index as usize].spawn(move |_core| {
            let _lock = _lock.lock();
            // #[cfg(debug_assertions)]
            // print!("高性能模式, 共享线程({i7}): {index} ");
            f();
            drop(_lock);
        });
    }

    #[inline(always)]
    pub fn spawn_fast<F>(&self, i7: i32, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let mut on_fast_idx = -1;

        // 找最少任务数的线程
        #[cfg(not(feature = "thread_dispatch"))]
        let thread_idx = self.threads_fast_idx[i7].load().unwrap_or_else(|| {
            let min = self.count_task_min(i7);
            let idx = min.get_idx();
            if let IdxStatus::Remember(idx) = &min {
                on_fast_idx = *idx as i32;
            }
            idx
        });

        // 如果当前任务堆积小于5个, 则使用当前线程; 否则就去找最少任务数的线程
        // 有任务调度的线程方法, 如果有任务堆积则通过找任务数最少的线程来提交任务
        #[cfg(feature = "thread_dispatch")]
        let thread_idx = match self.threads_fast_idx[i7].load() {
            Some(i) if self.threads_fast[i].count.load() < 1000 => i,
            _ => {
                let min = self.count_task_min(i7);
                let idx = min.get_idx();
                if let IdxStatus::Remember(idx) = &min {
                    on_fast_idx = *idx as i32;
                }
                idx
            }
        };

        // 提交任务
        let lock = self.stock_lock[i7].clone();
        self.threads_fast[thread_idx].spawn(move |core| {
            let lock_v = lock.lock();
            // print!(" {i7} theads_fast: {thread_idx} ");
            f();
            drop(lock_v);
            if on_fast_idx != -1 {
                warn!("on_fast thread; i7: {i7}, cpu: {core}");
            }
        });
    }

    #[inline(always)]
    pub fn spawn_is_fast<F>(&self, i7: i32, is_fast: bool, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        if is_fast {
            self.spawn_fast(i7, f);
        } else {
            self.spawn(i7, f);
        }
    }
}

pub enum IdxStatus {
    // 记住索引
    Remember(usize),
    // 丢弃索引
    Discard(usize),
}

impl IdxStatus {
    pub fn get_idx(&self) -> usize {
        match self {
            IdxStatus::Remember(idx) => *idx,
            IdxStatus::Discard(idx) => *idx,
        }
    }
}

#[test]
fn _test_pool() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    init();
    let pool = ThreadPool::new(2, 4);
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
            // std::thread::sleep(std::time::Duration::from_micros(i % 50));
            let time_hs = std::time::Instant::now();
            let count = count.clone();
            let elapsed_total = elapsed_total.clone();
            let elapsed_exp = elapsed_exp.clone();
            // spin::Barrier::new(i % 10).wait();
            // spin::relax::Loop::(Duration::from_micros(i % 50));
            let i7 = if i % 3 == 0 { 1000001 } else { 1000002 };
            pool.spawn_is_fast(i7, true, move || {
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
