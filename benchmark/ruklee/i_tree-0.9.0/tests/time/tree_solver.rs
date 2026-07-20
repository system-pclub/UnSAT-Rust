use crate::time::time_interval_value::TimeIntervalValue;
use crate::time::time_value::TimeValue;
use crate::time::tree_scan::TreeTimeScan;

pub(crate) struct TreeTimeSolver;

impl TreeTimeSolver {
    pub(crate) fn run(items: &[TimeIntervalValue], times: &[TimeValue]) -> Vec<i32> {
        let mut scan_list = TreeTimeScan::new();

        let mut result = Vec::with_capacity(items.len());

        let mut i = 0;
        for t in times {
            while i < items.len() && items[i].start <= t.time {
                if items[i].end > t.time {
                    scan_list.insert(items[i].clone(), t.time);
                }
                i += 1
            }

            result.push(scan_list.find_equal_or_lower(t))
        }

        result
    }
}