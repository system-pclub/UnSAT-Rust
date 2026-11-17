use crate::*;

use crate::optimize::CMLossComputer;
use ndarray::Array3;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Clusterings {
    n_clusterings: usize,
    n_items: usize,
    labels: Vec<LabelType>,
    n_clusters: Vec<LabelType>,
    max_clusters: LabelType,
}

impl Clusterings {
    pub fn unvalidated(
        n_clusterings: usize,
        n_items: usize,
        labels: Vec<LabelType>,
        n_clusters: Vec<LabelType>,
    ) -> Self {
        assert_eq!(n_clusterings * n_items, labels.len());
        assert_eq!(n_clusterings, n_clusters.len());
        let max_clusters = *n_clusters.iter().max().unwrap();
        Self {
            n_clusterings,
            n_items,
            labels,
            n_clusters,
            max_clusters,
        }
    }

    pub fn from_i32_column_major_order(original_labels: &[i32], n_items: usize) -> Self {
        let n_clusterings = original_labels.len() / n_items;
        let mut labels = Vec::with_capacity(n_clusterings * n_items);
        let mut n_clusters = Vec::with_capacity(n_clusterings);
        let mut map = HashMap::new();
        let mut max_clusters = 0;
        for i in 0..n_clusterings {
            map.clear();
            let mut next_new_label = 0;
            for j in 0..n_items {
                let c = *map
                    .entry(unsafe { original_labels.get_unchecked(j * n_clusterings + i) })
                    .or_insert_with(|| {
                        let c = next_new_label;
                        next_new_label += 1;
                        c
                    });
                labels.push(c);
            }
            n_clusters.push(next_new_label);
            if next_new_label > max_clusters {
                max_clusters = next_new_label
            }
        }
        Self {
            n_clusterings,
            n_items,
            labels,
            n_clusters,
            max_clusters,
        }
    }

    pub fn make_confusion_matrices(&self, state: &WorkingClustering) -> Array3<CountType> {
        let mut cms = Array3::<CountType>::zeros((
            state.max_clusters() as usize + 1,
            self.max_clusters() as usize,
            self.n_clusterings(),
        ));
        for item_index in 0..self.n_items {
            let state_index = state.get(item_index) as usize + 1;
            for draw_index in 0..self.n_clusterings() {
                let other_index = self.label(draw_index, item_index) as usize;
                cms[(0, other_index, draw_index)] += 1;
                cms[(state_index, other_index, draw_index)] += 1;
            }
        }
        cms
    }

    pub fn n_clusterings(&self) -> usize {
        self.n_clusterings
    }

    pub fn n_items(&self) -> usize {
        self.n_items
    }

    pub fn label(&self, draw_index: usize, item_index: usize) -> LabelType {
        unsafe {
            *self
                .labels
                .get_unchecked(draw_index * self.n_items + item_index)
        }
    }

    pub fn labels(&self, draw_index: usize) -> &[LabelType] {
        &self.labels[draw_index * self.n_items..(draw_index + 1) * self.n_items]
    }

    pub fn n_clusters(&self, draw_index: usize) -> LabelType {
        unsafe { *self.n_clusters.get_unchecked(draw_index) }
    }

    pub fn max_clusters(&self) -> LabelType {
        self.max_clusters
    }

    pub fn mean_and_sd_of_n_clusters(&self) -> (f64, f64) {
        let ndf = self.n_clusterings as f64;
        let (sum1, sum2) = self.n_clusters.iter().fold((0.0, 0.0), |(s1, s2), x| {
            let x = *x as f64;
            (s1 + x, s2 + x * x)
        });
        let mean = sum1 / ndf;
        let sd = ((sum2 - sum1 * sum1 / ndf) / (ndf - 1.0)).sqrt();
        (mean, sd)
    }
}

#[derive(Debug, Clone)]
pub struct WorkingClustering {
    labels: Vec<LabelType>,
    max_clusters: LabelType,
    sizes: Vec<CountType>,
    occupied_clusters: Vec<LabelType>,
    potentially_empty_label: LabelType,
}

impl WorkingClustering {
    pub fn empty(n_items: usize, max_clusters: LabelType) -> Self {
        let max_clusters = max_clusters.max(1);
        let sizes = vec![0; max_clusters as usize];
        let occupied_clusters = Vec::with_capacity(max_clusters as usize);
        Self {
            labels: vec![0; n_items],
            max_clusters,
            sizes,
            occupied_clusters,
            potentially_empty_label: 0,
        }
    }

