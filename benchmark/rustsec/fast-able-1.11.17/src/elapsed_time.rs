use std::{fmt::Display, time::Duration};

// 记录耗时的功能
#[derive(Debug, Clone)]
pub struct ElapsedTime {
    pub start_time: std::time::Instant,
    elapsed_logs: Vec<(&'static str, Duration)>,
    total: Duration,
}

impl Display for ElapsedTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.print())
    }
}

impl ElapsedTime {
    pub fn new(capacity: usize) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            elapsed_logs: Vec::with_capacity(capacity),
            total: Duration::ZERO,
        }
    }

    pub fn log(&mut self, log: &'static str) {
        let el = self.start_time.elapsed();
        let el2 = el - self.total;
        self.total = el;
        self.elapsed_logs.push((log, el2));
    }

    pub fn print_limit(&self, limit: Duration) -> Option<String> {
        if self.total < limit {
            return None;
        }
        Some(self.print())
    }

    pub fn print(&self) -> String {
        let logs = self
            .elapsed_logs
            .iter()
            .map(|(log, el)| format!("{log}: {el:?}"))
            .collect::<Vec<_>>()
            .join(", ");

        format!("Elapsed; total: {:?}, {logs}", self.total)
    }
}

#[test]
fn test_ElapsedTime() {
    let mut elapsed_time = ElapsedTime::new(10);
    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(i * 100));
        elapsed_time.log("log");
    }
    println!("{}", elapsed_time.print());
}
