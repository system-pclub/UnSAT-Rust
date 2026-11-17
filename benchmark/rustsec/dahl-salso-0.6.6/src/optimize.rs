use crate::clustering::{Clusterings, WorkingClustering};
use crate::log2cache::Log2Cache;
use crate::loss::*;
use crate::*;
use dahl_partition::*;
use ndarray::{Array2, Array3, Axis};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::sync::mpsc;
use std::time::Instant;

// **************************************************************
// Implementation of SALSO for losses based on confusion matrices
// **************************************************************

pub trait CMLossComputer {
    #[allow(unused_variables)]
    fn initialize(&mut self, state: &WorkingClustering, cms: &Array3<CountType>) {}

    fn compute_loss(&self, state: &WorkingClustering, cms: &Array3<CountType>) -> f64;

    fn change_in_loss(
        &self,
        item_index: usize,
        to_label: LabelType,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) -> f64;

    #[allow(unused_variables)]
    fn decision_callback(
        &mut self,
        item_index: usize,
        to_label_option: Option<LabelType>,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) {
    }
}

// Expectation of the Binder loss

pub struct BinderCMLossComputer {
    a: f64,
}

impl BinderCMLossComputer {
    pub fn new(a: f64) -> Self {
        Self { a }
    }

    pub fn n_squared(x: CountType) -> f64 {
        let x = x as f64;
        x * x
    }
}

impl CMLossComputer for BinderCMLossComputer {
    fn compute_loss(&self, state: &WorkingClustering, cms: &Array3<CountType>) -> f64 {
        let n_draws = cms.len_of(Axis(2));
        let sum2 = state
            .occupied_clusters()
            .iter()
            .map(|i| BinderCMLossComputer::n_squared(state.size_of(*i)))
            .sum::<f64>()
            * n_draws as f64;
        let mut sum1 = 0.0;
        let mut sum3 = 0.0;
        for draw_index in 0..n_draws {
            for other_index in 0..cms.len_of(Axis(1)) {
                let n = cms[(0, other_index, draw_index)];
                if n > 0 {
                    sum1 += BinderCMLossComputer::n_squared(cms[(0, other_index, draw_index)]);
                    for main_label in state.occupied_clusters().iter() {
                        sum3 += BinderCMLossComputer::n_squared(
                            cms[(*main_label as usize + 1, other_index, draw_index)],
                        );
                    }
                }
            }
        }
        (self.a * sum1 + (2.0 - self.a) * sum2 - 2.0 * sum3)
            / (n_draws as f64 * BinderCMLossComputer::n_squared(state.n_items()))
    }

    fn change_in_loss(
        &self,
        item_index: usize,
        to_label: LabelType,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) -> f64 {
        let offset = if from_label_option.is_some() && to_label == from_label_option.unwrap() {
            1
        } else {
            0
        };
        let n_draws = cms.len_of(Axis(2));
        let sum2 = (n_draws as f64) * ((state.size_of(to_label) - offset) as f64);
        let to_index = to_label as usize + 1;
        let mut sum3 = 0.0;
        for draw_index in 0..n_draws {
            let other_index = draws.label(draw_index, item_index) as usize;
            sum3 += (cms[(to_index, other_index, draw_index)] - offset) as f64;
        }
        (2.0 - self.a) * sum2 - 2.0 * sum3
    }
}

// Expectation of one minus the adjusted Rand index loss

pub struct OMARICMLossComputer {
    n: CountType,
    sum: f64,
    sums: Array2<f64>,
}

impl OMARICMLossComputer {
    pub fn new(n_draws: usize) -> Self {
        Self {
            n: 0,
            sum: 0.0,
            sums: Array2::<f64>::zeros((n_draws, 2)),
        }
    }

    pub fn n_choose_2_times_2(x: CountType) -> f64 {
        let x = x as f64;
        x * (x - 1.0)
    }
}

impl CMLossComputer for OMARICMLossComputer {
    fn initialize(&mut self, state: &WorkingClustering, cms: &Array3<CountType>) {
        self.n = state.n_items();
        self.sum = state
            .occupied_clusters()
            .iter()
            .map(|i| OMARICMLossComputer::n_choose_2_times_2(state.size_of(*i)))
            .sum();
        let n_draws = cms.len_of(Axis(2));
        for draw_index in 0..n_draws {
            for other_index in 0..cms.len_of(Axis(1)) {
                let n = cms[(0, other_index, draw_index)];
                if n > 0 {
                    self.sums[(draw_index, 0)] += OMARICMLossComputer::n_choose_2_times_2(n);
                    for main_label in state.occupied_clusters().iter() {
                        self.sums[(draw_index, 1)] += OMARICMLossComputer::n_choose_2_times_2(
                            cms[(*main_label as usize + 1, other_index, draw_index)],
                        );
                    }
                }
            }
        }
    }

    fn compute_loss(&self, _state: &WorkingClustering, _cms: &Array3<CountType>) -> f64 {
        let mut sum = 0.0;
        let sum2 = self.sum;
        let sum2_div_denom = sum2 / OMARICMLossComputer::n_choose_2_times_2(self.n);
        let n_draws = self.sums.len_of(Axis(0));
        for draw_index in 0..n_draws {
            let sum1 = self.sums[(draw_index, 0)];
            let offset = sum1 * sum2_div_denom;
            let denominator = 0.5 * (sum1 + sum2) - offset;
            if denominator > 0.0 {
                let numerator = self.sums[(draw_index, 1)] - offset;
                sum += numerator / denominator;
            }
        }
        1.0 - sum / (n_draws as f64)
    }

    fn change_in_loss(
        &self,
        item_index: usize,
        to_label: LabelType,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) -> f64 {
        let offset = if from_label_option.is_some() && to_label == from_label_option.unwrap() {
            1
        } else {
            0
        };
        let mut sum2 = if offset == 0 {
            self.sum + 2.0 * state.size_of(to_label) as f64
        } else {
            self.sum
        };
        let to_index = to_label as usize + 1;
        let mut n = self.n;
        let from_index = match from_label_option {
            Some(from_label) => {
                if offset == 0 {
                    sum2 -= 2.0 * (state.size_of(from_label) - 1) as f64;
                }
                from_label as usize + 1
            }
            None => {
                n += 1;
                0
            }
        };
        let sum2_div_denom = sum2 / OMARICMLossComputer::n_choose_2_times_2(n);
        let mut sum = 0.0;
        let n_draws = self.sums.len_of(Axis(0));
        for draw_index in 0..n_draws {
            let other_index = draws.label(draw_index, item_index) as usize;
            let sum1 = self.sums[(draw_index, 0)]
                + if from_label_option.is_none() {
                    2.0 * n as f64
                } else {
                    0.0
                };
            let chance_adjustment = sum1 * sum2_div_denom;
            let denominator = 0.5 * (sum1 + sum2) - chance_adjustment;
            if denominator > 0.0 {
                let mut unadjusted_numerator = self.sums[(draw_index, 1)];
                if offset == 0 {
                    unadjusted_numerator += 2.0 * cms[(to_index, other_index, draw_index)] as f64;
                    if from_label_option.is_some() {
                        unadjusted_numerator -=
                            2.0 * (cms[(from_index, other_index, draw_index)] - 1) as f64;
                    }
                }
                let numerator = unadjusted_numerator - chance_adjustment;
                sum += numerator / denominator;
            }
        }
        -sum
    }

