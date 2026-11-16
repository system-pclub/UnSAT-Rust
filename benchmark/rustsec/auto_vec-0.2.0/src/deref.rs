use super::*;
impl<T> Deref for AutoVec<T> {
    type Target = RawAutoVec<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
impl<T> DerefMut for AutoVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}
impl<T> Deref for AutoChild<T> {
    type Target = Pin<Box<RawAutoChild<T>>>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
impl<T> DerefMut for AutoChild<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}
impl<T> Deref for RawAutoChild<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.child
    }
}
impl<T> DerefMut for RawAutoChild<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}