use std::fmt::Debug;

pub struct Chunker<T> {
    buffer: Vec<T>,
    chunk_size: usize,
    remain_size: usize,
}

impl<T: PartialEq + Clone + Debug> Chunker<T> {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            buffer: Vec::new(),
            chunk_size,
            remain_size: 0,
        }
    }
    fn extract_cache(&mut self) {
        let shrink_to_size = self.buffer.len() - self.remain_size;
        self.buffer.drain(..shrink_to_size);
        self.remain_size = 0;
    }
    pub fn chunk<'a>(&'a mut self, data_input: &[T]) -> Vec<&'a [T]> {
        if data_input.is_empty() {
            return Vec::new();
        }
        self.extract_cache();
        let mut v = Vec::new();
        self.buffer.extend_from_slice(data_input);

        let mut iter = self.buffer.chunks_exact(self.chunk_size);
        for c in iter.by_ref() {
            v.push(c);
        }
        self.remain_size = iter.remainder().len();
        v
    }
}
