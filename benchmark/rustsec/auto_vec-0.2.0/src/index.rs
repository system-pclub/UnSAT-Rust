use super::*;
use std::ops::Index;
use std::ops::IndexMut;
/// ```
/// use auto_vec::*;
/// let mut c = AutoChild::new(1);
/// let mut v = AutoVec::new();
/// v.add(&mut c);
/// assert_eq!(1, v[0]);
/// ```
impl<T> Index<usize> for AutoVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {&((&*self.children[index]).child)}
    }
}
impl<T> IndexMut<usize> for AutoVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {&mut((&mut*(self.children[index] as *mut RawAutoChild<T>)).child)}
    }
}