    fn decision_callback(
        &mut self,
        item_index: usize,
        to_label_option: Option<LabelType>,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) {
        let to_index = match to_label_option {
            Some(to_label) => {
                self.sum += 2.0 * state.size_of(to_label) as f64;
                to_label as usize + 1
            }
            None => {
                self.n -= 1;
                0
            }
        };
        let from_index = match from_label_option {
            Some(from_label) => {
                self.sum -= 2.0 * (state.size_of(from_label) - 1) as f64;
                from_label as usize + 1
            }
            None => {
                self.n += 1;
                0
            }
        };
        let n_draws = draws.n_clusterings();
        for draw_index in 0..n_draws {
            let other_index = draws.label(draw_index, item_index) as usize;
            if from_label_option.is_some() {
                self.sums[(draw_index, 1)] -=
                    2.0 * (cms[(from_index, other_index, draw_index)] - 1) as f64;
            } else {
                self.sums[(draw_index, 0)] += 2.0 * cms[(0, other_index, draw_index)] as f64;
            }
            if to_label_option.is_some() {
                self.sums[(draw_index, 1)] += 2.0 * cms[(to_index, other_index, draw_index)] as f64;
            } else {
                self.sums[(draw_index, 0)] -= 2.0 * (cms[(0, other_index, draw_index)] - 1) as f64;
            }
        }
    }
}

// Expectation of variation of information loss

pub struct VICMLossComputer<'a> {
    a: f64,
    cache: &'a Log2Cache,
}

impl<'a> VICMLossComputer<'a> {
    pub fn new(a: f64, cache: &'a Log2Cache) -> Self {
        Self { a, cache }
    }
}

impl<'a> CMLossComputer for VICMLossComputer<'a> {
    fn compute_loss(&self, state: &WorkingClustering, cms: &Array3<CountType>) -> f64 {
        let sum2: f64 = state
            .occupied_clusters()
            .iter()
            .map(|i| self.cache.nlog2n(state.size_of(*i)))
            .sum();
        let n_draws = cms.len_of(Axis(2));
        let mut sum = 0.0;
        for draw_index in 0..n_draws {
            let mut sum1 = 0.0;
            let mut sum3 = 0.0;
            for other_index in 0..cms.len_of(Axis(1)) {
                let n = cms[(0, other_index, draw_index)];
                if n > 0 {
                    sum1 += self.cache.nlog2n(cms[(0, other_index, draw_index)]);
                    for main_label in state.occupied_clusters().iter() {
                        sum3 += self
                            .cache
                            .nlog2n(cms[(*main_label as usize + 1, other_index, draw_index)]);
                    }
                }
            }
            sum += self.a * sum1 - 2.0 * sum3;
        }
        ((2.0 - self.a) * sum2 + (sum / (n_draws as f64))) / (state.n_items() as f64)
    }

    fn change_in_loss(
        &self,
        item_index: usize,
        to_label: LabelType,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) -> f64 {
        let offset = if from_label_option.is_some() && to_label == from_label_option.unwrap() {
            1
        } else {
            0
        };
        let n_draws = cms.len_of(Axis(2));
        let sum2 = (n_draws as f64)
            * self
                .cache
                .nlog2n_difference(state.size_of(to_label) - offset);
        let to_index = to_label as usize + 1;
        let mut sum3 = 0.0;
        for draw_index in 0..n_draws {
            let other_index = draws.label(draw_index, item_index) as usize;
            sum3 += self
                .cache
                .nlog2n_difference(cms[(to_index, other_index, draw_index)] - offset);
        }
        (2.0 - self.a) * sum2 - 2.0 * sum3
    }
}

// Expectation of general information based loss

pub struct GeneralInformationBasedCMLossComputer<'a, T>
where
    T: InformationBasedLoss,
{
    cache: &'a Log2Cache,
    n: CountType,
    sum: f64,
    sums: Array2<f64>,
    ibloss: T,
}

impl<'a, T> GeneralInformationBasedCMLossComputer<'a, T>
where
    T: InformationBasedLoss,
{
    pub fn new(n_draws: usize, cache: &'a Log2Cache, ibloss: T) -> Self {
        Self {
            cache,
            n: 0,
            sum: 0.0,
            sums: Array2::<f64>::zeros((n_draws, 2)),
            ibloss,
        }
    }
}

