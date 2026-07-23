//! Splay tree based data structures
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![warn(missing_docs)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(not(feature = "std"))]
#[macro_use]
pub extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use alloc::*;
    pub use core::{borrow, cmp, fmt, hash, iter, mem, ops, slice, u32};
}

pub mod heap;
mod iter;
pub mod map;
pub mod set;
mod tree_core;
mod vec_like;

#[doc(inline)]
pub use map::SplayMap;

#[doc(inline)]
pub use set::SplaySet;

#[doc(inline)]
pub use heap::SplayHeap;

#[cfg(test)]
mod unsat_ir_instantiations {
    #[test]
    #[ignore = "compile-only LLVM IR instantiation"]
    fn instantiate_tree_core_and_vec_like_helpers() {
        let mut tree = crate::tree_core::Tree::<u32, u32>::new();
        let _ = tree.insert(1, 10);
        let _ = tree.root_ref();

        let view = crate::vec_like::VecLike::new(&tree);
        let _ = view.get(0);

        let mut view_mut = crate::vec_like::VecLikeMut::new(&mut tree);
        let _ = view_mut.get(0);
        let _ = view_mut.get_mut(0);
    }
}
