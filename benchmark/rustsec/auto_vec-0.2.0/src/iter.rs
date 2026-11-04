use super::*;
use std::marker::PhantomData;
impl<T> AutoVec<T> {
    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            last: &self.children[self.len() - 1] as *const _,
            current: unsafe {(&self.children[0] as *const *const RawAutoChild<T>).sub(1)},
            lifetime: PhantomData,
        }
    }
    pub fn iter(&self) -> Iter<T> {
        Iter {
            last: &self.children[self.len() - 1] as *const _,
            current: unsafe {(&self.children[0] as *const *const RawAutoChild<T>).sub(1)},
            lifetime: PhantomData,
        }
    }
}
pub struct Iter<'a, T> {
    last: *const *const RawAutoChild<T>,
    current: *const *const RawAutoChild<T>,
    lifetime: PhantomData<&'a T>
}
pub struct IterMut<'a, T> {
    last: *const *const RawAutoChild<T>,
    current: *const *const RawAutoChild<T>,
    lifetime: PhantomData<&'a T>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.current = self.current.add(1)};
        if self.current > self.last {
            None
        } else {
            unsafe {Some(&((&**self.current).child))}
        }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.current = self.current.add(1)};
        if self.current > self.last {
            None
        } else {
            unsafe {Some(&mut (&mut *(*self.current as *mut RawAutoChild<T>)).child)}
        }
    }
}
impl<'a, T> IntoIterator for &'a AutoVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut AutoVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}