impl<'a, T> CMLossComputer for GeneralInformationBasedCMLossComputer<'a, T>
where
    T: InformationBasedLoss,
{
    fn initialize(&mut self, state: &WorkingClustering, cms: &Array3<CountType>) {
        self.n = state.n_items();
        self.sum = state
            .occupied_clusters()
            .iter()
            .map(|i| self.cache.nlog2n(state.size_of(*i)))
            .sum();
        let n_draws = cms.len_of(Axis(2));
        for draw_index in 0..n_draws {
            for other_index in 0..cms.len_of(Axis(1)) {
                let n = cms[(0, other_index, draw_index)];
                if n > 0 {
                    self.sums[(draw_index, 0)] +=
                        self.cache.nlog2n(cms[(0, other_index, draw_index)]);
                    for main_label in state.occupied_clusters().iter() {
                        self.sums[(draw_index, 1)] += self
                            .cache
                            .nlog2n(cms[(*main_label as usize + 1, other_index, draw_index)]);
                    }
                }
            }
        }
    }

    fn compute_loss(&self, _state: &WorkingClustering, _cms: &Array3<CountType>) -> f64 {
        let ni = self.n as f64;
        let nlog2n = ni * ni.log2();
        let u = self.sum;
        let mut sum = 0.0;
        let n_draws = self.sums.len_of(Axis(0));
        for draw_index in 0..n_draws {
            let v = self.sums[(draw_index, 0)];
            let uv = self.sums[(draw_index, 1)];
            sum += self.ibloss.compute_kernel(u, v, uv, nlog2n);
        }
        self.ibloss.finalize(sum, n_draws as f64, ni)
    }

    fn change_in_loss(
        &self,
        item_index: usize,
        to_label: LabelType,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) -> f64 {
        let offset = if from_label_option.is_some() && to_label == from_label_option.unwrap() {
            1
        } else {
            0
        };
        let mut v = if offset == 0 {
            self.sum + self.cache.nlog2n_difference(state.size_of(to_label))
        } else {
            self.sum
        };
        let to_index = to_label as usize + 1;
        let (n, from_index) = match from_label_option {
            Some(from_label) => {
                if offset == 0 {
                    v -= self.cache.nlog2n_difference(state.size_of(from_label) - 1);
                }
                (self.n, from_label as usize + 1)
            }
            None => (self.n + 1, 0),
        };
        let ni = n as f64;
        let nlog2n = ni * ni.log2();
        let mut sum = 0.0;
        let n_draws = self.sums.len_of(Axis(0));
        for draw_index in 0..n_draws {
            let other_index = draws.label(draw_index, item_index) as usize;
            let u = self.sums[(draw_index, 0)]
                + if from_label_option.is_none() {
                    self.cache.nlog2n_difference(n)
                } else {
                    0.0
                };
            let mut uv = self.sums[(draw_index, 1)];
            if offset == 0 {
                uv += self
                    .cache
                    .nlog2n_difference(cms[(to_index, other_index, draw_index)]);
                if from_label_option.is_some() {
                    uv -= self
                        .cache
                        .nlog2n_difference(cms[(from_index, other_index, draw_index)] - 1)
                }
            }
            sum += self.ibloss.compute_kernel(u, v, uv, nlog2n);
        }
        self.ibloss.score(sum)
    }

    fn decision_callback(
        &mut self,
        item_index: usize,
        to_label_option: Option<LabelType>,
        from_label_option: Option<LabelType>,
        state: &WorkingClustering,
        cms: &Array3<CountType>,
        draws: &Clusterings,
    ) {
        let to_index = match to_label_option {
            Some(to_label) => {
                self.sum += self.cache.nlog2n_difference(state.size_of(to_label));
                to_label as usize + 1
            }
            None => {
                self.n -= 1;
                0
            }
        };
        let from_index = match from_label_option {
            Some(from_label) => {
                self.sum -= self.cache.nlog2n_difference(state.size_of(from_label) - 1);
                from_label as usize + 1
            }
            None => {
                self.n += 1;
                0
            }
        };
        let n_draws = draws.n_clusterings();
        for draw_index in 0..n_draws {
            let other_index = draws.label(draw_index, item_index) as usize;
            if from_label_option.is_some() {
                self.sums[(draw_index, 1)] -= self
                    .cache
                    .nlog2n_difference(cms[(from_index, other_index, draw_index)] - 1);
            } else {
                self.sums[(draw_index, 0)] += self
                    .cache
                    .nlog2n_difference(cms[(0, other_index, draw_index)]);
            }
            if to_label_option.is_some() {
                self.sums[(draw_index, 1)] += self
                    .cache
                    .nlog2n_difference(cms[(to_index, other_index, draw_index)]);
            } else {
                self.sums[(draw_index, 0)] -= self
                    .cache
                    .nlog2n_difference(cms[(0, other_index, draw_index)] - 1);
            }
        }
    }
}

pub trait InformationBasedLoss {
    fn compute_kernel(&self, u: f64, v: f64, uv: f64, nlog2n: f64) -> f64;
    fn finalize(&self, sum: f64, n_draws: f64, n_items: f64) -> f64;
    fn score(&self, sum: f64) -> f64;
}

// Expectation of normalized variation of information loss

pub struct NVIInformationBasedLoss;

impl InformationBasedLoss for NVIInformationBasedLoss {
    fn compute_kernel(&self, u: f64, v: f64, uv: f64, nlog2n: f64) -> f64 {
        (u + v - 2.0 * uv) / (nlog2n - uv)
    }
    fn finalize(&self, sum: f64, n_draws: f64, _n_items: f64) -> f64 {
        sum / n_draws
    }
    fn score(&self, sum: f64) -> f64 {
        sum
    }
}

// Expectation of information distance loss

pub struct IDInformationBasedLoss;

impl InformationBasedLoss for IDInformationBasedLoss {
    fn compute_kernel(&self, u: f64, v: f64, uv: f64, _nlog2n: f64) -> f64 {
        u + v - uv - u.min(v)
    }
    fn finalize(&self, sum: f64, n_draws: f64, n_items: f64) -> f64 {
        sum / (n_draws * n_items)
    }
    fn score(&self, sum: f64) -> f64 {
        sum
    }
}

// Expectation of normalized information distance loss

pub struct NIDInformationBasedLoss;

impl InformationBasedLoss for NIDInformationBasedLoss {
    fn compute_kernel(&self, u: f64, v: f64, uv: f64, nlog2n: f64) -> f64 {
        (nlog2n + uv - u - v) / (nlog2n - u.min(v))
    }
    fn finalize(&self, sum: f64, n_draws: f64, _n_items: f64) -> f64 {
        1.0 - sum / n_draws
    }
    fn score(&self, sum: f64) -> f64 {
        -sum
    }
}

// Common

fn find_label_of_minimum<I: Iterator<Item = (LabelType, f64)>>(pairs: I) -> LabelType {
    let mut s0 = f64::INFINITY;
    let mut l0 = 0;
    for pair in pairs {
        if pair.1 < s0 {
            s0 = pair.1;
            l0 = pair.0;
        }
    }
    l0
}

fn allocation_scan<T: CMLossComputer>(
    sweetening_scan: bool,
    singletons_initialization: bool,
    state: &mut WorkingClustering,
    cms: &mut Array3<CountType>,
    permutation: &[usize],
    loss_computer: &mut T,
    draws: &Clusterings,
) -> bool {
    let mut state_changed = false;
    let mut chosen_label_option = None;
    for item_index in permutation {
        let item_index = *item_index;
        let label_of_empty_cluster = state.label_of_empty_cluster();
        let from_label_option = match sweetening_scan {
            true => Some(state.get(item_index)),
            false => None,
        };
        let to_label = match label_of_empty_cluster {
            Some(loec) if !sweetening_scan && singletons_initialization => loec,
            Some(_) | None => {
                let iter = state
                    .occupied_clusters()
                    .iter()
                    .chain(label_of_empty_cluster.iter())
                    .map(|to_label| {
                        (
                            *to_label,
                            loss_computer.change_in_loss(
                                item_index,
                                *to_label,
                                from_label_option,
                                state,
                                cms,
                                draws,
                            ),
                        )
                    });
                find_label_of_minimum(iter)
            }
        };
        if !sweetening_scan {
            state.assign(item_index, to_label, loss_computer, cms, draws);
            if chosen_label_option.is_none() {
                chosen_label_option = Some(to_label);
                state_changed = state.size_of(to_label) != 1;
            }
            if to_label != chosen_label_option.unwrap() {
                state_changed = true;
            }
        } else if to_label != from_label_option.unwrap() {
            state.reassign(item_index, to_label, loss_computer, cms, draws);
            state_changed = true;
        }
    }
    state_changed
}

#[allow(clippy::too_many_arguments)]
fn sweetening_scans<T: CMLossComputer, U: Rng>(
    state: &mut WorkingClustering,
    cms: &mut Array3<CountType>,
    permutation: &mut Vec<usize>,
    loss_computer: &mut T,
    scan_counter: &mut u32,
    draws: &Clusterings,
    p: &SALSOParameters,
    rng: &mut U,
) -> bool {
    let mut global_state_changed = false;
    let mut state_changed = true;
    while state_changed && *scan_counter < p.max_scans {
        *scan_counter += 1;
        permutation.shuffle(rng);
        state_changed = allocation_scan(true, false, state, cms, permutation, loss_computer, draws);
        global_state_changed = global_state_changed || state_changed;
    }
    global_state_changed
}

