use i_float::int::point::IntPoint;
use crate::edge::direct_scan::DirectScan;
use crate::edge::segment::IdSegment;

pub(crate) struct DirectPointSolver;

impl DirectPointSolver {
    pub(crate) fn run(items: &Vec<IdSegment>, points: &Vec<IntPoint>) -> Vec<usize> {
        let mut scan_list = DirectScan::new();

        let mut result = Vec::with_capacity(items.len());

        let mut i = 0;
        for p in points {
            while i < items.len() && items[i].segment.a.x <= p.x {
                if !items[i].segment.is_vertical && items[i].segment.b.x > p.x {
                    scan_list.insert(items[i].clone())
                }
                i += 1
            }

            if let Some(seg) = scan_list.find_under(p, p.x) {
                result.push(seg.index);
            } else {
                result.push(usize::MAX);
            }
        }

        result
    }
}