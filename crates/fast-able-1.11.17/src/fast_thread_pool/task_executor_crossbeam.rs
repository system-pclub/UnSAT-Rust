use std::{
    sync::{mpsc::Sender, Arc},
    thread,
};

use core_affinity::CoreId;
use crossbeam::{atomic::AtomicCell, queue};

/// 简易线程池
pub struct TaskExecutor {
    jobs: crossbeam::channel::Sender<Box<dyn FnOnce(&usize) + Send + 'static>>,
    _handle: thread::JoinHandle<()>,
    pub count: Arc<AtomicCell<i64>>,
    core: usize,
}

impl std::fmt::Debug for TaskExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskExecutor")
            .field("_handle", &self._handle)
            .field("count", &self.count)
            .finish()
    }
}

impl TaskExecutor {
    pub fn new(core: CoreId) -> TaskExecutor {
        let (tx, rx) = crossbeam::channel::unbounded::<Box<dyn FnOnce(&usize) + Send + 'static>>();
        let count = Arc::new(AtomicCell::new(0_i64));

        let count_c = count.clone();
        let _handle = thread::spawn(move || {
            // 绑核
            {
                let core_id = core.id;
                let b = core_affinity::set_for_current(core);

                // 绑核成功, 输出日志
                debug!("bind {core_id} {b}");
            }
            let ref core = core.id;

            if cfg!(feature = "thread_dispatch") {
                let mut n = 0;
                loop {
                    if let Ok(job) = rx.try_recv() {
                        job(core);
                        count_c.fetch_sub(1);
                        n = 0;
                    } else {
                        n += 1;

                        if n > 1000 {
                            n = 0;
                            // if let Ok(job) = rx.recv_timeout(std::time::Duration::from_micros(500))
                            if let Ok(job) = rx.recv() {
                                job(core);
                                count_c.fetch_sub(1);
                            }
                        }
                    }
                }
            } else {
                loop {
                    if let Ok(job) = rx.try_recv() {
                        job(core);
                        count_c.fetch_sub(1);
                    }
                }
            }
        });

        TaskExecutor {
            jobs: tx,
            _handle,
            count,
            core: core.id,
        }
    }

    #[inline(always)]
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce(&usize),
        F: Send + 'static,
    {
        self.count.fetch_add(1);
        if let Err(e) = self.jobs.send(Box::new(f)) {
            error!("TaskExecutor send error: {:?}", e);
        }
    }
}
