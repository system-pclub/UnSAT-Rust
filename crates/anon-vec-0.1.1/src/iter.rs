
use std::any::Any;

/// An Immutable Iterator over an Anonymously Typed Vec.
#[derive(Copy, Clone)]
pub struct AnonIter<T>
where
    T: Any + 'static,
{
    pub(crate) data: *const T,
    pub(crate) curr: usize,
    pub(crate) len: usize,
}

impl<T> AnonIter<T>
where
    T: Any + 'static,
{
    pub fn next_unchecked(&mut self) -> &'static T {
        unsafe { &*self.data.add(self.curr - 1) }
    }
}


impl<T> Iterator for AnonIter<T>
where
    T: Any + 'static,
{
    type Item = &'static T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.len {
            None
        } else {
            self.curr += 1;
            unsafe {
                Some(&*self.data.add(self.curr - 1))
            }
        }
    }
}

/// A Mutable Iterator over an Anonymously Typed Vec.
#[derive(Copy, Clone)]
pub struct AnonIterMut<T>
where
    T: Any + 'static,
{
    pub(crate) data: *mut T,
    pub(crate) curr: usize,
    pub(crate) len: usize,
}

impl<T> AnonIterMut<T>
where
    T: Any + 'static,
{
    pub fn next_unchecked(&mut self) -> &'static mut T {
        unsafe { &mut *self.data.add(self.curr - 1) }
    }
}

impl<T> Iterator for AnonIterMut<T>
where
    T: Any + 'static,
{
    type Item = &'static mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.len {
            None
        } else {
            self.curr += 1;
            unsafe {
                Some(&mut *self.data.add(self.curr - 1))
            }
        }
    }
}

#[derive(Clone)]
pub struct AnonChain<T>
where
    T: Any + 'static,
{
    iters: Vec<AnonIter<T>>,
}

impl<T> AnonChain<T> 
where
    T: 'static,
{
    pub fn push(&mut self, iter: AnonIter<T>) {
        self.iters.push(iter)
    }

    pub fn new() -> Self {
        Self {
            iters: Vec::with_capacity(16),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.iters.is_empty()
    }
}

impl<T> Iterator for AnonChain<T> {
    type Item = &'static T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr = &mut self.iters[0];

        if curr.curr == curr.len {
            self.iters.remove(0);
            if self.is_empty() {
                return None;
            }
        }
        curr = &mut self.iters[0];
        Some(curr.next_unchecked())
    }
}

#[derive(Clone)]
pub struct AnonChainMut<T>
where
    T: Any + 'static,
{
    iters: Vec<AnonIterMut<T>>,
}

impl<T> AnonChainMut<T> 
where
    T: 'static,
{
    pub fn push(&mut self, iter: AnonIterMut<T>) {
        self.iters.push(iter)
    }

    pub fn new() -> Self {
        Self {
            iters: Vec::with_capacity(16),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.iters.is_empty()
    }
}

impl<T> Iterator for AnonChainMut<T> {
    type Item = &'static mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr = &mut self.iters[0];

        if curr.curr == curr.len {
            self.iters.remove(0);
            if self.is_empty() {
                return None;
            }
        }
        curr = &mut self.iters[0];
        Some(curr.next_unchecked())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn anon_iter() {

    }

    #[test]
    fn anon_iter_mut() {

    }

    #[test]
    fn anon_chain() {

    }

    #[test]
    fn anon_chain_mut() {

    }
}