    pub fn random<T: Rng>(n_items: usize, max_clusters: LabelType, rng: &mut T) -> Self {
        let labels = Self::sample_1tok(n_items, max_clusters, rng);
        WorkingClustering::from_vector(labels, max_clusters)
    }

    pub fn random_as_rf<T: Rng>(
        n_items: usize,
        max_clusters: LabelType,
        max_clusters_observed: LabelType,
        rng: &mut T,
    ) -> Self {
        let labels = Self::sample_1tok(n_items, max_clusters, rng);
        let a = WorkingClustering::from_vector(labels, max_clusters);
        let labels = a.standardize();
        let max_clusters =
            max_clusters_observed.max(*labels.iter().max().unwrap() as LabelType + 1);
        WorkingClustering::from_vector(labels, max_clusters)
    }

    fn sample_1tok<T: Rng>(n_items: usize, max_clusters: LabelType, rng: &mut T) -> Vec<LabelType> {
        let mut v = Vec::with_capacity(n_items);
        v.resize_with(n_items, || rng.gen_range(0..max_clusters));
        v
    }

    pub fn one_cluster(n_items: usize, max_clusters: LabelType) -> Self {
        WorkingClustering::from_vector(vec![0; n_items], max_clusters)
    }

    pub fn from_slice(labels: &[LabelType], max_clusters: LabelType) -> Self {
        Self::from_vector(labels.to_vec(), max_clusters)
    }

    pub fn from_vector(labels: Vec<LabelType>, max_clusters: LabelType) -> Self {
        let max_clusters = max_clusters.max(1);
        let mut x = Self {
            labels,
            max_clusters,
            sizes: vec![0; max_clusters as usize],
            occupied_clusters: Vec::with_capacity(max_clusters as usize),
            potentially_empty_label: 0,
        };
        for label in &x.labels {
            x.sizes[*label as usize] += 1;
        }
        for (index, size) in x.sizes.iter().enumerate() {
            if *size > 0 {
                x.occupied_clusters.push(index as LabelType)
            }
        }
        x
    }

    pub fn n_items(&self) -> CountType {
        self.labels.len() as CountType
    }

    pub fn occupied_clusters(&self) -> &Vec<LabelType> {
        &self.occupied_clusters
    }

    pub fn label_of_empty_cluster(&mut self) -> Option<LabelType> {
        if self.occupied_clusters.len() >= self.max_clusters as usize {
            None
        } else if self.sizes[self.potentially_empty_label as usize] == 0 {
            Some(self.potentially_empty_label)
        } else {
            match self.sizes.iter().position(|&size| size == 0) {
                Some(index) => {
                    self.potentially_empty_label = index as LabelType;
                    Some(self.potentially_empty_label)
                }
                None => None,
            }
        }
    }

    pub fn clone_labels(&self) -> Vec<LabelType> {
        self.labels.clone()
    }

    pub fn standardize(&self) -> Vec<LabelType> {
        let n_items = self.labels.len();
        let mut labels = Vec::with_capacity(n_items);
        let mut map = HashMap::new();
        let mut next_new_label = 0;
        for j in 0..n_items {
            let c = *map.entry(self.labels[j]).or_insert_with(|| {
                let c = next_new_label;
                next_new_label += 1;
                c
            });
            labels.push(c);
        }
        labels
    }

    pub fn as_slice(&self) -> &[LabelType] {
        &self.labels[..]
    }

    pub fn max_clusters(&self) -> LabelType {
        self.max_clusters
    }

    pub fn n_clusters(&self) -> LabelType {
        self.occupied_clusters.len() as LabelType
    }

    pub fn size_of(&self, label: LabelType) -> CountType {
        self.sizes[label as usize]
    }

    pub fn get(&self, item_index: usize) -> LabelType {
        self.labels[item_index]
    }

    /// # Safety
    ///
    /// There is no checking for the `item_index`.
    pub unsafe fn get_unchecked(&self, item_index: usize) -> LabelType {
        *self.labels.get_unchecked(item_index)
    }

