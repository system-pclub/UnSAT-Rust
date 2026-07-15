mod edge;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use i_float::int::point::IntPoint;
    use rand::Rng;
    use crate::edge::cross_solver::CrossSolver;
    use crate::edge::direct_solver::DirectPointSolver;
    use crate::edge::segment::{IdSegment, Segment};
    use crate::edge::tree_solver::TreePointSolver;

    #[test]
    fn test_00() {
        let edges = vec![
            IdSegment { index: 1, segment: Segment::new(IntPoint { x: 3, y: 3 }, IntPoint { x: 4, y: 2 }) },
            IdSegment { index: 0, segment: Segment::new(IntPoint { x: 4, y: 1 }, IntPoint { x: 6, y: 7 }) },
            IdSegment { index: 2, segment: Segment::new(IntPoint { x: 5, y: 1 }, IntPoint { x: 6, y: 0 }) },
        ];

        let points = vec![IntPoint { x: 3, y: 7 }, IntPoint { x: 5, y: 7 }];
        let result0 = DirectPointSolver::run(&edges, &points);
        let result1 = TreePointSolver::run(&edges, &points);

        assert_eq!(result0, result1);
    }

    #[test]
    fn test_01() {
        let edges = vec![
            IdSegment { index: 0, segment: Segment::new(IntPoint { x: 1, y: 5 }, IntPoint { x: 1, y: 6 }) },
            IdSegment { index: 1, segment: Segment::new(IntPoint { x: 1, y: 5 }, IntPoint { x: 6, y: 0 }) },
            IdSegment { index: 2, segment: Segment::new(IntPoint { x: 1, y: 3 }, IntPoint { x: 2, y: 2 }) },
            IdSegment { index: 3, segment: Segment::new(IntPoint { x: 4, y: 4 }, IntPoint { x: 5, y: 5 }) },
        ];

        let points = vec![IntPoint { x: 1, y: 7 }];
        let result0 = DirectPointSolver::run(&edges, &points);
        let result1 = TreePointSolver::run(&edges, &points);

        assert_eq!(result0, result1);
    }

    #[test]
    fn test_02() {
        let edges = vec![
            IdSegment { index: 3, segment: Segment::new(IntPoint { x: 0, y: 3 }, IntPoint { x: 7, y: 6 }) },
            IdSegment { index: 5, segment: Segment::new(IntPoint { x: 0, y: 5 }, IntPoint { x: 0, y: 5 }) },
            IdSegment { index: 2, segment: Segment::new(IntPoint { x: 1, y: 0 }, IntPoint { x: 7, y: 5 }) },
            IdSegment { index: 4, segment: Segment::new(IntPoint { x: 1, y: 4 }, IntPoint { x: 2, y: 4 }) },
            IdSegment { index: 7, segment: Segment::new(IntPoint { x: 1, y: 4 }, IntPoint { x: 2, y: 5 }) },
            IdSegment { index: 6, segment: Segment::new(IntPoint { x: 2, y: 2 }, IntPoint { x: 2, y: 2 }) },
            IdSegment { index: 0, segment: Segment::new(IntPoint { x: 3, y: 3 }, IntPoint { x: 4, y: 4 }) },
            IdSegment { index: 1, segment: Segment::new(IntPoint { x: 3, y: 3 }, IntPoint { x: 4, y: 3 }) },
        ];

        let points = vec![
            IntPoint { x: 2, y: 1 },
            IntPoint { x: 3, y: 1 },
            IntPoint { x: 6, y: 5 },
            IntPoint { x: 6, y: 4 },
            IntPoint { x: 6, y: 0 },
        ];
        let result0 = DirectPointSolver::run(&edges, &points);
        let result1 = TreePointSolver::run(&edges, &points);

        assert_eq!(result0, result1);
    }

    #[test]
    fn test_single_random() {
        let edges = random_edges(0..10, 2..6, 20);
        let points = random_points(0..10, 20, &edges);

        let result0 = DirectPointSolver::run(&edges, &points);
        let result1 = TreePointSolver::run(&edges, &points);

        assert_eq!(result0, result1, "Edges: {:?}\nPoints: {:?}", edges, points);
    }

    #[test]
    fn test_small_random() {
        for _ in 0..10000 {
            let edges = random_edges(0..8, 2..6, 8);
            let points = random_points(0..8, 5, &edges);

            let result0 = DirectPointSolver::run(&edges, &points);
            let result1 = TreePointSolver::run(&edges, &points);

            let is_equal = result0 == result1;
            if !is_equal {
                assert_eq!(result0, result1, "Edges: {:?}\nPoints: {:?}", edges, points);
            }
        }
    }

    fn random_edges(range: std::ops::Range<i32>, length: std::ops::Range<i32>, count: usize) -> Vec<IdSegment> {
        let list = CrossSolver::random_segments(range, length, count);
        let mut result = Vec::with_capacity(list.len());

        for i in 0..list.len() {
            let e = &list[i];
            let segment = if e.a.x <= e.b.x { Segment::new(e.a, e.b) } else { Segment::new(e.b, e.a) };
            result.push(IdSegment { index: i, segment });
        }

        result.sort_by(|s0, s1| s0.segment.a.x.cmp(&s1.segment.a.x));

        result
    }

    fn random_points(range: std::ops::Range<i32>, count: usize, exclude: &Vec<IdSegment>) -> Vec<IntPoint> {
        let mut p_set: HashSet<IntPoint> = HashSet::with_capacity(2 * exclude.len());
        for e in exclude.iter() {
            p_set.insert(e.segment.a.clone());
            p_set.insert(e.segment.b.clone());
        }

        let mut rng = rand::thread_rng();
        let mut result = Vec::with_capacity(count);

        while result.len() < count {
            let p = IntPoint {
                x: rng.gen_range(range.clone()),
                y: rng.gen_range(range.clone()),
            };
            if !p_set.contains(&p) {
                p_set.insert(p);
                result.push(p);
            }
        }

        result.sort_by(|a, b| a.x.cmp(&b.x));
        result
    }
}