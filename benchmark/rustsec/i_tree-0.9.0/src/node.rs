pub const EMPTY_REF: u32 = u32::MAX;

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    Red,
    Black
}

pub struct Node<T> {
    pub parent: u32,
    pub left: u32,
    pub right: u32,
    pub color: Color,
    pub value: T
}