pub fn minimize_once_by_salso_v2<'a, T: CMLossComputer, U: Rng>(
    loss_computer_factory: Box<dyn Fn() -> T + 'a>,
    draws: &Clusterings,
    p: &SALSOParameters,
    seconds_target: f64,
    rng: &mut U,
) -> SALSOResults {
    let n_items = draws.n_items();
    let max_size = match p.max_size {
        0 => draws.max_clusters(),
        _ => p.max_size,
    };
    let mut permutation: Vec<usize> = (0..p.n_items).collect();
    let mut best = SALSOResults::dummy(max_size);
    let start_time = Instant::now();
    let uniform_0_1 = Uniform::from(0.0..1.0);
    for run_counter in 1..=p.n_runs {
        let mut loss_computer = loss_computer_factory();
        let singletons_initialization = uniform_0_1.sample(rng) < p.prob_singletons_initialization;
        let (mut state, mut cms, initialization_method) =
            if uniform_0_1.sample(rng) < p.prob_sequential_allocation {
                let mut state = WorkingClustering::empty(n_items, max_size);
                let mut cms = Array3::<CountType>::zeros((
                    max_size as usize + 1,
                    draws.max_clusters() as usize,
                    draws.n_clusterings(),
                ));
                // Sequential allocation
                permutation.shuffle(rng);
                allocation_scan(
                    false,
                    singletons_initialization,
                    &mut state,
                    &mut cms,
                    &permutation,
                    &mut loss_computer,
                    draws,
                );
                let initialization_method = if singletons_initialization {
                    InitializationMethod::SequentialFromSingletons
                } else {
                    InitializationMethod::SequentialFromEmpty
                };
                (state, cms, initialization_method)
            } else {
                let state = if p.max_size_as_rf {
                    WorkingClustering::random_as_rf(n_items, max_size, draws.max_clusters(), rng)
                } else {
                    WorkingClustering::random(n_items, max_size, rng)
                };
                let cms = draws.make_confusion_matrices(&state);
                loss_computer.initialize(&state, &cms);
                (
                    state,
                    cms,
                    InitializationMethod::SampleOne2MaxWithReplacement,
                )
            };
        let mut scan_counter = 0;
        sweetening_scans(
            &mut state,
            &mut cms,
            &mut permutation,
            &mut loss_computer,
            &mut scan_counter,
            draws,
            p,
            rng,
        );
        let mut expected_loss = loss_computer.compute_loss(&state, &cms);
        let (mut n_zealous_accepts, mut n_zealous_attempts) = (0, 0);
        if n_zealous_attempts < p.max_zealous_updates {
            let labels = {
                let mut x = state.occupied_clusters().clone();
                x.shuffle(rng);
                x
            };
            for label in labels {
                let s = state.size_of(label) as usize;
                if s <= 1 {
                    // Already covered by sweetening scans.
                    continue;
                }
                n_zealous_attempts += 1;
                if n_zealous_attempts > p.max_zealous_updates {
                    break;
                }
                let mut active_items = Vec::with_capacity(s);
                for item_index in 0..(state.n_items() as usize) {
                    if state.get(item_index) == label {
                        state.remove(item_index, &mut loss_computer, &mut cms, draws);
                        active_items.push(item_index)
                    }
                    if active_items.len() == s {
                        break;
                    }
                }
                active_items.shuffle(rng);
                let state_changed = allocation_scan(
                    false,
                    singletons_initialization,
                    &mut state,
                    &mut cms,
                    &active_items,
                    &mut loss_computer,
                    draws,
                );
                if state_changed {
                    let labels_for_undo = state.clone_labels();
                    let state_changed = sweetening_scans(
                        &mut state,
                        &mut cms,
                        &mut permutation,
                        &mut loss_computer,
                        &mut scan_counter,
                        draws,
                        p,
                        rng,
                    );
                    let expected_loss_of_candidate = loss_computer.compute_loss(&state, &cms);
                    if expected_loss_of_candidate < expected_loss {
                        // Keep changes
                        expected_loss = expected_loss_of_candidate;
                        n_zealous_accepts += 1;
                    } else {
                        // Undo changes
                        if state_changed {
                            for (item_index, new_label) in labels_for_undo.iter().enumerate() {
                                state.reassign(
                                    item_index,
                                    *new_label,
                                    &mut loss_computer,
                                    &mut cms,
                                    draws,
                                )
                            }
                        }
                        for item_index in active_items {
                            state.reassign(item_index, label, &mut loss_computer, &mut cms, draws)
                        }
                    }
                }
            }
        }
        // Tidy up
        if expected_loss < best.expected_loss {
            let clustering = state.standardize().iter().map(|x| *x as usize).collect();
            best = SALSOResults {
                clustering,
                expected_loss,
                n_scans: scan_counter,
                n_zealous_accepts,
                n_zealous_attempts,
                initialization_method,
                max_size: state.max_clusters(),
                ..best
            }
        }
        if seconds_target.is_finite() {
            let seconds = start_time.elapsed().as_secs_f64();
            let rc = run_counter as f64;
            let seconds_per_run = seconds / rc;
            let projected_seconds_if_another = (rc + 1.0) * seconds_per_run;
            if projected_seconds_if_another >= seconds_target {
                let prob_stop = (projected_seconds_if_another - seconds_target) / seconds_per_run;
                if uniform_0_1.sample(rng) < prob_stop {
                    return SALSOResults {
                        n_runs: run_counter,
                        seconds,
                        ..best
                    };
                }
            }
        }
    }
    SALSOResults {
        n_runs: p.n_runs,
        seconds: start_time.elapsed().as_secs_f64(),
        ..best
    }
}

// ******************************************
// Implementation of SALSO for general losses
// ******************************************

pub trait GeneralLossComputer {
    fn expected_loss_kernel(&self) -> f64;
    fn speculative_add(&mut self, partition: &Partition, i: usize, subset_index: LabelType) -> f64;
    fn new_subset(&mut self, partition: &mut Partition);
    fn add_with_index(&mut self, partition: &mut Partition, i: usize, subset_index: LabelType);
    fn remove(&mut self, partition: &mut Partition, i: usize) -> LabelType;
}

// Expectation of the Binder loss

#[derive(Debug)]
struct BinderSubsetCalculations {
    committed_loss: f64,
    speculative_loss: f64,
}

pub struct BinderGLossComputer<'a> {
    subsets: Vec<BinderSubsetCalculations>,
    psm: &'a SquareMatrixBorrower<'a>,
}

impl<'a> BinderGLossComputer<'a> {
    pub fn new(psm: &'a SquareMatrixBorrower<'a>) -> Self {
        Self {
            subsets: Vec::new(),
            psm,
        }
    }
    fn expected_loss_from_kernel(psm: &'a SquareMatrixBorrower<'a>, kernel: f64) -> f64 {
        let nif = psm.n_items() as f64;
        (2.0 * kernel + psm.sum_of_triangle()) * 2.0 / (nif * nif)
    }
}

