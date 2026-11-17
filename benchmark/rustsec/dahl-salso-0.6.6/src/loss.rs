use dahl_partition::*;

use crate::clustering::{Clusterings, WorkingClustering};
use crate::optimize::CMLossComputer;
use crate::*;

// Expectation of Binder loss

pub fn binder_single_kernel(labels: &[LabelType], psm: &SquareMatrixBorrower) -> f64 {
    let ni = labels.len();
    assert_eq!(ni, psm.n_items());
    let mut sum = 0.0;
    for j in 0..ni {
        for i in 0..j {
            let p = unsafe { *psm.get_unchecked((i, j)) };
            sum += if unsafe { *labels.get_unchecked(i) == *labels.get_unchecked(j) } {
                1.0 - p
            } else {
                p
            }
        }
    }
    sum
}

pub fn binder_single(partition: &[LabelType], psm: &SquareMatrixBorrower) -> f64 {
    let nif = psm.n_items() as f64;
    binder_single_kernel(partition, psm) * 2.0 / (nif * nif)
}

pub fn binder_multiple(
    partitions: &PartitionsHolderBorrower,
    psm: &SquareMatrixBorrower,
    results: &mut [f64],
) {
    let ni = partitions.n_items();
    assert_eq!(ni, psm.n_items());
    let mut sum_p = 0.0;
    for j in 0..ni {
        for i in 0..j {
            sum_p += unsafe { *psm.get_unchecked((i, j)) };
        }
    }
    let nif = ni as f64;
    let multiplier = 2.0 / (nif * nif);
    for k in 0..partitions.n_partitions() {
        let mut sum = 0.0;
        for j in 0..ni {
            let cj = unsafe { *partitions.get_unchecked((k, j)) };
            for i in 0..j {
                if unsafe { *partitions.get_unchecked((k, i)) == cj } {
                    sum += 1.0 - 2.0 * unsafe { *psm.get_unchecked((i, j)) };
                }
            }
        }
        unsafe { *results.get_unchecked_mut(k) = multiplier * (sum + sum_p) };
    }
}

// Approximation of expectation of one minus adjusted Rand index

pub fn omariapprox_single(partition: &[LabelType], psm: &SquareMatrixBorrower) -> f64 {
    let ni = partition.len();
    assert_eq!(ni, psm.n_items());
    let mut sum_p = 0.0;
    let mut sum_ip = 0.0;
    let mut sum_i = 0.0;
    for j in 0..ni {
        let cj = unsafe { *partition.get_unchecked(j) };
        for i in 0..j {
            let p = unsafe { *psm.get_unchecked((i, j)) };
            sum_p += p;
            if unsafe { *partition.get_unchecked(i) } == cj {
                sum_ip += p;
                sum_i += 1.0;
            }
        }
    }
    let no2 = (ni * (ni - 1) / 2) as f64;
    let correc = (sum_i * sum_p) / no2;
    1.0 - (sum_ip - correc) / (0.5 * (sum_p + sum_i) - correc)
}

pub fn omariapprox_multiple(
    partitions: &PartitionsHolderBorrower,
    psm: &SquareMatrixBorrower,
    results: &mut [f64],
) {
    let ni = partitions.n_items();
    assert_eq!(ni, psm.n_items());
    let no2 = (ni * (ni - 1) / 2) as f64;
    let mut sum_p = 0.0;
    for j in 0..ni {
        for i in 0..j {
            sum_p += unsafe { *psm.get_unchecked((i, j)) };
        }
    }
    for k in 0..partitions.n_partitions() {
        let mut sum_ip = 0.0;
        let mut sum_i = 0.0;
        for j in 0..ni {
            let cj = unsafe { *partitions.get_unchecked((k, j)) };
            for i in 0..j {
                if unsafe { *partitions.get_unchecked((k, i)) } == cj {
                    sum_ip += unsafe { *psm.get_unchecked((i, j)) };
                    sum_i += 1.0;
                }
            }
        }
        let correc = (sum_i * sum_p) / no2;
        let omariapprox = 1.0 - (sum_ip - correc) / (0.5 * (sum_p + sum_i) - correc);
        unsafe { *results.get_unchecked_mut(k) = omariapprox };
    }
}

// Lower bound of the expectation of the variation of information

pub fn vilb_expected_loss_constant(psm: &SquareMatrixBorrower) -> f64 {
    let ni = psm.n_items();
    let mut s1: f64 = 0.0;
    for i in 0..ni {
        let mut s2: f64 = 0.0;
        for j in 0..ni {
            s2 += unsafe { *psm.get_unchecked((i, j)) };
        }
        s1 += s2.log2();
    }
    s1
}

pub fn vilb_single_kernel(partition: &[LabelType], psm: &SquareMatrixBorrower) -> f64 {
    let ni = partition.len();
    assert_eq!(ni, psm.n_items());
    let mut sum = 0.0;
    for i in 0..ni {
        let mut s1 = 0u32;
        let mut s2 = 0.0;
        for j in 0..ni {
            if unsafe { *partition.get_unchecked(i) == *partition.get_unchecked(j) } {
                s1 += 1;
                s2 += unsafe { *psm.get_unchecked((i, j)) };
            }
        }
        sum += (s1 as f64).log2() - 2.0 * s2.log2();
    }
    sum
}

