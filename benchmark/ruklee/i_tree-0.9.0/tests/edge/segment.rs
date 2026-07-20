use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Segment {
    pub(crate) a: IntPoint,
    pub(crate) b: IntPoint,
    pub(crate) is_vertical: bool,
}

impl Segment {
    pub(crate) fn new(a: IntPoint, b: IntPoint) -> Self {
        debug_assert!(a.x <= b.x);
        Segment {
            a,
            b,
            is_vertical: a.x == b.x,
        }
    }

    pub(crate) fn is_under(&self, p: &IntPoint) -> bool {
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        debug_assert!(p != &self.a && p != &self.b);
        Triangle::is_clockwise_point(self.a, p.clone(), self.b)
    }

    pub(crate) fn is_under_segment(&self, other: &Segment) -> bool {
        if self.a == other.a {
            Triangle::is_clockwise_point(self.a, other.b, self.b)
        } else if self.a.x < other.a.x {
            Triangle::is_clockwise_point(self.a, other.a, self.b)
        } else {
            Triangle::is_clockwise_point(other.a, other.b, self.a)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct IdSegment {
    pub(crate) index: usize,
    pub(crate) segment: Segment,
}

impl PartialOrd for IdSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.segment.is_under_segment(&other.segment) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl Eq for IdSegment {}

impl Ord for IdSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}