impl<'a> GeneralLossComputer for BinderGLossComputer<'a> {
    fn expected_loss_kernel(&self) -> f64 {
        self.subsets
            .iter()
            .fold(0.0, |s, subset| s + subset.committed_loss)
    }

    fn speculative_add(&mut self, partition: &Partition, i: usize, subset_index: LabelType) -> f64 {
        self.subsets[subset_index as usize].speculative_loss = partition.subsets()
            [subset_index as usize]
            .items()
            .iter()
            .fold(0.0, |s, j| {
                s + 0.5 - unsafe { *self.psm.get_unchecked((i, *j)) }
            });
        self.subsets[subset_index as usize].speculative_loss
    }

    fn new_subset(&mut self, partition: &mut Partition) {
        partition.new_subset();
        self.subsets.push(BinderSubsetCalculations {
            committed_loss: 0.0,
            speculative_loss: 0.0,
        });
    }

    fn add_with_index(&mut self, partition: &mut Partition, i: usize, subset_index: LabelType) {
        self.subsets[subset_index as usize].committed_loss +=
            self.subsets[subset_index as usize].speculative_loss;
        partition.add_with_index(i, subset_index as usize);
    }

    fn remove(&mut self, partition: &mut Partition, i: usize) -> LabelType {
        let subset_index = partition.label_of(i).unwrap();
        self.subsets[subset_index].committed_loss -= partition.subsets()[subset_index]
            .items()
            .iter()
            .fold(0.0, |s, j| {
                let jj = *j;
                s + if jj != i {
                    0.5 - unsafe { *self.psm.get_unchecked((i, jj)) }
                } else {
                    0.0
                }
            });
        partition.remove_clean_and_relabel(i, |killed_subset_index, moved_subset_index| {
            self.subsets.swap_remove(killed_subset_index);
            assert_eq!(moved_subset_index, self.subsets.len());
        });
        subset_index as LabelType
    }
}

// First-order approximation of expectation of one minus the adjusted Rand index loss

#[derive(Debug)]
struct OMARIApproxSubsetCalculations {
    committed_ip: f64,
    committed_i: f64,
    speculative_ip: f64,
    speculative_i: f64,
}

pub struct OMARIApproxGLossComputer<'a> {
    committed_n_items: usize,
    committed_sum_psm: f64,
    speculative_sum_psm: f64,
    subsets: Vec<OMARIApproxSubsetCalculations>,
    psm: &'a SquareMatrixBorrower<'a>,
}

impl<'a> OMARIApproxGLossComputer<'a> {
    pub fn new(psm: &'a SquareMatrixBorrower<'a>) -> Self {
        Self {
            committed_n_items: 0,
            committed_sum_psm: 0.0,
            speculative_sum_psm: f64::NEG_INFINITY,
            subsets: Vec::new(),
            psm,
        }
    }
}

impl<'a> OMARIApproxGLossComputer<'a> {
    fn engine(
        &self,
        speculative_n_items: usize,
        speculative_ip: f64,
        speculative_i: f64,
        speculative_sum_psm: f64,
    ) -> f64 {
        let n_items = speculative_n_items + self.committed_n_items;
        if n_items <= 1 {
            return f64::INFINITY;
        }
        let n_choose_2 = (n_items * (n_items - 1) / 2) as f64;
        let all_ip = speculative_ip + self.subsets.iter().fold(0.0, |s, c| s + c.committed_ip);
        let all_i = speculative_i + self.subsets.iter().fold(0.0, |s, c| s + c.committed_i);
        let all_p = speculative_sum_psm + self.committed_sum_psm;
        let subtractor = all_i * all_p / n_choose_2;
        let numerator = all_ip - subtractor;
        let denominator = 0.5 * (all_i + all_p) - subtractor;
        1.0 - numerator / denominator
    }
}

impl<'a> GeneralLossComputer for OMARIApproxGLossComputer<'a> {
    fn expected_loss_kernel(&self) -> f64 {
        self.engine(0, 0.0, 0.0, 0.0)
    }

    fn speculative_add(&mut self, partition: &Partition, i: usize, subset_index: LabelType) -> f64 {
        let s = &partition.subsets()[subset_index as usize];
        self.subsets[subset_index as usize].speculative_ip = s
            .items()
            .iter()
            .fold(0.0, |s, j| s + unsafe { *self.psm.get_unchecked((i, *j)) });
        self.subsets[subset_index as usize].speculative_i = s.n_items() as f64;
        if self.speculative_sum_psm == f64::NEG_INFINITY {
            self.speculative_sum_psm = partition.subsets().iter().fold(0.0, |s, subset| {
                // We use the NEG_INFINITY flag to see if we need to do the computation.
                s + subset.items().iter().fold(0.0, |ss, j| {
                    ss + unsafe { *self.psm.get_unchecked((i, *j)) }
                })
            })
        }
        self.engine(
            1,
            self.subsets[subset_index as usize].speculative_ip,
            self.subsets[subset_index as usize].speculative_i,
            self.speculative_sum_psm,
        )
    }

    fn new_subset(&mut self, partition: &mut Partition) {
        partition.new_subset();
        self.subsets.push(OMARIApproxSubsetCalculations {
            committed_ip: 0.0,
            committed_i: 0.0,
            speculative_ip: 0.0,
            speculative_i: 0.0,
        });
    }

    fn add_with_index(&mut self, partition: &mut Partition, i: usize, subset_index: LabelType) {
        let mut sc = &mut self.subsets[subset_index as usize];
        sc.committed_ip += sc.speculative_ip;
        sc.committed_i += sc.speculative_i;
        self.committed_n_items += 1;
        self.committed_sum_psm += self.speculative_sum_psm;
        self.speculative_sum_psm = f64::NEG_INFINITY;
        partition.add_with_index(i, subset_index as usize);
    }

    fn remove(&mut self, partition: &mut Partition, i: usize) -> LabelType {
        let subset_index = partition.label_of(i).unwrap();
        self.subsets[subset_index].committed_ip -= partition.subsets()[subset_index]
            .items()
            .iter()
            .fold(0.0, |s, j| {
                let jj = *j;
                s + if jj != i {
                    unsafe { *self.psm.get_unchecked((i, jj)) }
                } else {
                    0.0
                }
            });
        self.subsets[subset_index].committed_i -=
            (partition.subsets()[subset_index].n_items() - 1) as f64;
        self.committed_n_items -= 1;
        self.committed_sum_psm -= partition.subsets().iter().fold(0.0, |s, subset| {
            // We use the NEG_INFINITY flag to see if we need to do the computation.
            s + subset.items().iter().fold(0.0, |ss, j| {
                let jj = *j;
                ss + if jj != i {
                    unsafe { *self.psm.get_unchecked((i, jj)) }
                } else {
                    0.0
                }
            })
        });
        partition.remove_clean_and_relabel(i, |killed_subset_index, moved_subset_index| {
            self.subsets.swap_remove(killed_subset_index);
            assert_eq!(moved_subset_index, self.subsets.len());
        });
        subset_index as LabelType
    }
}