pub fn vilb_single_kernel_for_partial_partition(
    partition: &Partition,
    psm: &SquareMatrixBorrower,
) -> f64 {
    let labels = partition.labels();
    let ni = partition.n_items();
    assert_eq!(ni, psm.n_items());
    let mut sum = 0.0;
    for i in 0..ni {
        if labels[i].is_none() {
            continue;
        }
        let mut s1 = 0u32;
        let mut s2 = 0.0;
        for (j, item) in labels.iter().enumerate() {
            if item.is_none() {
                continue;
            }
            if partition.label_of(i) == partition.label_of(j) {
                s1 += 1;
                s2 += unsafe { *psm.get_unchecked((i, j)) };
            }
        }
        sum += (s1 as f64).log2() - 2.0 * s2.log2();
    }
    sum
}

pub fn vilb_single(partition: &[LabelType], psm: &SquareMatrixBorrower) -> f64 {
    (vilb_single_kernel(partition, psm) + vilb_expected_loss_constant(psm)) / (psm.n_items() as f64)
}

pub fn vilb_multiple(
    partitions: &PartitionsHolderBorrower,
    psm: &SquareMatrixBorrower,
    results: &mut [f64],
) {
    let ni = partitions.n_items();
    assert_eq!(ni, psm.n_items());
    let constant = vilb_expected_loss_constant(psm);
    for k in 0..partitions.n_partitions() {
        let mut sum = constant;
        for i in 0..ni {
            let mut s1 = 0u32;
            let mut s3 = 0.0;
            for j in 0..ni {
                if unsafe { *partitions.get_unchecked((k, i)) == *partitions.get_unchecked((k, j)) }
                {
                    s1 += 1;
                    s3 += unsafe { *psm.get_unchecked((i, j)) };
                }
            }
            sum += (s1 as f64).log2() - 2.0 * s3.log2();
        }
        let vilb = sum / (psm.n_items() as f64);
        unsafe { *results.get_unchecked_mut(k) = vilb };
    }
}

// General computation of expected loss for losses based on confusion matrices.

pub fn compute_loss_multiple<'a, T: CMLossComputer>(
    loss_computer_factory: Box<dyn Fn() -> T + 'a>,
    partitions: &PartitionsHolderBorrower,
    draws: &PartitionsHolderBorrower,
    results: &mut [f64],
) {
    let n_items = partitions.n_items();
    assert_eq!(n_items, draws.n_items());
    let clusterings = Clusterings::from_i32_column_major_order(partitions.data(), n_items);
    let draws = Clusterings::from_i32_column_major_order(draws.data(), n_items);
    for k in 0..clusterings.n_clusterings() {
        let state = WorkingClustering::from_slice(clusterings.labels(k), clusterings.n_clusters(k));
        let cms = draws.make_confusion_matrices(&state);
        let mut loss_computer = loss_computer_factory();
        loss_computer.initialize(&state, &cms);
        unsafe { *results.get_unchecked_mut(k) = loss_computer.compute_loss(&state, &cms) };
    }
}

#[cfg(test)]
mod tests_loss {
    use super::*;
    use crate::log2cache::Log2Cache;
    use crate::optimize::{OMARICMLossComputer, VICMLossComputer};

    #[test]
    fn test_computations() {
        let n_items = 5;
        let mut samples = PartitionsHolder::new(n_items);
        for labels in Partition::iter(n_items) {
            samples.push_partition(&Partition::from(&labels[..]));
        }
        let n_partitions = samples.n_partitions();
        let mut psm = crate::psm::psm(&samples.view(), 2);
        let samples_view = &samples.view();
        let psm_view = &psm.view();
        let mut results = vec![0.0; n_partitions];
        binder_multiple(samples_view, psm_view, &mut results[..]);
        for k in 0..n_partitions {
            let part: Vec<LabelType> = samples_view
                .get(k)
                .labels_via_copying()
                .iter()
                .map(|x| *x as LabelType)
                .collect();
            assert_relative_eq!(binder_single(&part[..], psm_view), results[k]);
        }
        compute_loss_multiple(
            Box::new(|| OMARICMLossComputer::new(n_partitions)),
            samples_view,
            samples_view,
            &mut results[..],
        );
        omariapprox_multiple(samples_view, psm_view, &mut results[..]);
        for k in 0..n_partitions {
            let part: Vec<LabelType> = samples_view
                .get(k)
                .labels_via_copying()
                .iter()
                .map(|x| *x as LabelType)
                .collect();
            assert_relative_eq!(omariapprox_single(&part[..], psm_view), results[k]);
        }
        let cache = Log2Cache::new(n_items);
        compute_loss_multiple(
            Box::new(|| VICMLossComputer::new(2.0, &cache)),
            samples_view,
            samples_view,
            &mut results[..],
        );
        vilb_multiple(samples_view, psm_view, &mut results[..]);
        for k in 0..n_partitions {
            let part: Vec<LabelType> = samples_view
                .get(k)
                .labels_via_copying()
                .iter()
                .map(|x| *x as LabelType)
                .collect();
            assert_ulps_eq!(vilb_single(&part[..], psm_view), results[k]);
        }
        for k in 1..n_partitions {
            let part: Vec<LabelType> = samples_view
                .get(k)
                .labels_via_copying()
                .iter()
                .map(|x| *x as LabelType)
                .collect();
            let part0: Vec<LabelType> = samples_view
                .get(k - 1)
                .labels_via_copying()
                .iter()
                .map(|x| *x as LabelType)
                .collect();
            assert_ulps_eq!(
                ((1.0 / (n_items as f64))
                    * (vilb_single_kernel(&part[..], psm_view)
                        - vilb_single_kernel(&part0[..], psm_view))) as f32,
                (results[k] - results[k - 1]) as f32,
            );
        }
    }
}
