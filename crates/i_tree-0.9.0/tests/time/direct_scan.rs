use crate::time::time_interval_value::TimeIntervalValue;
use crate::time::time_value::TimeValue;

pub(super) struct DirectScan {
    buffer: Vec<TimeIntervalValue>,
}

impl DirectScan {
    pub(super) fn new() -> Self {
        DirectScan { buffer: Vec::new() }
    }

    pub(super) fn insert(&mut self, item: TimeIntervalValue) {
        self.buffer.push(item);
    }

    pub(super) fn find_equal_or_lower(&mut self, t: &TimeValue) -> i32 {
        let mut i = 0;
        let mut result = i32::MIN;
        while i < self.buffer.len() {
            if self.buffer[i].end <= t.time {
                self.buffer.smart_remove(i);
            } else {
                let value = self.buffer[i].value;
                if value == t.value {
                    return value;
                } else if value < t.value {
                    result = value.max(result);
                }

                i += 1
            }
        }

        result
    }
}

trait SmartRemove {
    fn smart_remove(&mut self, index: usize);
}

impl<T> SmartRemove for Vec<T> {
    fn smart_remove(&mut self, index: usize) {
        if self.len() >= index {
            self.swap_remove(index);
        } else {
            _ = self.pop()
        }
    }
}