// Lower bound of the expectation of variation of information loss

#[derive(Debug)]
struct VILBCacheUnit {
    item: usize,
    committed_sum: f64,
    committed_contribution: f64,
    speculative_sum: f64,
    speculative_contribution: f64,
}

#[derive(Debug)]
struct VILBSubsetCalculations {
    cached_units: Vec<VILBCacheUnit>,
    committed_loss: f64,
    speculative_loss: f64,
}

pub struct VILBGLossComputer<'a> {
    subsets: Vec<VILBSubsetCalculations>,
    psm: &'a SquareMatrixBorrower<'a>,
}

impl<'a> VILBGLossComputer<'a> {
    pub fn new(psm: &'a SquareMatrixBorrower<'a>) -> Self {
        Self {
            subsets: Vec::new(),
            psm,
        }
    }
    pub fn expected_loss_from_kernel(psm: &'a SquareMatrixBorrower<'a>, kernel: f64) -> f64 {
        let nif = psm.n_items() as f64;
        (kernel + vilb_expected_loss_constant(psm)) / nif
    }
}

impl<'a> GeneralLossComputer for VILBGLossComputer<'a> {
    fn expected_loss_kernel(&self) -> f64 {
        self.subsets
            .iter()
            .fold(0.0, |s, subset| s + subset.committed_loss)
    }

    fn speculative_add(&mut self, partition: &Partition, i: usize, subset_index: LabelType) -> f64 {
        let subset_of_partition = &partition.subsets()[subset_index as usize];
        if subset_of_partition.n_items() == 0 {
            self.subsets[subset_index as usize]
                .cached_units
                .push(VILBCacheUnit {
                    item: i,
                    committed_sum: 0.0,
                    committed_contribution: 0.0,
                    speculative_sum: 1.0,
                    speculative_contribution: 0.0,
                });
            return 0.0;
        }
        for cu in self.subsets[subset_index as usize].cached_units.iter_mut() {
            cu.speculative_sum =
                cu.committed_sum + unsafe { *self.psm.get_unchecked((cu.item, i)) };
            cu.speculative_contribution = cu.speculative_sum.log2();
        }
        let sum = subset_of_partition
            .items()
            .iter()
            .fold(0.0, |s, j| s + unsafe { *self.psm.get_unchecked((i, *j)) })
            + 1.0; // Because self.psm[(i, i)] == 1;
        self.subsets[subset_index as usize]
            .cached_units
            .push(VILBCacheUnit {
                item: i,
                committed_sum: 0.0,
                committed_contribution: 0.0,
                speculative_sum: sum,
                speculative_contribution: sum.log2(),
            });
        let nif = subset_of_partition.n_items() as f64;
        let s1 = (nif + 1.0) * (nif + 1.0).log2();
        let s2 = self.subsets[subset_index as usize]
            .cached_units
            .iter()
            .fold(0.0, |s, cu| s + cu.speculative_contribution);
        self.subsets[subset_index as usize].speculative_loss = s1 - 2.0 * s2;
        self.subsets[subset_index as usize].speculative_loss
            - self.subsets[subset_index as usize].committed_loss
    }

    fn new_subset(&mut self, partition: &mut Partition) {
        partition.new_subset();
        self.subsets.push(VILBSubsetCalculations {
            cached_units: Vec::new(),
            committed_loss: 0.0,
            speculative_loss: 0.0,
        })
    }

    fn add_with_index(&mut self, partition: &mut Partition, i: usize, subset_index: LabelType) {
        for (index, subset) in self.subsets.iter_mut().enumerate() {
            if index == (subset_index as usize) {
                for cu in subset.cached_units.iter_mut() {
                    cu.committed_sum = cu.speculative_sum;
                    cu.committed_contribution = cu.speculative_contribution;
                }
            } else {
                subset.cached_units.pop();
            }
        }
        self.subsets[subset_index as usize].committed_loss =
            self.subsets[subset_index as usize].speculative_loss;
        partition.add_with_index(i, subset_index as usize);
    }

    fn remove(&mut self, partition: &mut Partition, i: usize) -> LabelType {
        let subset_index = partition.label_of(i).unwrap();
        for cu in self.subsets[subset_index].cached_units.iter_mut() {
            cu.committed_sum -= unsafe { *self.psm.get_unchecked((cu.item, i)) };
            cu.committed_contribution = cu.committed_sum.log2();
        }
        let pos = self.subsets[subset_index]
            .cached_units
            .iter()
            .enumerate()
            .find(|cu| cu.1.item == i)
            .unwrap()
            .0;
        self.subsets[subset_index].cached_units.swap_remove(pos);
        self.subsets[subset_index].committed_loss =
            match partition.subsets()[subset_index].n_items() {
                0 => 0.0,
                ni => {
                    let nif = ni as f64;
                    nif * nif.log2()
                        - 2.0
                            * self.subsets[subset_index]
                                .cached_units
                                .iter()
                                .fold(0.0, |s, cu| s + cu.committed_contribution)
                }
            };
        partition.remove_clean_and_relabel(i, |killed_subset_index, moved_subset_index| {
            self.subsets.swap_remove(killed_subset_index);
            assert_eq!(moved_subset_index, self.subsets.len());
        });
        subset_index as LabelType
    }
}

// Common

fn label_of_empty_cluster<U: GeneralLossComputer>(
    partition: &mut Partition,
    computer: &mut U,
    max_label: LabelType,
) -> Option<LabelType> {
    match partition.subsets().last() {
        None => {
            computer.new_subset(partition);
            Some(0)
        }
        Some(last) => {
            if last.is_empty() {
                Some(partition.n_subsets() as LabelType - 1)
            } else if partition.n_subsets() <= max_label as usize {
                computer.new_subset(partition);
                Some(partition.n_subsets() as LabelType - 1)
            } else {
                None
            }
        }
    }
}

fn micro_optimized_allocation<U: GeneralLossComputer>(
    partition: &mut Partition,
    computer: &mut U,
    i: usize,
    label_to_take: Option<LabelType>,
) -> LabelType {
    let max_label = partition.n_subsets() as LabelType - 1;
    let iter = (0..=max_label).map(|subset_index| {
        let value = computer.speculative_add(partition, i, subset_index as LabelType);
        match label_to_take {
            Some(label_to_take) => {
                if subset_index == label_to_take {
                    (subset_index, f64::NEG_INFINITY)
                } else {
                    (subset_index, f64::INFINITY)
                }
            }
            None => (subset_index, value),
        }
    });
    let subset_index = find_label_of_minimum(iter);
    computer.add_with_index(partition, i, subset_index);
    subset_index as LabelType
}

