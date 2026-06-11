use super::search::{filter_in, filter_in_if, search_in};

#[allow(non_snake_case)]
pub fn chunkBy_once<'a, T: PartialEq>(input: &'a [T], iden: &[T]) -> Vec<&'a [T]> {
    let mut v = Vec::new();

    {
        let mut o = filter_in(input, iden);
        o.push(input.len());
        o
    }
    .windows(2)
    .for_each(|i| v.push(&input[i[0]..i[1]]));

    v
}

#[allow(non_snake_case)]
pub fn chunkByIf_once<T: PartialEq>(input: &[T], win_size: usize, f: impl Fn(usize, &[T]) -> bool) -> Vec<&[T]> {
    let mut v = Vec::new();

    {
        let mut o = filter_in_if(input, win_size, f);
        o.push(input.len());
        o
    }
    .windows(2)
    .for_each(|i| v.push(&input[i[0]..i[1]]));

    v
}

pub struct ChunkerBy<T> {
    iden: Vec<T>,
    buffer: Vec<T>,
    last_start: Option<usize>,
}

impl<T: PartialEq + Clone> ChunkerBy<T> {
    pub fn new(iden: &[T]) -> Self {
        Self {
            iden: iden.to_vec(),
            buffer: Vec::new(),
            last_start: None,
        }
    }
    fn extract_cache(&mut self) {
        if let Some(last_start) = self.last_start.take() {
            self.buffer.drain(..last_start);
        }
    }
    pub fn flush(&mut self) -> &[T] {
        self.extract_cache();
        &self.buffer
    }
    pub fn chunk<'a>(&'a mut self, data_input: &[T]) -> Vec<&'a [T]> {
        if data_input.is_empty() {
            return Vec::new();
        }
        self.extract_cache();

        self.buffer.extend_from_slice(data_input);

        let mut v = Vec::new();

        let mut nalu_start = None;
        loop {
            match nalu_start {
                None => match search_in(&self.buffer, &self.iden) {
                    Some(start) => {
                        nalu_start = Some(start);
                        continue;
                    },
                    None => {
                        break;
                    },
                },
                Some(start) => {
                    let find_offset = start + self.iden.len();

                    let find_next = search_in(&self.buffer[find_offset..], &self.iden);

                    match find_next {
                        None => {
                            self.last_start = Some(start);
                            break;
                        },
                        Some(next_step) => {
                            let next = find_offset + next_step;

                            let s = start;
                            nalu_start = Some(next);
                            v.push(&self.buffer[s..next]);
                        },
                    }
                },
            }
        }
        v
    }
}
