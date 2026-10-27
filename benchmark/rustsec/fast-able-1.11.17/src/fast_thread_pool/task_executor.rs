use std::{sync::Arc, thread};

use core_affinity::CoreId;
use crossbeam::{atomic::AtomicCell, queue};


/// 简易线程池
pub struct TaskExecutor {
    jobs: Arc<queue::SegQueue<Box<dyn FnOnce(&usize) + Send + 'static>>>,
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
        let queue: queue::SegQueue<Box<dyn FnOnce(&usize) + Send + 'static>> =
            queue::SegQueue::new();
        let queue = Arc::new(queue);
        let queue_c = queue.clone();
        let count = Arc::new(AtomicCell::new(0_i64));

        let count_c = count.clone();
        let _handle = thread::spawn(move || {
            // 绑核
            {
                let core_id = core.id;
                let b = core_affinity::set_for_current(core);
                
                // 绑核成功, 输出日志
                warn!("bind {core_id} {b}");
            }
            let ref core = core.id;
            loop {
                if let Some(job) = queue_c.pop() {
                    job(core);
                    count_c.fetch_sub(1);
                }
            }
        });

        TaskExecutor {
            jobs: queue,
            _handle,
            count,
            core: core.id,
        }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce(&usize),
        F: Send + 'static,
    {
        self.count.fetch_add(1);
        self.jobs.push(Box::new(f));
    }
}

