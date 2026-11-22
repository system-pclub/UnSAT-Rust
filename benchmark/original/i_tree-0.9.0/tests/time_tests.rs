mod time;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::time::direct_solver::DirectTimeSolver;
    use crate::time::generator::TimeValueGenerator;
    use crate::time::time_interval_value::TimeIntervalValue;
    use crate::time::time_value::TimeValue;
    use crate::time::tree_solver::TreeTimeSolver;

    #[test]
    fn test_00() {
        let items = [
            TimeIntervalValue { value: 2, start: 0, end: 4 },
            TimeIntervalValue { value: 6, start: 0, end: 4 },
            TimeIntervalValue { value: 1, start: 0, end: 5 },
        ];

        let times = [
            TimeValue { value: 1, time: 2 },
            TimeValue { value: 5, time: 2 },
            TimeValue { value: 7, time: 4 }
        ];

        let result0 = DirectTimeSolver::run(&items, &times);
        let result1 = TreeTimeSolver::run(&items, &times);

        assert_eq!(result0, result1);
    }

    #[test]
    fn test_01() {
        let items = [
            TimeIntervalValue { value: 5, start: 1, end: 6 },
            TimeIntervalValue { value: 9, start: 2, end: 7 },
            TimeIntervalValue { value: 6, start: 3, end: 4 },
            TimeIntervalValue { value: 0, start: 3, end: 8 },
        ];

        let times = [
            TimeValue { value: 3, time: 3 },
            TimeValue { value: 0, time: 6 }
        ];

        let result0 = DirectTimeSolver::run(&items, &times);
        let result1 = TreeTimeSolver::run(&items, &times);

        assert_eq!(result0, result1);
    }

    #[test]
    fn test_02() {
        let items = [
            TimeIntervalValue { value: 5, start: 1, end: 7 },
            TimeIntervalValue { value: 7, start: 2, end: 6 },
            TimeIntervalValue { value: 0, start: 3, end: 6 },
            TimeIntervalValue { value: 3, start: 3, end: 5 },
            TimeIntervalValue { value: 0, start: 4, end: 5 },
        ];

        let times = [
            TimeValue { value: 7, time: 2 },
            TimeValue { value: 6, time: 4 },
            TimeValue { value: 6, time: 5 },
            TimeValue { value: 0, time: 5 },
            TimeValue { value: 3, time: 7 },
            TimeValue { value: 7, time: 7 },
        ];

        let result0 = DirectTimeSolver::run(&items, &times);
        let result1 = TreeTimeSolver::run(&items, &times);

        assert_eq!(result0, result1);
    }

    #[test]
    fn test_single_random() {
        let data = random_data(0..8, 0..8, 10);
        let result0 = DirectTimeSolver::run(&data.0, &data.1);
        let result1 = TreeTimeSolver::run(&data.0, &data.1);

        assert_eq!(result0, result1);
    }

    #[test]
    fn test_small_random() {
        for _ in 0..100000 {
            let data = random_data(0..8, 0..8, 3);
            let result0 = DirectTimeSolver::run(&data.0, &data.1);
            let result1 = TreeTimeSolver::run(&data.0, &data.1);

            assert_eq!(result0, result1);
        }
    }

    #[test]
    fn test_big_random() {
        for _ in 0..10000 {
            let data = random_data(0..100, 0..100, 100);
            let result0 = DirectTimeSolver::run(&data.0, &data.1);
            let result1 = TreeTimeSolver::run(&data.0, &data.1);

            assert_eq!(result0, result1);
        }
    }

    fn random_data(range: std::ops::Range<i32>, time: std::ops::Range<i32>, count: usize) -> (Vec<TimeIntervalValue>, Vec<TimeValue>) {
        let r_values = TimeValueGenerator::random_time_ranges(&range, &time, count);
        let t_values = TimeValueGenerator::random_time_values(&range, &time, count);

        let mut map: HashMap<i32, Vec<TimeIntervalValue>> = HashMap::new();

        for r_value in r_values {
            map.entry(r_value.value).or_insert_with(Vec::new).push(r_value);
        }

        let mut ranges = Vec::new();
        for array in map.values() {
            let mut t = -1;
            for a in array {
                if a.start < t {
                    ranges.push(a.clone());
                    t = a.end;
                }
            }
        }

        let times: HashSet<_> = t_values.into_iter().collect();
        let mut times: Vec<_> = times.into_iter().collect();
        times.sort_by(|a, b| a.time.cmp(&b.time));

        (ranges, times)
    }
}