pub fn minimize_once_by_salso<'a, T: Rng, U: GeneralLossComputer>(
    computer_factory: Box<dyn Fn() -> U + 'a>,
    p: &SALSOParameters,
    seconds_target: f64,
    rng: &mut T,
) -> SALSOResults {
    let start_time = Instant::now();
    let max_label = if p.max_size == 0 {
        MAX_LABEL
    } else {
        p.max_size.max(1) - 1
    };
    let mut best = SALSOResults::dummy(max_label + 1);
    let mut permutation: Vec<usize> = (0..p.n_items).collect();
    let uniform_0_1 = Uniform::from(0.0..1.0);
    let mut run_counter = 0;
    while run_counter < p.n_runs {
        let mut computer = computer_factory();
        let (mut partition, initialization_method) = if uniform_0_1.sample(rng)
            < p.prob_sequential_allocation
        {
            let mut partition = Partition::new(p.n_items);
            permutation.shuffle(rng);
            let singletons_initialization =
                uniform_0_1.sample(rng) < p.prob_singletons_initialization;
            // Sequential allocation
            for i in 0..p.n_items {
                let ii = unsafe { *permutation.get_unchecked(i) };
                let empty_label = label_of_empty_cluster(&mut partition, &mut computer, max_label);
                if singletons_initialization {
                    micro_optimized_allocation(&mut partition, &mut computer, ii, empty_label);
                } else {
                    micro_optimized_allocation(&mut partition, &mut computer, ii, None);
                }
            }
            let initialization_method = if singletons_initialization {
                InitializationMethod::SequentialFromSingletons
            } else {
                InitializationMethod::SequentialFromEmpty
            };
            (partition, initialization_method)
        } else {
            // Something about this code isn't working when max_label is small.
            // Since pairwise similarity based computations aren't recommend, it may not be worth
            // pursuing this further.  So, we'll just panic here and disable it in the R package.
            panic!("Disabled because it's problematic.");
            // let mut partition = Partition::new(p.n_items);
            // let destiny = {
            //     let mut v = Vec::with_capacity(p.n_items);
            //     v.resize_with(p.n_items, || rng.gen_range(0, max_label + 1));
            //     Partition::from(&v) // Already canonicalized
            // };
            // for i in 0..p.n_items {
            //     let label_to_take = Some(destiny.label_of(i).unwrap() as LabelType);
            //     label_of_empty_cluster(&mut partition, &mut computer, max_label);
            //     micro_optimized_allocation(&mut partition, &mut computer, i, label_to_take);
            // }
            // (
            //     partition,
            //     InitializationMethod::SampleOne2MaxWithReplacement,
            // )
        };
        // Sweetening scans
        let mut n_scans = 0;
        let mut change = true;
        while change && n_scans < p.max_scans {
            change = false;
            n_scans += 1;
            permutation.shuffle(rng);
            for i in 0..p.n_items {
                label_of_empty_cluster(&mut partition, &mut computer, max_label);
                let ii = unsafe { *permutation.get_unchecked(i) };
                let previous_subset_index = computer.remove(&mut partition, ii);
                let subset_index =
                    micro_optimized_allocation(&mut partition, &mut computer, ii, None);
                if subset_index != previous_subset_index {
                    change = true;
                };
            }
        }
        let expected_loss = computer.expected_loss_kernel();
        if expected_loss < best.expected_loss {
            partition.canonicalize();
            best = SALSOResults {
                clustering: partition.labels_via_copying(),
                expected_loss,
                n_scans,
                initialization_method,
                ..best
            };
        }
        run_counter += 1;
        if seconds_target.is_finite() {
            let seconds = start_time.elapsed().as_secs_f64();
            let rc = run_counter as f64;
            let seconds_per_run = seconds / rc;
            let projected_seconds_if_another = (rc + 1.0) * seconds_per_run;
            if projected_seconds_if_another >= seconds_target {
                let prob_stop = (projected_seconds_if_another - seconds_target) / seconds_per_run;
                if uniform_0_1.sample(rng) < prob_stop {
                    return SALSOResults {
                        n_runs: run_counter,
                        seconds,
                        ..best
                    };
                }
            }
        }
    }
    SALSOResults {
        n_runs: p.n_runs,
        seconds: start_time.elapsed().as_secs_f64(),
        ..best
    }
}

// Common implementation for SALSO.

#[derive(Debug, Clone)]
pub struct SALSOParameters {
    pub n_items: usize,
    pub max_size: LabelType,
    pub max_size_as_rf: bool,
    pub max_scans: u32,
    pub max_zealous_updates: u32,
    pub n_runs: u32,
    pub prob_sequential_allocation: f64,
    pub prob_singletons_initialization: f64,
}

pub struct SALSOResults {
    pub clustering: Vec<usize>,
    pub expected_loss: f64,
    pub n_scans: u32,
    pub n_zealous_accepts: u32,
    pub n_zealous_attempts: u32,
    pub initialization_method: InitializationMethod,
    pub n_runs: u32,
    pub max_size: LabelType,
    pub seconds: f64,
}

impl SALSOResults {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        clustering: Vec<usize>,
        expected_loss: f64,
        n_scans: u32,
        n_zealous_accepts: u32,
        n_zealous_attempts: u32,
        initialization_method: InitializationMethod,
        n_runs: u32,
        max_size: LabelType,
        seconds: f64,
    ) -> Self {
        Self {
            clustering,
            expected_loss,
            n_scans,
            n_zealous_accepts,
            n_zealous_attempts,
            initialization_method,
            n_runs,
            max_size,
            seconds,
        }
    }

    pub fn dummy(max_size: LabelType) -> Self {
        SALSOResults::new(
            vec![0usize; 0],
            std::f64::INFINITY,
            0,
            0,
            0,
            InitializationMethod::SequentialFromEmpty,
            0,
            max_size,
            0.0,
        )
    }
}

