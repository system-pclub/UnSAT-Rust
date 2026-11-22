use i_float::fix_float::FixMath;
use i_float::fix_vec::FixVec;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use rand::Rng;

pub(crate) enum CrossType {
    Pure,
    Overlap,
    SameEnd,
    NotCross,
    Equal,
}

pub(crate) struct RandomEdge {
    pub(crate) a: IntPoint,
    pub(crate) b: IntPoint,
}

impl RandomEdge {
    fn sqr_length(&self) -> i64 {
        let dx = (self.a.x - self.b.x) as i64;
        let dy = (self.a.y - self.b.y) as i64;
        dx * dx + dy * dy
    }
}

pub(crate) struct CrossSolver;

impl CrossSolver {
    fn is_cross_edge(e0: &RandomEdge, e1: &RandomEdge) -> CrossType {
        let a0 = FixVec::new_point(e0.a);
        let b0 = FixVec::new_point(e0.b);
        let a1 = FixVec::new_point(e1.a);
        let b1 = FixVec::new_point(e1.b);
        Self::is_cross(a0, b0, a1, b1)
    }

    fn is_cross(a0: FixVec, b0: FixVec, a1: FixVec, b1: FixVec) -> CrossType {
        // box cross

        // all x from 1 is less all x from 0
        let boundary_test = a1.x < a0.x && b1.x < a0.x && a1.x < b0.x && b1.x < b0.x
            // all x from 0 is less all x from 1
            || a0.x < a1.x && b0.x < a1.x && a0.x < b1.x && b0.x < b1.x
            // all y from 1 is less all y from 0
            || a1.y < a0.y && b1.y < a0.y && a1.y < b0.y && b1.y < b0.y
            // all y from 0 is less all y from 1
            || a0.y < a1.y && b0.y < a1.y && a0.y < b1.y && b0.y < b1.y;

        if boundary_test {
            return CrossType::NotCross;
        }

        // cross

        let a0b0a1 = Triangle::clock_direction(a0, b0, a1);
        let a0b0b1 = Triangle::clock_direction(a0, b0, b1);

        let a1b1a0 = Triangle::clock_direction(a1, b1, a0);
        let a1b1b0 = Triangle::clock_direction(a1, b1, b0);

        let is_end0 = a0 == a1 || a0 == b1;
        let is_end1 = b0 == a1 || b0 == b1;

        if is_end0 && is_end1 {
            return CrossType::Equal;
        }

        let is_collinear = a0b0a1 == 0 && a0b0b1 == 0 && a1b1a0 == 0 && a1b1b0 == 0;

        if (is_end0 || is_end1) && is_collinear {
            let dot_product = if is_end0 {
                let e = if a0 == a1 { b1 } else { a1 };
                (a0 - b0).dot_product(a0 - e)
            } else {
                let e = if b0 == a1 { b1 } else { a1 };
                (b0 - a0).dot_product(b0 - e)
            };

            return if dot_product < 0 {
                CrossType::SameEnd
            } else {
                CrossType::Overlap
            };
        } else if is_collinear {
            return CrossType::Overlap;
        } else if is_end0 || is_end1 {
            return CrossType::SameEnd;
        }

        let not_same0 = a0b0a1 != a0b0b1;
        let not_same1 = a1b1a0 != a1b1b0;

        if not_same0 && not_same1 {
            CrossType::Pure
        } else {
            CrossType::NotCross
        }
    }

    pub(crate) fn random_segments(range: std::ops::Range<i32>, length: std::ops::Range<i32>, count: usize) -> Vec<RandomEdge> {
        let mut result = Vec::with_capacity(count);

        let min_sqr_length = (length.start as i64).sqr();
        let max_sqr_length = (length.end as i64).sqr();
        let sqr_length = min_sqr_length..max_sqr_length;

        for _ in 0..count {
            let mut not_find = true;
            let mut e = Self::random_edge(range.clone(), sqr_length.clone());
            while not_find {
                not_find = false;
                'outer:
                for ei in result.iter() {
                    match Self::is_cross_edge(&e, ei) {
                        CrossType::Pure | CrossType::Overlap | CrossType::Equal => {
                            not_find = true;
                            e = Self::random_edge(range.clone(), sqr_length.clone());
                            break 'outer;
                        }
                        _ => {
                            not_find = false;
                        }
                    }
                }
            }

            result.push(e);
        }

        result
    }

    fn random_edge(range: std::ops::Range<i32>, sqr_length_range: std::ops::Range<i64>) -> RandomEdge {
        let mut rng = rand::thread_rng();

        let x0 = rng.gen_range(range.clone());
        let y0 = rng.gen_range(range.clone());
        let mut x1 = rng.gen_range(range.clone());
        let mut y1 = rng.gen_range(range.clone());

        let mut edge = RandomEdge { a: IntPoint::new(x0, y0), b: IntPoint::new(x1, y1) };

        let mut sqr_length = edge.sqr_length();

        while sqr_length_range.contains(&sqr_length) {
            x1 = rng.gen_range(range.clone());
            y1 = rng.gen_range(range.clone());
            edge = RandomEdge { a: IntPoint::new(x0, y0), b: IntPoint::new(x1, y1) };
            sqr_length = edge.sqr_length()
        }

        edge
    }
}