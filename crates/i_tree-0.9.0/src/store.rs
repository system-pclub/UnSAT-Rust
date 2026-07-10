use crate::node::{Color, EMPTY_REF, Node};

pub struct Store<T> {
    pub(super) buffer: Vec<Node<T>>,
    pub(super) unused: Vec<u32>,
    empty: T,
}

impl<T: Clone> Store<T> {

    #[inline(always)]
    pub(super) fn new(empty: T, capacity: usize) -> Self {
        let capacity = capacity.max(8);
        let mut store = Self {
            buffer: Vec::with_capacity(capacity),
            unused: Vec::with_capacity(capacity),
            empty,
        };
        store.reserve(capacity);
        store
    }

    #[inline]
    fn reserve(&mut self, length: usize) {
        let n = self.buffer.len() as u32;
        let l = length as u32;
        for i in 0..l {
            let node = Node {
                parent: EMPTY_REF,
                left: EMPTY_REF,
                right: EMPTY_REF,
                color: Color::Red,
                value: self.empty.clone(),
            };
            self.buffer.push(node);
            self.unused.push(n + l - i - 1);
        }
    }

    #[inline(always)]
    pub fn get_free_index(&mut self) -> u32 {
        if self.unused.is_empty() {
            let extra_capacity = self.unused.capacity() >> 1;
            self.reserve(extra_capacity);
        }
        self.unused.pop().unwrap()
    }


    #[inline(always)]
    pub fn put_back(&mut self, index: u32) {
        self.unused.push(index)
    }
}