pub fn minimize_by_salso<T: Rng>(
    pdi: PartitionDistributionInformation,
    loss_function: LossFunction,
    p: &SALSOParameters,
    seconds: f64,
    n_cores: u32,
    mut rng: &mut T,
) -> SALSOResults {
    let start_time = Instant::now();
    let cache = Log2Cache::new(match loss_function {
        LossFunction::VI(_) | LossFunction::NVI | LossFunction::ID | LossFunction::NID => p.n_items,
        _ => 0,
    });
    let result = if n_cores == 1 {
        match loss_function {
            LossFunction::BinderDraws(a) => minimize_once_by_salso_v2(
                Box::new(|| BinderCMLossComputer::new(a)),
                pdi.draws(),
                p,
                seconds,
                rng,
            ),
            LossFunction::BinderPSM => minimize_once_by_salso(
                Box::new(|| BinderGLossComputer::new(pdi.psm())),
                p,
                seconds,
                rng,
            ),
            LossFunction::OneMinusARI => minimize_once_by_salso_v2(
                Box::new(|| OMARICMLossComputer::new(pdi.draws().n_clusterings())),
                pdi.draws(),
                p,
                seconds,
                rng,
            ),
            LossFunction::OneMinusARIapprox => minimize_once_by_salso(
                Box::new(|| OMARIApproxGLossComputer::new(pdi.psm())),
                p,
                seconds,
                rng,
            ),
            LossFunction::VI(a) => minimize_once_by_salso_v2(
                Box::new(|| VICMLossComputer::new(a, &cache)),
                pdi.draws(),
                p,
                seconds,
                rng,
            ),
            LossFunction::VIlb => minimize_once_by_salso(
                Box::new(|| VILBGLossComputer::new(pdi.psm())),
                p,
                seconds,
                rng,
            ),
            LossFunction::NVI => minimize_once_by_salso_v2(
                Box::new(|| {
                    GeneralInformationBasedCMLossComputer::new(
                        pdi.draws().n_clusterings(),
                        &cache,
                        NVIInformationBasedLoss {},
                    )
                }),
                pdi.draws(),
                p,
                seconds,
                rng,
            ),
            LossFunction::ID => minimize_once_by_salso_v2(
                Box::new(|| {
                    GeneralInformationBasedCMLossComputer::new(
                        pdi.draws().n_clusterings(),
                        &cache,
                        IDInformationBasedLoss {},
                    )
                }),
                pdi.draws(),
                p,
                seconds,
                rng,
            ),
            LossFunction::NID => minimize_once_by_salso_v2(
                Box::new(|| {
                    GeneralInformationBasedCMLossComputer::new(
                        pdi.draws().n_clusterings(),
                        &cache,
                        NIDInformationBasedLoss {},
                    )
                }),
                pdi.draws(),
                p,
                seconds,
                rng,
            ),
        }
    } else {
        let (tx, rx) = mpsc::channel();
        let n_cores = if n_cores == 0 {
            num_cpus::get() as u32
        } else {
            n_cores
        };
        let p = p.clone();
        let p = SALSOParameters {
            n_runs: (p.n_runs + n_cores - 1) / n_cores,
            ..p
        };
        let cache_ref = &cache;
        crossbeam::scope(|s| {
            for _ in 0..n_cores {
                let tx = mpsc::Sender::clone(&tx);
                let mut child_rng = Pcg64Mcg::from_rng(&mut rng).unwrap();
                let p = p.clone();
                s.spawn(move |_| {
                    let result = match loss_function {
                        LossFunction::BinderDraws(a) => minimize_once_by_salso_v2(
                            Box::new(|| BinderCMLossComputer::new(a)),
                            pdi.draws(),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::BinderPSM => minimize_once_by_salso(
                            Box::new(|| BinderGLossComputer::new(pdi.psm())),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::OneMinusARI => minimize_once_by_salso_v2(
                            Box::new(|| OMARICMLossComputer::new(pdi.draws().n_clusterings())),
                            pdi.draws(),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::OneMinusARIapprox => minimize_once_by_salso(
                            Box::new(|| OMARIApproxGLossComputer::new(pdi.psm())),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::VI(a) => minimize_once_by_salso_v2(
                            Box::new(|| VICMLossComputer::new(a, cache_ref)),
                            pdi.draws(),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::VIlb => minimize_once_by_salso(
                            Box::new(|| VILBGLossComputer::new(pdi.psm())),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::NVI => minimize_once_by_salso_v2(
                            Box::new(|| {
                                GeneralInformationBasedCMLossComputer::new(
                                    pdi.draws().n_clusterings(),
                                    cache_ref,
                                    NVIInformationBasedLoss {},
                                )
                            }),
                            pdi.draws(),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::ID => minimize_once_by_salso_v2(
                            Box::new(|| {
                                GeneralInformationBasedCMLossComputer::new(
                                    pdi.draws().n_clusterings(),
                                    cache_ref,
                                    IDInformationBasedLoss {},
                                )
                            }),
                            pdi.draws(),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                        LossFunction::NID => minimize_once_by_salso_v2(
                            Box::new(|| {
                                GeneralInformationBasedCMLossComputer::new(
                                    pdi.draws().n_clusterings(),
                                    cache_ref,
                                    NIDInformationBasedLoss {},
                                )
                            }),
                            pdi.draws(),
                            &p,
                            seconds,
                            &mut child_rng,
                        ),
                    };
                    tx.send(result).unwrap();
                });
            }
        })
        .unwrap();
        std::mem::drop(tx); // Because of the cloning in the loop.
        let mut best = SALSOResults::dummy(p.max_size);
        let mut run_counter = 0;
        for candidate in rx {
            run_counter += candidate.n_runs;
            if candidate.expected_loss < best.expected_loss {
                best = candidate;
            }
        }
        best.n_runs = run_counter;
        best
    };
    let result = SALSOResults {
        expected_loss: match loss_function {
            LossFunction::BinderPSM => {
                BinderGLossComputer::expected_loss_from_kernel(pdi.psm(), result.expected_loss)
            }
            LossFunction::VIlb => {
                VILBGLossComputer::expected_loss_from_kernel(pdi.psm(), result.expected_loss)
            }
            _ => result.expected_loss,
        },
        seconds: start_time.elapsed().as_secs_f64(),
        ..result
    };
    result
}

pub fn minimize_by_enumeration(
    f: fn(&[LabelType], &SquareMatrixBorrower) -> f64,
    psm: &SquareMatrixBorrower,
) -> Vec<usize> {
    let (tx, rx) = mpsc::channel();
    crossbeam::scope(|s| {
        for iter in Partition::iter_sharded(num_cpus::get() as u32, psm.n_items()) {
            let tx = mpsc::Sender::clone(&tx);
            s.spawn(move |_| {
                let mut working_minimum = std::f64::INFINITY;
                let mut working_minimizer = vec![0usize; psm.n_items()];
                for partition in iter {
                    let part: Vec<LabelType> = partition.iter().map(|x| *x as LabelType).collect();
                    let value = f(&part[..], psm);
                    if value < working_minimum {
                        working_minimum = value;
                        working_minimizer = partition;
                    }
                }
                tx.send(working_minimizer).unwrap();
            });
        }
    })
    .unwrap();
    std::mem::drop(tx); // Because of the cloning in the loop.
    let mut working_minimum = std::f64::INFINITY;
    let mut working_minimizer = vec![0usize; psm.n_items()];
    for partition in rx {
        let part: Vec<LabelType> = partition.iter().map(|x| *x as LabelType).collect();
        let value = f(&part[..], psm);
        if value < working_minimum {
            working_minimum = value;
            working_minimizer = partition;
        }
    }
    working_minimizer
}

#[cfg(test)]
mod tests_optimize {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_max_scan() {
        let n_items = 5;
        let mut psm = SquareMatrix::identity(n_items);
        let psm_view = &psm.view();
        let p = SALSOParameters {
            n_items,
            max_size: 2,
            max_size_as_rf: true,
            max_scans: 10,
            max_zealous_updates: 10,
            n_runs: 100,
            prob_sequential_allocation: 1.0,
            prob_singletons_initialization: 0.5,
        };
        minimize_by_salso(
            PartitionDistributionInformation::PairwiseSimilarityMatrix(psm_view),
            LossFunction::BinderPSM,
            &p,
            5.0,
            1,
            &mut thread_rng(),
        );
    }
}