    pub fn assign<T: CMLossComputer>(
        &mut self,
        item_index: usize,
        label: LabelType,
        loss_computer: &mut T,
        cms: &mut Array3<CountType>,
        draws: &Clusterings,
    ) {
        loss_computer.decision_callback(item_index, Some(label), None, self, cms, draws);
        self.labels[item_index] = label;
        if self.sizes[label as usize] == 0 {
            self.occupied_clusters.push(label);
        }
        self.sizes[label as usize] += 1;
        let to_index = label as usize + 1;
        for draw_index in 0..draws.n_clusterings() {
            let other_index = draws.label(draw_index, item_index) as usize;
            cms[(0, other_index, draw_index)] += 1;
            cms[(to_index, other_index, draw_index)] += 1;
        }
    }

    /*
    pub unsafe fn assign_unchecked(&mut self, item_index: usize, label: LabelType) {
        *self.labels.get_unchecked_mut(item_index) = label;
        if *self.sizes.get_unchecked(label as usize) == 0 {
            self.occupied_clusters.push(label);
        }
        *self.sizes.get_unchecked_mut(label as usize) += 1;
    }
    */

    pub fn reassign<T: CMLossComputer>(
        &mut self,
        item_index: usize,
        new_label: LabelType,
        loss_computer: &mut T,
        cms: &mut Array3<CountType>,
        draws: &Clusterings,
    ) {
        let old_label = self.labels[item_index];
        if new_label != old_label {
            loss_computer.decision_callback(
                item_index,
                Some(new_label),
                Some(old_label),
                self,
                cms,
                draws,
            );
            self.labels[item_index] = new_label;
            self.sizes[old_label as usize] -= 1;
            if self.sizes[old_label as usize] == 0 {
                self.occupied_clusters.swap_remove(
                    self.occupied_clusters
                        .iter()
                        .position(|x| *x == old_label)
                        .unwrap(),
                );
            }
            if self.sizes[new_label as usize] == 0 {
                self.occupied_clusters.push(new_label);
            }
            self.sizes[new_label as usize] += 1;
            let to_index = new_label as usize + 1;
            let from_index = old_label as usize + 1;
            for draw_index in 0..draws.n_clusterings() {
                let other_index = draws.label(draw_index, item_index) as usize;
                cms[(from_index, other_index, draw_index)] -= 1;
                cms[(to_index, other_index, draw_index)] += 1;
            }
        }
    }

    /*
    pub unsafe fn reassign_unchecked(&mut self, item_index: usize, new_label: LabelType) {
        let old_label = *self.labels.get_unchecked(item_index);
        if new_label != old_label {
            *self.labels.get_unchecked_mut(item_index) = new_label;
            *self.sizes.get_unchecked_mut(old_label as usize) -= 1;
            if *self.sizes.get_unchecked(old_label as usize) == 0 {
                self.occupied_clusters.swap_remove(
                    self.occupied_clusters
                        .iter()
                        .position(|x| *x == old_label)
                        .unwrap(),
                );
            }
            if *self.sizes.get_unchecked(new_label as usize) == 0 {
                self.occupied_clusters.push(new_label);
            }
            *self.sizes.get_unchecked_mut(new_label as usize) += 1;
        }
    }
    */

    pub fn remove<T: CMLossComputer>(
        &mut self,
        item_index: usize,
        loss_computer: &mut T,
        cms: &mut Array3<CountType>,
        draws: &Clusterings,
    ) {
        let old_label = self.labels[item_index];
        loss_computer.decision_callback(item_index, None, Some(old_label), self, cms, draws);
        self.sizes[old_label as usize] -= 1;
        if self.sizes[old_label as usize] == 0 {
            self.occupied_clusters.swap_remove(
                self.occupied_clusters
                    .iter()
                    .position(|x| *x == old_label)
                    .unwrap(),
            );
        }
        let from_index = old_label as usize + 1;
        for draw_index in 0..draws.n_clusterings() {
            let other_index = draws.label(draw_index, item_index) as usize;
            cms[(0, other_index, draw_index)] -= 1;
            cms[(from_index, other_index, draw_index)] -= 1;
        }
    }

    /*
    pub unsafe fn remove_unchecked(&mut self, item_index: usize) {
        let old_label = *self.labels.get_unchecked(item_index);
        *self.sizes.get_unchecked_mut(old_label as usize) -= 1;
        if *self.sizes.get_unchecked(old_label as usize) == 0 {
            self.occupied_clusters.swap_remove(
                self.occupied_clusters
                    .iter()
                    .position(|x| *x == old_label)
                    .unwrap(),
            );
        }
    }
    */
}
