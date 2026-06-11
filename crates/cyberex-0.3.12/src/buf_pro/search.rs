pub fn search_in<T: PartialEq>(input: &[T], iden: &[T]) -> Option<usize> {
    input
        .windows(iden.len())
        .enumerate()
        .find_map(|(i, w)| if w == iden { Some(i) } else { None })
}
pub fn filter_in<T: PartialEq>(input: &[T], iden: &[T]) -> Vec<usize> {
    input
        .windows(iden.len())
        .enumerate()
        .filter_map(|(i, w)| if w == iden { Some(i) } else { None })
        .collect()
}
pub fn filter_in_if<T: PartialEq>(input: &[T], win_size: usize, f: impl Fn(usize, &[T]) -> bool) -> Vec<usize> {
    input
        .windows(win_size)
        .enumerate()
        .filter_map(move |(i, w)| if f(i, w) { Some(i) } else { None })
        .collect()
}
