/// Chained iterators. Just a convinience wrapper around std stuff.
pub type ChainedIter<'a, T> =
    std::iter::Chain<std::slice::IterMut<'a, T>, std::slice::IterMut<'a, T>>;


pub trait Splittable<T> {    
    /// Extracts a mutable reference to an element at a given index, and an iterator to all the other elements    
    fn extract_at_index_mut_iter(&mut self, index: usize) -> (&mut T, ChainedIter<'_, T>);
    /// Extracts a mutable reference to an element at a given index, and slices containing elements before and after current
    fn extract_at_index_mut(&mut self, index: usize) -> (&mut T, (&mut [T], &mut [T]));
}

impl<T> Splittable<T> for [T] {
    fn extract_at_index_mut(&mut self, index: usize) -> (&mut T, (&mut [T], &mut [T])) {
        let n = self.len();
        debug_assert!(index < n, "Provided index must be valid for given array!");
        match index {
            0 => {
                let (r1, r2) = self.split_at_mut(1);
                (&mut r1[0], (r2, &mut []))
            }

            x if x == n => {
                let (r1, r2) = self.split_at_mut(index);
                (&mut r2[0], (r1, &mut []))
            }
            _ => {
                let (r1, r2) = self.split_at_mut(index);
                let (r3, r4) = r2.split_at_mut(1);
                (&mut r3[0], (r1, r4))
            }
        }
    }

    fn extract_at_index_mut_iter(&mut self, index: usize) -> (&mut T, ChainedIter<'_, T>) {
        let (v, (r1, r2)) = self.extract_at_index_mut(index);
        (v, r1.iter_mut().chain(r2))
    }
}

impl<T> Splittable<T> for Vec<T> {
    fn extract_at_index_mut(&mut self, index: usize) -> (&mut T, (&mut [T], &mut [T])) {
        self.as_mut_slice().extract_at_index_mut(index)
    }
    fn extract_at_index_mut_iter(&mut self, index: usize) -> (&mut T, ChainedIter<'_, T>) {
        self.as_mut_slice().extract_at_index_mut_iter(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::splittable_container::Splittable;
    #[test]
    pub fn extract_at_index_mut() {
        let mut data = [0, 1, 2, 3, 4, 5];
        for split_idx in [0, 3, 5] {
            let ans = data[split_idx];
            let (&mut a, (r1, r2)) = data.extract_at_index_mut(split_idx);
            dbg!(split_idx, a, &r1, &r2);
            assert_eq!(a, ans);
            assert_eq!(r1.len() + r2.len(), data.len() - 1);
        }
    }
    #[test]
    pub fn waat(){
        
    }
}
