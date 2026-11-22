use rand::Rng;
use crate::time::time_interval_value::TimeIntervalValue;
use crate::time::time_value::TimeValue;

pub(crate) struct TimeValueGenerator;

impl TimeValueGenerator {
    pub(crate) fn random_time_values(range: &std::ops::Range<i32>, time: &std::ops::Range<i32>, count: usize) -> Vec<TimeValue> {
        let mut result = Vec::with_capacity(count);

        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let value = rng.gen_range(range.clone());
            let t = rng.gen_range(time.clone());
            result.push(TimeValue { value, time: t })
        }

        result
    }

    pub(crate) fn random_time_ranges(range: &std::ops::Range<i32>, time: &std::ops::Range<i32>, count: usize) -> Vec<TimeIntervalValue> {
        let mut result = Vec::with_capacity(count);
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let value = rng.gen_range(range.clone());
            let t0 = rng.gen_range(time.clone());
            let t1 = rng.gen_range(time.clone());

            let start: i32;
            let end: i32;
            if t0 == t1 {
                start = t0;
                end = t0 + 1;
            } else if t0 < t1 {
                start = t0;
                end = t1;
            } else {
                start = t1;
                end = t0;
            }

            result.push(TimeIntervalValue {
                value,
                start,
                end,
            })
        }

        result
    }
}