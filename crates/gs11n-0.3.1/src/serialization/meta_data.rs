use rustc_hash::FxHashMap;

/// Used to cache information used for encoding, currently only cache size info
/// each value should have a corresponded metadata, the `size` filed represent the space used for
/// this value.
///
/// If the value is a struct, then children save all the info of the sturct's fields.
/// If the value is a vector/map/array/etc, then children save all the info of the vector's elements.
#[derive(Default)]
pub struct Metadata {
    /// the size of represented data
    pub size: usize,
    children: FxHashMap<usize, Metadata>,
}

impl Metadata {
    /// get child metadata by id
    pub fn get(&mut self, index: usize) -> &mut Metadata {
        self.children.entry(index).or_insert_with(Metadata::default);
        self.children.get_mut(&index).unwrap()
    }
}
