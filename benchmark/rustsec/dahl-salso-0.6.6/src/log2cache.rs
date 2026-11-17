use crate::*;

#[derive(Debug, Clone)]
pub struct Log2Cache {
    log2n: Vec<f64>,
    nlog2n: Vec<f64>,
    nlog2n_difference: Vec<f64>,
}

impl Log2Cache {
    pub fn new(n: usize) -> Self {
        let mut log2n = Vec::with_capacity(n + 1);
        let mut nlog2n = Vec::with_capacity(n + 1);
        let mut nlog2n_difference = Vec::with_capacity(n);
        log2n.push(0.0);
        nlog2n.push(0.0);
        for i in 1..=n {
            let i = i as f64;
            let log2i = i.log2();
            log2n.push(log2i);
            let ilog2i = i * log2i;
            let ilog2i_last = *nlog2n.last().unwrap();
            nlog2n.push(ilog2i);
            nlog2n_difference.push(ilog2i - ilog2i_last);
        }
        Self {
            log2n,
            nlog2n,
            nlog2n_difference,
        }
    }

    pub fn plog2p(&self, x: CountType, n: CountType) -> f64 {
        let p = (x as f64) / (n as f64);
        let log2p = unsafe {
            *self.log2n.get_unchecked(x as usize) - *self.log2n.get_unchecked(n as usize)
        };
        p * log2p
    }

    pub fn nlog2n(&self, n: CountType) -> f64 {
        unsafe { *self.nlog2n.get_unchecked(n as usize) }
    }

    pub fn nlog2n_difference(&self, n: CountType) -> f64 {
        unsafe { *self.nlog2n_difference.get_unchecked(n as usize) }
    }
}
