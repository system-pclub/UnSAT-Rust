use i_float::int::point::IntPoint;
use crate::edge::segment::IdSegment;

pub(super) struct DirectScan {
    buffer: Vec<IdSegment>,
}

impl DirectScan {
    pub(super) fn new() -> Self {
        DirectScan { buffer: Vec::new() }
    }

    pub(super) fn insert(&mut self, item: IdSegment) {
        self.buffer.push(item);
    }

    pub(super) fn find_under(&mut self, p: &IntPoint, stop: i32) -> Option<IdSegment> {
        let mut i = 0;
        let mut result: Option<IdSegment> = None;
        while i < self.buffer.len() {
            if self.buffer[i].segment.b.x <= stop {
                self.buffer.smart_remove(i)
            } else {
                let segment = &self.buffer[i].segment;
                if segment.is_under(&p) {
                    if let Some(best_seg) = &result {
                        if best_seg.segment.is_under_segment(segment) {
                            result = Some(self.buffer[i].clone());
                        }
                    } else {
                        result = Some(self.buffer[i].clone());
                    }
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