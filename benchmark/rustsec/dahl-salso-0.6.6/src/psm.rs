use dahl_partition::*;
use std::slice;

pub fn psm(partitions: &PartitionsHolderBorrower, n_cores: u32) -> SquareMatrix {
    let mut psm = SquareMatrix::zeros(partitions.n_items());
    psm_engine(
        partitions.n_partitions(),
        partitions.n_items(),
        n_cores,
        partitions,
        &mut psm.view(),
    );
    psm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psm() {
        let mut partitions = PartitionsHolder::new(4);
        partitions.push_partition(&Partition::from("AABB".as_bytes()));
        partitions.push_partition(&Partition::from("AAAB".as_bytes()));
        partitions.push_partition(&Partition::from("ABBB".as_bytes()));
        partitions.push_partition(&Partition::from("AAAB".as_bytes()));
        let partitions_view = partitions.view();
        let psm1 = psm(&partitions_view, 2);
        assert_eq!(format!("{:?}", psm1.data()), "[1.0, 0.75, 0.5, 0.0, 0.75, 1.0, 0.75, 0.25, 0.5, 0.75, 1.0, 0.5, 0.0, 0.25, 0.5, 1.0]");
        let psm2 = psm(&partitions_view, 1);
        assert_eq!(format!("{:?}", psm2.data()), "[1.0, 0.75, 0.5, 0.0, 0.75, 1.0, 0.75, 0.25, 0.5, 0.75, 1.0, 0.5, 0.0, 0.25, 0.5, 1.0]");
    }
}

pub fn psm_engine(
    n_partitions: usize,
    n_items: usize,
    n_cores: u32,
    partitions: &PartitionsHolderBorrower,
    psm: &mut SquareMatrixBorrower,
) {
    if n_cores == 1 {
        engine2(n_partitions, n_items, None, partitions, psm);
    } else {
        let n_cores = if n_cores == 0 {
            num_cpus::get()
        } else {
            n_cores as usize
        };
        let n_pairs = n_items * (n_items - 1) / 2;
        let step_size = n_pairs / n_cores + 1;
        let mut s = 0usize;
        let mut plan = Vec::with_capacity(n_cores + 1);
        plan.push(0);
        for i in 0..n_items {
            if s > step_size {
                plan.push(i);
                s = 0;
            }
            s += i;
        }
        while plan.len() < n_cores + 1 {
            plan.push(n_items);
        }
        crossbeam::scope(|s| {
            for i in 0..n_cores {
                let ptr = unsafe {
                    slice::from_raw_parts_mut(psm.data_mut().as_mut_ptr(), psm.data().len())
                };
                let lower = plan[i];
                let upper = plan[i + 1];
                s.spawn(move |_| {
                    let psm2 = &mut SquareMatrixBorrower::from_slice(ptr, n_items);
                    engine2(n_partitions, n_items, Some(lower..upper), partitions, psm2);
                });
            }
        })
        .unwrap();
    }
}

fn engine2(
    n_partitions: usize,
    n_items: usize,
    range: Option<std::ops::Range<usize>>,
    partitions: &PartitionsHolderBorrower,
    psm: &mut SquareMatrixBorrower,
) {
    let npf = n_partitions as f64;
    let indices = range.unwrap_or(0..n_items);
    for j in indices {
        for i in 0..j {
            let mut count = 0usize;
            for k in 0..n_partitions {
                unsafe {
                    if partitions.get_unchecked((k, i)) == partitions.get_unchecked((k, j)) {
                        count += 1;
                    }
                }
            }
            let proportion = count as f64 / npf;
            unsafe {
                *psm.get_unchecked_mut((i, j)) = proportion;
                *psm.get_unchecked_mut((j, i)) = proportion;
            }
        }
        unsafe {
            *psm.get_unchecked_mut((j, j)) = 1.0;
        }
    }
}

