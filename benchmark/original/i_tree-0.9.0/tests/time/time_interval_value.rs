use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct TimeIntervalValue {
    pub(crate) value: i32,
    pub(crate) start: i32,
    pub(crate) end: i32,
}

impl PartialOrd<Self> for TimeIntervalValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value < other.value {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Eq for TimeIntervalValue {}

impl Ord for TimeIntervalValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value < other.value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}