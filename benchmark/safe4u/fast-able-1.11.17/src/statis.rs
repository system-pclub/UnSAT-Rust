use std::{sync::Arc, time::Duration};

pub struct Statis {
    _rt: std::thread::JoinHandle<()>,
    sum: Arc<std::sync::atomic::AtomicU64>,
}

impl Statis {
    pub fn new<P: Fn(u64) + Send + 'static>(print: P) -> Self {
        let sum = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let sum_clone = sum.clone();
        let rt = std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_millis(1000));
            let v = sum_clone.fetch_and(0, std::sync::atomic::Ordering::SeqCst);
            if v > 0 {
                print(v);
            }
        });
        Self { _rt: rt, sum }
    }

    pub fn add(&self) {
        _ = self.sum.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

impl Drop for Statis {
    fn drop(&mut self) {}
}

#[test]
fn test() {
    pub static RECKON_BY_SEC: once_cell::sync::Lazy<Statis> =
        once_cell::sync::Lazy::new(|| Statis::new(|v| println!("one sec run sum: {v}")));

    let rt = std::thread::spawn(|| {
        for i in 0..10000_0000 {
            RECKON_BY_SEC.add();
        }
    });
    let rt2 = std::thread::spawn(|| {
        for i in 0..10000_0000 {
            RECKON_BY_SEC.add();
        }
    });
    let rt3 = std::thread::spawn(|| {
        for i in 0..10000_0000 {
            RECKON_BY_SEC.add();
        }
    });

    rt.join();
    rt2.join();
    rt3.join();
}
