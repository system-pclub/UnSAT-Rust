# Source-derived Rust member catalog

One catalog for every API and rule. Generic definitions are preserved.

| Receiver | Selector | Result | Source |
| --- | --- | --- | --- |
| `alloc::borrow::Cow` | `.Borrowed` | `alloc::borrow::Cow::Borrowed<$B>` | tools/rust/library/alloc/src/borrow.rs:185 |
| `alloc::borrow::Cow` | `.Owned` | `alloc::borrow::Cow::Owned<$B>` | tools/rust/library/alloc/src/borrow.rs:189 |
| `alloc::borrow::Cow::Borrowed` | `[0]` | `$B` | tools/rust/library/alloc/src/borrow.rs:185 |
| `alloc::borrow::Cow::Owned` | `[0]` | `opaque(< B as ToOwned > :: Owned)` | tools/rust/library/alloc/src/borrow.rs:189 |
| `alloc::boxed::Box` | `[0]` | `core::ptr::unique::Unique<$T>` | tools/rust/library/alloc/src/boxed.rs:234 |
| `alloc::boxed::Box` | `[1]` | `$A` | tools/rust/library/alloc/src/boxed.rs:234 |
| `alloc::boxed::thin::ThinBox` | `.ptr` | `alloc::boxed::thin::WithOpaqueHeader` | tools/rust/library/alloc/src/boxed/thin.rs:40 |
| `alloc::boxed::thin::ThinBox` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/alloc/src/boxed/thin.rs:41 |
| `alloc::boxed::thin::WithHeader` | `[0]` | `core::ptr::non_null::NonNull<u8>` | tools/rust/library/alloc/src/boxed/thin.rs:197 |
| `alloc::boxed::thin::WithHeader` | `[1]` | `core::marker::PhantomData<$H>` | tools/rust/library/alloc/src/boxed/thin.rs:197 |
| `alloc::boxed::thin::WithOpaqueHeader` | `[0]` | `core::ptr::non_null::NonNull<u8>` | tools/rust/library/alloc/src/boxed/thin.rs:202 |
| `alloc::bstr::ByteString` | `[0]` | `alloc::vec::Vec<u8>` | tools/rust/library/alloc/src/bstr.rs:47 |
| `alloc::collections::TryReserveError` | `.kind` | `alloc::collections::TryReserveErrorKind` | tools/rust/library/alloc/src/collections/mod.rs:71 |
| `alloc::collections::TryReserveErrorKind` | `.AllocError` | `alloc::collections::TryReserveErrorKind::AllocError` | tools/rust/library/alloc/src/collections/mod.rs:106 |
| `alloc::collections::TryReserveErrorKind::AllocError` | `.layout` | `alloc::Layout` | tools/rust/library/alloc/src/collections/mod.rs:108 |
| `alloc::collections::TryReserveErrorKind::AllocError` | `.non_exhaustive` | `()` | tools/rust/library/alloc/src/collections/mod.rs:119 |
| `alloc::collections::binary_heap::BinaryHeap` | `.data` | `alloc::vec::Vec<$T, $A>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:278 |
| `alloc::collections::binary_heap::Drain` | `.iter` | `alloc::vec::drain::Drain<$T, $A>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1744 |
| `alloc::collections::binary_heap::DrainSorted` | `.inner` | `alloc::collections::binary_heap::BinaryHeap<$T, $A>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1801 |
| `alloc::collections::binary_heap::Hole` | `.data` | `[$T]` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1433 |
| `alloc::collections::binary_heap::Hole` | `.elt` | `core::mem::manually_drop::ManuallyDrop<$T>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1434 |
| `alloc::collections::binary_heap::Hole` | `.pos` | `usize` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1435 |
| `alloc::collections::binary_heap::IntoIter` | `.iter` | `alloc::vec::into_iter::IntoIter<$T, $A>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1590 |
| `alloc::collections::binary_heap::IntoIterSorted` | `.inner` | `alloc::collections::binary_heap::BinaryHeap<$T, $A>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1695 |
| `alloc::collections::binary_heap::Iter` | `.iter` | `core::slice::iter::Iter<$T>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:1508 |
| `alloc::collections::binary_heap::PeekMut` | `.heap` | `alloc::collections::binary_heap::BinaryHeap<$T, $A>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:294 |
| `alloc::collections::binary_heap::PeekMut` | `.original_len` | `Option<core::num::nonzero::NonZero<usize>>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:297 |
| `alloc::collections::binary_heap::RebuildOnDrop` | `.heap` | `alloc::collections::binary_heap::BinaryHeap<$T, $A>` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:489 |
| `alloc::collections::binary_heap::RebuildOnDrop` | `.rebuild_from` | `usize` | tools/rust/library/alloc/src/collections/binary_heap/mod.rs:490 |
| `alloc::collections::btree::append::MergeIter` | `[0]` | `alloc::collections::btree::merge_iter::MergeIterInner<$I>` | tools/rust/library/alloc/src/collections/btree/append.rs:99 |
| `alloc::collections::btree::borrow::DormantMutRef` | `.ptr` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/alloc/src/collections/btree/borrow.rs:15 |
| `alloc::collections::btree::borrow::DormantMutRef` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/alloc/src/collections/btree/borrow.rs:16 |
| `alloc::collections::btree::dedup_sorted_iter::DedupSortedIter` | `.iter` | `core::iter::adapters::peekable::Peekable<$I>` | tools/rust/library/alloc/src/collections/btree/dedup_sorted_iter.rs:13 |
| `alloc::collections::btree::map::BTreeMap` | `.root` | `Option<alloc::collections::btree::node::Root<$K, $V>>` | tools/rust/library/alloc/src/collections/btree/map.rs:178 |
| `alloc::collections::btree::map::BTreeMap` | `.length` | `usize` | tools/rust/library/alloc/src/collections/btree/map.rs:179 |
| `alloc::collections::btree::map::BTreeMap` | `.alloc` | `core::mem::manually_drop::ManuallyDrop<$A>` | tools/rust/library/alloc/src/collections/btree/map.rs:181 |
| `alloc::collections::btree::map::BTreeMap` | `._marker` | `core::marker::PhantomData<alloc::boxed::Box<($K, $V), $A>>` | tools/rust/library/alloc/src/collections/btree/map.rs:183 |
| `alloc::collections::btree::map::Cursor` | `.current` | `Option<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Immut, $K, $V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>` | tools/rust/library/alloc/src/collections/btree/map.rs:2862 |
| `alloc::collections::btree::map::Cursor` | `.root` | `Option<alloc::collections::btree::node::Root<$K, $V>>` | tools/rust/library/alloc/src/collections/btree/map.rs:2863 |
| `alloc::collections::btree::map::CursorMut` | `.inner` | `alloc::collections::btree::map::CursorMutKey<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map.rs:2900 |
| `alloc::collections::btree::map::CursorMutKey` | `.current` | `Option<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>` | tools/rust/library/alloc/src/collections/btree/map.rs:2939 |
| `alloc::collections::btree::map::CursorMutKey` | `.root` | `alloc::collections::btree::borrow::DormantMutRef<Option<alloc::collections::btree::node::Root<$K, $V>>>` | tools/rust/library/alloc/src/collections/btree/map.rs:2940 |
| `alloc::collections::btree::map::CursorMutKey` | `.length` | `usize` | tools/rust/library/alloc/src/collections/btree/map.rs:2941 |
| `alloc::collections::btree::map::CursorMutKey` | `.alloc` | `$A` | tools/rust/library/alloc/src/collections/btree/map.rs:2942 |
| `alloc::collections::btree::map::ExtractIf` | `.pred` | `$F` | tools/rust/library/alloc/src/collections/btree/map.rs:1923 |
| `alloc::collections::btree::map::ExtractIf` | `.inner` | `alloc::collections::btree::map::ExtractIfInner<$K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:1924 |
| `alloc::collections::btree::map::ExtractIf` | `.alloc` | `$A` | tools/rust/library/alloc/src/collections/btree/map.rs:1926 |
| `alloc::collections::btree::map::ExtractIfInner` | `.length` | `usize` | tools/rust/library/alloc/src/collections/btree/map.rs:1932 |
| `alloc::collections::btree::map::ExtractIfInner` | `.dormant_root` | `Option<alloc::collections::btree::borrow::DormantMutRef<alloc::collections::btree::node::Root<$K, $V>>>` | tools/rust/library/alloc/src/collections/btree/map.rs:1935 |
| `alloc::collections::btree::map::ExtractIfInner` | `.cur_leaf_edge` | `Option<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>` | tools/rust/library/alloc/src/collections/btree/map.rs:1939 |
| `alloc::collections::btree::map::IntoIter` | `.range` | `alloc::collections::btree::navigate::LazyLeafRange<alloc::collections::btree::node::marker::Dying, $K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:430 |
| `alloc::collections::btree::map::IntoIter` | `.length` | `usize` | tools/rust/library/alloc/src/collections/btree/map.rs:431 |
| `alloc::collections::btree::map::IntoIter` | `.alloc` | `$A` | tools/rust/library/alloc/src/collections/btree/map.rs:433 |
| `alloc::collections::btree::map::IntoKeys` | `.inner` | `alloc::collections::btree::map::IntoIter<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map.rs:534 |
| `alloc::collections::btree::map::IntoValues` | `.inner` | `alloc::collections::btree::map::IntoIter<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map.rs:557 |
| `alloc::collections::btree::map::Iter` | `.range` | `alloc::collections::btree::navigate::LazyLeafRange<alloc::collections::btree::node::marker::Immut, $K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:354 |
| `alloc::collections::btree::map::Iter` | `.length` | `usize` | tools/rust/library/alloc/src/collections/btree/map.rs:355 |
| `alloc::collections::btree::map::IterMut` | `.range` | `alloc::collections::btree::navigate::LazyLeafRange<alloc::collections::btree::node::marker::ValMut, $K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:387 |
| `alloc::collections::btree::map::IterMut` | `.length` | `usize` | tools/rust/library/alloc/src/collections/btree/map.rs:388 |
| `alloc::collections::btree::map::IterMut` | `._marker` | `core::marker::PhantomData<($K, $V)>` | tools/rust/library/alloc/src/collections/btree/map.rs:391 |
| `alloc::collections::btree::map::Keys` | `.inner` | `alloc::collections::btree::map::Iter<$K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:477 |
| `alloc::collections::btree::map::Range` | `.inner` | `alloc::collections::btree::navigate::LeafRange<alloc::collections::btree::node::marker::Immut, $K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:576 |
| `alloc::collections::btree::map::RangeMut` | `.inner` | `alloc::collections::btree::navigate::LeafRange<alloc::collections::btree::node::marker::ValMut, $K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:595 |
| `alloc::collections::btree::map::RangeMut` | `._marker` | `core::marker::PhantomData<($K, $V)>` | tools/rust/library/alloc/src/collections/btree/map.rs:598 |
| `alloc::collections::btree::map::Values` | `.inner` | `alloc::collections::btree::map::Iter<$K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:496 |
| `alloc::collections::btree::map::ValuesMut` | `.inner` | `alloc::collections::btree::map::IterMut<$K, $V>` | tools/rust/library/alloc/src/collections/btree/map.rs:515 |
| `alloc::collections::btree::map::entry::Entry` | `.Vacant` | `alloc::collections::btree::map::entry::Entry::Vacant<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:27 |
| `alloc::collections::btree::map::entry::Entry` | `.Occupied` | `alloc::collections::btree::map::entry::Entry::Occupied<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:31 |
| `alloc::collections::btree::map::entry::Entry::Occupied` | `[0]` | `alloc::collections::btree::map::entry::OccupiedEntry<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:31 |
| `alloc::collections::btree::map::entry::Entry::Vacant` | `[0]` | `alloc::collections::btree::map::entry::VacantEntry<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:27 |
| `alloc::collections::btree::map::entry::OccupiedEntry` | `.handle` | `alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, alloc::collections::btree::node::marker::LeafOrInternal>, alloc::collections::btree::node::marker::KV>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:81 |
| `alloc::collections::btree::map::entry::OccupiedEntry` | `.dormant_map` | `alloc::collections::btree::borrow::DormantMutRef<alloc::collections::btree::map::BTreeMap<$K, $V, $A>>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:82 |
| `alloc::collections::btree::map::entry::OccupiedEntry` | `.alloc` | `$A` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:85 |
| `alloc::collections::btree::map::entry::OccupiedEntry` | `._marker` | `core::marker::PhantomData<($K, $V)>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:88 |
| `alloc::collections::btree::map::entry::OccupiedError` | `.entry` | `alloc::collections::btree::map::entry::OccupiedEntry<$K, $V, $A>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:104 |
| `alloc::collections::btree::map::entry::OccupiedError` | `.value` | `$V` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:106 |
| `alloc::collections::btree::map::entry::VacantEntry` | `.key` | `$K` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:53 |
| `alloc::collections::btree::map::entry::VacantEntry` | `.handle` | `Option<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:55 |
| `alloc::collections::btree::map::entry::VacantEntry` | `.dormant_map` | `alloc::collections::btree::borrow::DormantMutRef<alloc::collections::btree::map::BTreeMap<$K, $V, $A>>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:56 |
| `alloc::collections::btree::map::entry::VacantEntry` | `.alloc` | `$A` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:59 |
| `alloc::collections::btree::map::entry::VacantEntry` | `._marker` | `core::marker::PhantomData<($K, $V)>` | tools/rust/library/alloc/src/collections/btree/map/entry.rs:62 |
| `alloc::collections::btree::merge_iter::MergeIterInner` | `.a` | `$I` | tools/rust/library/alloc/src/collections/btree/merge_iter.rs:8 |
| `alloc::collections::btree::merge_iter::MergeIterInner` | `.b` | `$I` | tools/rust/library/alloc/src/collections/btree/merge_iter.rs:9 |
| `alloc::collections::btree::merge_iter::MergeIterInner` | `.peeked` | `Option<alloc::collections::btree::merge_iter::Peeked<$I>>` | tools/rust/library/alloc/src/collections/btree/merge_iter.rs:10 |
| `alloc::collections::btree::merge_iter::Peeked` | `.A` | `alloc::collections::btree::merge_iter::Peeked::A<$I>` | tools/rust/library/alloc/src/collections/btree/merge_iter.rs:17 |
| `alloc::collections::btree::merge_iter::Peeked` | `.B` | `alloc::collections::btree::merge_iter::Peeked::B<$I>` | tools/rust/library/alloc/src/collections/btree/merge_iter.rs:18 |
| `alloc::collections::btree::merge_iter::Peeked::A` | `[0]` | `I::Item` | tools/rust/library/alloc/src/collections/btree/merge_iter.rs:17 |
| `alloc::collections::btree::merge_iter::Peeked::B` | `[0]` | `I::Item` | tools/rust/library/alloc/src/collections/btree/merge_iter.rs:18 |
| `alloc::collections::btree::navigate::LazyLeafHandle` | `.Root` | `alloc::collections::btree::navigate::LazyLeafHandle::Root<$BorrowType, $K, $V>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:104 |
| `alloc::collections::btree::navigate::LazyLeafHandle` | `.Edge` | `alloc::collections::btree::navigate::LazyLeafHandle::Edge<$BorrowType, $K, $V>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:105 |
| `alloc::collections::btree::navigate::LazyLeafHandle::Edge` | `[0]` | `alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:105 |
| `alloc::collections::btree::navigate::LazyLeafHandle::Root` | `[0]` | `alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, alloc::collections::btree::node::marker::LeafOrInternal>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:104 |
| `alloc::collections::btree::navigate::LazyLeafRange` | `.front` | `Option<alloc::collections::btree::navigate::LazyLeafHandle<$BorrowType, $K, $V>>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:128 |
| `alloc::collections::btree::navigate::LazyLeafRange` | `.back` | `Option<alloc::collections::btree::navigate::LazyLeafHandle<$BorrowType, $K, $V>>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:129 |
| `alloc::collections::btree::navigate::LeafRange` | `.front` | `Option<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:11 |
| `alloc::collections::btree::navigate::LeafRange` | `.back` | `Option<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:12 |
| `alloc::collections::btree::navigate::Position` | `.Leaf` | `alloc::collections::btree::navigate::Position::Leaf<$BorrowType, $K, $V>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:659 |
| `alloc::collections::btree::navigate::Position` | `.Internal` | `alloc::collections::btree::navigate::Position::Internal<$BorrowType, $K, $V>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:660 |
| `alloc::collections::btree::navigate::Position::Internal` | `[0]` | `alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, alloc::collections::btree::node::marker::Internal>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:660 |
| `alloc::collections::btree::navigate::Position::Leaf` | `[0]` | `alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, alloc::collections::btree::node::marker::Leaf>` | tools/rust/library/alloc/src/collections/btree/navigate.rs:659 |
| `alloc::collections::btree::node::BalancingContext` | `.parent` | `alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, alloc::collections::btree::node::marker::Internal>, alloc::collections::btree::node::marker::KV>` | tools/rust/library/alloc/src/collections/btree/node.rs:1304 |
| `alloc::collections::btree::node::BalancingContext` | `.left_child` | `alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, alloc::collections::btree::node::marker::LeafOrInternal>` | tools/rust/library/alloc/src/collections/btree/node.rs:1305 |
| `alloc::collections::btree::node::BalancingContext` | `.right_child` | `alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, alloc::collections::btree::node::marker::LeafOrInternal>` | tools/rust/library/alloc/src/collections/btree/node.rs:1306 |
| `alloc::collections::btree::node::BoxedNode` | `(alias)` | `core::ptr::non_null::NonNull<alloc::collections::btree::node::LeafNode<$K, $V>>` | tools/rust/library/alloc/src/collections/btree/node.rs:129 |
| `alloc::collections::btree::node::ForceResult` | `.Leaf` | `alloc::collections::btree::node::ForceResult::Leaf<$Leaf, $Internal>` | tools/rust/library/alloc/src/collections/btree/node.rs:1745 |
| `alloc::collections::btree::node::ForceResult` | `.Internal` | `alloc::collections::btree::node::ForceResult::Internal<$Leaf, $Internal>` | tools/rust/library/alloc/src/collections/btree/node.rs:1746 |
| `alloc::collections::btree::node::ForceResult::Internal` | `[0]` | `$Internal` | tools/rust/library/alloc/src/collections/btree/node.rs:1746 |
| `alloc::collections::btree::node::ForceResult::Leaf` | `[0]` | `$Leaf` | tools/rust/library/alloc/src/collections/btree/node.rs:1745 |
| `alloc::collections::btree::node::Handle` | `.node` | `$Node` | tools/rust/library/alloc/src/collections/btree/node.rs:763 |
| `alloc::collections::btree::node::Handle` | `.idx` | `usize` | tools/rust/library/alloc/src/collections/btree/node.rs:764 |
| `alloc::collections::btree::node::Handle` | `._marker` | `core::marker::PhantomData<$Type>` | tools/rust/library/alloc/src/collections/btree/node.rs:765 |
| `alloc::collections::btree::node::InternalNode` | `.data` | `alloc::collections::btree::node::LeafNode<$K, $V>` | tools/rust/library/alloc/src/collections/btree/node.rs:98 |
| `alloc::collections::btree::node::InternalNode` | `.edges` | `[core::mem::MaybeUninit<alloc::collections::btree::node::BoxedNode<$K, $V>>; 2 * B]` | tools/rust/library/alloc/src/collections/btree/node.rs:103 |
| `alloc::collections::btree::node::LeafNode` | `.parent` | `Option<core::ptr::non_null::NonNull<alloc::collections::btree::node::InternalNode<$K, $V>>>` | tools/rust/library/alloc/src/collections/btree/node.rs:52 |
| `alloc::collections::btree::node::LeafNode` | `.parent_idx` | `core::mem::MaybeUninit<u16>` | tools/rust/library/alloc/src/collections/btree/node.rs:57 |
| `alloc::collections::btree::node::LeafNode` | `.len` | `u16` | tools/rust/library/alloc/src/collections/btree/node.rs:60 |
| `alloc::collections::btree::node::LeafNode` | `.keys` | `[core::mem::MaybeUninit<$K>; CAPACITY]` | tools/rust/library/alloc/src/collections/btree/node.rs:64 |
| `alloc::collections::btree::node::LeafNode` | `.vals` | `[core::mem::MaybeUninit<$V>; CAPACITY]` | tools/rust/library/alloc/src/collections/btree/node.rs:65 |
| `alloc::collections::btree::node::LeftOrRight` | `.Left` | `alloc::collections::btree::node::LeftOrRight::Left<$T>` | tools/rust/library/alloc/src/collections/btree/node.rs:896 |
| `alloc::collections::btree::node::LeftOrRight` | `.Right` | `alloc::collections::btree::node::LeftOrRight::Right<$T>` | tools/rust/library/alloc/src/collections/btree/node.rs:897 |
| `alloc::collections::btree::node::LeftOrRight::Left` | `[0]` | `$T` | tools/rust/library/alloc/src/collections/btree/node.rs:896 |
| `alloc::collections::btree::node::LeftOrRight::Right` | `[0]` | `$T` | tools/rust/library/alloc/src/collections/btree/node.rs:897 |
| `alloc::collections::btree::node::NodeRef` | `.height` | `usize` | tools/rust/library/alloc/src/collections/btree/node.rs:188 |
| `alloc::collections::btree::node::NodeRef` | `.node` | `core::ptr::non_null::NonNull<alloc::collections::btree::node::LeafNode<$K, $V>>` | tools/rust/library/alloc/src/collections/btree/node.rs:191 |
| `alloc::collections::btree::node::NodeRef` | `._marker` | `core::marker::PhantomData<($BorrowType, $Type)>` | tools/rust/library/alloc/src/collections/btree/node.rs:192 |
| `alloc::collections::btree::node::Root` | `(alias)` | `alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Owned, $K, $V, alloc::collections::btree::node::marker::LeafOrInternal>` | tools/rust/library/alloc/src/collections/btree/node.rs:198 |
| `alloc::collections::btree::node::SplitResult` | `.left` | `alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, $K, $V, $NodeType>` | tools/rust/library/alloc/src/collections/btree/node.rs:1752 |
| `alloc::collections::btree::node::SplitResult` | `.kv` | `($K, $V)` | tools/rust/library/alloc/src/collections/btree/node.rs:1754 |
| `alloc::collections::btree::node::SplitResult` | `.right` | `alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Owned, $K, $V, $NodeType>` | tools/rust/library/alloc/src/collections/btree/node.rs:1756 |
| `alloc::collections::btree::node::marker::Immut` | `[0]` | `core::marker::PhantomData<()>` | tools/rust/library/alloc/src/collections/btree/node.rs:1781 |
| `alloc::collections::btree::node::marker::Mut` | `[0]` | `core::marker::PhantomData<()>` | tools/rust/library/alloc/src/collections/btree/node.rs:1782 |
| `alloc::collections::btree::node::marker::ValMut` | `[0]` | `core::marker::PhantomData<()>` | tools/rust/library/alloc/src/collections/btree/node.rs:1783 |
| `alloc::collections::btree::search::IndexResult` | `.KV` | `alloc::collections::btree::search::IndexResult::KV` | tools/rust/library/alloc/src/collections/btree/search.rs:38 |
| `alloc::collections::btree::search::IndexResult` | `.Edge` | `alloc::collections::btree::search::IndexResult::Edge` | tools/rust/library/alloc/src/collections/btree/search.rs:39 |
| `alloc::collections::btree::search::IndexResult::Edge` | `[0]` | `usize` | tools/rust/library/alloc/src/collections/btree/search.rs:39 |
| `alloc::collections::btree::search::IndexResult::KV` | `[0]` | `usize` | tools/rust/library/alloc/src/collections/btree/search.rs:38 |
| `alloc::collections::btree::search::SearchBound` | `.Included` | `alloc::collections::btree::search::SearchBound::Included<$T>` | tools/rust/library/alloc/src/collections/btree/search.rs:13 |
| `alloc::collections::btree::search::SearchBound` | `.Excluded` | `alloc::collections::btree::search::SearchBound::Excluded<$T>` | tools/rust/library/alloc/src/collections/btree/search.rs:15 |
| `alloc::collections::btree::search::SearchBound::Excluded` | `[0]` | `$T` | tools/rust/library/alloc/src/collections/btree/search.rs:15 |
| `alloc::collections::btree::search::SearchBound::Included` | `[0]` | `$T` | tools/rust/library/alloc/src/collections/btree/search.rs:13 |
| `alloc::collections::btree::search::SearchResult` | `.Found` | `alloc::collections::btree::search::SearchResult::Found<$BorrowType, $K, $V, $FoundType, $GoDownType>` | tools/rust/library/alloc/src/collections/btree/search.rs:33 |
| `alloc::collections::btree::search::SearchResult` | `.GoDown` | `alloc::collections::btree::search::SearchResult::GoDown<$BorrowType, $K, $V, $FoundType, $GoDownType>` | tools/rust/library/alloc/src/collections/btree/search.rs:34 |
| `alloc::collections::btree::search::SearchResult::Found` | `[0]` | `alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, $FoundType>, alloc::collections::btree::node::marker::KV>` | tools/rust/library/alloc/src/collections/btree/search.rs:33 |
| `alloc::collections::btree::search::SearchResult::GoDown` | `[0]` | `alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<$BorrowType, $K, $V, $GoDownType>, alloc::collections::btree::node::marker::Edge>` | tools/rust/library/alloc/src/collections/btree/search.rs:34 |
| `alloc::collections::btree::set::BTreeSet` | `.map` | `alloc::collections::btree::map::BTreeMap<$T, alloc::collections::btree::set_val::SetValZST, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:82 |
| `alloc::collections::btree::set::Cursor` | `.inner` | `alloc::collections::btree::map::Cursor<$K, alloc::collections::btree::set_val::SetValZST>` | tools/rust/library/alloc/src/collections/btree/set.rs:2115 |
| `alloc::collections::btree::set::CursorMut` | `.inner` | `alloc::collections::btree::map::CursorMut<$K, alloc::collections::btree::set_val::SetValZST, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:2140 |
| `alloc::collections::btree::set::CursorMutKey` | `.inner` | `alloc::collections::btree::map::CursorMutKey<$K, alloc::collections::btree::set_val::SetValZST, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:2177 |
| `alloc::collections::btree::set::Difference` | `.inner` | `alloc::collections::btree::set::DifferenceInner<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:188 |
| `alloc::collections::btree::set::DifferenceInner` | `.Stitch` | `alloc::collections::btree::set::DifferenceInner::Stitch<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:191 |
| `alloc::collections::btree::set::DifferenceInner` | `.Search` | `alloc::collections::btree::set::DifferenceInner::Search<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:196 |
| `alloc::collections::btree::set::DifferenceInner` | `.Iterate` | `alloc::collections::btree::set::DifferenceInner::Iterate<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:201 |
| `alloc::collections::btree::set::DifferenceInner::Iterate` | `[0]` | `alloc::collections::btree::set::Iter<$T>` | tools/rust/library/alloc/src/collections/btree/set.rs:201 |
| `alloc::collections::btree::set::DifferenceInner::Search` | `.self_iter` | `alloc::collections::btree::set::Iter<$T>` | tools/rust/library/alloc/src/collections/btree/set.rs:198 |
| `alloc::collections::btree::set::DifferenceInner::Search` | `.other_set` | `alloc::collections::btree::set::BTreeSet<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:199 |
| `alloc::collections::btree::set::DifferenceInner::Stitch` | `.self_iter` | `alloc::collections::btree::set::Iter<$T>` | tools/rust/library/alloc/src/collections/btree/set.rs:193 |
| `alloc::collections::btree::set::DifferenceInner::Stitch` | `.other_iter` | `core::iter::adapters::peekable::Peekable<alloc::collections::btree::set::Iter<$T>>` | tools/rust/library/alloc/src/collections/btree/set.rs:194 |
| `alloc::collections::btree::set::ExtractIf` | `.pred` | `$F` | tools/rust/library/alloc/src/collections/btree/set.rs:1563 |
| `alloc::collections::btree::set::ExtractIf` | `.inner` | `alloc::collections::btree::map::ExtractIfInner<$T, alloc::collections::btree::set_val::SetValZST>` | tools/rust/library/alloc/src/collections/btree/set.rs:1564 |
| `alloc::collections::btree::set::ExtractIf` | `.alloc` | `$A` | tools/rust/library/alloc/src/collections/btree/set.rs:1566 |
| `alloc::collections::btree::set::Intersection` | `.inner` | `alloc::collections::btree::set::IntersectionInner<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:262 |
| `alloc::collections::btree::set::IntersectionInner` | `.Stitch` | `alloc::collections::btree::set::IntersectionInner::Stitch<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:265 |
| `alloc::collections::btree::set::IntersectionInner` | `.Search` | `alloc::collections::btree::set::IntersectionInner::Search<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:270 |
| `alloc::collections::btree::set::IntersectionInner` | `.Answer` | `alloc::collections::btree::set::IntersectionInner::Answer<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:275 |
| `alloc::collections::btree::set::IntersectionInner::Answer` | `[0]` | `Option<$T>` | tools/rust/library/alloc/src/collections/btree/set.rs:275 |
| `alloc::collections::btree::set::IntersectionInner::Search` | `.small_iter` | `alloc::collections::btree::set::Iter<$T>` | tools/rust/library/alloc/src/collections/btree/set.rs:272 |
| `alloc::collections::btree::set::IntersectionInner::Search` | `.large_set` | `alloc::collections::btree::set::BTreeSet<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:273 |
| `alloc::collections::btree::set::IntersectionInner::Stitch` | `.a` | `alloc::collections::btree::set::Iter<$T>` | tools/rust/library/alloc/src/collections/btree/set.rs:267 |
| `alloc::collections::btree::set::IntersectionInner::Stitch` | `.b` | `alloc::collections::btree::set::Iter<$T>` | tools/rust/library/alloc/src/collections/btree/set.rs:268 |
| `alloc::collections::btree::set::IntoIter` | `.iter` | `alloc::collections::btree::map::IntoIter<$T, alloc::collections::btree::set_val::SetValZST, $A>` | tools/rust/library/alloc/src/collections/btree/set.rs:158 |
| `alloc::collections::btree::set::Iter` | `.iter` | `alloc::collections::btree::map::Keys<$T, alloc::collections::btree::set_val::SetValZST>` | tools/rust/library/alloc/src/collections/btree/set.rs:136 |
| `alloc::collections::btree::set::Range` | `.iter` | `alloc::collections::btree::map::Range<$T, alloc::collections::btree::set_val::SetValZST>` | tools/rust/library/alloc/src/collections/btree/set.rs:171 |
| `alloc::collections::btree::set::SymmetricDifference` | `[0]` | `alloc::collections::btree::merge_iter::MergeIterInner<alloc::collections::btree::set::Iter<$T>>` | tools/rust/library/alloc/src/collections/btree/set.rs:239 |
| `alloc::collections::btree::set::Union` | `[0]` | `alloc::collections::btree::merge_iter::MergeIterInner<alloc::collections::btree::set::Iter<$T>>` | tools/rust/library/alloc/src/collections/btree/set.rs:311 |
| `alloc::collections::btree::set::entry::Entry` | `.Occupied` | `alloc::collections::btree::set::entry::Entry::Occupied<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set/entry.rs:64 |
| `alloc::collections::btree::set::entry::Entry` | `.Vacant` | `alloc::collections::btree::set::entry::Entry::Vacant<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set/entry.rs:83 |
| `alloc::collections::btree::set::entry::Entry::Occupied` | `[0]` | `alloc::collections::btree::set::entry::OccupiedEntry<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set/entry.rs:64 |
| `alloc::collections::btree::set::entry::Entry::Vacant` | `[0]` | `alloc::collections::btree::set::entry::VacantEntry<$T, $A>` | tools/rust/library/alloc/src/collections/btree/set/entry.rs:83 |
| `alloc::collections::btree::set::entry::OccupiedEntry` | `.inner` | `alloc::collections::btree::map::entry::OccupiedEntry<$T, alloc::collections::btree::set_val::SetValZST, $A>` | tools/rust/library/alloc/src/collections/btree/set/entry.rs:138 |
| `alloc::collections::btree::set::entry::VacantEntry` | `.inner` | `alloc::collections::btree::map::entry::VacantEntry<$T, alloc::collections::btree::set_val::SetValZST, $A>` | tools/rust/library/alloc/src/collections/btree/set/entry.rs:180 |
| `alloc::collections::linked_list::Cursor` | `.index` | `usize` | tools/rust/library/alloc/src/collections/linked_list.rs:1331 |
| `alloc::collections::linked_list::Cursor` | `.current` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:1332 |
| `alloc::collections::linked_list::Cursor` | `.list` | `alloc::collections::linked_list::LinkedList<$T, $A>` | tools/rust/library/alloc/src/collections/linked_list.rs:1333 |
| `alloc::collections::linked_list::CursorMut` | `.index` | `usize` | tools/rust/library/alloc/src/collections/linked_list.rs:1367 |
| `alloc::collections::linked_list::CursorMut` | `.current` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:1368 |
| `alloc::collections::linked_list::CursorMut` | `.list` | `alloc::collections::linked_list::LinkedList<$T, $A>` | tools/rust/library/alloc/src/collections/linked_list.rs:1369 |
| `alloc::collections::linked_list::ExtractIf` | `.list` | `alloc::collections::linked_list::LinkedList<$T, $A>` | tools/rust/library/alloc/src/collections/linked_list.rs:1942 |
| `alloc::collections::linked_list::ExtractIf` | `.it` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:1943 |
| `alloc::collections::linked_list::ExtractIf` | `.pred` | `$F` | tools/rust/library/alloc/src/collections/linked_list.rs:1944 |
| `alloc::collections::linked_list::ExtractIf` | `.idx` | `usize` | tools/rust/library/alloc/src/collections/linked_list.rs:1945 |
| `alloc::collections::linked_list::ExtractIf` | `.old_len` | `usize` | tools/rust/library/alloc/src/collections/linked_list.rs:1946 |
| `alloc::collections::linked_list::IntoIter` | `.list` | `alloc::collections::linked_list::LinkedList<$T, $A>` | tools/rust/library/alloc/src/collections/linked_list.rs:145 |
| `alloc::collections::linked_list::Iter` | `.head` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:74 |
| `alloc::collections::linked_list::Iter` | `.tail` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:75 |
| `alloc::collections::linked_list::Iter` | `.len` | `usize` | tools/rust/library/alloc/src/collections/linked_list.rs:76 |
| `alloc::collections::linked_list::Iter` | `.marker` | `core::marker::PhantomData<alloc::collections::linked_list::Node<$T>>` | tools/rust/library/alloc/src/collections/linked_list.rs:77 |
| `alloc::collections::linked_list::IterMut` | `.head` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:111 |
| `alloc::collections::linked_list::IterMut` | `.tail` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:112 |
| `alloc::collections::linked_list::IterMut` | `.len` | `usize` | tools/rust/library/alloc/src/collections/linked_list.rs:113 |
| `alloc::collections::linked_list::IterMut` | `.marker` | `core::marker::PhantomData<alloc::collections::linked_list::Node<$T>>` | tools/rust/library/alloc/src/collections/linked_list.rs:114 |
| `alloc::collections::linked_list::LinkedList` | `.head` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:54 |
| `alloc::collections::linked_list::LinkedList` | `.tail` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:55 |
| `alloc::collections::linked_list::LinkedList` | `.len` | `usize` | tools/rust/library/alloc/src/collections/linked_list.rs:56 |
| `alloc::collections::linked_list::LinkedList` | `.alloc` | `$A` | tools/rust/library/alloc/src/collections/linked_list.rs:57 |
| `alloc::collections::linked_list::LinkedList` | `.marker` | `core::marker::PhantomData<alloc::boxed::Box<alloc::collections::linked_list::Node<$T>, $A>>` | tools/rust/library/alloc/src/collections/linked_list.rs:58 |
| `alloc::collections::linked_list::Node` | `.next` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:62 |
| `alloc::collections::linked_list::Node` | `.prev` | `Option<core::ptr::non_null::NonNull<alloc::collections::linked_list::Node<$T>>>` | tools/rust/library/alloc/src/collections/linked_list.rs:63 |
| `alloc::collections::linked_list::Node` | `.element` | `$T` | tools/rust/library/alloc/src/collections/linked_list.rs:64 |
| `alloc::collections::vec_deque::VecDeque` | `.head` | `usize` | tools/rust/library/alloc/src/collections/vec_deque/mod.rs:96 |
| `alloc::collections::vec_deque::VecDeque` | `.len` | `usize` | tools/rust/library/alloc/src/collections/vec_deque/mod.rs:100 |
| `alloc::collections::vec_deque::VecDeque` | `.buf` | `alloc::raw_vec::RawVec<$T, $A>` | tools/rust/library/alloc/src/collections/vec_deque/mod.rs:101 |
| `alloc::collections::vec_deque::drain::Drain` | `.deque` | `core::ptr::non_null::NonNull<alloc::collections::vec_deque::VecDeque<$T, $A>>` | tools/rust/library/alloc/src/collections/vec_deque/drain.rs:24 |
| `alloc::collections::vec_deque::drain::Drain` | `.drain_len` | `usize` | tools/rust/library/alloc/src/collections/vec_deque/drain.rs:26 |
| `alloc::collections::vec_deque::drain::Drain` | `.idx` | `usize` | tools/rust/library/alloc/src/collections/vec_deque/drain.rs:28 |
| `alloc::collections::vec_deque::drain::Drain` | `.new_len` | `usize` | tools/rust/library/alloc/src/collections/vec_deque/drain.rs:30 |
| `alloc::collections::vec_deque::drain::Drain` | `.remaining` | `usize` | tools/rust/library/alloc/src/collections/vec_deque/drain.rs:31 |
| `alloc::collections::vec_deque::drain::Drain` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/alloc/src/collections/vec_deque/drain.rs:33 |
| `alloc::collections::vec_deque::into_iter::IntoIter` | `.inner` | `alloc::collections::vec_deque::VecDeque<$T, $A>` | tools/rust/library/alloc/src/collections/vec_deque/into_iter.rs:22 |
| `alloc::collections::vec_deque::iter::Iter` | `.i1` | `core::slice::iter::Iter<$T>` | tools/rust/library/alloc/src/collections/vec_deque/iter.rs:14 |
| `alloc::collections::vec_deque::iter::Iter` | `.i2` | `core::slice::iter::Iter<$T>` | tools/rust/library/alloc/src/collections/vec_deque/iter.rs:15 |
| `alloc::collections::vec_deque::iter_mut::IterMut` | `.i1` | `core::slice::iter::IterMut<$T>` | tools/rust/library/alloc/src/collections/vec_deque/iter_mut.rs:14 |
| `alloc::collections::vec_deque::iter_mut::IterMut` | `.i2` | `core::slice::iter::IterMut<$T>` | tools/rust/library/alloc/src/collections/vec_deque/iter_mut.rs:15 |
| `alloc::ffi::c_str::CString` | `.inner` | `alloc::boxed::Box<[u8]>` | tools/rust/library/alloc/src/ffi/c_str.rs:111 |
| `alloc::ffi::c_str::FromBytesWithNulErrorKind` | `.InteriorNul` | `alloc::ffi::c_str::FromBytesWithNulErrorKind::InteriorNul` | tools/rust/library/alloc/src/ffi/c_str.rs:135 |
| `alloc::ffi::c_str::FromBytesWithNulErrorKind::InteriorNul` | `[0]` | `usize` | tools/rust/library/alloc/src/ffi/c_str.rs:135 |
| `alloc::ffi::c_str::FromVecWithNulError` | `.error_kind` | `alloc::ffi::c_str::FromBytesWithNulErrorKind` | tools/rust/library/alloc/src/ffi/c_str.rs:157 |
| `alloc::ffi::c_str::FromVecWithNulError` | `.bytes` | `alloc::vec::Vec<u8>` | tools/rust/library/alloc/src/ffi/c_str.rs:158 |
| `alloc::ffi::c_str::IntoStringError` | `.inner` | `alloc::ffi::c_str::CString` | tools/rust/library/alloc/src/ffi/c_str.rs:223 |
| `alloc::ffi::c_str::IntoStringError` | `.error` | `core::str::error::Utf8Error` | tools/rust/library/alloc/src/ffi/c_str.rs:224 |
| `alloc::ffi::c_str::NulError` | `[0]` | `usize` | tools/rust/library/alloc/src/ffi/c_str.rs:131 |
| `alloc::ffi::c_str::NulError` | `[1]` | `alloc::vec::Vec<u8>` | tools/rust/library/alloc/src/ffi/c_str.rs:131 |
| `alloc::raw_vec::Cap` | `(alias)` | `core::num::niche_types::UsizeNoHighBit` | tools/rust/library/alloc/src/raw_vec/mod.rs:40 |
| `alloc::raw_vec::RawVec` | `.inner` | `alloc::raw_vec::RawVecInner<$A>` | tools/rust/library/alloc/src/raw_vec/mod.rs:75 |
| `alloc::raw_vec::RawVec` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/alloc/src/raw_vec/mod.rs:76 |
| `alloc::raw_vec::RawVecInner` | `.ptr` | `core::ptr::unique::Unique<u8>` | tools/rust/library/alloc/src/raw_vec/mod.rs:87 |
| `alloc::raw_vec::RawVecInner` | `.cap` | `alloc::raw_vec::Cap` | tools/rust/library/alloc/src/raw_vec/mod.rs:93 |
| `alloc::raw_vec::RawVecInner` | `.alloc` | `$A` | tools/rust/library/alloc/src/raw_vec/mod.rs:94 |
| `alloc::rc::Rc` | `.ptr` | `core::ptr::non_null::NonNull<alloc::rc::RcInner<$T>>` | tools/rust/library/alloc/src/rc.rs:314 |
| `alloc::rc::Rc` | `.phantom` | `core::marker::PhantomData<alloc::rc::RcInner<$T>>` | tools/rust/library/alloc/src/rc.rs:315 |
| `alloc::rc::Rc` | `.alloc` | `$A` | tools/rust/library/alloc/src/rc.rs:316 |
| `alloc::rc::RcInner` | `.strong` | `core::cell::Cell<usize>` | tools/rust/library/alloc/src/rc.rs:282 |
| `alloc::rc::RcInner` | `.weak` | `core::cell::Cell<usize>` | tools/rust/library/alloc/src/rc.rs:283 |
| `alloc::rc::RcInner` | `.value` | `$T` | tools/rust/library/alloc/src/rc.rs:284 |
| `alloc::rc::UniqueRc` | `.ptr` | `core::ptr::non_null::NonNull<alloc::rc::RcInner<$T>>` | tools/rust/library/alloc/src/rc.rs:3724 |
| `alloc::rc::UniqueRc` | `._marker` | `core::marker::PhantomData<alloc::rc::RcInner<$T>>` | tools/rust/library/alloc/src/rc.rs:3726 |
| `alloc::rc::UniqueRc` | `._marker2` | `core::marker::PhantomData<*mut $T>` | tools/rust/library/alloc/src/rc.rs:3729 |
| `alloc::rc::UniqueRc` | `.alloc` | `$A` | tools/rust/library/alloc/src/rc.rs:3730 |
| `alloc::rc::UniqueRcUninit` | `.ptr` | `core::ptr::non_null::NonNull<alloc::rc::RcInner<$T>>` | tools/rust/library/alloc/src/rc.rs:4106 |
| `alloc::rc::UniqueRcUninit` | `.layout_for_value` | `alloc::Layout` | tools/rust/library/alloc/src/rc.rs:4107 |
| `alloc::rc::UniqueRcUninit` | `.alloc` | `Option<$A>` | tools/rust/library/alloc/src/rc.rs:4108 |
| `alloc::rc::Weak` | `.ptr` | `core::ptr::non_null::NonNull<alloc::rc::RcInner<$T>>` | tools/rust/library/alloc/src/rc.rs:3008 |
| `alloc::rc::Weak` | `.alloc` | `$A` | tools/rust/library/alloc/src/rc.rs:3009 |
| `alloc::rc::WeakInner` | `.weak` | `core::cell::Cell<usize>` | tools/rust/library/alloc/src/rc.rs:3075 |
| `alloc::rc::WeakInner` | `.strong` | `core::cell::Cell<usize>` | tools/rust/library/alloc/src/rc.rs:3076 |
| `alloc::string::Drain` | `.string` | `*mut alloc::string::String` | tools/rust/library/alloc/src/string.rs:3314 |
| `alloc::string::Drain` | `.start` | `usize` | tools/rust/library/alloc/src/string.rs:3316 |
| `alloc::string::Drain` | `.end` | `usize` | tools/rust/library/alloc/src/string.rs:3318 |
| `alloc::string::Drain` | `.iter` | `core::str::iter::Chars` | tools/rust/library/alloc/src/string.rs:3320 |
| `alloc::string::FromUtf16Error` | `[0]` | `()` | tools/rust/library/alloc/src/string.rs:420 |
| `alloc::string::FromUtf8Error` | `.bytes` | `alloc::vec::Vec<u8>` | tools/rust/library/alloc/src/string.rs:399 |
| `alloc::string::FromUtf8Error` | `.error` | `core::str::error::Utf8Error` | tools/rust/library/alloc/src/string.rs:400 |
| `alloc::string::IntoChars` | `.bytes` | `alloc::vec::into_iter::IntoIter<u8>` | tools/rust/library/alloc/src/string.rs:3188 |
| `alloc::string::ParseError` | `(alias)` | `core::convert::Infallible` | tools/rust/library/alloc/src/string.rs:2697 |
| `alloc::string::String` | `.vec` | `alloc::vec::Vec<u8>` | tools/rust/library/alloc/src/string.rs:361 |
| `alloc::sync::Arc` | `.ptr` | `core::ptr::non_null::NonNull<alloc::sync::ArcInner<$T>>` | tools/rust/library/alloc/src/sync.rs:265 |
| `alloc::sync::Arc` | `.phantom` | `core::marker::PhantomData<alloc::sync::ArcInner<$T>>` | tools/rust/library/alloc/src/sync.rs:266 |
| `alloc::sync::Arc` | `.alloc` | `$A` | tools/rust/library/alloc/src/sync.rs:267 |
| `alloc::sync::ArcInner` | `.strong` | `core::sync::atomic::AtomicUsize` | tools/rust/library/alloc/src/sync.rs:372 |
| `alloc::sync::ArcInner` | `.weak` | `core::sync::atomic::AtomicUsize` | tools/rust/library/alloc/src/sync.rs:377 |
| `alloc::sync::ArcInner` | `.data` | `$T` | tools/rust/library/alloc/src/sync.rs:379 |
| `alloc::sync::SliceArcInnerForStatic` | `.inner` | `alloc::sync::ArcInner<[u8; 1]>` | tools/rust/library/alloc/src/sync.rs:3531 |
| `alloc::sync::UniqueArcUninit` | `.ptr` | `core::ptr::non_null::NonNull<alloc::sync::ArcInner<$T>>` | tools/rust/library/alloc/src/sync.rs:3992 |
| `alloc::sync::UniqueArcUninit` | `.layout_for_value` | `alloc::Layout` | tools/rust/library/alloc/src/sync.rs:3993 |
| `alloc::sync::UniqueArcUninit` | `.alloc` | `Option<$A>` | tools/rust/library/alloc/src/sync.rs:3994 |
| `alloc::sync::Weak` | `.ptr` | `core::ptr::non_null::NonNull<alloc::sync::ArcInner<$T>>` | tools/rust/library/alloc/src/sync.rs:346 |
| `alloc::sync::Weak` | `.alloc` | `$A` | tools/rust/library/alloc/src/sync.rs:347 |
| `alloc::sync::WeakInner` | `.weak` | `core::sync::atomic::AtomicUsize` | tools/rust/library/alloc/src/sync.rs:2763 |
| `alloc::sync::WeakInner` | `.strong` | `core::sync::atomic::AtomicUsize` | tools/rust/library/alloc/src/sync.rs:2764 |
| `alloc::vec::Vec` | `.buf` | `alloc::raw_vec::RawVec<$T, $A>` | tools/rust/library/alloc/src/vec/mod.rs:410 |
| `alloc::vec::Vec` | `.len` | `usize` | tools/rust/library/alloc/src/vec/mod.rs:411 |
| `alloc::vec::drain::Drain` | `.tail_start` | `usize` | tools/rust/library/alloc/src/vec/drain.rs:28 |
| `alloc::vec::drain::Drain` | `.tail_len` | `usize` | tools/rust/library/alloc/src/vec/drain.rs:30 |
| `alloc::vec::drain::Drain` | `.iter` | `core::slice::iter::Iter<$T>` | tools/rust/library/alloc/src/vec/drain.rs:32 |
| `alloc::vec::drain::Drain` | `.vec` | `core::ptr::non_null::NonNull<alloc::vec::Vec<$T, $A>>` | tools/rust/library/alloc/src/vec/drain.rs:33 |
| `alloc::vec::extract_if::ExtractIf` | `.vec` | `alloc::vec::Vec<$T, $A>` | tools/rust/library/alloc/src/vec/extract_if.rs:27 |
| `alloc::vec::extract_if::ExtractIf` | `.idx` | `usize` | tools/rust/library/alloc/src/vec/extract_if.rs:29 |
| `alloc::vec::extract_if::ExtractIf` | `.end` | `usize` | tools/rust/library/alloc/src/vec/extract_if.rs:31 |
| `alloc::vec::extract_if::ExtractIf` | `.del` | `usize` | tools/rust/library/alloc/src/vec/extract_if.rs:33 |
| `alloc::vec::extract_if::ExtractIf` | `.old_len` | `usize` | tools/rust/library/alloc/src/vec/extract_if.rs:35 |
| `alloc::vec::extract_if::ExtractIf` | `.pred` | `$F` | tools/rust/library/alloc/src/vec/extract_if.rs:37 |
| `alloc::vec::in_place_drop::InPlaceDrop` | `.inner` | `*mut $T` | tools/rust/library/alloc/src/vec/in_place_drop.rs:11 |
| `alloc::vec::in_place_drop::InPlaceDrop` | `.dst` | `*mut $T` | tools/rust/library/alloc/src/vec/in_place_drop.rs:12 |
| `alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop` | `.ptr` | `core::ptr::non_null::NonNull<$Dest>` | tools/rust/library/alloc/src/vec/in_place_drop.rs:34 |
| `alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop` | `.len` | `usize` | tools/rust/library/alloc/src/vec/in_place_drop.rs:35 |
| `alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop` | `.src_cap` | `usize` | tools/rust/library/alloc/src/vec/in_place_drop.rs:36 |
| `alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop` | `.src` | `core::marker::PhantomData<$Src>` | tools/rust/library/alloc/src/vec/in_place_drop.rs:37 |
| `alloc::vec::into_iter::IntoIter` | `.buf` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/alloc/src/vec/into_iter.rs:49 |
| `alloc::vec::into_iter::IntoIter` | `.phantom` | `core::marker::PhantomData<$T>` | tools/rust/library/alloc/src/vec/into_iter.rs:50 |
| `alloc::vec::into_iter::IntoIter` | `.cap` | `usize` | tools/rust/library/alloc/src/vec/into_iter.rs:51 |
| `alloc::vec::into_iter::IntoIter` | `.alloc` | `core::mem::manually_drop::ManuallyDrop<$A>` | tools/rust/library/alloc/src/vec/into_iter.rs:54 |
| `alloc::vec::into_iter::IntoIter` | `.ptr` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/alloc/src/vec/into_iter.rs:55 |
| `alloc::vec::into_iter::IntoIter` | `.end` | `*const $T` | tools/rust/library/alloc/src/vec/into_iter.rs:60 |
| `alloc::vec::set_len_on_drop::SetLenOnDrop` | `.len` | `usize` | tools/rust/library/alloc/src/vec/set_len_on_drop.rs:7 |
| `alloc::vec::set_len_on_drop::SetLenOnDrop` | `.local_len` | `usize` | tools/rust/library/alloc/src/vec/set_len_on_drop.rs:8 |
| `alloc::vec::splice::Splice` | `.drain` | `alloc::vec::drain::Drain<I::Item, $A>` | tools/rust/library/alloc/src/vec/splice.rs:26 |
| `alloc::vec::splice::Splice` | `.replace_with` | `$I` | tools/rust/library/alloc/src/vec/splice.rs:27 |
| `core::alloc::layout::Layout` | `.size` | `usize` | tools/rust/library/core/src/alloc/layout.rs:42 |
| `core::alloc::layout::Layout` | `.align` | `core::ptr::alignment::Alignment` | tools/rust/library/core/src/alloc/layout.rs:51 |
| `core::alloc::layout::LayoutErr` | `(alias)` | `core::alloc::layout::LayoutError` | tools/rust/library/core/src/alloc/layout.rs:531 |
| `core::any::TypeId` | `.t` | `(u64, u64)` | tools/rust/library/core/src/any.rs:713 |
| `core::any::TypeId` | `.name` | `str` | tools/rust/library/core/src/any.rs:715 |
| `core::array::Guard` | `.array_mut` | `[mem::MaybeUninit<$T>]` | tools/rust/library/core/src/array/mod.rs:883 |
| `core::array::Guard` | `.initialized` | `usize` | tools/rust/library/core/src/array/mod.rs:885 |
| `core::array::TryFromSliceError` | `[0]` | `()` | tools/rust/library/core/src/array/mod.rs:160 |
| `core::array::drain::Drain` | `[0]` | `core::slice::iter::IterMut<$T>` | tools/rust/library/core/src/array/drain.rs:32 |
| `core::array::iter::IntoIter` | `.data` | `[mem::MaybeUninit<$T>; N]` | tools/rust/library/core/src/array/iter.rs:27 |
| `core::array::iter::IntoIter` | `.alive` | `core::ops::index_range::IndexRange` | tools/rust/library/core/src/array/iter.rs:35 |
| `core::ascii::EscapeDefault` | `[0]` | `core::escape::EscapeIterInner<const(4)>` | tools/rust/library/core/src/ascii.rs:27 |
| `core::asserting::Capture` | `.elem` | `Option<$E>` | tools/rust/library/core/src/asserting.rs:88 |
| `core::asserting::Capture` | `.phantom` | `core::marker::PhantomData<$M>` | tools/rust/library/core/src/asserting.rs:89 |
| `core::asserting::Wrapper` | `[0]` | `$T` | tools/rust/library/core/src/asserting.rs:101 |
| `core::async_iter::from_iter::FromIter` | `.iter` | `$I` | tools/rust/library/core/src/async_iter/from_iter.rs:14 |
| `core::bstr::ByteStr` | `[0]` | `[u8]` | tools/rust/library/core/src/bstr.rs:43 |
| `core::cell::BorrowError` | `.location` | `core::panic::location::Location` | tools/rust/library/core/src/cell.rs:746 |
| `core::cell::BorrowFlag` | `(alias)` | `isize` | tools/rust/library/core/src/cell.rs:824 |
| `core::cell::BorrowMutError` | `.location` | `core::panic::location::Location` | tools/rust/library/core/src/cell.rs:773 |
| `core::cell::BorrowRef` | `.borrow` | `core::cell::Cell<core::cell::BorrowFlag>` | tools/rust/library/core/src/cell.rs:1409 |
| `core::cell::BorrowRefMut` | `.borrow` | `core::cell::Cell<core::cell::BorrowFlag>` | tools/rust/library/core/src/cell.rs:1803 |
| `core::cell::Cell` | `.value` | `core::cell::UnsafeCell<$T>` | tools/rust/library/core/src/cell.rs:312 |
| `core::cell::Ref` | `.value` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/core/src/cell.rs:1472 |
| `core::cell::Ref` | `.borrow` | `core::cell::BorrowRef` | tools/rust/library/core/src/cell.rs:1473 |
| `core::cell::RefCell` | `.borrow` | `core::cell::Cell<core::cell::BorrowFlag>` | tools/rust/library/core/src/cell.rs:731 |
| `core::cell::RefCell` | `.borrowed_at` | `core::cell::Cell<Option<core::panic::location::Location>>` | tools/rust/library/core/src/cell.rs:737 |
| `core::cell::RefCell` | `.value` | `core::cell::UnsafeCell<$T>` | tools/rust/library/core/src/cell.rs:738 |
| `core::cell::RefMut` | `.value` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/core/src/cell.rs:1856 |
| `core::cell::RefMut` | `.borrow` | `core::cell::BorrowRefMut` | tools/rust/library/core/src/cell.rs:1857 |
| `core::cell::RefMut` | `.marker` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/cell.rs:1859 |
| `core::cell::SyncUnsafeCell` | `.value` | `core::cell::UnsafeCell<$T>` | tools/rust/library/core/src/cell.rs:2367 |
| `core::cell::UnsafeCell` | `.value` | `$T` | tools/rust/library/core/src/cell.rs:2076 |
| `core::cell::lazy::LazyCell` | `.state` | `core::cell::UnsafeCell<core::cell::lazy::State<$T, $F>>` | tools/rust/library/core/src/cell/lazy.rs:39 |
| `core::cell::lazy::State` | `.Uninit` | `core::cell::lazy::State::Uninit<$T, $F>` | tools/rust/library/core/src/cell/lazy.rs:7 |
| `core::cell::lazy::State` | `.Init` | `core::cell::lazy::State::Init<$T, $F>` | tools/rust/library/core/src/cell/lazy.rs:8 |
| `core::cell::lazy::State::Init` | `[0]` | `$T` | tools/rust/library/core/src/cell/lazy.rs:8 |
| `core::cell::lazy::State::Uninit` | `[0]` | `$F` | tools/rust/library/core/src/cell/lazy.rs:7 |
| `core::cell::once::OnceCell` | `.inner` | `core::cell::UnsafeCell<Option<$T>>` | tools/rust/library/core/src/cell/once.rs:37 |
| `core::char::CaseMappingIter` | `[0]` | `core::array::iter::IntoIter<char, const(3)>` | tools/rust/library/core/src/char/mod.rs:511 |
| `core::char::EscapeDebug` | `[0]` | `core::char::EscapeDebugInner` | tools/rust/library/core/src/char/mod.rs:306 |
| `core::char::EscapeDebugInner` | `.Bytes` | `core::char::EscapeDebugInner::Bytes` | tools/rust/library/core/src/char/mod.rs:314 |
| `core::char::EscapeDebugInner` | `.Char` | `core::char::EscapeDebugInner::Char` | tools/rust/library/core/src/char/mod.rs:315 |
| `core::char::EscapeDebugInner::Bytes` | `[0]` | `core::escape::EscapeIterInner<const(10)>` | tools/rust/library/core/src/char/mod.rs:314 |
| `core::char::EscapeDebugInner::Char` | `[0]` | `char` | tools/rust/library/core/src/char/mod.rs:315 |
| `core::char::EscapeDefault` | `[0]` | `core::escape::EscapeIterInner<const(10)>` | tools/rust/library/core/src/char/mod.rs:230 |
| `core::char::EscapeUnicode` | `[0]` | `core::escape::EscapeIterInner<const(10)>` | tools/rust/library/core/src/char/mod.rs:164 |
| `core::char::TryFromCharError` | `[0]` | `()` | tools/rust/library/core/src/char/mod.rs:619 |
| `core::char::convert::CharTryFromError` | `[0]` | `()` | tools/rust/library/core/src/char/convert.rs:262 |
| `core::char::convert::ParseCharError` | `.kind` | `core::char::convert::CharErrorKind` | tools/rust/library/core/src/char/convert.rs:180 |
| `core::char::decode::DecodeUtf16` | `.iter` | `$I` | tools/rust/library/core/src/char/decode.rs:19 |
| `core::char::decode::DecodeUtf16` | `.buf` | `Option<u16>` | tools/rust/library/core/src/char/decode.rs:20 |
| `core::char::decode::DecodeUtf16Error` | `.code` | `u16` | tools/rust/library/core/src/char/decode.rs:29 |
| `core::char::methods::EscapeDebugExtArgs` | `.escape_grapheme_extended` | `bool` | tools/rust/library/core/src/char/methods.rs:1757 |
| `core::char::methods::EscapeDebugExtArgs` | `.escape_single_quote` | `bool` | tools/rust/library/core/src/char/methods.rs:1760 |
| `core::char::methods::EscapeDebugExtArgs` | `.escape_double_quote` | `bool` | tools/rust/library/core/src/char/methods.rs:1763 |
| `core::clone::AssertParamIsClone` | `._field` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/clone.rs:252 |
| `core::clone::AssertParamIsCopy` | `._field` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/clone.rs:262 |
| `core::clone::uninit::InitializingSlice` | `.data` | `[mem::MaybeUninit<$T>]` | tools/rust/library/core/src/clone/uninit.rs:90 |
| `core::clone::uninit::InitializingSlice` | `.initialized_len` | `usize` | tools/rust/library/core/src/clone/uninit.rs:92 |
| `core::cmp::AssertParamIsEq` | `._field` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/cmp.rs:365 |
| `core::cmp::Reverse` | `[0]` | `$T` | tools/rust/library/core/src/cmp.rs:662 |
| `core::error::Request` | `[0]` | `core::error::Tagged<opaque(dyn Erased < 'a > + 'a)>` | tools/rust/library/core/src/error.rs:532 |
| `core::error::Source` | `.current` | `Option<opaque(dyn Error + 'static)>` | tools/rust/library/core/src/error.rs:1022 |
| `core::error::Tagged` | `.tag_id` | `core::any::TypeId` | tools/rust/library/core/src/error.rs:976 |
| `core::error::Tagged` | `.value` | `$E` | tools/rust/library/core/src/error.rs:977 |
| `core::error::TaggedOption` | `[0]` | `Option<I::Reified>` | tools/rust/library/core/src/error.rs:957 |
| `core::error::tags::MaybeSizedValue` | `[0]` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/error.rs:935 |
| `core::error::tags::Ref` | `[0]` | `core::marker::PhantomData<$I>` | tools/rust/library/core/src/error.rs:944 |
| `core::error::tags::Value` | `[0]` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/error.rs:927 |
| `core::escape::EscapeIterInner` | `.data` | `[core::ascii::ascii_char::AsciiChar; N]` | tools/rust/library/core/src/escape.rs:159 |
| `core::escape::EscapeIterInner` | `.alive` | `core::ops::range::Range<u8>` | tools/rust/library/core/src/escape.rs:162 |
| `core::ffi::c_str::Bytes` | `.ptr` | `core::ptr::non_null::NonNull<u8>` | tools/rust/library/core/src/ffi/c_str.rs:757 |
| `core::ffi::c_str::Bytes` | `.phantom` | `core::marker::PhantomData<[ffi::c_char]>` | tools/rust/library/core/src/ffi/c_str.rs:758 |
| `core::ffi::c_str::CStr` | `.inner` | `[ffi::c_char]` | tools/rust/library/core/src/ffi/c_str.rs:106 |
| `core::ffi::c_str::FromBytesUntilNulError` | `[0]` | `()` | tools/rust/library/core/src/ffi/c_str.rs:156 |
| `core::ffi::c_str::FromBytesWithNulError` | `.InteriorNul` | `core::ffi::c_str::FromBytesWithNulError::InteriorNul` | tools/rust/library/core/src/ffi/c_str.rs:128 |
| `core::ffi::c_str::FromBytesWithNulError::InteriorNul` | `.position` | `usize` | tools/rust/library/core/src/ffi/c_str.rs:130 |
| `core::ffi::primitives::c_ptrdiff_t` | `(alias)` | `isize` | tools/rust/library/core/src/ffi/primitives.rs:165 |
| `core::ffi::primitives::c_size_t` | `(alias)` | `usize` | tools/rust/library/core/src/ffi/primitives.rs:158 |
| `core::ffi::primitives::c_ssize_t` | `(alias)` | `isize` | tools/rust/library/core/src/ffi/primitives.rs:172 |
| `core::ffi::va_list::VaList` | `.inner` | `core::ffi::va_list::VaListImpl` | tools/rust/library/core/src/ffi/va_list.rs:147 |
| `core::ffi::va_list::VaList` | `.inner` | `core::ffi::va_list::VaListImpl` | tools/rust/library/core/src/ffi/va_list.rs:162 |
| `core::ffi::va_list::VaList` | `._marker` | `core::marker::PhantomData<core::ffi::va_list::VaListImpl>` | tools/rust/library/core/src/ffi/va_list.rs:164 |
| `core::fmt::Arguments` | `.pieces` | `[str]` | tools/rust/library/core/src/fmt/mod.rs:615 |
| `core::fmt::Arguments` | `.fmt` | `Option<[core::fmt::rt::Placeholder]>` | tools/rust/library/core/src/fmt/mod.rs:618 |
| `core::fmt::Arguments` | `.args` | `[core::fmt::rt::Argument]` | tools/rust/library/core/src/fmt/mod.rs:622 |
| `core::fmt::Formatter` | `.options` | `core::fmt::FormattingOptions` | tools/rust/library/core/src/fmt/mod.rs:563 |
| `core::fmt::Formatter` | `.buf` | `opaque(dyn Write + 'a)` | tools/rust/library/core/src/fmt/mod.rs:565 |
| `core::fmt::FormattingOptions` | `.flags` | `u32` | tools/rust/library/core/src/fmt/mod.rs:304 |
| `core::fmt::FormattingOptions` | `.width` | `u16` | tools/rust/library/core/src/fmt/mod.rs:306 |
| `core::fmt::FormattingOptions` | `.precision` | `u16` | tools/rust/library/core/src/fmt/mod.rs:308 |
| `core::fmt::PostPadding` | `.fill` | `char` | tools/rust/library/core/src/fmt/mod.rs:1578 |
| `core::fmt::PostPadding` | `.padding` | `u16` | tools/rust/library/core/src/fmt/mod.rs:1579 |
| `core::fmt::Result` | `(alias)` | `core::result::Result<(), core::fmt::Error>` | tools/rust/library/core/src/fmt/mod.rs:66 |
| `core::fmt::builders::DebugInner` | `.fmt` | `core::fmt::Formatter` | tools/rust/library/core/src/fmt/builders.rs:450 |
| `core::fmt::builders::DebugInner` | `.result` | `core::fmt::Result` | tools/rust/library/core/src/fmt/builders.rs:451 |
| `core::fmt::builders::DebugInner` | `.has_fields` | `bool` | tools/rust/library/core/src/fmt/builders.rs:452 |
| `core::fmt::builders::DebugList` | `.inner` | `core::fmt::builders::DebugInner` | tools/rust/library/core/src/fmt/builders.rs:707 |
| `core::fmt::builders::DebugMap` | `.fmt` | `core::fmt::Formatter` | tools/rust/library/core/src/fmt/builders.rs:898 |
| `core::fmt::builders::DebugMap` | `.result` | `core::fmt::Result` | tools/rust/library/core/src/fmt/builders.rs:899 |
| `core::fmt::builders::DebugMap` | `.has_fields` | `bool` | tools/rust/library/core/src/fmt/builders.rs:900 |
| `core::fmt::builders::DebugMap` | `.has_key` | `bool` | tools/rust/library/core/src/fmt/builders.rs:901 |
| `core::fmt::builders::DebugMap` | `.state` | `core::fmt::builders::PadAdapterState` | tools/rust/library/core/src/fmt/builders.rs:903 |
| `core::fmt::builders::DebugSet` | `.inner` | `core::fmt::builders::DebugInner` | tools/rust/library/core/src/fmt/builders.rs:515 |
| `core::fmt::builders::DebugStruct` | `.fmt` | `core::fmt::Formatter` | tools/rust/library/core/src/fmt/builders.rs:89 |
| `core::fmt::builders::DebugStruct` | `.result` | `core::fmt::Result` | tools/rust/library/core/src/fmt/builders.rs:90 |
| `core::fmt::builders::DebugStruct` | `.has_fields` | `bool` | tools/rust/library/core/src/fmt/builders.rs:91 |
| `core::fmt::builders::DebugTuple` | `.fmt` | `core::fmt::Formatter` | tools/rust/library/core/src/fmt/builders.rs:290 |
| `core::fmt::builders::DebugTuple` | `.result` | `core::fmt::Result` | tools/rust/library/core/src/fmt/builders.rs:291 |
| `core::fmt::builders::DebugTuple` | `.fields` | `usize` | tools/rust/library/core/src/fmt/builders.rs:292 |
| `core::fmt::builders::DebugTuple` | `.empty_name` | `bool` | tools/rust/library/core/src/fmt/builders.rs:293 |
| `core::fmt::builders::FromFn` | `[0]` | `$F` | tools/rust/library/core/src/fmt/builders.rs:1240 |
| `core::fmt::builders::PadAdapter` | `.buf` | `opaque(dyn fmt :: Write + 'buf)` | tools/rust/library/core/src/fmt/builders.rs:6 |
| `core::fmt::builders::PadAdapter` | `.state` | `core::fmt::builders::PadAdapterState` | tools/rust/library/core/src/fmt/builders.rs:7 |
| `core::fmt::builders::PadAdapterState` | `.on_newline` | `bool` | tools/rust/library/core/src/fmt/builders.rs:11 |
| `core::fmt::rt::Argument` | `.ty` | `core::fmt::rt::ArgumentType` | tools/rust/library/core/src/fmt/rt.rs:96 |
| `core::fmt::rt::ArgumentType` | `.Placeholder` | `core::fmt::rt::ArgumentType::Placeholder` | tools/rust/library/core/src/fmt/rt.rs:73 |
| `core::fmt::rt::ArgumentType` | `.Count` | `core::fmt::rt::ArgumentType::Count` | tools/rust/library/core/src/fmt/rt.rs:80 |
| `core::fmt::rt::ArgumentType::Count` | `[0]` | `u16` | tools/rust/library/core/src/fmt/rt.rs:80 |
| `core::fmt::rt::ArgumentType::Placeholder` | `.value` | `core::ptr::non_null::NonNull<()>` | tools/rust/library/core/src/fmt/rt.rs:76 |
| `core::fmt::rt::ArgumentType::Placeholder` | `.formatter` | `opaque(unsafe fn (NonNull < () > , & mut Formatter < '_ >) -> Result)` | tools/rust/library/core/src/fmt/rt.rs:77 |
| `core::fmt::rt::ArgumentType::Placeholder` | `._lifetime` | `PhantomData<()>` | tools/rust/library/core/src/fmt/rt.rs:78 |
| `core::fmt::rt::Count` | `.Is` | `core::fmt::rt::Count::Is` | tools/rust/library/core/src/fmt/rt.rs:61 |
| `core::fmt::rt::Count` | `.Is` | `core::fmt::rt::Count::Is` | tools/rust/library/core/src/fmt/rt.rs:64 |
| `core::fmt::rt::Count` | `.Param` | `core::fmt::rt::Count::Param` | tools/rust/library/core/src/fmt/rt.rs:66 |
| `core::fmt::rt::Count::Is` | `[0]` | `u16` | tools/rust/library/core/src/fmt/rt.rs:64 |
| `core::fmt::rt::Count::Param` | `[0]` | `usize` | tools/rust/library/core/src/fmt/rt.rs:66 |
| `core::fmt::rt::Placeholder` | `.position` | `usize` | tools/rust/library/core/src/fmt/rt.rs:13 |
| `core::fmt::rt::Placeholder` | `.fill` | `char` | tools/rust/library/core/src/fmt/rt.rs:15 |
| `core::fmt::rt::Placeholder` | `.align` | `core::fmt::rt::Alignment` | tools/rust/library/core/src/fmt/rt.rs:17 |
| `core::fmt::rt::Placeholder` | `.flags` | `u32` | tools/rust/library/core/src/fmt/rt.rs:18 |
| `core::fmt::rt::Placeholder` | `.precision` | `core::fmt::rt::Count` | tools/rust/library/core/src/fmt/rt.rs:19 |
| `core::fmt::rt::Placeholder` | `.width` | `core::fmt::rt::Count` | tools/rust/library/core/src/fmt/rt.rs:20 |
| `core::fmt::rt::UnsafeArg` | `._private` | `()` | tools/rust/library/core/src/fmt/rt.rs:226 |
| `core::future::ResumeTy` | `[0]` | `core::ptr::non_null::NonNull<core::task::wake::Context>` | tools/rust/library/core/src/future/mod.rs:51 |
| `core::future::async_drop::AsyncDropInPlace` | `[0]` | `opaque(< T as AsyncDestruct > :: AsyncDestructor)` | tools/rust/library/core/src/future/async_drop.rs:97 |
| `core::future::async_drop::AsyncDropOwning` | `.value` | `mem::MaybeUninit<$T>` | tools/rust/library/core/src/future/async_drop.rs:21 |
| `core::future::async_drop::AsyncDropOwning` | `.dtor` | `Option<core::future::async_drop::AsyncDropInPlace<$T>>` | tools/rust/library/core/src/future/async_drop.rs:22 |
| `core::future::async_drop::AsyncDropOwning` | `._pinned` | `core::marker::PhantomPinned` | tools/rust/library/core/src/future/async_drop.rs:23 |
| `core::future::async_drop::Fuse` | `.inner` | `Option<$T>` | tools/rust/library/core/src/future/async_drop.rs:171 |
| `core::future::join::MaybeDone` | `.Future` | `core::future::join::MaybeDone::Future<$F>` | tools/rust/library/core/src/future/join.rs:155 |
| `core::future::join::MaybeDone` | `.Done` | `core::future::join::MaybeDone::Done<$F>` | tools/rust/library/core/src/future/join.rs:156 |
| `core::future::join::MaybeDone::Done` | `[0]` | `F::Output` | tools/rust/library/core/src/future/join.rs:156 |
| `core::future::join::MaybeDone::Future` | `[0]` | `$F` | tools/rust/library/core/src/future/join.rs:155 |
| `core::future::pending::Pending` | `._data` | `core::marker::PhantomData<opaque(fn () -> T)>` | tools/rust/library/core/src/future/pending.rs:15 |
| `core::future::poll_fn::PollFn` | `.f` | `$F` | tools/rust/library/core/src/future/poll_fn.rs:129 |
| `core::future::ready::Ready` | `[0]` | `Option<$T>` | tools/rust/library/core/src/future/ready.rs:12 |
| `core::hash::BuildHasherDefault` | `[0]` | `core::marker::PhantomData<opaque(fn () -> H)>` | tools/rust/library/core/src/hash/mod.rs:751 |
| `core::hash::sip::Hasher` | `.k0` | `u64` | tools/rust/library/core/src/hash/sip.rs:51 |
| `core::hash::sip::Hasher` | `.k1` | `u64` | tools/rust/library/core/src/hash/sip.rs:52 |
| `core::hash::sip::Hasher` | `.length` | `usize` | tools/rust/library/core/src/hash/sip.rs:53 |
| `core::hash::sip::Hasher` | `.state` | `core::hash::sip::State` | tools/rust/library/core/src/hash/sip.rs:54 |
| `core::hash::sip::Hasher` | `.tail` | `u64` | tools/rust/library/core/src/hash/sip.rs:55 |
| `core::hash::sip::Hasher` | `.ntail` | `usize` | tools/rust/library/core/src/hash/sip.rs:56 |
| `core::hash::sip::Hasher` | `._marker` | `core::marker::PhantomData<$S>` | tools/rust/library/core/src/hash/sip.rs:57 |
| `core::hash::sip::SipHasher` | `[0]` | `core::hash::sip::SipHasher24` | tools/rust/library/core/src/hash/sip.rs:47 |
| `core::hash::sip::SipHasher13` | `.hasher` | `core::hash::sip::Hasher<core::hash::sip::Sip13Rounds>` | tools/rust/library/core/src/hash/sip.rs:19 |
| `core::hash::sip::SipHasher24` | `.hasher` | `core::hash::sip::Hasher<core::hash::sip::Sip24Rounds>` | tools/rust/library/core/src/hash/sip.rs:29 |
| `core::hash::sip::State` | `.v0` | `u64` | tools/rust/library/core/src/hash/sip.rs:67 |
| `core::hash::sip::State` | `.v2` | `u64` | tools/rust/library/core/src/hash/sip.rs:68 |
| `core::hash::sip::State` | `.v1` | `u64` | tools/rust/library/core/src/hash/sip.rs:69 |
| `core::hash::sip::State` | `.v3` | `u64` | tools/rust/library/core/src/hash/sip.rs:70 |
| `core::io::borrowed_buf::BorrowedBuf` | `.buf` | `[mem::MaybeUninit<u8>]` | tools/rust/library/core/src/io/borrowed_buf.rs:30 |
| `core::io::borrowed_buf::BorrowedBuf` | `.filled` | `usize` | tools/rust/library/core/src/io/borrowed_buf.rs:32 |
| `core::io::borrowed_buf::BorrowedBuf` | `.init` | `usize` | tools/rust/library/core/src/io/borrowed_buf.rs:34 |
| `core::io::borrowed_buf::BorrowedCursor` | `.buf` | `core::io::borrowed_buf::BorrowedBuf` | tools/rust/library/core/src/io/borrowed_buf.rs:190 |
| `core::io::borrowed_buf::BorrowedCursor` | `.start` | `usize` | tools/rust/library/core/src/io/borrowed_buf.rs:193 |
| `core::iter::adapters::GenericShunt` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/mod.rs:145 |
| `core::iter::adapters::GenericShunt` | `.residual` | `Option<$R>` | tools/rust/library/core/src/iter/adapters/mod.rs:146 |
| `core::iter::adapters::array_chunks::ArrayChunks` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/array_chunks.rs:20 |
| `core::iter::adapters::array_chunks::ArrayChunks` | `.remainder` | `Option<core::array::iter::IntoIter<I::Item, $N>>` | tools/rust/library/core/src/iter/adapters/array_chunks.rs:21 |
| `core::iter::adapters::by_ref_sized::ByRefSized` | `[0]` | `$I` | tools/rust/library/core/src/iter/adapters/by_ref_sized.rs:10 |
| `core::iter::adapters::chain::Chain` | `.a` | `Option<$A>` | tools/rust/library/core/src/iter/adapters/chain.rs:32 |
| `core::iter::adapters::chain::Chain` | `.b` | `Option<$B>` | tools/rust/library/core/src/iter/adapters/chain.rs:33 |
| `core::iter::adapters::cloned::Cloned` | `.it` | `$I` | tools/rust/library/core/src/iter/adapters/cloned.rs:19 |
| `core::iter::adapters::copied::Copied` | `.it` | `$I` | tools/rust/library/core/src/iter/adapters/copied.rs:20 |
| `core::iter::adapters::cycle::Cycle` | `.orig` | `$I` | tools/rust/library/core/src/iter/adapters/cycle.rs:16 |
| `core::iter::adapters::cycle::Cycle` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/cycle.rs:17 |
| `core::iter::adapters::enumerate::Enumerate` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/enumerate.rs:19 |
| `core::iter::adapters::enumerate::Enumerate` | `.count` | `usize` | tools/rust/library/core/src/iter/adapters/enumerate.rs:20 |
| `core::iter::adapters::filter::Filter` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/filter.rs:23 |
| `core::iter::adapters::filter::Filter` | `.predicate` | `$P` | tools/rust/library/core/src/iter/adapters/filter.rs:24 |
| `core::iter::adapters::filter_map::FilterMap` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/filter_map.rs:19 |
| `core::iter::adapters::filter_map::FilterMap` | `.f` | `$F` | tools/rust/library/core/src/iter/adapters/filter_map.rs:20 |
| `core::iter::adapters::flatten::FlatMap` | `.inner` | `core::iter::adapters::flatten::FlattenCompat<core::iter::adapters::map::Map<$I, $F>, opaque(< U as IntoIterator > :: IntoIter)>` | tools/rust/library/core/src/iter/adapters/flatten.rs:18 |
| `core::iter::adapters::flatten::Flatten` | `.inner` | `core::iter::adapters::flatten::FlattenCompat<$I, opaque(< I :: Item as IntoIterator > :: IntoIter)>` | tools/rust/library/core/src/iter/adapters/flatten.rs:185 |
| `core::iter::adapters::flatten::FlattenCompat` | `.iter` | `core::iter::adapters::fuse::Fuse<$I>` | tools/rust/library/core/src/iter/adapters/flatten.rs:357 |
| `core::iter::adapters::flatten::FlattenCompat` | `.frontiter` | `Option<$U>` | tools/rust/library/core/src/iter/adapters/flatten.rs:358 |
| `core::iter::adapters::flatten::FlattenCompat` | `.backiter` | `Option<$U>` | tools/rust/library/core/src/iter/adapters/flatten.rs:359 |
| `core::iter::adapters::fuse::Fuse` | `.iter` | `Option<$I>` | tools/rust/library/core/src/iter/adapters/fuse.rs:21 |
| `core::iter::adapters::inspect::Inspect` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/inspect.rs:19 |
| `core::iter::adapters::inspect::Inspect` | `.f` | `$F` | tools/rust/library/core/src/iter/adapters/inspect.rs:20 |
| `core::iter::adapters::intersperse::Intersperse` | `.started` | `bool` | tools/rust/library/core/src/iter/adapters/intersperse.rs:14 |
| `core::iter::adapters::intersperse::Intersperse` | `.separator` | `I::Item` | tools/rust/library/core/src/iter/adapters/intersperse.rs:15 |
| `core::iter::adapters::intersperse::Intersperse` | `.next_item` | `Option<I::Item>` | tools/rust/library/core/src/iter/adapters/intersperse.rs:16 |
| `core::iter::adapters::intersperse::Intersperse` | `.iter` | `core::iter::adapters::fuse::Fuse<$I>` | tools/rust/library/core/src/iter/adapters/intersperse.rs:17 |
| `core::iter::adapters::intersperse::IntersperseWith` | `.started` | `bool` | tools/rust/library/core/src/iter/adapters/intersperse.rs:95 |
| `core::iter::adapters::intersperse::IntersperseWith` | `.separator` | `$G` | tools/rust/library/core/src/iter/adapters/intersperse.rs:96 |
| `core::iter::adapters::intersperse::IntersperseWith` | `.next_item` | `Option<I::Item>` | tools/rust/library/core/src/iter/adapters/intersperse.rs:97 |
| `core::iter::adapters::intersperse::IntersperseWith` | `.iter` | `core::iter::adapters::fuse::Fuse<$I>` | tools/rust/library/core/src/iter/adapters/intersperse.rs:98 |
| `core::iter::adapters::map::Map` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/map.rs:63 |
| `core::iter::adapters::map::Map` | `.f` | `$F` | tools/rust/library/core/src/iter/adapters/map.rs:64 |
| `core::iter::adapters::map_while::MapWhile` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/map_while.rs:18 |
| `core::iter::adapters::map_while::MapWhile` | `.predicate` | `$P` | tools/rust/library/core/src/iter/adapters/map_while.rs:19 |
| `core::iter::adapters::map_windows::Buffer` | `.buffer` | `[[mem::MaybeUninit<$T>; N]; 2]` | tools/rust/library/core/src/iter/adapters/map_windows.rs:44 |
| `core::iter::adapters::map_windows::Buffer` | `.start` | `usize` | tools/rust/library/core/src/iter/adapters/map_windows.rs:45 |
| `core::iter::adapters::map_windows::MapWindows` | `.f` | `$F` | tools/rust/library/core/src/iter/adapters/map_windows.rs:12 |
| `core::iter::adapters::map_windows::MapWindows` | `.inner` | `core::iter::adapters::map_windows::MapWindowsInner<$I, $N>` | tools/rust/library/core/src/iter/adapters/map_windows.rs:13 |
| `core::iter::adapters::map_windows::MapWindowsInner` | `.iter` | `Option<$I>` | tools/rust/library/core/src/iter/adapters/map_windows.rs:20 |
| `core::iter::adapters::map_windows::MapWindowsInner` | `.buffer` | `Option<core::iter::adapters::map_windows::Buffer<I::Item, $N>>` | tools/rust/library/core/src/iter/adapters/map_windows.rs:33 |
| `core::iter::adapters::peekable::Peekable` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/peekable.rs:18 |
| `core::iter::adapters::peekable::Peekable` | `.peeked` | `Option<Option<I::Item>>` | tools/rust/library/core/src/iter/adapters/peekable.rs:20 |
| `core::iter::adapters::rev::Rev` | `.iter` | `$T` | tools/rust/library/core/src/iter/adapters/rev.rs:16 |
| `core::iter::adapters::scan::Scan` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/scan.rs:18 |
| `core::iter::adapters::scan::Scan` | `.f` | `$F` | tools/rust/library/core/src/iter/adapters/scan.rs:19 |
| `core::iter::adapters::scan::Scan` | `.state` | `$St` | tools/rust/library/core/src/iter/adapters/scan.rs:20 |
| `core::iter::adapters::skip::Skip` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/skip.rs:22 |
| `core::iter::adapters::skip::Skip` | `.n` | `usize` | tools/rust/library/core/src/iter/adapters/skip.rs:23 |
| `core::iter::adapters::skip_while::SkipWhile` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/skip_while.rs:18 |
| `core::iter::adapters::skip_while::SkipWhile` | `.flag` | `bool` | tools/rust/library/core/src/iter/adapters/skip_while.rs:19 |
| `core::iter::adapters::skip_while::SkipWhile` | `.predicate` | `$P` | tools/rust/library/core/src/iter/adapters/skip_while.rs:20 |
| `core::iter::adapters::step_by::StepBy` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/step_by.rs:23 |
| `core::iter::adapters::step_by::StepBy` | `.step_minus_one` | `usize` | tools/rust/library/core/src/iter/adapters/step_by.rs:28 |
| `core::iter::adapters::step_by::StepBy` | `.first_take` | `bool` | tools/rust/library/core/src/iter/adapters/step_by.rs:29 |
| `core::iter::adapters::take::Take` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/take.rs:18 |
| `core::iter::adapters::take::Take` | `.n` | `usize` | tools/rust/library/core/src/iter/adapters/take.rs:19 |
| `core::iter::adapters::take_while::TakeWhile` | `.iter` | `$I` | tools/rust/library/core/src/iter/adapters/take_while.rs:18 |
| `core::iter::adapters::take_while::TakeWhile` | `.flag` | `bool` | tools/rust/library/core/src/iter/adapters/take_while.rs:19 |
| `core::iter::adapters::take_while::TakeWhile` | `.predicate` | `$P` | tools/rust/library/core/src/iter/adapters/take_while.rs:20 |
| `core::iter::adapters::zip::Zip` | `.a` | `$A` | tools/rust/library/core/src/iter/adapters/zip.rs:16 |
| `core::iter::adapters::zip::Zip` | `.b` | `$B` | tools/rust/library/core/src/iter/adapters/zip.rs:17 |
| `core::iter::adapters::zip::Zip` | `.index` | `usize` | tools/rust/library/core/src/iter/adapters/zip.rs:19 |
| `core::iter::adapters::zip::Zip` | `.len` | `usize` | tools/rust/library/core/src/iter/adapters/zip.rs:20 |
| `core::iter::adapters::zip::Zip` | `.a_len` | `usize` | tools/rust/library/core/src/iter/adapters/zip.rs:21 |
| `core::iter::sources::empty::Empty` | `[0]` | `core::marker::PhantomData<opaque(fn () -> T)>` | tools/rust/library/core/src/iter/sources/empty.rs:30 |
| `core::iter::sources::from_coroutine::FromCoroutine` | `[0]` | `$G` | tools/rust/library/core/src/iter/sources/from_coroutine.rs:39 |
| `core::iter::sources::from_fn::FromFn` | `[0]` | `$F` | tools/rust/library/core/src/iter/sources/from_fn.rs:60 |
| `core::iter::sources::once::Once` | `.inner` | `core::option::IntoIter<$T>` | tools/rust/library/core/src/iter/sources/once.rs:67 |
| `core::iter::sources::once_with::OnceWith` | `.make` | `Option<$F>` | tools/rust/library/core/src/iter/sources/once_with.rs:73 |
| `core::iter::sources::repeat::Repeat` | `.element` | `$A` | tools/rust/library/core/src/iter/sources/repeat.rs:70 |
| `core::iter::sources::repeat_n::RepeatN` | `.count` | `usize` | tools/rust/library/core/src/iter/sources/repeat_n.rs:77 |
| `core::iter::sources::repeat_n::RepeatN` | `.element` | `mem::MaybeUninit<$A>` | tools/rust/library/core/src/iter/sources/repeat_n.rs:79 |
| `core::iter::sources::repeat_with::RepeatWith` | `.repeater` | `$F` | tools/rust/library/core/src/iter/sources/repeat_with.rs:77 |
| `core::iter::sources::successors::Successors` | `.next` | `Option<$T>` | tools/rust/library/core/src/iter/sources/successors.rs:42 |
| `core::iter::sources::successors::Successors` | `.succ` | `$F` | tools/rust/library/core/src/iter/sources/successors.rs:43 |
| `core::mem::Discriminant` | `[0]` | `opaque(< T as DiscriminantKind > :: Discriminant)` | tools/rust/library/core/src/mem/mod.rs:1027 |
| `core::mem::manually_drop::ManuallyDrop` | `.value` | `$T` | tools/rust/library/core/src/mem/manually_drop.rs:158 |
| `core::mem::maybe_uninit::Guard` | `.slice` | `[MaybeUninit<$T>]` | tools/rust/library/core/src/mem/maybe_uninit.rs:1585 |
| `core::mem::maybe_uninit::Guard` | `.initialized` | `usize` | tools/rust/library/core/src/mem/maybe_uninit.rs:1586 |
| `core::mem::transmutability::Assume` | `.alignment` | `bool` | tools/rust/library/core/src/mem/transmutability.rs:191 |
| `core::mem::transmutability::Assume` | `.lifetimes` | `bool` | tools/rust/library/core/src/mem/transmutability.rs:198 |
| `core::mem::transmutability::Assume` | `.safety` | `bool` | tools/rust/library/core/src/mem/transmutability.rs:247 |
| `core::mem::transmutability::Assume` | `.validity` | `bool` | tools/rust/library/core/src/mem/transmutability.rs:287 |
| `core::net::display_buffer::DisplayBuffer` | `.buf` | `[mem::MaybeUninit<u8>; SIZE]` | tools/rust/library/core/src/net/display_buffer.rs:6 |
| `core::net::display_buffer::DisplayBuffer` | `.len` | `usize` | tools/rust/library/core/src/net/display_buffer.rs:7 |
| `core::net::ip_addr::IpAddr` | `.V4` | `core::net::ip_addr::IpAddr::V4` | tools/rust/library/core/src/net/ip_addr.rs:34 |
| `core::net::ip_addr::IpAddr` | `.V6` | `core::net::ip_addr::IpAddr::V6` | tools/rust/library/core/src/net/ip_addr.rs:37 |
| `core::net::ip_addr::IpAddr::V4` | `[0]` | `core::net::ip_addr::Ipv4Addr` | tools/rust/library/core/src/net/ip_addr.rs:34 |
| `core::net::ip_addr::IpAddr::V6` | `[0]` | `core::net::ip_addr::Ipv6Addr` | tools/rust/library/core/src/net/ip_addr.rs:37 |
| `core::net::ip_addr::Ipv4Addr` | `.octets` | `[u8; 4]` | tools/rust/library/core/src/net/ip_addr.rs:74 |
| `core::net::ip_addr::Ipv6Addr` | `.octets` | `[u8; 16]` | tools/rust/library/core/src/net/ip_addr.rs:166 |
| `core::net::parser::AddrParseError` | `[0]` | `core::net::parser::AddrKind` | tools/rust/library/core/src/net/parser.rs:496 |
| `core::net::parser::Parser` | `.state` | `[u8]` | tools/rust/library/core/src/net/parser.rs:35 |
| `core::net::socket_addr::SocketAddr` | `.V4` | `core::net::socket_addr::SocketAddr::V4` | tools/rust/library/core/src/net/socket_addr.rs:36 |
| `core::net::socket_addr::SocketAddr` | `.V6` | `core::net::socket_addr::SocketAddr::V6` | tools/rust/library/core/src/net/socket_addr.rs:39 |
| `core::net::socket_addr::SocketAddr::V4` | `[0]` | `core::net::socket_addr::SocketAddrV4` | tools/rust/library/core/src/net/socket_addr.rs:36 |
| `core::net::socket_addr::SocketAddr::V6` | `[0]` | `core::net::socket_addr::SocketAddrV6` | tools/rust/library/core/src/net/socket_addr.rs:39 |
| `core::net::socket_addr::SocketAddrV4` | `.ip` | `core::net::ip_addr::Ipv4Addr` | tools/rust/library/core/src/net/socket_addr.rs:82 |
| `core::net::socket_addr::SocketAddrV4` | `.port` | `u16` | tools/rust/library/core/src/net/socket_addr.rs:83 |
| `core::net::socket_addr::SocketAddrV6` | `.ip` | `core::net::ip_addr::Ipv6Addr` | tools/rust/library/core/src/net/socket_addr.rs:148 |
| `core::net::socket_addr::SocketAddrV6` | `.port` | `u16` | tools/rust/library/core/src/net/socket_addr.rs:149 |
| `core::net::socket_addr::SocketAddrV6` | `.flowinfo` | `u32` | tools/rust/library/core/src/net/socket_addr.rs:150 |
| `core::net::socket_addr::SocketAddrV6` | `.scope_id` | `u32` | tools/rust/library/core/src/net/socket_addr.rs:151 |
| `core::num::bignum::Digit32` | `(alias)` | `u32` | tools/rust/library/core/src/num/bignum.rs:425 |
| `core::num::dec2flt::ParseFloatError` | `.kind` | `core::num::dec2flt::FloatErrorKind` | tools/rust/library/core/src/num/dec2flt/mod.rs:194 |
| `core::num::dec2flt::common::BiasedFp` | `.m` | `u64` | tools/rust/library/core/src/num/dec2flt/common.rs:71 |
| `core::num::dec2flt::common::BiasedFp` | `.p_biased` | `i32` | tools/rust/library/core/src/num/dec2flt/common.rs:73 |
| `core::num::dec2flt::decimal::Decimal` | `.exponent` | `i64` | tools/rust/library/core/src/num/dec2flt/decimal.rs:28 |
| `core::num::dec2flt::decimal::Decimal` | `.mantissa` | `u64` | tools/rust/library/core/src/num/dec2flt/decimal.rs:29 |
| `core::num::dec2flt::decimal::Decimal` | `.negative` | `bool` | tools/rust/library/core/src/num/dec2flt/decimal.rs:30 |
| `core::num::dec2flt::decimal::Decimal` | `.many_digits` | `bool` | tools/rust/library/core/src/num/dec2flt/decimal.rs:31 |
| `core::num::dec2flt::decimal_seq::DecimalSeq` | `.num_digits` | `usize` | tools/rust/library/core/src/num/dec2flt/decimal_seq.rs:18 |
| `core::num::dec2flt::decimal_seq::DecimalSeq` | `.decimal_point` | `i32` | tools/rust/library/core/src/num/dec2flt/decimal_seq.rs:20 |
| `core::num::dec2flt::decimal_seq::DecimalSeq` | `.truncated` | `bool` | tools/rust/library/core/src/num/dec2flt/decimal_seq.rs:22 |
| `core::num::dec2flt::decimal_seq::DecimalSeq` | `.digits` | `[u8; Self :: MAX_DIGITS]` | tools/rust/library/core/src/num/dec2flt/decimal_seq.rs:24 |
| `core::num::dec2flt::fpu::fpu_precision::FPUControlWord` | `[0]` | `u16` | tools/rust/library/core/src/num/dec2flt/fpu.rs:44 |
| `core::num::diy_float::Fp` | `.f` | `u64` | tools/rust/library/core/src/num/diy_float.rs:17 |
| `core::num::diy_float::Fp` | `.e` | `i16` | tools/rust/library/core/src/num/diy_float.rs:19 |
| `core::num::error::ParseIntError` | `.kind` | `core::num::error::IntErrorKind` | tools/rust/library/core/src/num/error.rs:67 |
| `core::num::error::TryFromIntError` | `[0]` | `()` | tools/rust/library/core/src/num/error.rs:10 |
| `core::num::flt2dec::decoder::Decoded` | `.mant` | `u64` | tools/rust/library/core/src/num/flt2dec/decoder.rs:16 |
| `core::num::flt2dec::decoder::Decoded` | `.minus` | `u64` | tools/rust/library/core/src/num/flt2dec/decoder.rs:18 |
| `core::num::flt2dec::decoder::Decoded` | `.plus` | `u64` | tools/rust/library/core/src/num/flt2dec/decoder.rs:20 |
| `core::num::flt2dec::decoder::Decoded` | `.exp` | `i16` | tools/rust/library/core/src/num/flt2dec/decoder.rs:22 |
| `core::num::flt2dec::decoder::Decoded` | `.inclusive` | `bool` | tools/rust/library/core/src/num/flt2dec/decoder.rs:26 |
| `core::num::flt2dec::decoder::FullDecoded` | `.Finite` | `core::num::flt2dec::decoder::FullDecoded::Finite` | tools/rust/library/core/src/num/flt2dec/decoder.rs:39 |
| `core::num::flt2dec::decoder::FullDecoded::Finite` | `[0]` | `core::num::flt2dec::decoder::Decoded` | tools/rust/library/core/src/num/flt2dec/decoder.rs:39 |
| `core::num::fmt::Formatted` | `.sign` | `str` | tools/rust/library/core/src/num/fmt.rs:77 |
| `core::num::fmt::Formatted` | `.parts` | `[core::num::fmt::Part]` | tools/rust/library/core/src/num/fmt.rs:79 |
| `core::num::fmt::Part` | `.Zero` | `core::num::fmt::Part::Zero` | tools/rust/library/core/src/num/fmt.rs:13 |
| `core::num::fmt::Part` | `.Num` | `core::num::fmt::Part::Num` | tools/rust/library/core/src/num/fmt.rs:15 |
| `core::num::fmt::Part` | `.Copy` | `core::num::fmt::Part::Copy` | tools/rust/library/core/src/num/fmt.rs:17 |
| `core::num::fmt::Part::Copy` | `[0]` | `[u8]` | tools/rust/library/core/src/num/fmt.rs:17 |
| `core::num::fmt::Part::Num` | `[0]` | `u16` | tools/rust/library/core/src/num/fmt.rs:15 |
| `core::num::fmt::Part::Zero` | `[0]` | `usize` | tools/rust/library/core/src/num/fmt.rs:13 |
| `core::num::niche_types::NotAllOnes` | `(alias)` | `opaque(< T as NotAllOnesHelper > :: Type)` | tools/rust/library/core/src/num/niche_types.rs:166 |
| `core::num::nonzero::NonZero` | `[0]` | `T::NonZeroInner` | tools/rust/library/core/src/num/nonzero.rs:117 |
| `core::num::saturating::Saturating` | `[0]` | `$T` | tools/rust/library/core/src/num/saturating.rs:38 |
| `core::num::wrapping::Wrapping` | `[0]` | `$T` | tools/rust/library/core/src/num/wrapping.rs:43 |
| `core::ops::control_flow::ControlFlow` | `.Continue` | `core::ops::control_flow::ControlFlow::Continue<$B, $C>` | tools/rust/library/core/src/ops/control_flow.rs:91 |
| `core::ops::control_flow::ControlFlow` | `.Break` | `core::ops::control_flow::ControlFlow::Break<$B, $C>` | tools/rust/library/core/src/ops/control_flow.rs:95 |
| `core::ops::control_flow::ControlFlow::Break` | `[0]` | `$B` | tools/rust/library/core/src/ops/control_flow.rs:95 |
| `core::ops::control_flow::ControlFlow::Continue` | `[0]` | `$C` | tools/rust/library/core/src/ops/control_flow.rs:91 |
| `core::ops::coroutine::CoroutineState` | `.Yielded` | `core::ops::coroutine::CoroutineState::Yielded<$Y, $R>` | tools/rust/library/core/src/ops/coroutine.rs:18 |
| `core::ops::coroutine::CoroutineState` | `.Complete` | `core::ops::coroutine::CoroutineState::Complete<$Y, $R>` | tools/rust/library/core/src/ops/coroutine.rs:25 |
| `core::ops::coroutine::CoroutineState::Complete` | `[0]` | `$R` | tools/rust/library/core/src/ops/coroutine.rs:25 |
| `core::ops::coroutine::CoroutineState::Yielded` | `[0]` | `$Y` | tools/rust/library/core/src/ops/coroutine.rs:18 |
| `core::ops::index_range::IndexRange` | `.start` | `usize` | tools/rust/library/core/src/ops/index_range.rs:13 |
| `core::ops::index_range::IndexRange` | `.end` | `usize` | tools/rust/library/core/src/ops/index_range.rs:14 |
| `core::ops::range::Bound` | `.Included` | `core::ops::range::Bound::Included<$T>` | tools/rust/library/core/src/ops/range.rs:675 |
| `core::ops::range::Bound` | `.Excluded` | `core::ops::range::Bound::Excluded<$T>` | tools/rust/library/core/src/ops/range.rs:678 |
| `core::ops::range::Bound::Excluded` | `[0]` | `$T` | tools/rust/library/core/src/ops/range.rs:678 |
| `core::ops::range::Bound::Included` | `[0]` | `$T` | tools/rust/library/core/src/ops/range.rs:675 |
| `core::ops::range::Range` | `.start` | `$Idx` | tools/rust/library/core/src/ops/range.rs:83 |
| `core::ops::range::Range` | `.end` | `$Idx` | tools/rust/library/core/src/ops/range.rs:86 |
| `core::ops::range::RangeFrom` | `.start` | `$Idx` | tools/rust/library/core/src/ops/range.rs:192 |
| `core::ops::range::RangeInclusive` | `.start` | `$Idx` | tools/rust/library/core/src/ops/range.rs:351 |
| `core::ops::range::RangeInclusive` | `.end` | `$Idx` | tools/rust/library/core/src/ops/range.rs:352 |
| `core::ops::range::RangeInclusive` | `.exhausted` | `bool` | tools/rust/library/core/src/ops/range.rs:360 |
| `core::ops::range::RangeTo` | `.end` | `$Idx` | tools/rust/library/core/src/ops/range.rs:274 |
| `core::ops::range::RangeToInclusive` | `.end` | `$Idx` | tools/rust/library/core/src/ops/range.rs:595 |
| `core::ops::try_trait::ChangeOutputType` | `(alias)` | `opaque(< T :: Residual as Residual < V > > :: TryType)` | tools/rust/library/core/src/ops/try_trait.rs:368 |
| `core::ops::try_trait::NeverShortCircuit` | `[0]` | `$T` | tools/rust/library/core/src/ops/try_trait.rs:379 |
| `core::ops::try_trait::Yeet` | `[0]` | `$T` | tools/rust/library/core/src/ops/try_trait.rs:431 |
| `core::option::IntoIter` | `.inner` | `core::option::Item<$A>` | tools/rust/library/core/src/option.rs:2369 |
| `core::option::Item` | `.opt` | `core::option::Option<$A>` | tools/rust/library/core/src/option.rs:2236 |
| `core::option::Iter` | `.inner` | `core::option::Item<$A>` | tools/rust/library/core/src/option.rs:2278 |
| `core::option::IterMut` | `.inner` | `core::option::Item<$A>` | tools/rust/library/core/src/option.rs:2328 |
| `core::option::Option` | `.Some` | `core::option::Option::Some<$T>` | tools/rust/library/core/src/option.rs:580 |
| `core::option::Option::Some` | `[0]` | `$T` | tools/rust/library/core/src/option.rs:580 |
| `core::panic::location::Location` | `.file` | `str` | tools/rust/library/core/src/panic/location.rs:35 |
| `core::panic::location::Location` | `.line` | `u32` | tools/rust/library/core/src/panic/location.rs:36 |
| `core::panic::location::Location` | `.col` | `u32` | tools/rust/library/core/src/panic/location.rs:37 |
| `core::panic::panic_info::PanicInfo` | `.message` | `core::fmt::Arguments` | tools/rust/library/core/src/panic/panic_info.rs:15 |
| `core::panic::panic_info::PanicInfo` | `.location` | `core::panic::location::Location` | tools/rust/library/core/src/panic/panic_info.rs:16 |
| `core::panic::panic_info::PanicInfo` | `.can_unwind` | `bool` | tools/rust/library/core/src/panic/panic_info.rs:17 |
| `core::panic::panic_info::PanicInfo` | `.force_no_backtrace` | `bool` | tools/rust/library/core/src/panic/panic_info.rs:18 |
| `core::panic::panic_info::PanicMessage` | `.message` | `core::fmt::Arguments` | tools/rust/library/core/src/panic/panic_info.rs:29 |
| `core::panic::unwind_safe::AssertUnwindSafe` | `[0]` | `$T` | tools/rust/library/core/src/panic/unwind_safe.rs:170 |
| `core::pin::Pin` | `.__pointer` | `$Ptr` | tools/rust/library/core/src/pin.rs:1103 |
| `core::ptr::alignment::Alignment` | `[0]` | `core::ptr::alignment::AlignmentEnum` | tools/rust/library/core/src/ptr/alignment.rs:13 |
| `core::ptr::metadata::DynMetadata` | `._vtable_ptr` | `core::ptr::non_null::NonNull<VTable>` | tools/rust/library/core/src/ptr/metadata.rs:156 |
| `core::ptr::metadata::DynMetadata` | `._phantom` | `core::marker::PhantomData<$Dyn>` | tools/rust/library/core/src/ptr/metadata.rs:157 |
| `core::ptr::non_null::NonNull` | `.pointer` | `*const $T` | tools/rust/library/core/src/ptr/non_null.rs:73 |
| `core::ptr::unique::Unique` | `.pointer` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/core/src/ptr/unique.rs:38 |
| `core::ptr::unique::Unique` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/ptr/unique.rs:44 |
| `core::range::Range` | `.start` | `$Idx` | tools/rust/library/core/src/range.rs:59 |
| `core::range::Range` | `.end` | `$Idx` | tools/rust/library/core/src/range.rs:62 |
| `core::range::RangeFrom` | `.start` | `$Idx` | tools/rust/library/core/src/range.rs:417 |
| `core::range::RangeInclusive` | `.start` | `$Idx` | tools/rust/library/core/src/range.rs:225 |
| `core::range::RangeInclusive` | `.end` | `$Idx` | tools/rust/library/core/src/range.rs:228 |
| `core::range::iter::IterRange` | `[0]` | `core::ops::range::Range<$A>` | tools/rust/library/core/src/range/iter.rs:10 |
| `core::range::iter::IterRangeFrom` | `[0]` | `core::ops::range::RangeFrom<$A>` | tools/rust/library/core/src/range/iter.rs:296 |
| `core::range::iter::IterRangeInclusive` | `[0]` | `core::ops::range::RangeInclusive<$A>` | tools/rust/library/core/src/range/iter.rs:156 |
| `core::result::IntoIter` | `.inner` | `Option<$T>` | tools/rust/library/core/src/result.rs:1908 |
| `core::result::Iter` | `.inner` | `Option<$T>` | tools/rust/library/core/src/result.rs:1813 |
| `core::result::IterMut` | `.inner` | `Option<$T>` | tools/rust/library/core/src/result.rs:1862 |
| `core::result::Result` | `.Ok` | `core::result::Result::Ok<$T, $E>` | tools/rust/library/core/src/result.rs:532 |
| `core::result::Result` | `.Err` | `core::result::Result::Err<$T, $E>` | tools/rust/library/core/src/result.rs:537 |
| `core::result::Result::Err` | `[0]` | `$E` | tools/rust/library/core/src/result.rs:537 |
| `core::result::Result::Ok` | `[0]` | `$T` | tools/rust/library/core/src/result.rs:532 |
| `core::slice::ascii::EscapeAscii` | `.inner` | `core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<u8>, core::ascii::EscapeDefault, EscapeByte>` | tools/rust/library/core/src/slice/ascii.rs:237 |
| `core::slice::iter::ArrayChunks` | `.iter` | `core::slice::iter::Iter<[$T; N]>` | tools/rust/library/core/src/slice/iter.rs:2332 |
| `core::slice::iter::ArrayChunks` | `.rem` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:2333 |
| `core::slice::iter::ArrayChunksMut` | `.iter` | `core::slice::iter::IterMut<[$T; N]>` | tools/rust/library/core/src/slice/iter.rs:2458 |
| `core::slice::iter::ArrayChunksMut` | `.rem` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:2459 |
| `core::slice::iter::ArrayWindows` | `.slice_head` | `*const $T` | tools/rust/library/core/src/slice/iter.rs:2206 |
| `core::slice::iter::ArrayWindows` | `.num` | `usize` | tools/rust/library/core/src/slice/iter.rs:2207 |
| `core::slice::iter::ArrayWindows` | `.marker` | `core::marker::PhantomData<[$T; N]>` | tools/rust/library/core/src/slice/iter.rs:2208 |
| `core::slice::iter::ChunkBy` | `.slice` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:3305 |
| `core::slice::iter::ChunkBy` | `.predicate` | `$P` | tools/rust/library/core/src/slice/iter.rs:3306 |
| `core::slice::iter::ChunkByMut` | `.slice` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:3392 |
| `core::slice::iter::ChunkByMut` | `.predicate` | `$P` | tools/rust/library/core/src/slice/iter.rs:3393 |
| `core::slice::iter::Chunks` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:1484 |
| `core::slice::iter::Chunks` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:1485 |
| `core::slice::iter::ChunksExact` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:1859 |
| `core::slice::iter::ChunksExact` | `.rem` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:1860 |
| `core::slice::iter::ChunksExact` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:1861 |
| `core::slice::iter::ChunksExactMut` | `.v` | `*mut [$T]` | tools/rust/library/core/src/slice/iter.rs:2038 |
| `core::slice::iter::ChunksExactMut` | `.rem` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:2039 |
| `core::slice::iter::ChunksExactMut` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:2040 |
| `core::slice::iter::ChunksExactMut` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/slice/iter.rs:2041 |
| `core::slice::iter::ChunksMut` | `.v` | `*mut [$T]` | tools/rust/library/core/src/slice/iter.rs:1673 |
| `core::slice::iter::ChunksMut` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:1674 |
| `core::slice::iter::ChunksMut` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/slice/iter.rs:1675 |
| `core::slice::iter::GenericSplitN` | `.iter` | `$I` | tools/rust/library/core/src/slice/iter.rs:1106 |
| `core::slice::iter::GenericSplitN` | `.count` | `usize` | tools/rust/library/core/src/slice/iter.rs:1107 |
| `core::slice::iter::Iter` | `.ptr` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/core/src/slice/iter.rs:74 |
| `core::slice::iter::Iter` | `.end_or_len` | `*const $T` | tools/rust/library/core/src/slice/iter.rs:78 |
| `core::slice::iter::Iter` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/slice/iter.rs:79 |
| `core::slice::iter::IterMut` | `.ptr` | `core::ptr::non_null::NonNull<$T>` | tools/rust/library/core/src/slice/iter.rs:199 |
| `core::slice::iter::IterMut` | `.end_or_len` | `*mut $T` | tools/rust/library/core/src/slice/iter.rs:203 |
| `core::slice::iter::IterMut` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/slice/iter.rs:204 |
| `core::slice::iter::RChunks` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:2576 |
| `core::slice::iter::RChunks` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:2577 |
| `core::slice::iter::RChunksExact` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:2946 |
| `core::slice::iter::RChunksExact` | `.rem` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:2947 |
| `core::slice::iter::RChunksExact` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:2948 |
| `core::slice::iter::RChunksExactMut` | `.v` | `*mut [$T]` | tools/rust/library/core/src/slice/iter.rs:3128 |
| `core::slice::iter::RChunksExactMut` | `.rem` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:3129 |
| `core::slice::iter::RChunksExactMut` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:3130 |
| `core::slice::iter::RChunksMut` | `.v` | `*mut [$T]` | tools/rust/library/core/src/slice/iter.rs:2755 |
| `core::slice::iter::RChunksMut` | `.chunk_size` | `usize` | tools/rust/library/core/src/slice/iter.rs:2756 |
| `core::slice::iter::RChunksMut` | `._marker` | `core::marker::PhantomData<$T>` | tools/rust/library/core/src/slice/iter.rs:2757 |
| `core::slice::iter::RSplit` | `.inner` | `core::slice::iter::Split<$T, $P>` | tools/rust/library/core/src/slice/iter.rs:938 |
| `core::slice::iter::RSplitMut` | `.inner` | `core::slice::iter::SplitMut<$T, $P>` | tools/rust/library/core/src/slice/iter.rs:1035 |
| `core::slice::iter::RSplitN` | `.inner` | `core::slice::iter::GenericSplitN<core::slice::iter::RSplit<$T, $P>>` | tools/rust/library/core/src/slice/iter.rs:1205 |
| `core::slice::iter::RSplitNMut` | `.inner` | `core::slice::iter::GenericSplitN<core::slice::iter::RSplitMut<$T, $P>>` | tools/rust/library/core/src/slice/iter.rs:1286 |
| `core::slice::iter::Split` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:406 |
| `core::slice::iter::Split` | `.pred` | `$P` | tools/rust/library/core/src/slice/iter.rs:407 |
| `core::slice::iter::Split` | `.finished` | `bool` | tools/rust/library/core/src/slice/iter.rs:409 |
| `core::slice::iter::SplitInclusive` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:561 |
| `core::slice::iter::SplitInclusive` | `.pred` | `$P` | tools/rust/library/core/src/slice/iter.rs:562 |
| `core::slice::iter::SplitInclusive` | `.finished` | `bool` | tools/rust/library/core/src/slice/iter.rs:563 |
| `core::slice::iter::SplitInclusiveMut` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:811 |
| `core::slice::iter::SplitInclusiveMut` | `.pred` | `$P` | tools/rust/library/core/src/slice/iter.rs:812 |
| `core::slice::iter::SplitInclusiveMut` | `.finished` | `bool` | tools/rust/library/core/src/slice/iter.rs:813 |
| `core::slice::iter::SplitMut` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:682 |
| `core::slice::iter::SplitMut` | `.pred` | `$P` | tools/rust/library/core/src/slice/iter.rs:683 |
| `core::slice::iter::SplitMut` | `.finished` | `bool` | tools/rust/library/core/src/slice/iter.rs:684 |
| `core::slice::iter::SplitN` | `.inner` | `core::slice::iter::GenericSplitN<core::slice::iter::Split<$T, $P>>` | tools/rust/library/core/src/slice/iter.rs:1161 |
| `core::slice::iter::SplitNMut` | `.inner` | `core::slice::iter::GenericSplitN<core::slice::iter::SplitMut<$T, $P>>` | tools/rust/library/core/src/slice/iter.rs:1245 |
| `core::slice::iter::Windows` | `.v` | `[$T]` | tools/rust/library/core/src/slice/iter.rs:1332 |
| `core::slice::iter::Windows` | `.size` | `core::num::nonzero::NonZero<usize>` | tools/rust/library/core/src/slice/iter.rs:1333 |
| `core::slice::rotate::BufType` | `(alias)` | `[usize; 32]` | tools/rust/library/core/src/slice/rotate.rs:4 |
| `core::slice::sort::shared::smallsort::CopyOnDrop` | `.src` | `*const $T` | tools/rust/library/core/src/slice/sort/shared/smallsort.rs:296 |
| `core::slice::sort::shared::smallsort::CopyOnDrop` | `.dst` | `*mut $T` | tools/rust/library/core/src/slice/sort/shared/smallsort.rs:297 |
| `core::slice::sort::shared::smallsort::CopyOnDrop` | `.len` | `usize` | tools/rust/library/core/src/slice/sort/shared/smallsort.rs:298 |
| `core::slice::sort::stable::AlignedStorage` | `._align` | `[$T; 0]` | tools/rust/library/core/src/slice/sort/stable/mod.rs:150 |
| `core::slice::sort::stable::AlignedStorage` | `.storage` | `[mem::MaybeUninit<u8>; N]` | tools/rust/library/core/src/slice/sort/stable/mod.rs:151 |
| `core::slice::sort::stable::drift::DriftsortRun` | `[0]` | `usize` | tools/rust/library/core/src/slice/sort/stable/drift.rs:276 |
| `core::slice::sort::stable::merge::MergeState` | `.start` | `*mut $T` | tools/rust/library/core/src/slice/sort/stable/merge.rs:67 |
| `core::slice::sort::stable::merge::MergeState` | `.end` | `*mut $T` | tools/rust/library/core/src/slice/sort/stable/merge.rs:68 |
| `core::slice::sort::stable::merge::MergeState` | `.dst` | `*mut $T` | tools/rust/library/core/src/slice/sort/stable/merge.rs:69 |
| `core::slice::sort::stable::quicksort::PartitionState` | `.scratch_base` | `*mut $T` | tools/rust/library/core/src/slice/sort/stable/quicksort.rs:187 |
| `core::slice::sort::stable::quicksort::PartitionState` | `.scan` | `*const $T` | tools/rust/library/core/src/slice/sort/stable/quicksort.rs:189 |
| `core::slice::sort::stable::quicksort::PartitionState` | `.num_left` | `usize` | tools/rust/library/core/src/slice/sort/stable/quicksort.rs:192 |
| `core::slice::sort::stable::quicksort::PartitionState` | `.scratch_rev` | `*mut $T` | tools/rust/library/core/src/slice/sort/stable/quicksort.rs:194 |
| `core::slice::sort::unstable::quicksort::GapGuard` | `.pos` | `*mut $T` | tools/rust/library/core/src/slice/sort/unstable/quicksort.rs:363 |
| `core::slice::sort::unstable::quicksort::GapGuard` | `.value` | `core::mem::manually_drop::ManuallyDrop<$T>` | tools/rust/library/core/src/slice/sort/unstable/quicksort.rs:364 |
| `core::slice::sort::unstable::quicksort::GapGuardRaw` | `.pos` | `*mut $T` | tools/rust/library/core/src/slice/sort/unstable/quicksort.rs:381 |
| `core::slice::sort::unstable::quicksort::GapGuardRaw` | `.value` | `*mut $T` | tools/rust/library/core/src/slice/sort/unstable/quicksort.rs:382 |
| `core::slice::sort::unstable::quicksort::PartitionState` | `.right` | `*mut $T` | tools/rust/library/core/src/slice/sort/unstable/quicksort.rs:241 |
| `core::slice::sort::unstable::quicksort::PartitionState` | `.num_lt` | `usize` | tools/rust/library/core/src/slice/sort/unstable/quicksort.rs:244 |
| `core::slice::sort::unstable::quicksort::PartitionState` | `.gap` | `core::slice::sort::unstable::quicksort::GapGuardRaw<$T>` | tools/rust/library/core/src/slice/sort/unstable/quicksort.rs:246 |
| `core::str::error::Utf8Error` | `.valid_up_to` | `usize` | tools/rust/library/core/src/str/error.rs:48 |
| `core::str::error::Utf8Error` | `.error_len` | `Option<u8>` | tools/rust/library/core/src/str/error.rs:49 |
| `core::str::iter::Bytes` | `[0]` | `core::iter::adapters::copied::Copied<core::slice::iter::Iter<u8>>` | tools/rust/library/core/src/str/iter.rs:287 |
| `core::str::iter::CharIndices` | `.front_offset` | `usize` | tools/rust/library/core/src/str/iter.rs:176 |
| `core::str::iter::CharIndices` | `.iter` | `core::str::iter::Chars` | tools/rust/library/core/src/str/iter.rs:177 |
| `core::str::iter::Chars` | `.iter` | `core::slice::iter::Iter<u8>` | tools/rust/library/core/src/str/iter.rs:31 |
| `core::str::iter::EncodeUtf16` | `.chars` | `core::str::iter::Chars` | tools/rust/library/core/src/str/iter.rs:1490 |
| `core::str::iter::EncodeUtf16` | `.extra` | `u16` | tools/rust/library/core/src/str/iter.rs:1491 |
| `core::str::iter::EscapeDebug` | `.inner` | `core::iter::adapters::chain::Chain<core::iter::adapters::flatten::Flatten<core::option::IntoIter<core::char::EscapeDebug>>, core::iter::adapters::flatten::FlatMap<core::str::iter::Chars, core::char::EscapeDebug, CharEscapeDebugContinue>>` | tools/rust/library/core/src/str/iter.rs:1551 |
| `core::str::iter::EscapeDefault` | `.inner` | `core::iter::adapters::flatten::FlatMap<core::str::iter::Chars, core::char::EscapeDefault, CharEscapeDefault>` | tools/rust/library/core/src/str/iter.rs:1561 |
| `core::str::iter::EscapeUnicode` | `.inner` | `core::iter::adapters::flatten::FlatMap<core::str::iter::Chars, core::char::EscapeUnicode, CharEscapeUnicode>` | tools/rust/library/core/src/str/iter.rs:1568 |
| `core::str::iter::Lines` | `[0]` | `core::iter::adapters::map::Map<core::str::iter::SplitInclusive<char>, LinesMap>` | tools/rust/library/core/src/str/iter.rs:1165 |
| `core::str::iter::LinesAny` | `[0]` | `core::str::iter::Lines` | tools/rust/library/core/src/str/iter.rs:1231 |
| `core::str::iter::MatchIndicesInternal` | `[0]` | `P::Searcher` | tools/rust/library/core/src/str/iter.rs:1049 |
| `core::str::iter::MatchesInternal` | `[0]` | `P::Searcher` | tools/rust/library/core/src/str/iter.rs:1104 |
| `core::str::iter::SplitAsciiWhitespace` | `.inner` | `core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Split<u8, IsAsciiWhitespace>, BytesIsNotEmpty>, UnsafeBytesToStr>` | tools/rust/library/core/src/str/iter.rs:1286 |
| `core::str::iter::SplitInclusive` | `[0]` | `core::str::iter::SplitInternal<$P>` | tools/rust/library/core/src/str/iter.rs:1299 |
| `core::str::iter::SplitInternal` | `.start` | `usize` | tools/rust/library/core/src/str/iter.rs:621 |
| `core::str::iter::SplitInternal` | `.end` | `usize` | tools/rust/library/core/src/str/iter.rs:622 |
| `core::str::iter::SplitInternal` | `.matcher` | `P::Searcher` | tools/rust/library/core/src/str/iter.rs:623 |
| `core::str::iter::SplitInternal` | `.allow_trailing_empty` | `bool` | tools/rust/library/core/src/str/iter.rs:624 |
| `core::str::iter::SplitInternal` | `.finished` | `bool` | tools/rust/library/core/src/str/iter.rs:625 |
| `core::str::iter::SplitNInternal` | `.iter` | `core::str::iter::SplitInternal<$P>` | tools/rust/library/core/src/str/iter.rs:923 |
| `core::str::iter::SplitNInternal` | `.count` | `usize` | tools/rust/library/core/src/str/iter.rs:925 |
| `core::str::iter::SplitWhitespace` | `.inner` | `core::iter::adapters::filter::Filter<Split<IsWhitespace>, IsNotEmpty>` | tools/rust/library/core/src/str/iter.rs:1272 |
| `core::str::lossy::Debug` | `[0]` | `[u8]` | tools/rust/library/core/src/str/lossy.rs:110 |
| `core::str::lossy::Utf8Chunk` | `.valid` | `str` | tools/rust/library/core/src/str/lossy.rs:72 |
| `core::str::lossy::Utf8Chunk` | `.invalid` | `[u8]` | tools/rust/library/core/src/str/lossy.rs:73 |
| `core::str::lossy::Utf8Chunks` | `.source` | `[u8]` | tools/rust/library/core/src/str/lossy.rs:180 |
| `core::str::pattern::CharArrayRefSearcher` | `[0]` | `opaque(< MultiCharEqPattern < & 'b [char ; N] > as Pattern > :: Searcher < 'a >)` | tools/rust/library/core/src/str/pattern.rs:818 |
| `core::str::pattern::CharArraySearcher` | `[0]` | `opaque(< MultiCharEqPattern < [char ; N] > as Pattern > :: Searcher < 'a >)` | tools/rust/library/core/src/str/pattern.rs:812 |
| `core::str::pattern::CharPredicateSearcher` | `[0]` | `opaque(< MultiCharEqPattern < F > as Pattern > :: Searcher < 'a >)` | tools/rust/library/core/src/str/pattern.rs:903 |
| `core::str::pattern::CharSearcher` | `.haystack` | `str` | tools/rust/library/core/src/str/pattern.rs:370 |
| `core::str::pattern::CharSearcher` | `.finger` | `usize` | tools/rust/library/core/src/str/pattern.rs:378 |
| `core::str::pattern::CharSearcher` | `.finger_back` | `usize` | tools/rust/library/core/src/str/pattern.rs:383 |
| `core::str::pattern::CharSearcher` | `.needle` | `char` | tools/rust/library/core/src/str/pattern.rs:385 |
| `core::str::pattern::CharSearcher` | `.utf8_size` | `u8` | tools/rust/library/core/src/str/pattern.rs:389 |
| `core::str::pattern::CharSearcher` | `.utf8_encoded` | `[u8; 4]` | tools/rust/library/core/src/str/pattern.rs:391 |
| `core::str::pattern::CharSliceSearcher` | `[0]` | `opaque(< MultiCharEqPattern < & 'b [char] > as Pattern > :: Searcher < 'a >)` | tools/rust/library/core/src/str/pattern.rs:873 |
| `core::str::pattern::EmptyNeedle` | `.position` | `usize` | tools/rust/library/core/src/str/pattern.rs:1074 |
| `core::str::pattern::EmptyNeedle` | `.end` | `usize` | tools/rust/library/core/src/str/pattern.rs:1075 |
| `core::str::pattern::EmptyNeedle` | `.is_match_fw` | `bool` | tools/rust/library/core/src/str/pattern.rs:1076 |
| `core::str::pattern::EmptyNeedle` | `.is_match_bw` | `bool` | tools/rust/library/core/src/str/pattern.rs:1077 |
| `core::str::pattern::EmptyNeedle` | `.is_finished` | `bool` | tools/rust/library/core/src/str/pattern.rs:1079 |
| `core::str::pattern::MultiCharEqPattern` | `[0]` | `$C` | tools/rust/library/core/src/str/pattern.rs:665 |
| `core::str::pattern::MultiCharEqSearcher` | `.char_eq` | `$C` | tools/rust/library/core/src/str/pattern.rs:669 |
| `core::str::pattern::MultiCharEqSearcher` | `.haystack` | `str` | tools/rust/library/core/src/str/pattern.rs:670 |
| `core::str::pattern::MultiCharEqSearcher` | `.char_indices` | `core::str::iter::CharIndices` | tools/rust/library/core/src/str/pattern.rs:671 |
| `core::str::pattern::SearchStep` | `.Match` | `core::str::pattern::SearchStep::Match` | tools/rust/library/core/src/str/pattern.rs:188 |
| `core::str::pattern::SearchStep` | `.Reject` | `core::str::pattern::SearchStep::Reject` | tools/rust/library/core/src/str/pattern.rs:194 |
| `core::str::pattern::SearchStep::Match` | `[0]` | `usize` | tools/rust/library/core/src/str/pattern.rs:188 |
| `core::str::pattern::SearchStep::Match` | `[1]` | `usize` | tools/rust/library/core/src/str/pattern.rs:188 |
| `core::str::pattern::SearchStep::Reject` | `[0]` | `usize` | tools/rust/library/core/src/str/pattern.rs:194 |
| `core::str::pattern::SearchStep::Reject` | `[1]` | `usize` | tools/rust/library/core/src/str/pattern.rs:194 |
| `core::str::pattern::StrSearcher` | `.haystack` | `str` | tools/rust/library/core/src/str/pattern.rs:1060 |
| `core::str::pattern::StrSearcher` | `.needle` | `str` | tools/rust/library/core/src/str/pattern.rs:1061 |
| `core::str::pattern::StrSearcher` | `.searcher` | `core::str::pattern::StrSearcherImpl` | tools/rust/library/core/src/str/pattern.rs:1063 |
| `core::str::pattern::StrSearcherImpl` | `.Empty` | `core::str::pattern::StrSearcherImpl::Empty` | tools/rust/library/core/src/str/pattern.rs:1068 |
| `core::str::pattern::StrSearcherImpl` | `.TwoWay` | `core::str::pattern::StrSearcherImpl::TwoWay` | tools/rust/library/core/src/str/pattern.rs:1069 |
| `core::str::pattern::StrSearcherImpl::Empty` | `[0]` | `core::str::pattern::EmptyNeedle` | tools/rust/library/core/src/str/pattern.rs:1068 |
| `core::str::pattern::StrSearcherImpl::TwoWay` | `[0]` | `core::str::pattern::TwoWaySearcher` | tools/rust/library/core/src/str/pattern.rs:1069 |
| `core::str::pattern::TwoWaySearcher` | `.crit_pos` | `usize` | tools/rust/library/core/src/str/pattern.rs:1282 |
| `core::str::pattern::TwoWaySearcher` | `.crit_pos_back` | `usize` | tools/rust/library/core/src/str/pattern.rs:1284 |
| `core::str::pattern::TwoWaySearcher` | `.period` | `usize` | tools/rust/library/core/src/str/pattern.rs:1285 |
| `core::str::pattern::TwoWaySearcher` | `.byteset` | `u64` | tools/rust/library/core/src/str/pattern.rs:1289 |
| `core::str::pattern::TwoWaySearcher` | `.position` | `usize` | tools/rust/library/core/src/str/pattern.rs:1292 |
| `core::str::pattern::TwoWaySearcher` | `.end` | `usize` | tools/rust/library/core/src/str/pattern.rs:1293 |
| `core::str::pattern::TwoWaySearcher` | `.memory` | `usize` | tools/rust/library/core/src/str/pattern.rs:1295 |
| `core::str::pattern::TwoWaySearcher` | `.memory_back` | `usize` | tools/rust/library/core/src/str/pattern.rs:1297 |
| `core::str::pattern::Utf8Pattern` | `.StringPattern` | `core::str::pattern::Utf8Pattern::StringPattern` | tools/rust/library/core/src/str/pattern.rs:176 |
| `core::str::pattern::Utf8Pattern` | `.CharPattern` | `core::str::pattern::Utf8Pattern::CharPattern` | tools/rust/library/core/src/str/pattern.rs:178 |
| `core::str::pattern::Utf8Pattern::CharPattern` | `[0]` | `char` | tools/rust/library/core/src/str/pattern.rs:178 |
| `core::str::pattern::Utf8Pattern::StringPattern` | `[0]` | `[u8]` | tools/rust/library/core/src/str/pattern.rs:176 |
| `core::sync::atomic::AtomicBool` | `.v` | `core::cell::UnsafeCell<u8>` | tools/rust/library/core/src/sync/atomic.rs:272 |
| `core::sync::atomic::AtomicPtr` | `.p` | `core::cell::UnsafeCell<*mut $T>` | tools/rust/library/core/src/sync/atomic.rs:303 |
| `core::sync::exclusive::Exclusive` | `.inner` | `$T` | tools/rust/library/core/src/sync/exclusive.rs:84 |
| `core::task::poll::Poll` | `.Ready` | `core::task::poll::Poll::Ready<$T>` | tools/rust/library/core/src/task/poll.rs:18 |
| `core::task::poll::Poll::Ready` | `[0]` | `$T` | tools/rust/library/core/src/task/poll.rs:18 |
| `core::task::wake::Context` | `.waker` | `core::task::wake::Waker` | tools/rust/library/core/src/task/wake.rs:218 |
| `core::task::wake::Context` | `.local_waker` | `core::task::wake::LocalWaker` | tools/rust/library/core/src/task/wake.rs:219 |
| `core::task::wake::Context` | `.ext` | `core::panic::unwind_safe::AssertUnwindSafe<core::task::wake::ExtData>` | tools/rust/library/core/src/task/wake.rs:220 |
| `core::task::wake::Context` | `._marker` | `core::marker::PhantomData<opaque(fn (& 'a ()) -> & 'a ())>` | tools/rust/library/core/src/task/wake.rs:225 |
| `core::task::wake::Context` | `._marker2` | `core::marker::PhantomData<*mut ()>` | tools/rust/library/core/src/task/wake.rs:228 |
| `core::task::wake::ContextBuilder` | `.waker` | `core::task::wake::Waker` | tools/rust/library/core/src/task/wake.rs:301 |
| `core::task::wake::ContextBuilder` | `.local_waker` | `core::task::wake::LocalWaker` | tools/rust/library/core/src/task/wake.rs:302 |
| `core::task::wake::ContextBuilder` | `.ext` | `core::task::wake::ExtData` | tools/rust/library/core/src/task/wake.rs:303 |
| `core::task::wake::ContextBuilder` | `._marker` | `core::marker::PhantomData<opaque(fn (& 'a ()) -> & 'a ())>` | tools/rust/library/core/src/task/wake.rs:308 |
| `core::task::wake::ContextBuilder` | `._marker2` | `core::marker::PhantomData<*mut ()>` | tools/rust/library/core/src/task/wake.rs:311 |
| `core::task::wake::ExtData` | `.Some` | `core::task::wake::ExtData::Some` | tools/rust/library/core/src/task/wake.rs:207 |
| `core::task::wake::ExtData` | `.None` | `core::task::wake::ExtData::None` | tools/rust/library/core/src/task/wake.rs:208 |
| `core::task::wake::ExtData::None` | `[0]` | `()` | tools/rust/library/core/src/task/wake.rs:208 |
| `core::task::wake::ExtData::Some` | `[0]` | `opaque(dyn Any)` | tools/rust/library/core/src/task/wake.rs:207 |
| `core::task::wake::LocalWaker` | `.waker` | `core::task::wake::RawWaker` | tools/rust/library/core/src/task/wake.rs:718 |
| `core::task::wake::RawWaker` | `.data` | `*const ()` | tools/rust/library/core/src/task/wake.rs:28 |
| `core::task::wake::RawWaker` | `.vtable` | `core::task::wake::RawWakerVTable` | tools/rust/library/core/src/task/wake.rs:30 |
| `core::task::wake::RawWakerVTable` | `.clone` | `opaque(unsafe fn (* const ()) -> RawWaker)` | tools/rust/library/core/src/task/wake.rs:116 |
| `core::task::wake::RawWakerVTable` | `.wake` | `opaque(unsafe fn (* const ()))` | tools/rust/library/core/src/task/wake.rs:124 |
| `core::task::wake::RawWakerVTable` | `.wake_by_ref` | `opaque(unsafe fn (* const ()))` | tools/rust/library/core/src/task/wake.rs:131 |
| `core::task::wake::RawWakerVTable` | `.drop` | `opaque(unsafe fn (* const ()))` | tools/rust/library/core/src/task/wake.rs:138 |
| `core::task::wake::Waker` | `.waker` | `core::task::wake::RawWaker` | tools/rust/library/core/src/task/wake.rs:407 |
| `core::time::Duration` | `.secs` | `u64` | tools/rust/library/core/src/time.rs:82 |
| `core::time::Duration` | `.nanos` | `num::niche_types::Nanoseconds` | tools/rust/library/core/src/time.rs:83 |
| `core::time::TryFromFloatSecsError` | `.kind` | `core::time::TryFromFloatSecsErrorKind` | tools/rust/library/core/src/time.rs:1436 |
| `std::backtrace::Backtrace` | `.inner` | `std::backtrace::Inner` | tools/rust/library/std/src/backtrace.rs:109 |
| `std::backtrace::BacktraceFrame` | `.frame` | `std::backtrace::RawFrame` | tools/rust/library/std/src/backtrace.rs:151 |
| `std::backtrace::BacktraceFrame` | `.symbols` | `Vec<std::backtrace::BacktraceSymbol>` | tools/rust/library/std/src/backtrace.rs:152 |
| `std::backtrace::BacktraceSymbol` | `.name` | `Option<Vec<u8>>` | tools/rust/library/std/src/backtrace.rs:163 |
| `std::backtrace::BacktraceSymbol` | `.filename` | `Option<std::backtrace::BytesOrWide>` | tools/rust/library/std/src/backtrace.rs:164 |
| `std::backtrace::BacktraceSymbol` | `.lineno` | `Option<u32>` | tools/rust/library/std/src/backtrace.rs:165 |
| `std::backtrace::BacktraceSymbol` | `.colno` | `Option<u32>` | tools/rust/library/std/src/backtrace.rs:166 |
| `std::backtrace::BytesOrWide` | `.Bytes` | `std::backtrace::BytesOrWide::Bytes` | tools/rust/library/std/src/backtrace.rs:170 |
| `std::backtrace::BytesOrWide` | `.Wide` | `std::backtrace::BytesOrWide::Wide` | tools/rust/library/std/src/backtrace.rs:171 |
| `std::backtrace::BytesOrWide::Bytes` | `[0]` | `Vec<u8>` | tools/rust/library/std/src/backtrace.rs:170 |
| `std::backtrace::BytesOrWide::Wide` | `[0]` | `Vec<u16>` | tools/rust/library/std/src/backtrace.rs:171 |
| `std::backtrace::Capture` | `.actual_start` | `usize` | tools/rust/library/std/src/backtrace.rs:139 |
| `std::backtrace::Capture` | `.frames` | `Vec<std::backtrace::BacktraceFrame>` | tools/rust/library/std/src/backtrace.rs:140 |
| `std::backtrace::Inner` | `.Captured` | `std::backtrace::Inner::Captured` | tools/rust/library/std/src/backtrace.rs:135 |
| `std::backtrace::Inner::Captured` | `[0]` | `std::sync::lazy_lock::LazyLock<std::backtrace::Capture, LazyResolve>` | tools/rust/library/std/src/backtrace.rs:135 |
| `std::backtrace::RawFrame` | `.Actual` | `std::backtrace::RawFrame::Actual` | tools/rust/library/std/src/backtrace.rs:157 |
| `std::backtrace::RawFrame::Actual` | `[0]` | `backtrace_rs::Frame` | tools/rust/library/std/src/backtrace.rs:157 |
| `std::backtrace::helper::LazyResolve` | `(alias)` | `opaque(impl (FnOnce () -> Capture) + Send + Sync + UnwindSafe)` | tools/rust/library/std/src/backtrace.rs:433 |
| `std::collections::hash::map::Drain` | `.base` | `hashbrown::hash_map::Drain<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1654 |
| `std::collections::hash::map::Entry` | `.Occupied` | `std::collections::hash::map::Entry::Occupied<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1793 |
| `std::collections::hash::map::Entry` | `.Vacant` | `std::collections::hash::map::Entry::Vacant<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1797 |
| `std::collections::hash::map::Entry::Occupied` | `[0]` | `std::collections::hash::map::OccupiedEntry<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1793 |
| `std::collections::hash::map::Entry::Vacant` | `[0]` | `std::collections::hash::map::VacantEntry<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1797 |
| `std::collections::hash::map::ExtractIf` | `.base` | `hashbrown::hash_map::ExtractIf<$K, $V, $F>` | tools/rust/library/std/src/collections/hash/map.rs:1689 |
| `std::collections::hash::map::HashMap` | `.base` | `hashbrown::hash_map::HashMap<$K, $V, $S>` | tools/rust/library/std/src/collections/hash/map.rs:243 |
| `std::collections::hash::map::IntoIter` | `.base` | `hashbrown::hash_map::IntoIter<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1521 |
| `std::collections::hash::map::IntoKeys` | `.inner` | `std::collections::hash::map::IntoIter<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1742 |
| `std::collections::hash::map::IntoValues` | `.inner` | `std::collections::hash::map::IntoIter<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1772 |
| `std::collections::hash::map::Iter` | `.base` | `hashbrown::hash_map::Iter<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1436 |
| `std::collections::hash::map::IterMut` | `.base` | `hashbrown::hash_map::IterMut<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1483 |
| `std::collections::hash::map::Keys` | `.inner` | `std::collections::hash::map::Iter<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1560 |
| `std::collections::hash::map::OccupiedEntry` | `.base` | `hashbrown::hash_map::RustcOccupiedEntry<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1814 |
| `std::collections::hash::map::OccupiedError` | `.entry` | `std::collections::hash::map::OccupiedEntry<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1847 |
| `std::collections::hash::map::OccupiedError` | `.value` | `$V` | tools/rust/library/std/src/collections/hash/map.rs:1849 |
| `std::collections::hash::map::VacantEntry` | `.base` | `hashbrown::hash_map::RustcVacantEntry<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1831 |
| `std::collections::hash::map::Values` | `.inner` | `std::collections::hash::map::Iter<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1607 |
| `std::collections::hash::map::ValuesMut` | `.inner` | `std::collections::hash::map::IterMut<$K, $V>` | tools/rust/library/std/src/collections/hash/map.rs:1712 |
| `std::collections::hash::set::Difference` | `.iter` | `std::collections::hash::set::Iter<$T>` | tools/rust/library/std/src/collections/hash/set.rs:1453 |
| `std::collections::hash::set::Difference` | `.other` | `std::collections::hash::set::HashSet<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:1455 |
| `std::collections::hash::set::Drain` | `.base` | `hashbrown::hash_set::Drain<$K>` | tools/rust/library/std/src/collections/hash/set.rs:1376 |
| `std::collections::hash::set::Entry` | `.Occupied` | `std::collections::hash::set::Entry::Occupied<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:1997 |
| `std::collections::hash::set::Entry` | `.Vacant` | `std::collections::hash::set::Entry::Vacant<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:2015 |
| `std::collections::hash::set::Entry::Occupied` | `[0]` | `std::collections::hash::set::OccupiedEntry<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:1997 |
| `std::collections::hash::set::Entry::Vacant` | `[0]` | `std::collections::hash::set::VacantEntry<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:2015 |
| `std::collections::hash::set::ExtractIf` | `.base` | `hashbrown::hash_set::ExtractIf<$K, $F>` | tools/rust/library/std/src/collections/hash/set.rs:1401 |
| `std::collections::hash::set::HashSet` | `.base` | `hashbrown::hash_set::HashSet<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:126 |
| `std::collections::hash::set::Intersection` | `.iter` | `std::collections::hash::set::Iter<$T>` | tools/rust/library/std/src/collections/hash/set.rs:1426 |
| `std::collections::hash::set::Intersection` | `.other` | `std::collections::hash::set::HashSet<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:1428 |
| `std::collections::hash::set::IntoIter` | `.base` | `hashbrown::hash_set::IntoIter<$K>` | tools/rust/library/std/src/collections/hash/set.rs:1346 |
| `std::collections::hash::set::Iter` | `.base` | `hashbrown::hash_set::Iter<$K>` | tools/rust/library/std/src/collections/hash/set.rs:1317 |
| `std::collections::hash::set::OccupiedEntry` | `.base` | `hashbrown::hash_set::OccupiedEntry<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:2068 |
| `std::collections::hash::set::SymmetricDifference` | `.iter` | `iter::Chain<std::collections::hash::set::Difference<$T, $S>, std::collections::hash::set::Difference<$T, $S>>` | tools/rust/library/std/src/collections/hash/set.rs:1479 |
| `std::collections::hash::set::Union` | `.iter` | `iter::Chain<std::collections::hash::set::Iter<$T>, std::collections::hash::set::Difference<$T, $S>>` | tools/rust/library/std/src/collections/hash/set.rs:1503 |
| `std::collections::hash::set::VacantEntry` | `.base` | `hashbrown::hash_set::VacantEntry<$T, $S>` | tools/rust/library/std/src/collections/hash/set.rs:2108 |
| `std::env::Args` | `.inner` | `std::env::ArgsOs` | tools/rust/library/std/src/env.rs:774 |
| `std::env::ArgsOs` | `.inner` | `sys::args::Args` | tools/rust/library/std/src/env.rs:791 |
| `std::env::JoinPathsError` | `.inner` | `sys::os::JoinPathsError` | tools/rust/library/std/src/env.rs:517 |
| `std::env::SplitPaths` | `.inner` | `sys::os::SplitPaths` | tools/rust/library/std/src/env.rs:453 |
| `std::env::VarError` | `.NotUnicode` | `std::env::VarError::NotUnicode` | tools/rust/library/std/src/env.rs:281 |
| `std::env::VarError::NotUnicode` | `[0]` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/env.rs:281 |
| `std::env::Vars` | `.inner` | `std::env::VarsOs` | tools/rust/library/std/src/env.rs:89 |
| `std::env::VarsOs` | `.inner` | `sys::os::Env` | tools/rust/library/std/src/env.rs:99 |
| `std::error::Indented` | `.inner` | `$D` | tools/rust/library/std/src/error.rs:543 |
| `std::error::Report` | `.error` | `$E` | tools/rust/library/std/src/error.rs:223 |
| `std::error::Report` | `.show_backtrace` | `bool` | tools/rust/library/std/src/error.rs:225 |
| `std::error::Report` | `.pretty` | `bool` | tools/rust/library/std/src/error.rs:227 |
| `std::ffi::os_str::Display` | `.os_str` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/ffi/os_str.rs:1617 |
| `std::ffi::os_str::OsStr` | `.inner` | `sys::os_str::Slice` | tools/rust/library/std/src/ffi/os_str.rs:120 |
| `std::ffi::os_str::OsString` | `.inner` | `sys::os_str::Buf` | tools/rust/library/std/src/ffi/os_str.rs:94 |
| `std::fs::DirBuilder` | `.inner` | `sys::fs::DirBuilder` | tools/rust/library/std/src/fs.rs:237 |
| `std::fs::DirBuilder` | `.recursive` | `bool` | tools/rust/library/std/src/fs.rs:238 |
| `std::fs::DirEntry` | `[0]` | `sys::fs::DirEntry` | tools/rust/library/std/src/fs.rs:163 |
| `std::fs::File` | `.inner` | `sys::fs::File` | tools/rust/library/std/src/fs.rs:116 |
| `std::fs::FileTimes` | `[0]` | `sys::fs::FileTimes` | tools/rust/library/std/src/fs.rs:208 |
| `std::fs::FileType` | `[0]` | `sys::fs::FileType` | tools/rust/library/std/src/fs.rs:228 |
| `std::fs::Metadata` | `[0]` | `sys::fs::FileAttr` | tools/rust/library/std/src/fs.rs:127 |
| `std::fs::OpenOptions` | `[0]` | `sys::fs::OpenOptions` | tools/rust/library/std/src/fs.rs:203 |
| `std::fs::Permissions` | `[0]` | `sys::fs::FilePermissions` | tools/rust/library/std/src/fs.rs:221 |
| `std::fs::ReadDir` | `[0]` | `sys::fs::ReadDir` | tools/rust/library/std/src/fs.rs:145 |
| `std::hash::random::DefaultHasher` | `[0]` | `SipHasher13` | tools/rust/library/std/src/hash/random.rs:97 |
| `std::hash::random::RandomState` | `.k0` | `u64` | tools/rust/library/std/src/hash/random.rs:37 |
| `std::hash::random::RandomState` | `.k1` | `u64` | tools/rust/library/std/src/hash/random.rs:38 |
| `std::io::Bytes` | `.inner` | `$R` | tools/rust/library/std/src/io/mod.rs:3080 |
| `std::io::Chain` | `.first` | `$T` | tools/rust/library/std/src/io/mod.rs:2630 |
| `std::io::Chain` | `.second` | `$U` | tools/rust/library/std/src/io/mod.rs:2631 |
| `std::io::Chain` | `.done_first` | `bool` | tools/rust/library/std/src/io/mod.rs:2632 |
| `std::io::Guard` | `.buf` | `Vec<u8>` | tools/rust/library/std/src/io/mod.rs:352 |
| `std::io::Guard` | `.len` | `usize` | tools/rust/library/std/src/io/mod.rs:353 |
| `std::io::IoSlice` | `[0]` | `sys::io::IoSlice` | tools/rust/library/std/src/io/mod.rs:1437 |
| `std::io::IoSliceMut` | `[0]` | `sys::io::IoSliceMut` | tools/rust/library/std/src/io/mod.rs:1279 |
| `std::io::Lines` | `.buf` | `$B` | tools/rust/library/std/src/io/mod.rs:3237 |
| `std::io::SeekFrom` | `.Start` | `std::io::SeekFrom::Start` | tools/rust/library/std/src/io/mod.rs:2144 |
| `std::io::SeekFrom` | `.End` | `std::io::SeekFrom::End` | tools/rust/library/std/src/io/mod.rs:2152 |
| `std::io::SeekFrom` | `.Current` | `std::io::SeekFrom::Current` | tools/rust/library/std/src/io/mod.rs:2160 |
| `std::io::SeekFrom::Current` | `[0]` | `i64` | tools/rust/library/std/src/io/mod.rs:2160 |
| `std::io::SeekFrom::End` | `[0]` | `i64` | tools/rust/library/std/src/io/mod.rs:2152 |
| `std::io::SeekFrom::Start` | `[0]` | `u64` | tools/rust/library/std/src/io/mod.rs:2144 |
| `std::io::Split` | `.buf` | `$B` | tools/rust/library/std/src/io/mod.rs:3204 |
| `std::io::Split` | `.delim` | `u8` | tools/rust/library/std/src/io/mod.rs:3205 |
| `std::io::Take` | `.inner` | `$T` | tools/rust/library/std/src/io/mod.rs:2828 |
| `std::io::Take` | `.limit` | `u64` | tools/rust/library/std/src/io/mod.rs:2829 |
| `std::io::buffered::IntoInnerError` | `[0]` | `$W` | tools/rust/library/std/src/io/buffered/mod.rs:46 |
| `std::io::buffered::IntoInnerError` | `[1]` | `std::io::error::Error` | tools/rust/library/std/src/io/buffered/mod.rs:46 |
| `std::io::buffered::bufreader::BufReader` | `.buf` | `std::io::buffered::bufreader::buffer::Buffer` | tools/rust/library/std/src/io/buffered/bufreader.rs:52 |
| `std::io::buffered::bufreader::BufReader` | `.inner` | `$R` | tools/rust/library/std/src/io/buffered/bufreader.rs:53 |
| `std::io::buffered::bufreader::buffer::Buffer` | `.buf` | `Box<[mem::MaybeUninit<u8>]>` | tools/rust/library/std/src/io/buffered/bufreader/buffer.rs:18 |
| `std::io::buffered::bufreader::buffer::Buffer` | `.pos` | `usize` | tools/rust/library/std/src/io/buffered/bufreader/buffer.rs:20 |
| `std::io::buffered::bufreader::buffer::Buffer` | `.filled` | `usize` | tools/rust/library/std/src/io/buffered/bufreader/buffer.rs:23 |
| `std::io::buffered::bufreader::buffer::Buffer` | `.initialized` | `usize` | tools/rust/library/std/src/io/buffered/bufreader/buffer.rs:29 |
| `std::io::buffered::bufwriter::BufWriter` | `.buf` | `Vec<u8>` | tools/rust/library/std/src/io/buffered/bufwriter.rs:72 |
| `std::io::buffered::bufwriter::BufWriter` | `.panicked` | `bool` | tools/rust/library/std/src/io/buffered/bufwriter.rs:76 |
| `std::io::buffered::bufwriter::BufWriter` | `.inner` | `$W` | tools/rust/library/std/src/io/buffered/bufwriter.rs:77 |
| `std::io::buffered::bufwriter::WriterPanicked` | `.buf` | `Vec<u8>` | tools/rust/library/std/src/io/buffered/bufwriter.rs:484 |
| `std::io::buffered::linewriter::LineWriter` | `.inner` | `std::io::buffered::bufwriter::BufWriter<$W>` | tools/rust/library/std/src/io/buffered/linewriter.rs:69 |
| `std::io::buffered::linewritershim::LineWriterShim` | `.buffer` | `std::io::buffered::bufwriter::BufWriter<$W>` | tools/rust/library/std/src/io/buffered/linewritershim.rs:17 |
| `std::io::cursor::Cursor` | `.inner` | `$T` | tools/rust/library/std/src/io/cursor.rs:76 |
| `std::io::cursor::Cursor` | `.pos` | `u64` | tools/rust/library/std/src/io/cursor.rs:77 |
| `std::io::error::Custom` | `.kind` | `std::io::error::ErrorKind` | tools/rust/library/std/src/io/error.rs:194 |
| `std::io::error::Custom` | `.error` | `Box<opaque(dyn error :: Error + Send + Sync)>` | tools/rust/library/std/src/io/error.rs:195 |
| `std::io::error::Error` | `.repr` | `std::io::error::repr_unpacked::Repr` | tools/rust/library/std/src/io/error.rs:65 |
| `std::io::error::ErrorData` | `.Os` | `std::io::error::ErrorData::Os<$C>` | tools/rust/library/std/src/io/error.rs:123 |
| `std::io::error::ErrorData` | `.Simple` | `std::io::error::ErrorData::Simple<$C>` | tools/rust/library/std/src/io/error.rs:124 |
| `std::io::error::ErrorData` | `.SimpleMessage` | `std::io::error::ErrorData::SimpleMessage<$C>` | tools/rust/library/std/src/io/error.rs:125 |
| `std::io::error::ErrorData` | `.Custom` | `std::io::error::ErrorData::Custom<$C>` | tools/rust/library/std/src/io/error.rs:126 |
| `std::io::error::ErrorData::Custom` | `[0]` | `$C` | tools/rust/library/std/src/io/error.rs:126 |
| `std::io::error::ErrorData::Os` | `[0]` | `std::io::error::RawOsError` | tools/rust/library/std/src/io/error.rs:123 |
| `std::io::error::ErrorData::Simple` | `[0]` | `std::io::error::ErrorKind` | tools/rust/library/std/src/io/error.rs:124 |
| `std::io::error::ErrorData::SimpleMessage` | `[0]` | `std::io::error::SimpleMessage` | tools/rust/library/std/src/io/error.rs:125 |
| `std::io::error::RawOsError` | `(alias)` | `sys::RawOsError` | tools/rust/library/std/src/io/error.rs:138 |
| `std::io::error::Result` | `(alias)` | `result::Result<$T, std::io::error::Error>` | tools/rust/library/std/src/io/error.rs:51 |
| `std::io::error::SimpleMessage` | `.kind` | `std::io::error::ErrorKind` | tools/rust/library/std/src/io/error.rs:159 |
| `std::io::error::SimpleMessage` | `.message` | `str` | tools/rust/library/std/src/io/error.rs:160 |
| `std::io::error::repr_bitpacked::Repr` | `[0]` | `core::ptr::non_null::NonNull<()>` | tools/rust/library/std/src/io/error/repr_bitpacked.rs:129 |
| `std::io::error::repr_bitpacked::Repr` | `[1]` | `core::marker::PhantomData<std::io::error::ErrorData<Box<std::io::error::Custom>>>` | tools/rust/library/std/src/io/error/repr_bitpacked.rs:129 |
| `std::io::error::repr_unpacked::Inner` | `(alias)` | `std::io::error::ErrorData<Box<std::io::error::Custom>>` | tools/rust/library/std/src/io/error/repr_unpacked.rs:7 |
| `std::io::error::repr_unpacked::Repr` | `[0]` | `std::io::error::repr_unpacked::Inner` | tools/rust/library/std/src/io/error/repr_unpacked.rs:9 |
| `std::io::pipe::PipeReader` | `[0]` | `sys::anonymous_pipe::AnonPipe` | tools/rust/library/std/src/io/pipe.rs:79 |
| `std::io::pipe::PipeWriter` | `[0]` | `sys::anonymous_pipe::AnonPipe` | tools/rust/library/std/src/io/pipe.rs:84 |
| `std::io::stdio::LocalStream` | `(alias)` | `sync::Arc<std::sync::poison::mutex::Mutex<Vec<u8>>>` | tools/rust/library/std/src/io/stdio.rs:19 |
| `std::io::stdio::Stderr` | `.inner` | `std::sync::reentrant_lock::ReentrantLock<cell::RefCell<std::io::stdio::StderrRaw>>` | tools/rust/library/std/src/io/stdio.rs:893 |
| `std::io::stdio::StderrLock` | `.inner` | `std::sync::reentrant_lock::ReentrantLockGuard<cell::RefCell<std::io::stdio::StderrRaw>>` | tools/rust/library/std/src/io/stdio.rs:915 |
| `std::io::stdio::StderrRaw` | `[0]` | `sys::stdio::Stderr` | tools/rust/library/std/src/io/stdio.rs:58 |
| `std::io::stdio::Stdin` | `.inner` | `std::sync::poison::mutex::Mutex<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>` | tools/rust/library/std/src/io/stdio.rs:251 |
| `std::io::stdio::StdinLock` | `.inner` | `std::sync::poison::mutex::MutexGuard<std::io::buffered::bufreader::BufReader<std::io::stdio::StdinRaw>>` | tools/rust/library/std/src/io/stdio.rs:289 |
| `std::io::stdio::StdinRaw` | `[0]` | `sys::stdio::Stdin` | tools/rust/library/std/src/io/stdio.rs:46 |
| `std::io::stdio::Stdout` | `.inner` | `std::sync::reentrant_lock::ReentrantLock<cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>` | tools/rust/library/std/src/io/stdio.rs:612 |
| `std::io::stdio::StdoutLock` | `.inner` | `std::sync::reentrant_lock::ReentrantLockGuard<cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>` | tools/rust/library/std/src/io/stdio.rs:641 |
| `std::io::stdio::StdoutRaw` | `[0]` | `sys::stdio::Stdout` | tools/rust/library/std/src/io/stdio.rs:52 |
| `std::io::util::Repeat` | `.byte` | `u8` | tools/rust/library/std/src/io/util.rs:243 |
| `std::net::tcp::Incoming` | `.listener` | `std::net::tcp::TcpListener` | tools/rust/library/std/src/net/tcp.rs:105 |
| `std::net::tcp::IntoIncoming` | `.listener` | `std::net::tcp::TcpListener` | tools/rust/library/std/src/net/tcp.rs:117 |
| `std::net::tcp::TcpListener` | `[0]` | `sys::net::TcpListener` | tools/rust/library/std/src/net/tcp.rs:93 |
| `std::net::tcp::TcpStream` | `[0]` | `sys::net::TcpStream` | tools/rust/library/std/src/net/tcp.rs:57 |
| `std::net::udp::UdpSocket` | `[0]` | `sys::net::UdpSocket` | tools/rust/library/std/src/net/udp.rs:67 |
| `std::os::android::raw::pthread_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/android/raw.rs:16 |
| `std::os::darwin::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/darwin/raw.rs:5 |
| `std::os::darwin::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/darwin/raw.rs:7 |
| `std::os::darwin::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/darwin/raw.rs:9 |
| `std::os::darwin::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/darwin/raw.rs:11 |
| `std::os::darwin::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/darwin/raw.rs:13 |
| `std::os::darwin::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/darwin/raw.rs:15 |
| `std::os::darwin::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/darwin/raw.rs:17 |
| `std::os::darwin::raw::pthread_t` | `(alias)` | `usize` | tools/rust/library/std/src/os/darwin/raw.rs:22 |
| `std::os::darwin::raw::stat` | `.st_dev` | `i32` | tools/rust/library/std/src/os/darwin/raw.rs:30 |
| `std::os::darwin::raw::stat` | `.st_mode` | `u16` | tools/rust/library/std/src/os/darwin/raw.rs:32 |
| `std::os::darwin::raw::stat` | `.st_nlink` | `u16` | tools/rust/library/std/src/os/darwin/raw.rs:34 |
| `std::os::darwin::raw::stat` | `.st_ino` | `u64` | tools/rust/library/std/src/os/darwin/raw.rs:36 |
| `std::os::darwin::raw::stat` | `.st_uid` | `u32` | tools/rust/library/std/src/os/darwin/raw.rs:38 |
| `std::os::darwin::raw::stat` | `.st_gid` | `u32` | tools/rust/library/std/src/os/darwin/raw.rs:40 |
| `std::os::darwin::raw::stat` | `.st_rdev` | `i32` | tools/rust/library/std/src/os/darwin/raw.rs:42 |
| `std::os::darwin::raw::stat` | `.st_atime` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:44 |
| `std::os::darwin::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:46 |
| `std::os::darwin::raw::stat` | `.st_mtime` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:48 |
| `std::os::darwin::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:50 |
| `std::os::darwin::raw::stat` | `.st_ctime` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:52 |
| `std::os::darwin::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:54 |
| `std::os::darwin::raw::stat` | `.st_birthtime` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:56 |
| `std::os::darwin::raw::stat` | `.st_birthtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/darwin/raw.rs:58 |
| `std::os::darwin::raw::stat` | `.st_size` | `i64` | tools/rust/library/std/src/os/darwin/raw.rs:60 |
| `std::os::darwin::raw::stat` | `.st_blocks` | `i64` | tools/rust/library/std/src/os/darwin/raw.rs:62 |
| `std::os::darwin::raw::stat` | `.st_blksize` | `i32` | tools/rust/library/std/src/os/darwin/raw.rs:64 |
| `std::os::darwin::raw::stat` | `.st_flags` | `u32` | tools/rust/library/std/src/os/darwin/raw.rs:66 |
| `std::os::darwin::raw::stat` | `.st_gen` | `u32` | tools/rust/library/std/src/os/darwin/raw.rs:68 |
| `std::os::darwin::raw::stat` | `.st_lspare` | `i32` | tools/rust/library/std/src/os/darwin/raw.rs:70 |
| `std::os::darwin::raw::stat` | `.st_qspare` | `[i64; 2]` | tools/rust/library/std/src/os/darwin/raw.rs:72 |
| `std::os::darwin::raw::time_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/darwin/raw.rs:19 |
| `std::os::dragonfly::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/dragonfly/raw.rs:16 |
| `std::os::dragonfly::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/dragonfly/raw.rs:18 |
| `std::os::dragonfly::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/dragonfly/raw.rs:20 |
| `std::os::dragonfly::raw::fflags_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:22 |
| `std::os::dragonfly::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/dragonfly/raw.rs:24 |
| `std::os::dragonfly::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:26 |
| `std::os::dragonfly::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/dragonfly/raw.rs:28 |
| `std::os::dragonfly::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/dragonfly/raw.rs:30 |
| `std::os::dragonfly::raw::pthread_t` | `(alias)` | `usize` | tools/rust/library/std/src/os/dragonfly/raw.rs:35 |
| `std::os::dragonfly::raw::stat` | `.st_dev` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:42 |
| `std::os::dragonfly::raw::stat` | `.st_ino` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:44 |
| `std::os::dragonfly::raw::stat` | `.st_mode` | `u16` | tools/rust/library/std/src/os/dragonfly/raw.rs:46 |
| `std::os::dragonfly::raw::stat` | `.st_nlink` | `u16` | tools/rust/library/std/src/os/dragonfly/raw.rs:48 |
| `std::os::dragonfly::raw::stat` | `.st_uid` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:50 |
| `std::os::dragonfly::raw::stat` | `.st_gid` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:52 |
| `std::os::dragonfly::raw::stat` | `.st_rdev` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:54 |
| `std::os::dragonfly::raw::stat` | `.st_atime` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:56 |
| `std::os::dragonfly::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:58 |
| `std::os::dragonfly::raw::stat` | `.st_mtime` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:60 |
| `std::os::dragonfly::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:62 |
| `std::os::dragonfly::raw::stat` | `.st_ctime` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:64 |
| `std::os::dragonfly::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:66 |
| `std::os::dragonfly::raw::stat` | `.st_size` | `i64` | tools/rust/library/std/src/os/dragonfly/raw.rs:68 |
| `std::os::dragonfly::raw::stat` | `.st_blocks` | `i64` | tools/rust/library/std/src/os/dragonfly/raw.rs:70 |
| `std::os::dragonfly::raw::stat` | `.st_blksize` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:72 |
| `std::os::dragonfly::raw::stat` | `.st_flags` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:74 |
| `std::os::dragonfly::raw::stat` | `.st_gen` | `u32` | tools/rust/library/std/src/os/dragonfly/raw.rs:76 |
| `std::os::dragonfly::raw::stat` | `.st_lspare` | `i32` | tools/rust/library/std/src/os/dragonfly/raw.rs:78 |
| `std::os::dragonfly::raw::stat` | `.st_birthtime` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:80 |
| `std::os::dragonfly::raw::stat` | `.st_birthtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/dragonfly/raw.rs:82 |
| `std::os::dragonfly::raw::time_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/dragonfly/raw.rs:32 |
| `std::os::emscripten::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:24 |
| `std::os::emscripten::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:26 |
| `std::os::emscripten::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:16 |
| `std::os::emscripten::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:28 |
| `std::os::emscripten::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/emscripten/raw.rs:18 |
| `std::os::emscripten::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:30 |
| `std::os::emscripten::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:32 |
| `std::os::emscripten::raw::pthread_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/emscripten/raw.rs:21 |
| `std::os::emscripten::raw::stat` | `.st_dev` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:41 |
| `std::os::emscripten::raw::stat` | `.__pad1` | `os::raw::c_short` | tools/rust/library/std/src/os/emscripten/raw.rs:43 |
| `std::os::emscripten::raw::stat` | `.__st_ino` | `u32` | tools/rust/library/std/src/os/emscripten/raw.rs:45 |
| `std::os::emscripten::raw::stat` | `.st_mode` | `u32` | tools/rust/library/std/src/os/emscripten/raw.rs:47 |
| `std::os::emscripten::raw::stat` | `.st_nlink` | `u32` | tools/rust/library/std/src/os/emscripten/raw.rs:49 |
| `std::os::emscripten::raw::stat` | `.st_uid` | `u32` | tools/rust/library/std/src/os/emscripten/raw.rs:51 |
| `std::os::emscripten::raw::stat` | `.st_gid` | `u32` | tools/rust/library/std/src/os/emscripten/raw.rs:53 |
| `std::os::emscripten::raw::stat` | `.st_rdev` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:55 |
| `std::os::emscripten::raw::stat` | `.__pad2` | `os::raw::c_uint` | tools/rust/library/std/src/os/emscripten/raw.rs:57 |
| `std::os::emscripten::raw::stat` | `.st_size` | `i64` | tools/rust/library/std/src/os/emscripten/raw.rs:59 |
| `std::os::emscripten::raw::stat` | `.st_blksize` | `i32` | tools/rust/library/std/src/os/emscripten/raw.rs:61 |
| `std::os::emscripten::raw::stat` | `.st_blocks` | `i64` | tools/rust/library/std/src/os/emscripten/raw.rs:63 |
| `std::os::emscripten::raw::stat` | `.st_atime` | `std::os::emscripten::raw::time_t` | tools/rust/library/std/src/os/emscripten/raw.rs:65 |
| `std::os::emscripten::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/emscripten/raw.rs:67 |
| `std::os::emscripten::raw::stat` | `.st_mtime` | `std::os::emscripten::raw::time_t` | tools/rust/library/std/src/os/emscripten/raw.rs:69 |
| `std::os::emscripten::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/emscripten/raw.rs:71 |
| `std::os::emscripten::raw::stat` | `.st_ctime` | `std::os::emscripten::raw::time_t` | tools/rust/library/std/src/os/emscripten/raw.rs:73 |
| `std::os::emscripten::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/emscripten/raw.rs:75 |
| `std::os::emscripten::raw::stat` | `.st_ino` | `u64` | tools/rust/library/std/src/os/emscripten/raw.rs:77 |
| `std::os::emscripten::raw::time_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/emscripten/raw.rs:34 |
| `std::os::espidf::raw::blkcnt_t` | `(alias)` | `libc::blkcnt_t` | tools/rust/library/std/src/os/espidf/raw.rs:19 |
| `std::os::espidf::raw::blksize_t` | `(alias)` | `libc::blksize_t` | tools/rust/library/std/src/os/espidf/raw.rs:22 |
| `std::os::espidf::raw::dev_t` | `(alias)` | `libc::dev_t` | tools/rust/library/std/src/os/espidf/raw.rs:24 |
| `std::os::espidf::raw::ino_t` | `(alias)` | `libc::ino_t` | tools/rust/library/std/src/os/espidf/raw.rs:26 |
| `std::os::espidf::raw::mode_t` | `(alias)` | `libc::mode_t` | tools/rust/library/std/src/os/espidf/raw.rs:28 |
| `std::os::espidf::raw::nlink_t` | `(alias)` | `libc::nlink_t` | tools/rust/library/std/src/os/espidf/raw.rs:30 |
| `std::os::espidf::raw::off_t` | `(alias)` | `libc::off_t` | tools/rust/library/std/src/os/espidf/raw.rs:32 |
| `std::os::espidf::raw::pthread_t` | `(alias)` | `libc::pthread_t` | tools/rust/library/std/src/os/espidf/raw.rs:16 |
| `std::os::espidf::raw::stat` | `.st_dev` | `std::os::espidf::raw::dev_t` | tools/rust/library/std/src/os/espidf/raw.rs:42 |
| `std::os::espidf::raw::stat` | `.st_ino` | `std::os::espidf::raw::ino_t` | tools/rust/library/std/src/os/espidf/raw.rs:44 |
| `std::os::espidf::raw::stat` | `.st_mode` | `std::os::espidf::raw::mode_t` | tools/rust/library/std/src/os/espidf/raw.rs:46 |
| `std::os::espidf::raw::stat` | `.st_nlink` | `std::os::espidf::raw::nlink_t` | tools/rust/library/std/src/os/espidf/raw.rs:48 |
| `std::os::espidf::raw::stat` | `.st_uid` | `std::os::unix::raw::uid_t` | tools/rust/library/std/src/os/espidf/raw.rs:50 |
| `std::os::espidf::raw::stat` | `.st_gid` | `std::os::unix::raw::gid_t` | tools/rust/library/std/src/os/espidf/raw.rs:52 |
| `std::os::espidf::raw::stat` | `.st_rdev` | `std::os::espidf::raw::dev_t` | tools/rust/library/std/src/os/espidf/raw.rs:54 |
| `std::os::espidf::raw::stat` | `.st_size` | `std::os::espidf::raw::off_t` | tools/rust/library/std/src/os/espidf/raw.rs:56 |
| `std::os::espidf::raw::stat` | `.st_atime` | `std::os::espidf::raw::time_t` | tools/rust/library/std/src/os/espidf/raw.rs:58 |
| `std::os::espidf::raw::stat` | `.st_mtime` | `std::os::espidf::raw::time_t` | tools/rust/library/std/src/os/espidf/raw.rs:60 |
| `std::os::espidf::raw::stat` | `.st_ctime` | `std::os::espidf::raw::time_t` | tools/rust/library/std/src/os/espidf/raw.rs:62 |
| `std::os::espidf::raw::stat` | `.st_blksize` | `std::os::espidf::raw::blksize_t` | tools/rust/library/std/src/os/espidf/raw.rs:64 |
| `std::os::espidf::raw::stat` | `.st_blocks` | `std::os::espidf::raw::blkcnt_t` | tools/rust/library/std/src/os/espidf/raw.rs:66 |
| `std::os::espidf::raw::stat` | `.st_spare4` | `[os::raw::c_long; 2usize]` | tools/rust/library/std/src/os/espidf/raw.rs:68 |
| `std::os::espidf::raw::time_t` | `(alias)` | `libc::time_t` | tools/rust/library/std/src/os/espidf/raw.rs:35 |
| `std::os::fd::owned::BorrowedFd` | `.fd` | `std::os::fd::owned::ValidRawFd` | tools/rust/library/std/src/os/fd/owned.rs:48 |
| `std::os::fd::owned::BorrowedFd` | `._phantom` | `marker::PhantomData<std::os::fd::owned::OwnedFd>` | tools/rust/library/std/src/os/fd/owned.rs:49 |
| `std::os::fd::owned::OwnedFd` | `.fd` | `std::os::fd::owned::ValidRawFd` | tools/rust/library/std/src/os/fd/owned.rs:67 |
| `std::os::fd::owned::ValidRawFd` | `(alias)` | `core::num::niche_types::NotAllOnes<std::os::fd::raw::RawFd>` | tools/rust/library/std/src/os/fd/owned.rs:22 |
| `std::os::fortanix_sgx::arch::Align128` | `[0]` | `$T` | tools/rust/library/std/src/os/fortanix_sgx/arch.rs:19 |
| `std::os::fortanix_sgx::arch::Align16` | `[0]` | `$T` | tools/rust/library/std/src/os/fortanix_sgx/arch.rs:14 |
| `std::os::fortanix_sgx::arch::Align512` | `[0]` | `$T` | tools/rust/library/std/src/os/fortanix_sgx/arch.rs:24 |
| `std::os::fortanix_sgx::io::TcpListenerMetadata` | `.local_addr` | `Option<String>` | tools/rust/library/std/src/os/fortanix_sgx/io.rs:112 |
| `std::os::fortanix_sgx::io::TcpStreamMetadata` | `.local_addr` | `Option<String>` | tools/rust/library/std/src/os/fortanix_sgx/io.rs:91 |
| `std::os::fortanix_sgx::io::TcpStreamMetadata` | `.peer_addr` | `Option<String>` | tools/rust/library/std/src/os/fortanix_sgx/io.rs:93 |
| `std::os::freebsd::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/freebsd/raw.rs:16 |
| `std::os::freebsd::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/freebsd/raw.rs:18 |
| `std::os::freebsd::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/freebsd/raw.rs:20 |
| `std::os::freebsd::raw::fflags_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:22 |
| `std::os::freebsd::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/freebsd/raw.rs:24 |
| `std::os::freebsd::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:26 |
| `std::os::freebsd::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/freebsd/raw.rs:28 |
| `std::os::freebsd::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/freebsd/raw.rs:30 |
| `std::os::freebsd::raw::pthread_t` | `(alias)` | `usize` | tools/rust/library/std/src/os/freebsd/raw.rs:35 |
| `std::os::freebsd::raw::stat` | `.st_dev` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:42 |
| `std::os::freebsd::raw::stat` | `.st_ino` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:44 |
| `std::os::freebsd::raw::stat` | `.st_mode` | `u16` | tools/rust/library/std/src/os/freebsd/raw.rs:46 |
| `std::os::freebsd::raw::stat` | `.st_nlink` | `u16` | tools/rust/library/std/src/os/freebsd/raw.rs:48 |
| `std::os::freebsd::raw::stat` | `.st_uid` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:50 |
| `std::os::freebsd::raw::stat` | `.st_gid` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:52 |
| `std::os::freebsd::raw::stat` | `.st_rdev` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:54 |
| `std::os::freebsd::raw::stat` | `.st_atime` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:56 |
| `std::os::freebsd::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:58 |
| `std::os::freebsd::raw::stat` | `.st_mtime` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:60 |
| `std::os::freebsd::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:62 |
| `std::os::freebsd::raw::stat` | `.st_ctime` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:64 |
| `std::os::freebsd::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:66 |
| `std::os::freebsd::raw::stat` | `.st_size` | `i64` | tools/rust/library/std/src/os/freebsd/raw.rs:68 |
| `std::os::freebsd::raw::stat` | `.st_blocks` | `i64` | tools/rust/library/std/src/os/freebsd/raw.rs:70 |
| `std::os::freebsd::raw::stat` | `.st_blksize` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:72 |
| `std::os::freebsd::raw::stat` | `.st_flags` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:74 |
| `std::os::freebsd::raw::stat` | `.st_gen` | `u32` | tools/rust/library/std/src/os/freebsd/raw.rs:76 |
| `std::os::freebsd::raw::stat` | `.st_lspare` | `i32` | tools/rust/library/std/src/os/freebsd/raw.rs:78 |
| `std::os::freebsd::raw::stat` | `.st_birthtime` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:80 |
| `std::os::freebsd::raw::stat` | `.st_birthtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/freebsd/raw.rs:82 |
| `std::os::freebsd::raw::stat` | `.__unused` | `[u8; 8]` | tools/rust/library/std/src/os/freebsd/raw.rs:85 |
| `std::os::freebsd::raw::time_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/freebsd/raw.rs:32 |
| `std::os::fuchsia::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/fuchsia/raw.rs:16 |
| `std::os::fuchsia::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/fuchsia/raw.rs:18 |
| `std::os::fuchsia::raw::pthread_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/fuchsia/raw.rs:21 |
| `std::os::haiku::raw::blkcnt_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/haiku/raw.rs:21 |
| `std::os::haiku::raw::blksize_t` | `(alias)` | `i32` | tools/rust/library/std/src/os/haiku/raw.rs:23 |
| `std::os::haiku::raw::dev_t` | `(alias)` | `i32` | tools/rust/library/std/src/os/haiku/raw.rs:25 |
| `std::os::haiku::raw::ino_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/haiku/raw.rs:27 |
| `std::os::haiku::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/haiku/raw.rs:29 |
| `std::os::haiku::raw::nlink_t` | `(alias)` | `i32` | tools/rust/library/std/src/os/haiku/raw.rs:31 |
| `std::os::haiku::raw::off_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/haiku/raw.rs:33 |
| `std::os::haiku::raw::pthread_t` | `(alias)` | `usize` | tools/rust/library/std/src/os/haiku/raw.rs:18 |
| `std::os::haiku::raw::stat` | `.st_dev` | `std::os::haiku::raw::dev_t` | tools/rust/library/std/src/os/haiku/raw.rs:42 |
| `std::os::haiku::raw::stat` | `.st_ino` | `std::os::haiku::raw::ino_t` | tools/rust/library/std/src/os/haiku/raw.rs:44 |
| `std::os::haiku::raw::stat` | `.st_mode` | `std::os::haiku::raw::mode_t` | tools/rust/library/std/src/os/haiku/raw.rs:46 |
| `std::os::haiku::raw::stat` | `.st_nlink` | `std::os::haiku::raw::nlink_t` | tools/rust/library/std/src/os/haiku/raw.rs:48 |
| `std::os::haiku::raw::stat` | `.st_uid` | `std::os::unix::raw::uid_t` | tools/rust/library/std/src/os/haiku/raw.rs:50 |
| `std::os::haiku::raw::stat` | `.st_gid` | `std::os::unix::raw::gid_t` | tools/rust/library/std/src/os/haiku/raw.rs:52 |
| `std::os::haiku::raw::stat` | `.st_size` | `std::os::haiku::raw::off_t` | tools/rust/library/std/src/os/haiku/raw.rs:54 |
| `std::os::haiku::raw::stat` | `.st_rdev` | `std::os::haiku::raw::dev_t` | tools/rust/library/std/src/os/haiku/raw.rs:56 |
| `std::os::haiku::raw::stat` | `.st_blksize` | `std::os::haiku::raw::blksize_t` | tools/rust/library/std/src/os/haiku/raw.rs:58 |
| `std::os::haiku::raw::stat` | `.st_atime` | `std::os::haiku::raw::time_t` | tools/rust/library/std/src/os/haiku/raw.rs:60 |
| `std::os::haiku::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/haiku/raw.rs:62 |
| `std::os::haiku::raw::stat` | `.st_mtime` | `std::os::haiku::raw::time_t` | tools/rust/library/std/src/os/haiku/raw.rs:64 |
| `std::os::haiku::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/haiku/raw.rs:66 |
| `std::os::haiku::raw::stat` | `.st_ctime` | `std::os::haiku::raw::time_t` | tools/rust/library/std/src/os/haiku/raw.rs:68 |
| `std::os::haiku::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/haiku/raw.rs:70 |
| `std::os::haiku::raw::stat` | `.st_crtime` | `std::os::haiku::raw::time_t` | tools/rust/library/std/src/os/haiku/raw.rs:72 |
| `std::os::haiku::raw::stat` | `.st_crtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/haiku/raw.rs:74 |
| `std::os::haiku::raw::stat` | `.st_type` | `u32` | tools/rust/library/std/src/os/haiku/raw.rs:76 |
| `std::os::haiku::raw::stat` | `.st_blocks` | `std::os::haiku::raw::blkcnt_t` | tools/rust/library/std/src/os/haiku/raw.rs:78 |
| `std::os::haiku::raw::time_t` | `(alias)` | `i32` | tools/rust/library/std/src/os/haiku/raw.rs:35 |
| `std::os::horizon::raw::blkcnt_t` | `(alias)` | `libc::blkcnt_t` | tools/rust/library/std/src/os/horizon/raw.rs:20 |
| `std::os::horizon::raw::blksize_t` | `(alias)` | `libc::blksize_t` | tools/rust/library/std/src/os/horizon/raw.rs:23 |
| `std::os::horizon::raw::dev_t` | `(alias)` | `libc::dev_t` | tools/rust/library/std/src/os/horizon/raw.rs:25 |
| `std::os::horizon::raw::ino_t` | `(alias)` | `libc::ino_t` | tools/rust/library/std/src/os/horizon/raw.rs:27 |
| `std::os::horizon::raw::mode_t` | `(alias)` | `libc::mode_t` | tools/rust/library/std/src/os/horizon/raw.rs:29 |
| `std::os::horizon::raw::nlink_t` | `(alias)` | `libc::nlink_t` | tools/rust/library/std/src/os/horizon/raw.rs:31 |
| `std::os::horizon::raw::off_t` | `(alias)` | `libc::off_t` | tools/rust/library/std/src/os/horizon/raw.rs:33 |
| `std::os::horizon::raw::pthread_t` | `(alias)` | `libc::pthread_t` | tools/rust/library/std/src/os/horizon/raw.rs:17 |
| `std::os::horizon::raw::stat` | `.st_dev` | `std::os::horizon::raw::dev_t` | tools/rust/library/std/src/os/horizon/raw.rs:44 |
| `std::os::horizon::raw::stat` | `.st_ino` | `std::os::horizon::raw::ino_t` | tools/rust/library/std/src/os/horizon/raw.rs:46 |
| `std::os::horizon::raw::stat` | `.st_mode` | `std::os::horizon::raw::mode_t` | tools/rust/library/std/src/os/horizon/raw.rs:48 |
| `std::os::horizon::raw::stat` | `.st_nlink` | `std::os::horizon::raw::nlink_t` | tools/rust/library/std/src/os/horizon/raw.rs:50 |
| `std::os::horizon::raw::stat` | `.st_uid` | `std::os::unix::raw::uid_t` | tools/rust/library/std/src/os/horizon/raw.rs:52 |
| `std::os::horizon::raw::stat` | `.st_gid` | `std::os::unix::raw::gid_t` | tools/rust/library/std/src/os/horizon/raw.rs:54 |
| `std::os::horizon::raw::stat` | `.st_rdev` | `std::os::horizon::raw::dev_t` | tools/rust/library/std/src/os/horizon/raw.rs:56 |
| `std::os::horizon::raw::stat` | `.st_size` | `std::os::horizon::raw::off_t` | tools/rust/library/std/src/os/horizon/raw.rs:58 |
| `std::os::horizon::raw::stat` | `.st_atime` | `std::os::horizon::raw::time_t` | tools/rust/library/std/src/os/horizon/raw.rs:60 |
| `std::os::horizon::raw::stat` | `.st_mtime` | `std::os::horizon::raw::time_t` | tools/rust/library/std/src/os/horizon/raw.rs:62 |
| `std::os::horizon::raw::stat` | `.st_ctime` | `std::os::horizon::raw::time_t` | tools/rust/library/std/src/os/horizon/raw.rs:64 |
| `std::os::horizon::raw::stat` | `.st_blksize` | `std::os::horizon::raw::blksize_t` | tools/rust/library/std/src/os/horizon/raw.rs:66 |
| `std::os::horizon::raw::stat` | `.st_blocks` | `std::os::horizon::raw::blkcnt_t` | tools/rust/library/std/src/os/horizon/raw.rs:68 |
| `std::os::horizon::raw::stat` | `.st_spare4` | `[os::raw::c_long; 2usize]` | tools/rust/library/std/src/os/horizon/raw.rs:70 |
| `std::os::horizon::raw::time_t` | `(alias)` | `libc::time_t` | tools/rust/library/std/src/os/horizon/raw.rs:36 |
| `std::os::hurd::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/hurd/raw.rs:16 |
| `std::os::hurd::raw::blksize_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/hurd/raw.rs:18 |
| `std::os::hurd::raw::dev_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/hurd/raw.rs:20 |
| `std::os::hurd::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/hurd/raw.rs:22 |
| `std::os::hurd::raw::mode_t` | `(alias)` | `os::raw::c_uint` | tools/rust/library/std/src/os/hurd/raw.rs:24 |
| `std::os::hurd::raw::nlink_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/hurd/raw.rs:26 |
| `std::os::hurd::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/hurd/raw.rs:28 |
| `std::os::hurd::raw::pthread_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/hurd/raw.rs:33 |
| `std::os::hurd::raw::time_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/hurd/raw.rs:30 |
| `std::os::illumos::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/illumos/raw.rs:15 |
| `std::os::illumos::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/illumos/raw.rs:17 |
| `std::os::illumos::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/illumos/raw.rs:19 |
| `std::os::illumos::raw::fflags_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/illumos/raw.rs:21 |
| `std::os::illumos::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/illumos/raw.rs:23 |
| `std::os::illumos::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/illumos/raw.rs:25 |
| `std::os::illumos::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/illumos/raw.rs:27 |
| `std::os::illumos::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/illumos/raw.rs:29 |
| `std::os::illumos::raw::pthread_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/illumos/raw.rs:34 |
| `std::os::illumos::raw::stat` | `.st_dev` | `std::os::illumos::raw::dev_t` | tools/rust/library/std/src/os/illumos/raw.rs:41 |
| `std::os::illumos::raw::stat` | `.st_ino` | `std::os::illumos::raw::ino_t` | tools/rust/library/std/src/os/illumos/raw.rs:43 |
| `std::os::illumos::raw::stat` | `.st_mode` | `std::os::illumos::raw::mode_t` | tools/rust/library/std/src/os/illumos/raw.rs:45 |
| `std::os::illumos::raw::stat` | `.st_nlink` | `std::os::illumos::raw::nlink_t` | tools/rust/library/std/src/os/illumos/raw.rs:47 |
| `std::os::illumos::raw::stat` | `.st_uid` | `std::os::unix::raw::uid_t` | tools/rust/library/std/src/os/illumos/raw.rs:49 |
| `std::os::illumos::raw::stat` | `.st_gid` | `std::os::unix::raw::gid_t` | tools/rust/library/std/src/os/illumos/raw.rs:51 |
| `std::os::illumos::raw::stat` | `.st_rdev` | `std::os::illumos::raw::dev_t` | tools/rust/library/std/src/os/illumos/raw.rs:53 |
| `std::os::illumos::raw::stat` | `.st_size` | `std::os::illumos::raw::off_t` | tools/rust/library/std/src/os/illumos/raw.rs:55 |
| `std::os::illumos::raw::stat` | `.st_atime` | `std::os::illumos::raw::time_t` | tools/rust/library/std/src/os/illumos/raw.rs:57 |
| `std::os::illumos::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/illumos/raw.rs:59 |
| `std::os::illumos::raw::stat` | `.st_mtime` | `std::os::illumos::raw::time_t` | tools/rust/library/std/src/os/illumos/raw.rs:61 |
| `std::os::illumos::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/illumos/raw.rs:63 |
| `std::os::illumos::raw::stat` | `.st_ctime` | `std::os::illumos::raw::time_t` | tools/rust/library/std/src/os/illumos/raw.rs:65 |
| `std::os::illumos::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/illumos/raw.rs:67 |
| `std::os::illumos::raw::stat` | `.st_blksize` | `std::os::illumos::raw::blksize_t` | tools/rust/library/std/src/os/illumos/raw.rs:69 |
| `std::os::illumos::raw::stat` | `.st_blocks` | `std::os::illumos::raw::blkcnt_t` | tools/rust/library/std/src/os/illumos/raw.rs:71 |
| `std::os::illumos::raw::stat` | `.__unused` | `[u8; 16]` | tools/rust/library/std/src/os/illumos/raw.rs:73 |
| `std::os::illumos::raw::time_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/illumos/raw.rs:31 |
| `std::os::l4re::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/l4re/raw.rs:16 |
| `std::os::l4re::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/l4re/raw.rs:18 |
| `std::os::l4re::raw::pthread_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/l4re/raw.rs:21 |
| `std::os::linux::process::PidFd` | `.inner` | `std::os::linux::process::InnerPidFd` | tools/rust/library/std/src/os/linux/process.rs:52 |
| `std::os::linux::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/linux/raw.rs:16 |
| `std::os::linux::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/linux/raw.rs:18 |
| `std::os::linux::raw::pthread_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/linux/raw.rs:21 |
| `std::os::netbsd::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:17 |
| `std::os::netbsd::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:19 |
| `std::os::netbsd::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:21 |
| `std::os::netbsd::raw::fflags_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/netbsd/raw.rs:23 |
| `std::os::netbsd::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:25 |
| `std::os::netbsd::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/netbsd/raw.rs:27 |
| `std::os::netbsd::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:29 |
| `std::os::netbsd::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:31 |
| `std::os::netbsd::raw::pthread_t` | `(alias)` | `usize` | tools/rust/library/std/src/os/netbsd/raw.rs:36 |
| `std::os::netbsd::raw::stat` | `.st_dev` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:43 |
| `std::os::netbsd::raw::stat` | `.st_mode` | `u32` | tools/rust/library/std/src/os/netbsd/raw.rs:45 |
| `std::os::netbsd::raw::stat` | `.st_ino` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:47 |
| `std::os::netbsd::raw::stat` | `.st_nlink` | `u32` | tools/rust/library/std/src/os/netbsd/raw.rs:49 |
| `std::os::netbsd::raw::stat` | `.st_uid` | `std::os::unix::raw::uid_t` | tools/rust/library/std/src/os/netbsd/raw.rs:51 |
| `std::os::netbsd::raw::stat` | `.st_gid` | `std::os::unix::raw::gid_t` | tools/rust/library/std/src/os/netbsd/raw.rs:53 |
| `std::os::netbsd::raw::stat` | `.st_rdev` | `u64` | tools/rust/library/std/src/os/netbsd/raw.rs:55 |
| `std::os::netbsd::raw::stat` | `.st_atime` | `i64` | tools/rust/library/std/src/os/netbsd/raw.rs:57 |
| `std::os::netbsd::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/netbsd/raw.rs:59 |
| `std::os::netbsd::raw::stat` | `.st_mtime` | `i64` | tools/rust/library/std/src/os/netbsd/raw.rs:61 |
| `std::os::netbsd::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/netbsd/raw.rs:63 |
| `std::os::netbsd::raw::stat` | `.st_ctime` | `i64` | tools/rust/library/std/src/os/netbsd/raw.rs:65 |
| `std::os::netbsd::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/netbsd/raw.rs:67 |
| `std::os::netbsd::raw::stat` | `.st_birthtime` | `i64` | tools/rust/library/std/src/os/netbsd/raw.rs:69 |
| `std::os::netbsd::raw::stat` | `.st_birthtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/netbsd/raw.rs:71 |
| `std::os::netbsd::raw::stat` | `.st_size` | `i64` | tools/rust/library/std/src/os/netbsd/raw.rs:73 |
| `std::os::netbsd::raw::stat` | `.st_blocks` | `i64` | tools/rust/library/std/src/os/netbsd/raw.rs:75 |
| `std::os::netbsd::raw::stat` | `.st_blksize` | `i32` | tools/rust/library/std/src/os/netbsd/raw.rs:77 |
| `std::os::netbsd::raw::stat` | `.st_flags` | `u32` | tools/rust/library/std/src/os/netbsd/raw.rs:79 |
| `std::os::netbsd::raw::stat` | `.st_gen` | `u32` | tools/rust/library/std/src/os/netbsd/raw.rs:81 |
| `std::os::netbsd::raw::stat` | `.st_spare` | `[u32; 2]` | tools/rust/library/std/src/os/netbsd/raw.rs:82 |
| `std::os::netbsd::raw::time_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/netbsd/raw.rs:33 |
| `std::os::nto::raw::arch::blkcnt_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/nto/raw.rs:29 |
| `std::os::nto::raw::arch::blksize_t` | `(alias)` | `i32` | tools/rust/library/std/src/os/nto/raw.rs:31 |
| `std::os::nto::raw::arch::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/nto/raw.rs:33 |
| `std::os::nto::raw::arch::nlink_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/nto/raw.rs:35 |
| `std::os::nto::raw::arch::off_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/nto/raw.rs:37 |
| `std::os::nto::raw::arch::time_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/nto/raw.rs:39 |
| `std::os::nto::raw::dev_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/nto/raw.rs:14 |
| `std::os::nto::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/nto/raw.rs:16 |
| `std::os::nto::raw::pthread_t` | `(alias)` | `os::raw::c_int` | tools/rust/library/std/src/os/nto/raw.rs:19 |
| `std::os::nuttx::raw::blkcnt_t` | `(alias)` | `libc::blkcnt_t` | tools/rust/library/std/src/os/nuttx/raw.rs:17 |
| `std::os::nuttx::raw::blksize_t` | `(alias)` | `libc::blksize_t` | tools/rust/library/std/src/os/nuttx/raw.rs:20 |
| `std::os::nuttx::raw::dev_t` | `(alias)` | `libc::dev_t` | tools/rust/library/std/src/os/nuttx/raw.rs:22 |
| `std::os::nuttx::raw::ino_t` | `(alias)` | `libc::ino_t` | tools/rust/library/std/src/os/nuttx/raw.rs:24 |
| `std::os::nuttx::raw::mode_t` | `(alias)` | `libc::mode_t` | tools/rust/library/std/src/os/nuttx/raw.rs:26 |
| `std::os::nuttx::raw::nlink_t` | `(alias)` | `libc::nlink_t` | tools/rust/library/std/src/os/nuttx/raw.rs:28 |
| `std::os::nuttx::raw::off_t` | `(alias)` | `libc::off_t` | tools/rust/library/std/src/os/nuttx/raw.rs:30 |
| `std::os::nuttx::raw::pthread_t` | `(alias)` | `libc::pthread_t` | tools/rust/library/std/src/os/nuttx/raw.rs:14 |
| `std::os::nuttx::raw::time_t` | `(alias)` | `libc::time_t` | tools/rust/library/std/src/os/nuttx/raw.rs:33 |
| `std::os::openbsd::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/openbsd/raw.rs:16 |
| `std::os::openbsd::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/openbsd/raw.rs:18 |
| `std::os::openbsd::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/openbsd/raw.rs:20 |
| `std::os::openbsd::raw::fflags_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:22 |
| `std::os::openbsd::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/openbsd/raw.rs:24 |
| `std::os::openbsd::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:26 |
| `std::os::openbsd::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/openbsd/raw.rs:28 |
| `std::os::openbsd::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/openbsd/raw.rs:30 |
| `std::os::openbsd::raw::pthread_t` | `(alias)` | `usize` | tools/rust/library/std/src/os/openbsd/raw.rs:35 |
| `std::os::openbsd::raw::stat` | `.st_mode` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:42 |
| `std::os::openbsd::raw::stat` | `.st_dev` | `i32` | tools/rust/library/std/src/os/openbsd/raw.rs:44 |
| `std::os::openbsd::raw::stat` | `.st_ino` | `u64` | tools/rust/library/std/src/os/openbsd/raw.rs:46 |
| `std::os::openbsd::raw::stat` | `.st_nlink` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:48 |
| `std::os::openbsd::raw::stat` | `.st_uid` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:50 |
| `std::os::openbsd::raw::stat` | `.st_gid` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:52 |
| `std::os::openbsd::raw::stat` | `.st_rdev` | `i32` | tools/rust/library/std/src/os/openbsd/raw.rs:54 |
| `std::os::openbsd::raw::stat` | `.st_atime` | `i64` | tools/rust/library/std/src/os/openbsd/raw.rs:56 |
| `std::os::openbsd::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/openbsd/raw.rs:58 |
| `std::os::openbsd::raw::stat` | `.st_mtime` | `i64` | tools/rust/library/std/src/os/openbsd/raw.rs:60 |
| `std::os::openbsd::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/openbsd/raw.rs:62 |
| `std::os::openbsd::raw::stat` | `.st_ctime` | `i64` | tools/rust/library/std/src/os/openbsd/raw.rs:64 |
| `std::os::openbsd::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/openbsd/raw.rs:66 |
| `std::os::openbsd::raw::stat` | `.st_size` | `i64` | tools/rust/library/std/src/os/openbsd/raw.rs:68 |
| `std::os::openbsd::raw::stat` | `.st_blocks` | `i64` | tools/rust/library/std/src/os/openbsd/raw.rs:70 |
| `std::os::openbsd::raw::stat` | `.st_blksize` | `i32` | tools/rust/library/std/src/os/openbsd/raw.rs:72 |
| `std::os::openbsd::raw::stat` | `.st_flags` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:74 |
| `std::os::openbsd::raw::stat` | `.st_gen` | `u32` | tools/rust/library/std/src/os/openbsd/raw.rs:76 |
| `std::os::openbsd::raw::stat` | `.st_birthtime` | `i64` | tools/rust/library/std/src/os/openbsd/raw.rs:78 |
| `std::os::openbsd::raw::stat` | `.st_birthtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/openbsd/raw.rs:80 |
| `std::os::openbsd::raw::time_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/openbsd/raw.rs:32 |
| `std::os::redox::raw::blkcnt_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/redox/raw.rs:28 |
| `std::os::redox::raw::blksize_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/redox/raw.rs:30 |
| `std::os::redox::raw::dev_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/redox/raw.rs:16 |
| `std::os::redox::raw::gid_t` | `(alias)` | `os::raw::c_int` | tools/rust/library/std/src/os/redox/raw.rs:18 |
| `std::os::redox::raw::ino_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/redox/raw.rs:32 |
| `std::os::redox::raw::mode_t` | `(alias)` | `os::raw::c_int` | tools/rust/library/std/src/os/redox/raw.rs:20 |
| `std::os::redox::raw::nlink_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/redox/raw.rs:34 |
| `std::os::redox::raw::off_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/redox/raw.rs:36 |
| `std::os::redox::raw::pthread_t` | `(alias)` | `*mut os::raw::c_void` | tools/rust/library/std/src/os/redox/raw.rs:25 |
| `std::os::redox::raw::stat` | `.st_dev` | `std::os::redox::raw::dev_t` | tools/rust/library/std/src/os/redox/raw.rs:45 |
| `std::os::redox::raw::stat` | `.st_ino` | `std::os::redox::raw::ino_t` | tools/rust/library/std/src/os/redox/raw.rs:47 |
| `std::os::redox::raw::stat` | `.st_nlink` | `std::os::redox::raw::nlink_t` | tools/rust/library/std/src/os/redox/raw.rs:49 |
| `std::os::redox::raw::stat` | `.st_mode` | `std::os::redox::raw::mode_t` | tools/rust/library/std/src/os/redox/raw.rs:51 |
| `std::os::redox::raw::stat` | `.st_uid` | `std::os::redox::raw::uid_t` | tools/rust/library/std/src/os/redox/raw.rs:53 |
| `std::os::redox::raw::stat` | `.st_gid` | `std::os::redox::raw::gid_t` | tools/rust/library/std/src/os/redox/raw.rs:55 |
| `std::os::redox::raw::stat` | `.st_rdev` | `std::os::redox::raw::dev_t` | tools/rust/library/std/src/os/redox/raw.rs:57 |
| `std::os::redox::raw::stat` | `.st_size` | `std::os::redox::raw::off_t` | tools/rust/library/std/src/os/redox/raw.rs:59 |
| `std::os::redox::raw::stat` | `.st_blksize` | `std::os::redox::raw::blksize_t` | tools/rust/library/std/src/os/redox/raw.rs:61 |
| `std::os::redox::raw::stat` | `.st_blocks` | `std::os::redox::raw::blkcnt_t` | tools/rust/library/std/src/os/redox/raw.rs:63 |
| `std::os::redox::raw::stat` | `.st_atime` | `std::os::redox::raw::time_t` | tools/rust/library/std/src/os/redox/raw.rs:65 |
| `std::os::redox::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/redox/raw.rs:67 |
| `std::os::redox::raw::stat` | `.st_mtime` | `std::os::redox::raw::time_t` | tools/rust/library/std/src/os/redox/raw.rs:69 |
| `std::os::redox::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/redox/raw.rs:71 |
| `std::os::redox::raw::stat` | `.st_ctime` | `std::os::redox::raw::time_t` | tools/rust/library/std/src/os/redox/raw.rs:73 |
| `std::os::redox::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/redox/raw.rs:75 |
| `std::os::redox::raw::stat` | `._pad` | `[os::raw::c_char; 24]` | tools/rust/library/std/src/os/redox/raw.rs:77 |
| `std::os::redox::raw::time_t` | `(alias)` | `os::raw::c_long` | tools/rust/library/std/src/os/redox/raw.rs:38 |
| `std::os::redox::raw::uid_t` | `(alias)` | `os::raw::c_int` | tools/rust/library/std/src/os/redox/raw.rs:22 |
| `std::os::rtems::raw::blkcnt_t` | `(alias)` | `libc::blkcnt_t` | tools/rust/library/std/src/os/rtems/raw.rs:17 |
| `std::os::rtems::raw::blksize_t` | `(alias)` | `libc::blksize_t` | tools/rust/library/std/src/os/rtems/raw.rs:20 |
| `std::os::rtems::raw::dev_t` | `(alias)` | `libc::dev_t` | tools/rust/library/std/src/os/rtems/raw.rs:22 |
| `std::os::rtems::raw::ino_t` | `(alias)` | `libc::ino_t` | tools/rust/library/std/src/os/rtems/raw.rs:24 |
| `std::os::rtems::raw::mode_t` | `(alias)` | `libc::mode_t` | tools/rust/library/std/src/os/rtems/raw.rs:26 |
| `std::os::rtems::raw::nlink_t` | `(alias)` | `libc::nlink_t` | tools/rust/library/std/src/os/rtems/raw.rs:28 |
| `std::os::rtems::raw::off_t` | `(alias)` | `libc::off_t` | tools/rust/library/std/src/os/rtems/raw.rs:30 |
| `std::os::rtems::raw::pthread_t` | `(alias)` | `libc::pthread_t` | tools/rust/library/std/src/os/rtems/raw.rs:14 |
| `std::os::rtems::raw::time_t` | `(alias)` | `libc::time_t` | tools/rust/library/std/src/os/rtems/raw.rs:33 |
| `std::os::solaris::raw::blkcnt_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/solaris/raw.rs:17 |
| `std::os::solaris::raw::blksize_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/solaris/raw.rs:19 |
| `std::os::solaris::raw::dev_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/solaris/raw.rs:21 |
| `std::os::solaris::raw::fflags_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/solaris/raw.rs:23 |
| `std::os::solaris::raw::ino_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/solaris/raw.rs:25 |
| `std::os::solaris::raw::mode_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/solaris/raw.rs:27 |
| `std::os::solaris::raw::nlink_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/solaris/raw.rs:29 |
| `std::os::solaris::raw::off_t` | `(alias)` | `u64` | tools/rust/library/std/src/os/solaris/raw.rs:31 |
| `std::os::solaris::raw::pthread_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/solaris/raw.rs:36 |
| `std::os::solaris::raw::stat` | `.st_dev` | `std::os::solaris::raw::dev_t` | tools/rust/library/std/src/os/solaris/raw.rs:43 |
| `std::os::solaris::raw::stat` | `.st_ino` | `std::os::solaris::raw::ino_t` | tools/rust/library/std/src/os/solaris/raw.rs:45 |
| `std::os::solaris::raw::stat` | `.st_mode` | `std::os::solaris::raw::mode_t` | tools/rust/library/std/src/os/solaris/raw.rs:47 |
| `std::os::solaris::raw::stat` | `.st_nlink` | `std::os::solaris::raw::nlink_t` | tools/rust/library/std/src/os/solaris/raw.rs:49 |
| `std::os::solaris::raw::stat` | `.st_uid` | `std::os::unix::raw::uid_t` | tools/rust/library/std/src/os/solaris/raw.rs:51 |
| `std::os::solaris::raw::stat` | `.st_gid` | `std::os::unix::raw::gid_t` | tools/rust/library/std/src/os/solaris/raw.rs:53 |
| `std::os::solaris::raw::stat` | `.st_rdev` | `std::os::solaris::raw::dev_t` | tools/rust/library/std/src/os/solaris/raw.rs:55 |
| `std::os::solaris::raw::stat` | `.st_size` | `std::os::solaris::raw::off_t` | tools/rust/library/std/src/os/solaris/raw.rs:57 |
| `std::os::solaris::raw::stat` | `.st_atime` | `std::os::solaris::raw::time_t` | tools/rust/library/std/src/os/solaris/raw.rs:59 |
| `std::os::solaris::raw::stat` | `.st_atime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/solaris/raw.rs:61 |
| `std::os::solaris::raw::stat` | `.st_mtime` | `std::os::solaris::raw::time_t` | tools/rust/library/std/src/os/solaris/raw.rs:63 |
| `std::os::solaris::raw::stat` | `.st_mtime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/solaris/raw.rs:65 |
| `std::os::solaris::raw::stat` | `.st_ctime` | `std::os::solaris::raw::time_t` | tools/rust/library/std/src/os/solaris/raw.rs:67 |
| `std::os::solaris::raw::stat` | `.st_ctime_nsec` | `os::raw::c_long` | tools/rust/library/std/src/os/solaris/raw.rs:69 |
| `std::os::solaris::raw::stat` | `.st_blksize` | `std::os::solaris::raw::blksize_t` | tools/rust/library/std/src/os/solaris/raw.rs:71 |
| `std::os::solaris::raw::stat` | `.st_blocks` | `std::os::solaris::raw::blkcnt_t` | tools/rust/library/std/src/os/solaris/raw.rs:73 |
| `std::os::solaris::raw::stat` | `.__unused` | `[u8; 16]` | tools/rust/library/std/src/os/solaris/raw.rs:75 |
| `std::os::solaris::raw::time_t` | `(alias)` | `i64` | tools/rust/library/std/src/os/solaris/raw.rs:33 |
| `std::os::solid::io::BorrowedFd` | `.fd` | `std::os::solid::io::ValidRawFd` | tools/rust/library/std/src/os/solid/io.rs:77 |
| `std::os::solid::io::BorrowedFd` | `._phantom` | `marker::PhantomData<std::os::solid::io::OwnedFd>` | tools/rust/library/std/src/os/solid/io.rs:78 |
| `std::os::solid::io::OwnedFd` | `.fd` | `std::os::solid::io::ValidRawFd` | tools/rust/library/std/src/os/solid/io.rs:92 |
| `std::os::solid::io::RawFd` | `(alias)` | `i32` | tools/rust/library/std/src/os/solid/io.rs:55 |
| `std::os::solid::io::ValidRawFd` | `(alias)` | `core::num::niche_types::NotAllOnes<std::os::solid::io::RawFd>` | tools/rust/library/std/src/os/solid/io.rs:58 |
| `std::os::unix::net::addr::AddressKind` | `.Pathname` | `std::os::unix::net::addr::AddressKind::Pathname` | tools/rust/library/std/src/os/unix/net/addr.rs:63 |
| `std::os::unix::net::addr::AddressKind` | `.Abstract` | `std::os::unix::net::addr::AddressKind::Abstract` | tools/rust/library/std/src/os/unix/net/addr.rs:64 |
| `std::os::unix::net::addr::AddressKind::Abstract` | `[0]` | `[u8]` | tools/rust/library/std/src/os/unix/net/addr.rs:64 |
| `std::os::unix::net::addr::AddressKind::Pathname` | `[0]` | `std::path::Path` | tools/rust/library/std/src/os/unix/net/addr.rs:63 |
| `std::os::unix::net::addr::SocketAddr` | `.addr` | `std::os::unix::net::addr::libc::sockaddr_un` | tools/rust/library/std/src/os/unix/net/addr.rs:86 |
| `std::os::unix::net::addr::SocketAddr` | `.len` | `std::os::unix::net::addr::libc::socklen_t` | tools/rust/library/std/src/os/unix/net/addr.rs:87 |
| `std::os::unix::net::addr::libc::sockaddr_un` | `.sun_path` | `[u8; 1]` | tools/rust/library/std/src/os/unix/net/addr.rs:19 |
| `std::os::unix::net::addr::libc::socklen_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/unix/net/addr.rs:15 |
| `std::os::unix::net::ancillary::AncillaryData` | `.ScmRights` | `std::os::unix::net::ancillary::AncillaryData::ScmRights` | tools/rust/library/std/src/os/unix/net/ancillary.rs:457 |
| `std::os::unix::net::ancillary::AncillaryData` | `.ScmCredentials` | `std::os::unix::net::ancillary::AncillaryData::ScmCredentials` | tools/rust/library/std/src/os/unix/net/ancillary.rs:465 |
| `std::os::unix::net::ancillary::AncillaryData::ScmCredentials` | `[0]` | `std::os::unix::net::ancillary::ScmCredentials` | tools/rust/library/std/src/os/unix/net/ancillary.rs:465 |
| `std::os::unix::net::ancillary::AncillaryData::ScmRights` | `[0]` | `std::os::unix::net::ancillary::ScmRights` | tools/rust/library/std/src/os/unix/net/ancillary.rs:457 |
| `std::os::unix::net::ancillary::AncillaryDataIter` | `.data` | `[u8]` | tools/rust/library/std/src/os/unix/net/ancillary.rs:162 |
| `std::os::unix::net::ancillary::AncillaryDataIter` | `.phantom` | `marker::PhantomData<$T>` | tools/rust/library/std/src/os/unix/net/ancillary.rs:163 |
| `std::os::unix::net::ancillary::AncillaryError` | `.Unknown` | `std::os::unix::net::ancillary::AncillaryError::Unknown` | tools/rust/library/std/src/os/unix/net/ancillary.rs:451 |
| `std::os::unix::net::ancillary::AncillaryError::Unknown` | `.cmsg_level` | `i32` | tools/rust/library/std/src/os/unix/net/ancillary.rs:451 |
| `std::os::unix::net::ancillary::AncillaryError::Unknown` | `.cmsg_type` | `i32` | tools/rust/library/std/src/os/unix/net/ancillary.rs:451 |
| `std::os::unix::net::ancillary::Messages` | `.buffer` | `[u8]` | tools/rust/library/std/src/os/unix/net/ancillary.rs:532 |
| `std::os::unix::net::ancillary::Messages` | `.current` | `Option<std::os::unix::net::ancillary::libc::cmsghdr>` | tools/rust/library/std/src/os/unix/net/ancillary.rs:533 |
| `std::os::unix::net::ancillary::ScmRights` | `[0]` | `std::os::unix::net::ancillary::AncillaryDataIter<os::unix::io::RawFd>` | tools/rust/library/std/src/os/unix/net/ancillary.rs:394 |
| `std::os::unix::net::ancillary::SocketAncillary` | `.buffer` | `[u8]` | tools/rust/library/std/src/os/unix/net/ancillary.rs:602 |
| `std::os::unix::net::ancillary::SocketAncillary` | `.length` | `usize` | tools/rust/library/std/src/os/unix/net/ancillary.rs:603 |
| `std::os::unix::net::ancillary::SocketAncillary` | `.truncated` | `bool` | tools/rust/library/std/src/os/unix/net/ancillary.rs:604 |
| `std::os::unix::net::ancillary::libc::gid_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/unix/net/ancillary.rs:28 |
| `std::os::unix::net::ancillary::libc::pid_t` | `(alias)` | `i32` | tools/rust/library/std/src/os/unix/net/ancillary.rs:27 |
| `std::os::unix::net::ancillary::libc::uid_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/unix/net/ancillary.rs:29 |
| `std::os::unix::net::datagram::UnixDatagram` | `[0]` | `sys::net::Socket` | tools/rust/library/std/src/os/unix/net/datagram.rs:62 |
| `std::os::unix::net::listener::Incoming` | `.listener` | `std::os::unix::net::listener::UnixListener` | tools/rust/library/std/src/os/unix/net/listener.rs:404 |
| `std::os::unix::net::listener::UnixListener` | `[0]` | `sys::net::Socket` | tools/rust/library/std/src/os/unix/net/listener.rs:41 |
| `std::os::unix::net::stream::UnixStream` | `[0]` | `sys::net::Socket` | tools/rust/library/std/src/os/unix/net/stream.rs:45 |
| `std::os::unix::net::ucred::UCred` | `.uid` | `libc::uid_t` | tools/rust/library/std/src/os/unix/net/ucred.rs:15 |
| `std::os::unix::net::ucred::UCred` | `.gid` | `libc::gid_t` | tools/rust/library/std/src/os/unix/net/ucred.rs:18 |
| `std::os::unix::net::ucred::UCred` | `.pid` | `Option<libc::pid_t>` | tools/rust/library/std/src/os/unix/net/ucred.rs:23 |
| `std::os::unix::raw::gid_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/unix/raw.rs:19 |
| `std::os::unix::raw::pid_t` | `(alias)` | `i32` | tools/rust/library/std/src/os/unix/raw.rs:23 |
| `std::os::unix::raw::uid_t` | `(alias)` | `u32` | tools/rust/library/std/src/os/unix/raw.rs:15 |
| `std::os::unix::thread::RawPthread` | `(alias)` | `std::os::linux::raw::pthread_t` | tools/rust/library/std/src/os/unix/thread.rs:14 |
| `std::os::vita::raw::blkcnt_t` | `(alias)` | `libc::blkcnt_t` | tools/rust/library/std/src/os/vita/raw.rs:17 |
| `std::os::vita::raw::blksize_t` | `(alias)` | `libc::blksize_t` | tools/rust/library/std/src/os/vita/raw.rs:20 |
| `std::os::vita::raw::dev_t` | `(alias)` | `libc::dev_t` | tools/rust/library/std/src/os/vita/raw.rs:22 |
| `std::os::vita::raw::ino_t` | `(alias)` | `libc::ino_t` | tools/rust/library/std/src/os/vita/raw.rs:24 |
| `std::os::vita::raw::mode_t` | `(alias)` | `libc::mode_t` | tools/rust/library/std/src/os/vita/raw.rs:26 |
| `std::os::vita::raw::nlink_t` | `(alias)` | `libc::nlink_t` | tools/rust/library/std/src/os/vita/raw.rs:28 |
| `std::os::vita::raw::off_t` | `(alias)` | `libc::off_t` | tools/rust/library/std/src/os/vita/raw.rs:30 |
| `std::os::vita::raw::pthread_t` | `(alias)` | `libc::pthread_t` | tools/rust/library/std/src/os/vita/raw.rs:14 |
| `std::os::vita::raw::time_t` | `(alias)` | `libc::time_t` | tools/rust/library/std/src/os/vita/raw.rs:33 |
| `std::os::vxworks::raw::pthread_t` | `(alias)` | `os::raw::c_ulong` | tools/rust/library/std/src/os/vxworks/raw.rs:7 |
| `std::os::windows::io::handle::BorrowedHandle` | `.handle` | `std::os::windows::io::raw::RawHandle` | tools/rust/library/std/src/os/windows/io/handle.rs:39 |
| `std::os::windows::io::handle::BorrowedHandle` | `._phantom` | `marker::PhantomData<std::os::windows::io::handle::OwnedHandle>` | tools/rust/library/std/src/os/windows/io/handle.rs:40 |
| `std::os::windows::io::handle::HandleOrInvalid` | `[0]` | `std::os::windows::io::raw::RawHandle` | tools/rust/library/std/src/os/windows/io/handle.rs:109 |
| `std::os::windows::io::handle::HandleOrNull` | `[0]` | `std::os::windows::io::raw::RawHandle` | tools/rust/library/std/src/os/windows/io/handle.rs:90 |
| `std::os::windows::io::handle::InvalidHandleError` | `[0]` | `()` | tools/rust/library/std/src/os/windows/io/handle.rs:285 |
| `std::os::windows::io::handle::NullHandleError` | `[0]` | `()` | tools/rust/library/std/src/os/windows/io/handle.rs:267 |
| `std::os::windows::io::handle::OwnedHandle` | `.handle` | `std::os::windows::io::raw::RawHandle` | tools/rust/library/std/src/os/windows/io/handle.rs:67 |
| `std::os::windows::io::raw::RawHandle` | `(alias)` | `std::os::windows::raw::HANDLE` | tools/rust/library/std/src/os/windows/io/raw.rs:14 |
| `std::os::windows::io::raw::RawSocket` | `(alias)` | `std::os::windows::raw::SOCKET` | tools/rust/library/std/src/os/windows/io/raw.rs:18 |
| `std::os::windows::io::socket::BorrowedSocket` | `.socket` | `std::os::windows::io::socket::ValidRawSocket` | tools/rust/library/std/src/os/windows/io/socket.rs:33 |
| `std::os::windows::io::socket::BorrowedSocket` | `._phantom` | `marker::PhantomData<std::os::windows::io::socket::OwnedSocket>` | tools/rust/library/std/src/os/windows/io/socket.rs:34 |
| `std::os::windows::io::socket::OwnedSocket` | `.socket` | `std::os::windows::io::socket::ValidRawSocket` | tools/rust/library/std/src/os/windows/io/socket.rs:49 |
| `std::os::windows::io::socket::ValidRawSocket` | `(alias)` | `core::num::niche_types::NotAllOnes<std::os::windows::io::raw::RawSocket>` | tools/rust/library/std/src/os/windows/io/socket.rs:13 |
| `std::os::windows::process::ProcThreadAttributeList` | `.attribute_list` | `Box<[mem::MaybeUninit<u8>]>` | tools/rust/library/std/src/os/windows/process.rs:433 |
| `std::os::windows::process::ProcThreadAttributeList` | `._lifetime_marker` | `marker::PhantomData<()>` | tools/rust/library/std/src/os/windows/process.rs:434 |
| `std::os::windows::process::ProcThreadAttributeListBuilder` | `.attributes` | `alloc::collections::BTreeMap<usize, std::os::windows::process::ProcThreadAttributeValue>` | tools/rust/library/std/src/os/windows/process.rs:469 |
| `std::os::windows::process::ProcThreadAttributeListBuilder` | `._lifetime_marker` | `marker::PhantomData<()>` | tools/rust/library/std/src/os/windows/process.rs:470 |
| `std::os::windows::process::ProcThreadAttributeValue` | `.ptr` | `*const core::ffi::c_void` | tools/rust/library/std/src/os/windows/process.rs:661 |
| `std::os::windows::process::ProcThreadAttributeValue` | `.size` | `usize` | tools/rust/library/std/src/os/windows/process.rs:662 |
| `std::os::windows::raw::HANDLE` | `(alias)` | `*mut os::raw::c_void` | tools/rust/library/std/src/os/windows/raw.rs:8 |
| `std::os::xous::ffi::definitions::Connection` | `[0]` | `u32` | tools/rust/library/std/src/os/xous/ffi/definitions.rs:192 |
| `std::os::xous::ffi::definitions::ServerAddress` | `[0]` | `[u32; 4]` | tools/rust/library/std/src/os/xous/ffi/definitions.rs:231 |
| `std::os::xous::ffi::definitions::ThreadId` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/ffi/definitions.rs:263 |
| `std::os::xous::ffi::definitions::memoryflags::MemoryFlags` | `.bits` | `usize` | tools/rust/library/std/src/os/xous/ffi/definitions/memoryflags.rs:7 |
| `std::os::xous::services::log::LogScalar` | `.AppendPanicMessage` | `std::os::xous::services::log::LogScalar::AppendPanicMessage` | tools/rust/library/std/src/os/xous/services/log.rs:25 |
| `std::os::xous::services::log::LogScalar::AppendPanicMessage` | `[0]` | `[u8]` | tools/rust/library/std/src/os/xous/services/log.rs:25 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdGetTtlUdp` | `std::os::xous::services::net::NetBlockingScalar::StdGetTtlUdp` | tools/rust/library/std/src/os/xous/services/net.rs:7 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdSetTtlUdp` | `std::os::xous::services::net::NetBlockingScalar::StdSetTtlUdp` | tools/rust/library/std/src/os/xous/services/net.rs:8 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdGetTtlTcp` | `std::os::xous::services::net::NetBlockingScalar::StdGetTtlTcp` | tools/rust/library/std/src/os/xous/services/net.rs:9 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdSetTtlTcp` | `std::os::xous::services::net::NetBlockingScalar::StdSetTtlTcp` | tools/rust/library/std/src/os/xous/services/net.rs:10 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdGetNodelay` | `std::os::xous::services::net::NetBlockingScalar::StdGetNodelay` | tools/rust/library/std/src/os/xous/services/net.rs:11 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdSetNodelay` | `std::os::xous::services::net::NetBlockingScalar::StdSetNodelay` | tools/rust/library/std/src/os/xous/services/net.rs:12 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdTcpClose` | `std::os::xous::services::net::NetBlockingScalar::StdTcpClose` | tools/rust/library/std/src/os/xous/services/net.rs:13 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdUdpClose` | `std::os::xous::services::net::NetBlockingScalar::StdUdpClose` | tools/rust/library/std/src/os/xous/services/net.rs:14 |
| `std::os::xous::services::net::NetBlockingScalar` | `.StdTcpStreamShutdown` | `std::os::xous::services::net::NetBlockingScalar::StdTcpStreamShutdown` | tools/rust/library/std/src/os/xous/services/net.rs:15 |
| `std::os::xous::services::net::NetBlockingScalar::StdGetNodelay` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:11 |
| `std::os::xous::services::net::NetBlockingScalar::StdGetTtlTcp` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:9 |
| `std::os::xous::services::net::NetBlockingScalar::StdGetTtlUdp` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:7 |
| `std::os::xous::services::net::NetBlockingScalar::StdSetNodelay` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:12 |
| `std::os::xous::services::net::NetBlockingScalar::StdSetNodelay` | `[1]` | `bool` | tools/rust/library/std/src/os/xous/services/net.rs:12 |
| `std::os::xous::services::net::NetBlockingScalar::StdSetTtlTcp` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:10 |
| `std::os::xous::services::net::NetBlockingScalar::StdSetTtlTcp` | `[1]` | `u32` | tools/rust/library/std/src/os/xous/services/net.rs:10 |
| `std::os::xous::services::net::NetBlockingScalar::StdSetTtlUdp` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:8 |
| `std::os::xous::services::net::NetBlockingScalar::StdSetTtlUdp` | `[1]` | `u32` | tools/rust/library/std/src/os/xous/services/net.rs:8 |
| `std::os::xous::services::net::NetBlockingScalar::StdTcpClose` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:13 |
| `std::os::xous::services::net::NetBlockingScalar::StdTcpStreamShutdown` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:15 |
| `std::os::xous::services::net::NetBlockingScalar::StdTcpStreamShutdown` | `[1]` | `std::net::Shutdown` | tools/rust/library/std/src/os/xous/services/net.rs:15 |
| `std::os::xous::services::net::NetBlockingScalar::StdUdpClose` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:14 |
| `std::os::xous::services::net::NetLendMut` | `.StdTcpTx` | `std::os::xous::services::net::NetLendMut::StdTcpTx` | tools/rust/library/std/src/os/xous/services/net.rs:20 |
| `std::os::xous::services::net::NetLendMut` | `.StdTcpPeek` | `std::os::xous::services::net::NetLendMut::StdTcpPeek` | tools/rust/library/std/src/os/xous/services/net.rs:21 |
| `std::os::xous::services::net::NetLendMut` | `.StdTcpRx` | `std::os::xous::services::net::NetLendMut::StdTcpRx` | tools/rust/library/std/src/os/xous/services/net.rs:22 |
| `std::os::xous::services::net::NetLendMut` | `.StdGetAddress` | `std::os::xous::services::net::NetLendMut::StdGetAddress` | tools/rust/library/std/src/os/xous/services/net.rs:23 |
| `std::os::xous::services::net::NetLendMut` | `.StdUdpRx` | `std::os::xous::services::net::NetLendMut::StdUdpRx` | tools/rust/library/std/src/os/xous/services/net.rs:25 |
| `std::os::xous::services::net::NetLendMut` | `.StdUdpTx` | `std::os::xous::services::net::NetLendMut::StdUdpTx` | tools/rust/library/std/src/os/xous/services/net.rs:26 |
| `std::os::xous::services::net::NetLendMut` | `.StdTcpAccept` | `std::os::xous::services::net::NetLendMut::StdTcpAccept` | tools/rust/library/std/src/os/xous/services/net.rs:28 |
| `std::os::xous::services::net::NetLendMut::StdGetAddress` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:23 |
| `std::os::xous::services::net::NetLendMut::StdTcpAccept` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:28 |
| `std::os::xous::services::net::NetLendMut::StdTcpPeek` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:21 |
| `std::os::xous::services::net::NetLendMut::StdTcpPeek` | `[1]` | `bool` | tools/rust/library/std/src/os/xous/services/net.rs:21 |
| `std::os::xous::services::net::NetLendMut::StdTcpRx` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:22 |
| `std::os::xous::services::net::NetLendMut::StdTcpRx` | `[1]` | `bool` | tools/rust/library/std/src/os/xous/services/net.rs:22 |
| `std::os::xous::services::net::NetLendMut::StdTcpTx` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:20 |
| `std::os::xous::services::net::NetLendMut::StdUdpRx` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:25 |
| `std::os::xous::services::net::NetLendMut::StdUdpTx` | `[0]` | `u16` | tools/rust/library/std/src/os/xous/services/net.rs:26 |
| `std::os::xous::services::ns::ConnectRequest` | `.data` | `[u8; 4096]` | tools/rust/library/std/src/os/xous/services.rs:30 |
| `std::os::xous::services::ticktimer::TicktimerScalar` | `.SleepMs` | `std::os::xous::services::ticktimer::TicktimerScalar::SleepMs` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:7 |
| `std::os::xous::services::ticktimer::TicktimerScalar` | `.LockMutex` | `std::os::xous::services::ticktimer::TicktimerScalar::LockMutex` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:8 |
| `std::os::xous::services::ticktimer::TicktimerScalar` | `.UnlockMutex` | `std::os::xous::services::ticktimer::TicktimerScalar::UnlockMutex` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:9 |
| `std::os::xous::services::ticktimer::TicktimerScalar` | `.WaitForCondition` | `std::os::xous::services::ticktimer::TicktimerScalar::WaitForCondition` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:10 |
| `std::os::xous::services::ticktimer::TicktimerScalar` | `.NotifyCondition` | `std::os::xous::services::ticktimer::TicktimerScalar::NotifyCondition` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:11 |
| `std::os::xous::services::ticktimer::TicktimerScalar` | `.FreeMutex` | `std::os::xous::services::ticktimer::TicktimerScalar::FreeMutex` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:12 |
| `std::os::xous::services::ticktimer::TicktimerScalar` | `.FreeCondition` | `std::os::xous::services::ticktimer::TicktimerScalar::FreeCondition` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:13 |
| `std::os::xous::services::ticktimer::TicktimerScalar::FreeCondition` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:13 |
| `std::os::xous::services::ticktimer::TicktimerScalar::FreeMutex` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:12 |
| `std::os::xous::services::ticktimer::TicktimerScalar::LockMutex` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:8 |
| `std::os::xous::services::ticktimer::TicktimerScalar::NotifyCondition` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:11 |
| `std::os::xous::services::ticktimer::TicktimerScalar::NotifyCondition` | `[1]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:11 |
| `std::os::xous::services::ticktimer::TicktimerScalar::SleepMs` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:7 |
| `std::os::xous::services::ticktimer::TicktimerScalar::UnlockMutex` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:9 |
| `std::os::xous::services::ticktimer::TicktimerScalar::WaitForCondition` | `[0]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:10 |
| `std::os::xous::services::ticktimer::TicktimerScalar::WaitForCondition` | `[1]` | `usize` | tools/rust/library/std/src/os/xous/services/ticktimer.rs:10 |
| `std::panic::PanicHookInfo` | `.payload` | `opaque(dyn Any + Send)` | tools/rust/library/std/src/panic.rs:43 |
| `std::panic::PanicHookInfo` | `.location` | `core::panic::location::Location` | tools/rust/library/std/src/panic.rs:44 |
| `std::panic::PanicHookInfo` | `.can_unwind` | `bool` | tools/rust/library/std/src/panic.rs:45 |
| `std::panic::PanicHookInfo` | `.force_no_backtrace` | `bool` | tools/rust/library/std/src/panic.rs:46 |
| `std::panic::PanicInfo` | `(alias)` | `std::panic::PanicHookInfo` | tools/rust/library/std/src/panic.rs:21 |
| `std::panicking::Hook` | `.Custom` | `std::panicking::Hook::Custom` | tools/rust/library/std/src/panicking.rs:90 |
| `std::panicking::Hook::Custom` | `[0]` | `Box<opaque(dyn Fn (& PanicHookInfo < '_ >) + 'static + Sync + Send)>` | tools/rust/library/std/src/panicking.rs:90 |
| `std::path::Ancestors` | `.next` | `Option<std::path::Path>` | tools/rust/library/std/src/path.rs:1079 |
| `std::path::Component` | `.Prefix` | `std::path::Component::Prefix` | tools/rust/library/std/src/path.rs:501 |
| `std::path::Component` | `.Normal` | `std::path::Component::Normal` | tools/rust/library/std/src/path.rs:522 |
| `std::path::Component::Normal` | `[0]` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:522 |
| `std::path::Component::Prefix` | `[0]` | `std::path::PrefixComponent` | tools/rust/library/std/src/path.rs:501 |
| `std::path::Components` | `.path` | `[u8]` | tools/rust/library/std/src/path.rs:589 |
| `std::path::Components` | `.prefix` | `Option<std::path::Prefix>` | tools/rust/library/std/src/path.rs:592 |
| `std::path::Components` | `.has_physical_root` | `bool` | tools/rust/library/std/src/path.rs:597 |
| `std::path::Components` | `.front` | `std::path::State` | tools/rust/library/std/src/path.rs:601 |
| `std::path::Components` | `.back` | `std::path::State` | tools/rust/library/std/src/path.rs:602 |
| `std::path::Display` | `.inner` | `std::ffi::os_str::Display` | tools/rust/library/std/src/path.rs:3205 |
| `std::path::Iter` | `.inner` | `std::path::Components` | tools/rust/library/std/src/path.rs:615 |
| `std::path::Path` | `.inner` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:2130 |
| `std::path::PathBuf` | `.inner` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/path.rs:1169 |
| `std::path::Prefix` | `.Verbatim` | `std::path::Prefix::Verbatim` | tools/rust/library/std/src/path.rs:143 |
| `std::path::Prefix` | `.VerbatimUNC` | `std::path::Prefix::VerbatimUNC` | tools/rust/library/std/src/path.rs:151 |
| `std::path::Prefix` | `.VerbatimDisk` | `std::path::Prefix::VerbatimDisk` | tools/rust/library/std/src/path.rs:161 |
| `std::path::Prefix` | `.DeviceNS` | `std::path::Prefix::DeviceNS` | tools/rust/library/std/src/path.rs:168 |
| `std::path::Prefix` | `.UNC` | `std::path::Prefix::UNC` | tools/rust/library/std/src/path.rs:175 |
| `std::path::Prefix` | `.Disk` | `std::path::Prefix::Disk` | tools/rust/library/std/src/path.rs:182 |
| `std::path::Prefix::DeviceNS` | `[0]` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:168 |
| `std::path::Prefix::Disk` | `[0]` | `u8` | tools/rust/library/std/src/path.rs:182 |
| `std::path::Prefix::UNC` | `[0]` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:176 |
| `std::path::Prefix::UNC` | `[1]` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:177 |
| `std::path::Prefix::Verbatim` | `[0]` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:143 |
| `std::path::Prefix::VerbatimDisk` | `[0]` | `u8` | tools/rust/library/std/src/path.rs:161 |
| `std::path::Prefix::VerbatimUNC` | `[0]` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:152 |
| `std::path::Prefix::VerbatimUNC` | `[1]` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:153 |
| `std::path::PrefixComponent` | `.raw` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/path.rs:411 |
| `std::path::PrefixComponent` | `.parsed` | `std::path::Prefix` | tools/rust/library/std/src/path.rs:414 |
| `std::path::StripPrefixError` | `[0]` | `()` | tools/rust/library/std/src/path.rs:2141 |
| `std::process::Child` | `.handle` | `sys::process::Process` | tools/rust/library/std/src/process.rs:223 |
| `std::process::Child` | `.stdin` | `Option<std::process::ChildStdin>` | tools/rust/library/std/src/process.rs:235 |
| `std::process::Child` | `.stdout` | `Option<std::process::ChildStdout>` | tools/rust/library/std/src/process.rs:247 |
| `std::process::Child` | `.stderr` | `Option<std::process::ChildStderr>` | tools/rust/library/std/src/process.rs:259 |
| `std::process::ChildStderr` | `.inner` | `sys::pipe::AnonPipe` | tools/rust/library/std/src/process.rs:469 |
| `std::process::ChildStdin` | `.inner` | `sys::pipe::AnonPipe` | tools/rust/library/std/src/process.rs:313 |
| `std::process::ChildStdout` | `.inner` | `sys::pipe::AnonPipe` | tools/rust/library/std/src/process.rs:399 |
| `std::process::Command` | `.inner` | `sys::process::Command` | tools/rust/library/std/src/process.rs:589 |
| `std::process::CommandArgs` | `.inner` | `sys::process::CommandArgs` | tools/rust/library/std/src/process.rs:1243 |
| `std::process::ExitCode` | `[0]` | `sys::process::ExitCode` | tools/rust/library/std/src/process.rs:1993 |
| `std::process::ExitStatus` | `[0]` | `sys::process::ExitStatus` | tools/rust/library/std/src/process.rs:1707 |
| `std::process::ExitStatusError` | `[0]` | `sys::process::ExitStatusError` | tools/rust/library/std/src/process.rs:1852 |
| `std::process::Output` | `.status` | `std::process::ExitStatus` | tools/rust/library/std/src/process.rs:1280 |
| `std::process::Output` | `.stdout` | `Vec<u8>` | tools/rust/library/std/src/process.rs:1283 |
| `std::process::Output` | `.stderr` | `Vec<u8>` | tools/rust/library/std/src/process.rs:1286 |
| `std::process::Stdio` | `[0]` | `sys::process::Stdio` | tools/rust/library/std/src/process.rs:1321 |
| `std::sync::barrier::Barrier` | `.lock` | `std::sync::poison::mutex::Mutex<std::sync::barrier::BarrierState>` | tools/rust/library/std/src/sync/barrier.rs:30 |
| `std::sync::barrier::Barrier` | `.cvar` | `std::sync::poison::condvar::Condvar` | tools/rust/library/std/src/sync/barrier.rs:31 |
| `std::sync::barrier::Barrier` | `.num_threads` | `usize` | tools/rust/library/std/src/sync/barrier.rs:32 |
| `std::sync::barrier::BarrierState` | `.count` | `usize` | tools/rust/library/std/src/sync/barrier.rs:37 |
| `std::sync::barrier::BarrierState` | `.generation_id` | `usize` | tools/rust/library/std/src/sync/barrier.rs:38 |
| `std::sync::barrier::BarrierWaitResult` | `[0]` | `bool` | tools/rust/library/std/src/sync/barrier.rs:53 |
| `std::sync::lazy_lock::LazyLock` | `.once` | `std::sync::poison::once::Once` | tools/rust/library/std/src/sync/lazy_lock.rs:67 |
| `std::sync::lazy_lock::LazyLock` | `.data` | `cell::UnsafeCell<Data<$T, $F>>` | tools/rust/library/std/src/sync/lazy_lock.rs:68 |
| `std::sync::mpmc::IntoIter` | `.rx` | `std::sync::mpmc::Receiver<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:814 |
| `std::sync::mpmc::Iter` | `.rx` | `std::sync::mpmc::Receiver<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:733 |
| `std::sync::mpmc::Receiver` | `.flavor` | `std::sync::mpmc::ReceiverFlavor<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:698 |
| `std::sync::mpmc::ReceiverFlavor` | `.Array` | `std::sync::mpmc::ReceiverFlavor::Array<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:866 |
| `std::sync::mpmc::ReceiverFlavor` | `.List` | `std::sync::mpmc::ReceiverFlavor::List<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:869 |
| `std::sync::mpmc::ReceiverFlavor` | `.Zero` | `std::sync::mpmc::ReceiverFlavor::Zero<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:872 |
| `std::sync::mpmc::ReceiverFlavor::Array` | `[0]` | `std::sync::mpmc::counter::Receiver<std::sync::mpmc::array::Channel<$T>>` | tools/rust/library/std/src/sync/mpmc/mod.rs:866 |
| `std::sync::mpmc::ReceiverFlavor::List` | `[0]` | `std::sync::mpmc::counter::Receiver<std::sync::mpmc::list::Channel<$T>>` | tools/rust/library/std/src/sync/mpmc/mod.rs:869 |
| `std::sync::mpmc::ReceiverFlavor::Zero` | `[0]` | `std::sync::mpmc::counter::Receiver<std::sync::mpmc::zero::Channel<$T>>` | tools/rust/library/std/src/sync/mpmc/mod.rs:872 |
| `std::sync::mpmc::Sender` | `.flavor` | `std::sync::mpmc::SenderFlavor<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:300 |
| `std::sync::mpmc::SenderFlavor` | `.Array` | `std::sync::mpmc::SenderFlavor::Array<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:306 |
| `std::sync::mpmc::SenderFlavor` | `.List` | `std::sync::mpmc::SenderFlavor::List<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:309 |
| `std::sync::mpmc::SenderFlavor` | `.Zero` | `std::sync::mpmc::SenderFlavor::Zero<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:312 |
| `std::sync::mpmc::SenderFlavor::Array` | `[0]` | `std::sync::mpmc::counter::Sender<std::sync::mpmc::array::Channel<$T>>` | tools/rust/library/std/src/sync/mpmc/mod.rs:306 |
| `std::sync::mpmc::SenderFlavor::List` | `[0]` | `std::sync::mpmc::counter::Sender<std::sync::mpmc::list::Channel<$T>>` | tools/rust/library/std/src/sync/mpmc/mod.rs:309 |
| `std::sync::mpmc::SenderFlavor::Zero` | `[0]` | `std::sync::mpmc::counter::Sender<std::sync::mpmc::zero::Channel<$T>>` | tools/rust/library/std/src/sync/mpmc/mod.rs:312 |
| `std::sync::mpmc::TryIter` | `.rx` | `std::sync::mpmc::Receiver<$T>` | tools/rust/library/std/src/sync/mpmc/mod.rs:778 |
| `std::sync::mpmc::array::ArrayToken` | `.slot` | `*const u8` | tools/rust/library/std/src/sync/mpmc/array.rs:36 |
| `std::sync::mpmc::array::ArrayToken` | `.stamp` | `usize` | tools/rust/library/std/src/sync/mpmc/array.rs:39 |
| `std::sync::mpmc::array::Channel` | `.head` | `std::sync::mpmc::utils::CachePadded<sync::atomic::AtomicUsize>` | tools/rust/library/std/src/sync/mpmc/array.rs:58 |
| `std::sync::mpmc::array::Channel` | `.tail` | `std::sync::mpmc::utils::CachePadded<sync::atomic::AtomicUsize>` | tools/rust/library/std/src/sync/mpmc/array.rs:67 |
| `std::sync::mpmc::array::Channel` | `.buffer` | `Box<[std::sync::mpmc::array::Slot<$T>]>` | tools/rust/library/std/src/sync/mpmc/array.rs:70 |
| `std::sync::mpmc::array::Channel` | `.cap` | `usize` | tools/rust/library/std/src/sync/mpmc/array.rs:73 |
| `std::sync::mpmc::array::Channel` | `.one_lap` | `usize` | tools/rust/library/std/src/sync/mpmc/array.rs:76 |
| `std::sync::mpmc::array::Channel` | `.mark_bit` | `usize` | tools/rust/library/std/src/sync/mpmc/array.rs:79 |
| `std::sync::mpmc::array::Channel` | `.senders` | `std::sync::mpmc::waker::SyncWaker` | tools/rust/library/std/src/sync/mpmc/array.rs:82 |
| `std::sync::mpmc::array::Channel` | `.receivers` | `std::sync::mpmc::waker::SyncWaker` | tools/rust/library/std/src/sync/mpmc/array.rs:85 |
| `std::sync::mpmc::array::Slot` | `.stamp` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sync/mpmc/array.rs:25 |
| `std::sync::mpmc::array::Slot` | `.msg` | `cell::UnsafeCell<mem::MaybeUninit<$T>>` | tools/rust/library/std/src/sync/mpmc/array.rs:29 |
| `std::sync::mpmc::context::Context` | `.inner` | `sync::Arc<std::sync::mpmc::context::Inner>` | tools/rust/library/std/src/sync/mpmc/context.rs:15 |
| `std::sync::mpmc::context::Inner` | `.select` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sync/mpmc/context.rs:22 |
| `std::sync::mpmc::context::Inner` | `.packet` | `sync::atomic::AtomicPtr<()>` | tools/rust/library/std/src/sync/mpmc/context.rs:25 |
| `std::sync::mpmc::context::Inner` | `.thread` | `std::thread::Thread` | tools/rust/library/std/src/sync/mpmc/context.rs:28 |
| `std::sync::mpmc::context::Inner` | `.thread_id` | `usize` | tools/rust/library/std/src/sync/mpmc/context.rs:31 |
| `std::sync::mpmc::counter::Counter` | `.senders` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sync/mpmc/counter.rs:7 |
| `std::sync::mpmc::counter::Counter` | `.receivers` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sync/mpmc/counter.rs:10 |
| `std::sync::mpmc::counter::Counter` | `.destroy` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sync/mpmc/counter.rs:13 |
| `std::sync::mpmc::counter::Counter` | `.chan` | `$C` | tools/rust/library/std/src/sync/mpmc/counter.rs:16 |
| `std::sync::mpmc::counter::Receiver` | `.counter` | `*mut std::sync::mpmc::counter::Counter<$C>` | tools/rust/library/std/src/sync/mpmc/counter.rs:87 |
| `std::sync::mpmc::counter::Sender` | `.counter` | `*mut std::sync::mpmc::counter::Counter<$C>` | tools/rust/library/std/src/sync/mpmc/counter.rs:34 |
| `std::sync::mpmc::error::SendTimeoutError` | `.Timeout` | `std::sync::mpmc::error::SendTimeoutError::Timeout<$T>` | tools/rust/library/std/src/sync/mpmc/error.rs:16 |
| `std::sync::mpmc::error::SendTimeoutError` | `.Disconnected` | `std::sync::mpmc::error::SendTimeoutError::Disconnected<$T>` | tools/rust/library/std/src/sync/mpmc/error.rs:19 |
| `std::sync::mpmc::error::SendTimeoutError::Disconnected` | `[0]` | `$T` | tools/rust/library/std/src/sync/mpmc/error.rs:19 |
| `std::sync::mpmc::error::SendTimeoutError::Timeout` | `[0]` | `$T` | tools/rust/library/std/src/sync/mpmc/error.rs:16 |
| `std::sync::mpmc::list::Block` | `.next` | `sync::atomic::AtomicPtr<std::sync::mpmc::list::Block<$T>>` | tools/rust/library/std/src/sync/mpmc/list.rs:58 |
| `std::sync::mpmc::list::Block` | `.slots` | `[std::sync::mpmc::list::Slot<$T>; BLOCK_CAP]` | tools/rust/library/std/src/sync/mpmc/list.rs:61 |
| `std::sync::mpmc::list::Channel` | `.head` | `std::sync::mpmc::utils::CachePadded<std::sync::mpmc::list::Position<$T>>` | tools/rust/library/std/src/sync/mpmc/list.rs:145 |
| `std::sync::mpmc::list::Channel` | `.tail` | `std::sync::mpmc::utils::CachePadded<std::sync::mpmc::list::Position<$T>>` | tools/rust/library/std/src/sync/mpmc/list.rs:148 |
| `std::sync::mpmc::list::Channel` | `.receivers` | `std::sync::mpmc::waker::SyncWaker` | tools/rust/library/std/src/sync/mpmc/list.rs:151 |
| `std::sync::mpmc::list::Channel` | `._marker` | `marker::PhantomData<$T>` | tools/rust/library/std/src/sync/mpmc/list.rs:154 |
| `std::sync::mpmc::list::ListToken` | `.block` | `*const u8` | tools/rust/library/std/src/sync/mpmc/list.rs:123 |
| `std::sync::mpmc::list::ListToken` | `.offset` | `usize` | tools/rust/library/std/src/sync/mpmc/list.rs:126 |
| `std::sync::mpmc::list::Position` | `.index` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sync/mpmc/list.rs:113 |
| `std::sync::mpmc::list::Position` | `.block` | `sync::atomic::AtomicPtr<std::sync::mpmc::list::Block<$T>>` | tools/rust/library/std/src/sync/mpmc/list.rs:116 |
| `std::sync::mpmc::list::Slot` | `.msg` | `cell::UnsafeCell<mem::MaybeUninit<$T>>` | tools/rust/library/std/src/sync/mpmc/list.rs:37 |
| `std::sync::mpmc::list::Slot` | `.state` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sync/mpmc/list.rs:40 |
| `std::sync::mpmc::select::Operation` | `[0]` | `usize` | tools/rust/library/std/src/sync/mpmc/select.rs:15 |
| `std::sync::mpmc::select::Selected` | `.Operation` | `std::sync::mpmc::select::Selected::Operation` | tools/rust/library/std/src/sync/mpmc/select.rs:46 |
| `std::sync::mpmc::select::Selected::Operation` | `[0]` | `std::sync::mpmc::select::Operation` | tools/rust/library/std/src/sync/mpmc/select.rs:46 |
| `std::sync::mpmc::select::Token` | `.array` | `std::sync::mpmc::array::ArrayToken` | tools/rust/library/std/src/sync/mpmc/select.rs:7 |
| `std::sync::mpmc::select::Token` | `.list` | `std::sync::mpmc::list::ListToken` | tools/rust/library/std/src/sync/mpmc/select.rs:8 |
| `std::sync::mpmc::select::Token` | `.zero` | `std::sync::mpmc::zero::ZeroToken` | tools/rust/library/std/src/sync/mpmc/select.rs:10 |
| `std::sync::mpmc::utils::Backoff` | `.step` | `cell::Cell<u32>` | tools/rust/library/std/src/sync/mpmc/utils.rs:101 |
| `std::sync::mpmc::utils::CachePadded` | `.value` | `$T` | tools/rust/library/std/src/sync/mpmc/utils.rs:73 |
| `std::sync::mpmc::waker::Entry` | `.oper` | `std::sync::mpmc::select::Operation` | tools/rust/library/std/src/sync/mpmc/waker.rs:12 |
| `std::sync::mpmc::waker::Entry` | `.packet` | `*mut ()` | tools/rust/library/std/src/sync/mpmc/waker.rs:15 |
| `std::sync::mpmc::waker::Entry` | `.cx` | `std::sync::mpmc::context::Context` | tools/rust/library/std/src/sync/mpmc/waker.rs:18 |
| `std::sync::mpmc::waker::SyncWaker` | `.inner` | `std::sync::poison::mutex::Mutex<std::sync::mpmc::waker::Waker>` | tools/rust/library/std/src/sync/mpmc/waker.rs:137 |
| `std::sync::mpmc::waker::SyncWaker` | `.is_empty` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sync/mpmc/waker.rs:140 |
| `std::sync::mpmc::waker::Waker` | `.selectors` | `Vec<std::sync::mpmc::waker::Entry>` | tools/rust/library/std/src/sync/mpmc/waker.rs:27 |
| `std::sync::mpmc::waker::Waker` | `.observers` | `Vec<std::sync::mpmc::waker::Entry>` | tools/rust/library/std/src/sync/mpmc/waker.rs:30 |
| `std::sync::mpmc::zero::Channel` | `.inner` | `std::sync::poison::mutex::Mutex<std::sync::mpmc::zero::Inner>` | tools/rust/library/std/src/sync/mpmc/zero.rs:79 |
| `std::sync::mpmc::zero::Channel` | `._marker` | `marker::PhantomData<$T>` | tools/rust/library/std/src/sync/mpmc/zero.rs:82 |
| `std::sync::mpmc::zero::Inner` | `.senders` | `std::sync::mpmc::waker::Waker` | tools/rust/library/std/src/sync/mpmc/zero.rs:67 |
| `std::sync::mpmc::zero::Inner` | `.receivers` | `std::sync::mpmc::waker::Waker` | tools/rust/library/std/src/sync/mpmc/zero.rs:70 |
| `std::sync::mpmc::zero::Inner` | `.is_disconnected` | `bool` | tools/rust/library/std/src/sync/mpmc/zero.rs:73 |
| `std::sync::mpmc::zero::Packet` | `.on_stack` | `bool` | tools/rust/library/std/src/sync/mpmc/zero.rs:35 |
| `std::sync::mpmc::zero::Packet` | `.ready` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sync/mpmc/zero.rs:38 |
| `std::sync::mpmc::zero::Packet` | `.msg` | `cell::UnsafeCell<Option<$T>>` | tools/rust/library/std/src/sync/mpmc/zero.rs:41 |
| `std::sync::mpmc::zero::ZeroToken` | `[0]` | `*mut ()` | tools/rust/library/std/src/sync/mpmc/zero.rs:18 |
| `std::sync::mpsc::IntoIter` | `.rx` | `std::sync::mpsc::Receiver<$T>` | tools/rust/library/std/src/sync/mpsc.rs:296 |
| `std::sync::mpsc::Iter` | `.rx` | `std::sync::mpsc::Receiver<$T>` | tools/rust/library/std/src/sync/mpsc.rs:219 |
| `std::sync::mpsc::Receiver` | `.inner` | `std::sync::mpmc::Receiver<$T>` | tools/rust/library/std/src/sync/mpsc.rs:178 |
| `std::sync::mpsc::SendError` | `[0]` | `$T` | tools/rust/library/std/src/sync/mpsc.rs:407 |
| `std::sync::mpsc::Sender` | `.inner` | `std::sync::mpmc::Sender<$T>` | tools/rust/library/std/src/sync/mpsc.rs:334 |
| `std::sync::mpsc::SyncSender` | `.inner` | `std::sync::mpmc::Sender<$T>` | tools/rust/library/std/src/sync/mpsc.rs:393 |
| `std::sync::mpsc::TryIter` | `.rx` | `std::sync::mpsc::Receiver<$T>` | tools/rust/library/std/src/sync/mpsc.rs:262 |
| `std::sync::mpsc::TrySendError` | `.Full` | `std::sync::mpsc::TrySendError::Full<$T>` | tools/rust/library/std/src/sync/mpsc.rs:471 |
| `std::sync::mpsc::TrySendError` | `.Disconnected` | `std::sync::mpsc::TrySendError::Disconnected<$T>` | tools/rust/library/std/src/sync/mpsc.rs:476 |
| `std::sync::mpsc::TrySendError::Disconnected` | `[0]` | `$T` | tools/rust/library/std/src/sync/mpsc.rs:476 |
| `std::sync::mpsc::TrySendError::Full` | `[0]` | `$T` | tools/rust/library/std/src/sync/mpsc.rs:471 |
| `std::sync::once_lock::OnceLock` | `.once` | `std::sync::poison::once::Once` | tools/rust/library/std/src/sync/once_lock.rs:108 |
| `std::sync::once_lock::OnceLock` | `.value` | `cell::UnsafeCell<mem::MaybeUninit<$T>>` | tools/rust/library/std/src/sync/once_lock.rs:110 |
| `std::sync::once_lock::OnceLock` | `._marker` | `marker::PhantomData<$T>` | tools/rust/library/std/src/sync/once_lock.rs:128 |
| `std::sync::poison::Flag` | `.failed` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sync/poison.rs:91 |
| `std::sync::poison::Guard` | `.panicking` | `bool` | tools/rust/library/std/src/sync/poison.rs:164 |
| `std::sync::poison::LockResult` | `(alias)` | `Result<$T, std::sync::poison::PoisonError<$T>>` | tools/rust/library/std/src/sync/poison.rs:239 |
| `std::sync::poison::PoisonError` | `.data` | `$T` | tools/rust/library/std/src/sync/poison.rs:202 |
| `std::sync::poison::PoisonError` | `._never` | `opaque(!)` | tools/rust/library/std/src/sync/poison.rs:204 |
| `std::sync::poison::TryLockError` | `.Poisoned` | `std::sync::poison::TryLockError::Poisoned<$T>` | tools/rust/library/std/src/sync/poison.rs:221 |
| `std::sync::poison::TryLockError::Poisoned` | `[0]` | `std::sync::poison::PoisonError<$T>` | tools/rust/library/std/src/sync/poison.rs:221 |
| `std::sync::poison::TryLockResult` | `(alias)` | `Result<$Guard, std::sync::poison::TryLockError<$Guard>>` | tools/rust/library/std/src/sync/poison.rs:247 |
| `std::sync::poison::condvar::Condvar` | `.inner` | `sys::sync::Condvar` | tools/rust/library/std/src/sync/poison/condvar.rs:110 |
| `std::sync::poison::condvar::WaitTimeoutResult` | `[0]` | `bool` | tools/rust/library/std/src/sync/poison/condvar.rs:14 |
| `std::sync::poison::mutex::MappedMutexGuard` | `.data` | `ptr::NonNull<$T>` | tools/rust/library/std/src/sync/poison/mutex.rs:273 |
| `std::sync::poison::mutex::MappedMutexGuard` | `.inner` | `sys::sync::Mutex` | tools/rust/library/std/src/sync/poison/mutex.rs:274 |
| `std::sync::poison::mutex::MappedMutexGuard` | `.poison_flag` | `std::sync::poison::Flag` | tools/rust/library/std/src/sync/poison/mutex.rs:275 |
| `std::sync::poison::mutex::MappedMutexGuard` | `.poison` | `std::sync::poison::Guard` | tools/rust/library/std/src/sync/poison/mutex.rs:276 |
| `std::sync::poison::mutex::MappedMutexGuard` | `._variance` | `marker::PhantomData<$T>` | tools/rust/library/std/src/sync/poison/mutex.rs:277 |
| `std::sync::poison::mutex::Mutex` | `.inner` | `sys::sync::Mutex` | tools/rust/library/std/src/sync/poison/mutex.rs:176 |
| `std::sync::poison::mutex::Mutex` | `.poison` | `std::sync::poison::Flag` | tools/rust/library/std/src/sync/poison/mutex.rs:177 |
| `std::sync::poison::mutex::Mutex` | `.data` | `cell::UnsafeCell<$T>` | tools/rust/library/std/src/sync/poison/mutex.rs:178 |
| `std::sync::poison::mutex::MutexGuard` | `.lock` | `std::sync::poison::mutex::Mutex<$T>` | tools/rust/library/std/src/sync/poison/mutex.rs:226 |
| `std::sync::poison::mutex::MutexGuard` | `.poison` | `std::sync::poison::Guard` | tools/rust/library/std/src/sync/poison/mutex.rs:227 |
| `std::sync::poison::once::Once` | `.inner` | `sys::sync::Once` | tools/rust/library/std/src/sync/poison/once.rs:36 |
| `std::sync::poison::once::OnceState` | `.inner` | `sys::sync::OnceState` | tools/rust/library/std/src/sync/poison/once.rs:49 |
| `std::sync::poison::rwlock::MappedRwLockReadGuard` | `.data` | `ptr::NonNull<$T>` | tools/rust/library/std/src/sync/poison/rwlock.rs:166 |
| `std::sync::poison::rwlock::MappedRwLockReadGuard` | `.inner_lock` | `sys::sync::RwLock` | tools/rust/library/std/src/sync/poison/rwlock.rs:167 |
| `std::sync::poison::rwlock::MappedRwLockWriteGuard` | `.data` | `ptr::NonNull<$T>` | tools/rust/library/std/src/sync/poison/rwlock.rs:195 |
| `std::sync::poison::rwlock::MappedRwLockWriteGuard` | `.inner_lock` | `sys::sync::RwLock` | tools/rust/library/std/src/sync/poison/rwlock.rs:196 |
| `std::sync::poison::rwlock::MappedRwLockWriteGuard` | `.poison_flag` | `std::sync::poison::Flag` | tools/rust/library/std/src/sync/poison/rwlock.rs:197 |
| `std::sync::poison::rwlock::MappedRwLockWriteGuard` | `.poison` | `std::sync::poison::Guard` | tools/rust/library/std/src/sync/poison/rwlock.rs:198 |
| `std::sync::poison::rwlock::MappedRwLockWriteGuard` | `._variance` | `marker::PhantomData<$T>` | tools/rust/library/std/src/sync/poison/rwlock.rs:199 |
| `std::sync::poison::rwlock::RwLock` | `.inner` | `sys::sync::RwLock` | tools/rust/library/std/src/sync/poison/rwlock.rs:81 |
| `std::sync::poison::rwlock::RwLock` | `.poison` | `std::sync::poison::Flag` | tools/rust/library/std/src/sync/poison/rwlock.rs:82 |
| `std::sync::poison::rwlock::RwLock` | `.data` | `cell::UnsafeCell<$T>` | tools/rust/library/std/src/sync/poison/rwlock.rs:83 |
| `std::sync::poison::rwlock::RwLockReadGuard` | `.data` | `ptr::NonNull<$T>` | tools/rust/library/std/src/sync/poison/rwlock.rs:111 |
| `std::sync::poison::rwlock::RwLockReadGuard` | `.inner_lock` | `sys::sync::RwLock` | tools/rust/library/std/src/sync/poison/rwlock.rs:112 |
| `std::sync::poison::rwlock::RwLockWriteGuard` | `.lock` | `std::sync::poison::rwlock::RwLock<$T>` | tools/rust/library/std/src/sync/poison/rwlock.rs:137 |
| `std::sync::poison::rwlock::RwLockWriteGuard` | `.poison` | `std::sync::poison::Guard` | tools/rust/library/std/src/sync/poison/rwlock.rs:138 |
| `std::sync::reentrant_lock::ReentrantLock` | `.mutex` | `sys::sync::Mutex` | tools/rust/library/std/src/sync/reentrant_lock.rs:84 |
| `std::sync::reentrant_lock::ReentrantLock` | `.owner` | `Tid` | tools/rust/library/std/src/sync/reentrant_lock.rs:85 |
| `std::sync::reentrant_lock::ReentrantLock` | `.lock_count` | `cell::UnsafeCell<u32>` | tools/rust/library/std/src/sync/reentrant_lock.rs:86 |
| `std::sync::reentrant_lock::ReentrantLock` | `.data` | `$T` | tools/rust/library/std/src/sync/reentrant_lock.rs:87 |
| `std::sync::reentrant_lock::ReentrantLockGuard` | `.lock` | `std::sync::reentrant_lock::ReentrantLock<$T>` | tools/rust/library/std/src/sync/reentrant_lock.rs:207 |
| `std::sys::alloc::windows::Header` | `[0]` | `*mut u8` | tools/rust/library/std/src/sys/alloc/windows.rs:105 |
| `std::sys::anonymous_pipe::unix::AnonPipe` | `(alias)` | `sys::fd::FileDesc` | tools/rust/library/std/src/sys/anonymous_pipe/unix.rs:6 |
| `std::sys::anonymous_pipe::windows::AnonPipe` | `(alias)` | `sys::handle::Handle` | tools/rust/library/std/src/sys/anonymous_pipe/windows.rs:6 |
| `std::sys::backtrace::BacktraceLock` | `[0]` | `std::sync::poison::mutex::MutexGuard<()>` | tools/rust/library/std/src/sys/backtrace.rs:14 |
| `std::sys::fs::hermit::DirBuilder` | `.mode` | `u32` | tools/rust/library/std/src/sys/fs/hermit.rs:106 |
| `std::sys::fs::hermit::DirEntry` | `.root` | `std::path::PathBuf` | tools/rust/library/std/src/sys/fs/hermit.rs:57 |
| `std::sys::fs::hermit::DirEntry` | `.ino` | `u64` | tools/rust/library/std/src/sys/fs/hermit.rs:59 |
| `std::sys::fs::hermit::DirEntry` | `.type_` | `u8` | tools/rust/library/std/src/sys/fs/hermit.rs:61 |
| `std::sys::fs::hermit::DirEntry` | `.name` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/sys/fs/hermit.rs:63 |
| `std::sys::fs::hermit::File` | `[0]` | `sys::pal::fd::FileDesc` | tools/rust/library/std/src/sys/fs/hermit.rs:20 |
| `std::sys::fs::hermit::FileAttr` | `.stat_val` | `os::hermit::hermit_abi::stat` | tools/rust/library/std/src/sys/fs/hermit.rs:23 |
| `std::sys::fs::hermit::FilePermissions` | `.mode` | `u32` | tools/rust/library/std/src/sys/fs/hermit.rs:84 |
| `std::sys::fs::hermit::FileType` | `.mode` | `u8` | tools/rust/library/std/src/sys/fs/hermit.rs:89 |
| `std::sys::fs::hermit::InnerReadDir` | `.root` | `std::path::PathBuf` | tools/rust/library/std/src/sys/fs/hermit.rs:34 |
| `std::sys::fs::hermit::InnerReadDir` | `.dir` | `Vec<u8>` | tools/rust/library/std/src/sys/fs/hermit.rs:35 |
| `std::sys::fs::hermit::OpenOptions` | `.read` | `bool` | tools/rust/library/std/src/sys/fs/hermit.rs:69 |
| `std::sys::fs::hermit::OpenOptions` | `.write` | `bool` | tools/rust/library/std/src/sys/fs/hermit.rs:70 |
| `std::sys::fs::hermit::OpenOptions` | `.append` | `bool` | tools/rust/library/std/src/sys/fs/hermit.rs:71 |
| `std::sys::fs::hermit::OpenOptions` | `.truncate` | `bool` | tools/rust/library/std/src/sys/fs/hermit.rs:72 |
| `std::sys::fs::hermit::OpenOptions` | `.create` | `bool` | tools/rust/library/std/src/sys/fs/hermit.rs:73 |
| `std::sys::fs::hermit::OpenOptions` | `.create_new` | `bool` | tools/rust/library/std/src/sys/fs/hermit.rs:74 |
| `std::sys::fs::hermit::OpenOptions` | `.mode` | `i32` | tools/rust/library/std/src/sys/fs/hermit.rs:76 |
| `std::sys::fs::hermit::ReadDir` | `.inner` | `sync::Arc<std::sys::fs::hermit::InnerReadDir>` | tools/rust/library/std/src/sys/fs/hermit.rs:45 |
| `std::sys::fs::hermit::ReadDir` | `.pos` | `usize` | tools/rust/library/std/src/sys/fs/hermit.rs:46 |
| `std::sys::fs::solid::CIntNotMinusOne` | `(alias)` | `core::num::niche_types::NotAllOnes<os::raw::c_int>` | tools/rust/library/std/src/sys/fs/solid.rs:17 |
| `std::sys::fs::solid::DirEntry` | `.entry` | `sys::pal::abi::dirent` | tools/rust/library/std/src/sys/fs/solid.rs:58 |
| `std::sys::fs::solid::DirEntry` | `.inner` | `sync::Arc<std::sys::fs::solid::InnerReadDir>` | tools/rust/library/std/src/sys/fs/solid.rs:59 |
| `std::sys::fs::solid::File` | `.fd` | `std::sys::fs::solid::FileDesc` | tools/rust/library/std/src/sys/fs/solid.rs:39 |
| `std::sys::fs::solid::FileAttr` | `.stat` | `sys::pal::abi::stat` | tools/rust/library/std/src/sys/fs/solid.rs:44 |
| `std::sys::fs::solid::FileDesc` | `.fd` | `std::sys::fs::solid::CIntNotMinusOne` | tools/rust/library/std/src/sys/fs/solid.rs:22 |
| `std::sys::fs::solid::FilePermissions` | `[0]` | `os::raw::c_short` | tools/rust/library/std/src/sys/fs/solid.rs:79 |
| `std::sys::fs::solid::FileType` | `[0]` | `os::raw::c_short` | tools/rust/library/std/src/sys/fs/solid.rs:82 |
| `std::sys::fs::solid::InnerReadDir` | `.dirp` | `sys::pal::abi::S_DIR` | tools/rust/library/std/src/sys/fs/solid.rs:49 |
| `std::sys::fs::solid::InnerReadDir` | `.root` | `std::path::PathBuf` | tools/rust/library/std/src/sys/fs/solid.rs:50 |
| `std::sys::fs::solid::OpenOptions` | `.read` | `bool` | tools/rust/library/std/src/sys/fs/solid.rs:65 |
| `std::sys::fs::solid::OpenOptions` | `.write` | `bool` | tools/rust/library/std/src/sys/fs/solid.rs:66 |
| `std::sys::fs::solid::OpenOptions` | `.append` | `bool` | tools/rust/library/std/src/sys/fs/solid.rs:67 |
| `std::sys::fs::solid::OpenOptions` | `.truncate` | `bool` | tools/rust/library/std/src/sys/fs/solid.rs:68 |
| `std::sys::fs::solid::OpenOptions` | `.create` | `bool` | tools/rust/library/std/src/sys/fs/solid.rs:69 |
| `std::sys::fs::solid::OpenOptions` | `.create_new` | `bool` | tools/rust/library/std/src/sys/fs/solid.rs:70 |
| `std::sys::fs::solid::OpenOptions` | `.custom_flags` | `i32` | tools/rust/library/std/src/sys/fs/solid.rs:72 |
| `std::sys::fs::solid::ReadDir` | `.inner` | `sync::Arc<std::sys::fs::solid::InnerReadDir>` | tools/rust/library/std/src/sys/fs/solid.rs:54 |
| `std::sys::fs::uefi::DirEntry` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/uefi.rs:24 |
| `std::sys::fs::uefi::File` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/uefi.rs:14 |
| `std::sys::fs::uefi::FileAttr` | `.attr` | `u64` | tools/rust/library/std/src/sys/fs/uefi.rs:18 |
| `std::sys::fs::uefi::FileAttr` | `.size` | `u64` | tools/rust/library/std/src/sys/fs/uefi.rs:19 |
| `std::sys::fs::uefi::FilePermissions` | `[0]` | `bool` | tools/rust/library/std/src/sys/fs/uefi.rs:39 |
| `std::sys::fs::uefi::FileType` | `[0]` | `bool` | tools/rust/library/std/src/sys/fs/uefi.rs:43 |
| `std::sys::fs::uefi::OpenOptions` | `.mode` | `u64` | tools/rust/library/std/src/sys/fs/uefi.rs:28 |
| `std::sys::fs::uefi::OpenOptions` | `.append` | `bool` | tools/rust/library/std/src/sys/fs/uefi.rs:29 |
| `std::sys::fs::uefi::OpenOptions` | `.truncate` | `bool` | tools/rust/library/std/src/sys/fs/uefi.rs:30 |
| `std::sys::fs::uefi::OpenOptions` | `.create_new` | `bool` | tools/rust/library/std/src/sys/fs/uefi.rs:31 |
| `std::sys::fs::uefi::ReadDir` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/uefi.rs:22 |
| `std::sys::fs::uefi::uefi_fs::File` | `[0]` | `ptr::NonNull<r_efi::protocols::file::Protocol>` | tools/rust/library/std/src/sys/fs/uefi.rs:385 |
| `std::sys::fs::unix::Dir` | `[0]` | `*mut libc::DIR` | tools/rust/library/std/src/sys/fs/unix.rs:265 |
| `std::sys::fs::unix::DirBuilder` | `.mode` | `libc::mode_t` | tools/rust/library/std/src/sys/fs/unix.rs:381 |
| `std::sys::fs::unix::File` | `[0]` | `sys::fd::FileDesc` | tools/rust/library/std/src/sys/fs/unix.rs:94 |
| `std::sys::fs::unix::FilePermissions` | `.mode` | `libc::mode_t` | tools/rust/library/std/src/sys/fs/unix.rs:352 |
| `std::sys::fs::unix::FileTimes` | `.accessed` | `Option<sys::time::SystemTime>` | tools/rust/library/std/src/sys/fs/unix.rs:357 |
| `std::sys::fs::unix::FileTimes` | `.modified` | `Option<sys::time::SystemTime>` | tools/rust/library/std/src/sys/fs/unix.rs:358 |
| `std::sys::fs::unix::FileTimes` | `.created` | `Option<sys::time::SystemTime>` | tools/rust/library/std/src/sys/fs/unix.rs:360 |
| `std::sys::fs::unix::FileType` | `.mode` | `libc::mode_t` | tools/rust/library/std/src/sys/fs/unix.rs:365 |
| `std::sys::fs::unix::InnerReadDir` | `.dirp` | `std::sys::fs::unix::Dir` | tools/rust/library/std/src/sys/fs/unix.rs:250 |
| `std::sys::fs::unix::InnerReadDir` | `.root` | `std::path::PathBuf` | tools/rust/library/std/src/sys/fs/unix.rs:251 |
| `std::sys::fs::unix::Mode` | `[0]` | `libc::mode_t` | tools/rust/library/std/src/sys/fs/unix.rs:385 |
| `std::sys::fs::unix::OpenOptions` | `.read` | `bool` | tools/rust/library/std/src/sys/fs/unix.rs:339 |
| `std::sys::fs::unix::OpenOptions` | `.write` | `bool` | tools/rust/library/std/src/sys/fs/unix.rs:340 |
| `std::sys::fs::unix::OpenOptions` | `.append` | `bool` | tools/rust/library/std/src/sys/fs/unix.rs:341 |
| `std::sys::fs::unix::OpenOptions` | `.truncate` | `bool` | tools/rust/library/std/src/sys/fs/unix.rs:342 |
| `std::sys::fs::unix::OpenOptions` | `.create` | `bool` | tools/rust/library/std/src/sys/fs/unix.rs:343 |
| `std::sys::fs::unix::OpenOptions` | `.create_new` | `bool` | tools/rust/library/std/src/sys/fs/unix.rs:344 |
| `std::sys::fs::unix::OpenOptions` | `.custom_flags` | `i32` | tools/rust/library/std/src/sys/fs/unix.rs:346 |
| `std::sys::fs::unix::OpenOptions` | `.mode` | `libc::mode_t` | tools/rust/library/std/src/sys/fs/unix.rs:347 |
| `std::sys::fs::unix::ReadDir` | `.inner` | `sync::Arc<std::sys::fs::unix::InnerReadDir>` | tools/rust/library/std/src/sys/fs/unix.rs:255 |
| `std::sys::fs::unix::ReadDir` | `.end_of_stream` | `bool` | tools/rust/library/std/src/sys/fs/unix.rs:256 |
| `std::sys::fs::unix::cfm::CachedFileMetadata` | `[0]` | `std::fs::File` | tools/rust/library/std/src/sys/fs/unix.rs:2016 |
| `std::sys::fs::unix::cfm::CachedFileMetadata` | `[1]` | `std::fs::Metadata` | tools/rust/library/std/src/sys/fs/unix.rs:2016 |
| `std::sys::fs::unix::dirent64_min` | `.d_ino` | `u64` | tools/rust/library/std/src/sys/fs/unix.rs:307 |
| `std::sys::fs::unix::dirent64_min` | `.d_type` | `u8` | tools/rust/library/std/src/sys/fs/unix.rs:315 |
| `std::sys::fs::unsupported::DirEntry` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/unsupported.rs:15 |
| `std::sys::fs::unsupported::File` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/unsupported.rs:9 |
| `std::sys::fs::unsupported::FileAttr` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/unsupported.rs:11 |
| `std::sys::fs::unsupported::FilePermissions` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/unsupported.rs:23 |
| `std::sys::fs::unsupported::FileType` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/unsupported.rs:25 |
| `std::sys::fs::unsupported::ReadDir` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/fs/unsupported.rs:13 |
| `std::sys::fs::wasi::DirEntry` | `.meta` | `wasi::Dirent` | tools/rust/library/std/src/sys/fs/wasi.rs:57 |
| `std::sys::fs::wasi::DirEntry` | `.name` | `Vec<u8>` | tools/rust/library/std/src/sys/fs/wasi.rs:58 |
| `std::sys::fs::wasi::DirEntry` | `.inner` | `sync::Arc<std::sys::fs::wasi::ReadDirInner>` | tools/rust/library/std/src/sys/fs/wasi.rs:59 |
| `std::sys::fs::wasi::File` | `.fd` | `sys::fd::WasiFd` | tools/rust/library/std/src/sys/fs/wasi.rs:18 |
| `std::sys::fs::wasi::FileAttr` | `.meta` | `wasi::Filestat` | tools/rust/library/std/src/sys/fs/wasi.rs:23 |
| `std::sys::fs::wasi::FilePermissions` | `.readonly` | `bool` | tools/rust/library/std/src/sys/fs/wasi.rs:76 |
| `std::sys::fs::wasi::FileTimes` | `.accessed` | `Option<sys::time::SystemTime>` | tools/rust/library/std/src/sys/fs/wasi.rs:81 |
| `std::sys::fs::wasi::FileTimes` | `.modified` | `Option<sys::time::SystemTime>` | tools/rust/library/std/src/sys/fs/wasi.rs:82 |
| `std::sys::fs::wasi::FileType` | `.bits` | `wasi::Filetype` | tools/rust/library/std/src/sys/fs/wasi.rs:87 |
| `std::sys::fs::wasi::OpenOptions` | `.read` | `bool` | tools/rust/library/std/src/sys/fs/wasi.rs:64 |
| `std::sys::fs::wasi::OpenOptions` | `.write` | `bool` | tools/rust/library/std/src/sys/fs/wasi.rs:65 |
| `std::sys::fs::wasi::OpenOptions` | `.append` | `bool` | tools/rust/library/std/src/sys/fs/wasi.rs:66 |
| `std::sys::fs::wasi::OpenOptions` | `.dirflags` | `wasi::Lookupflags` | tools/rust/library/std/src/sys/fs/wasi.rs:67 |
| `std::sys::fs::wasi::OpenOptions` | `.fdflags` | `wasi::Fdflags` | tools/rust/library/std/src/sys/fs/wasi.rs:68 |
| `std::sys::fs::wasi::OpenOptions` | `.oflags` | `wasi::Oflags` | tools/rust/library/std/src/sys/fs/wasi.rs:69 |
| `std::sys::fs::wasi::OpenOptions` | `.rights_base` | `Option<wasi::Rights>` | tools/rust/library/std/src/sys/fs/wasi.rs:70 |
| `std::sys::fs::wasi::OpenOptions` | `.rights_inheriting` | `Option<wasi::Rights>` | tools/rust/library/std/src/sys/fs/wasi.rs:71 |
| `std::sys::fs::wasi::ReadDir` | `.inner` | `sync::Arc<std::sys::fs::wasi::ReadDirInner>` | tools/rust/library/std/src/sys/fs/wasi.rs:27 |
| `std::sys::fs::wasi::ReadDir` | `.state` | `std::sys::fs::wasi::ReadDirState` | tools/rust/library/std/src/sys/fs/wasi.rs:28 |
| `std::sys::fs::wasi::ReadDirInner` | `.root` | `std::path::PathBuf` | tools/rust/library/std/src/sys/fs/wasi.rs:52 |
| `std::sys::fs::wasi::ReadDirInner` | `.dir` | `std::sys::fs::wasi::File` | tools/rust/library/std/src/sys/fs/wasi.rs:53 |
| `std::sys::fs::wasi::ReadDirState` | `.FillBuffer` | `std::sys::fs::wasi::ReadDirState::FillBuffer` | tools/rust/library/std/src/sys/fs/wasi.rs:33 |
| `std::sys::fs::wasi::ReadDirState` | `.ProcessEntry` | `std::sys::fs::wasi::ReadDirState::ProcessEntry` | tools/rust/library/std/src/sys/fs/wasi.rs:37 |
| `std::sys::fs::wasi::ReadDirState` | `.RunUntilExhaustion` | `std::sys::fs::wasi::ReadDirState::RunUntilExhaustion` | tools/rust/library/std/src/sys/fs/wasi.rs:44 |
| `std::sys::fs::wasi::ReadDirState::FillBuffer` | `.next_read_offset` | `wasi::Dircookie` | tools/rust/library/std/src/sys/fs/wasi.rs:34 |
| `std::sys::fs::wasi::ReadDirState::FillBuffer` | `.buf` | `Vec<u8>` | tools/rust/library/std/src/sys/fs/wasi.rs:35 |
| `std::sys::fs::wasi::ReadDirState::ProcessEntry` | `.buf` | `Vec<u8>` | tools/rust/library/std/src/sys/fs/wasi.rs:38 |
| `std::sys::fs::wasi::ReadDirState::ProcessEntry` | `.next_read_offset` | `Option<wasi::Dircookie>` | tools/rust/library/std/src/sys/fs/wasi.rs:39 |
| `std::sys::fs::wasi::ReadDirState::ProcessEntry` | `.offset` | `usize` | tools/rust/library/std/src/sys/fs/wasi.rs:40 |
| `std::sys::fs::wasi::ReadDirState::RunUntilExhaustion` | `.buf` | `Vec<u8>` | tools/rust/library/std/src/sys/fs/wasi.rs:45 |
| `std::sys::fs::wasi::ReadDirState::RunUntilExhaustion` | `.offset` | `usize` | tools/rust/library/std/src/sys/fs/wasi.rs:46 |
| `std::sys::fs::windows::DirBuff` | `.buffer` | `Box<sys::Align8<[mem::MaybeUninit<u8>; Self :: BUFFER_SIZE]>>` | tools/rust/library/std/src/sys/fs/windows.rs:848 |
| `std::sys::fs::windows::DirBuffIter` | `.buffer` | `Option<[mem::MaybeUninit<u8>]>` | tools/rust/library/std/src/sys/fs/windows.rs:880 |
| `std::sys::fs::windows::DirBuffIter` | `.cursor` | `usize` | tools/rust/library/std/src/sys/fs/windows.rs:881 |
| `std::sys::fs::windows::DirEntry` | `.root` | `sync::Arc<std::path::PathBuf>` | tools/rust/library/std/src/sys/fs/windows.rs:60 |
| `std::sys::fs::windows::DirEntry` | `.data` | `sys::c::WIN32_FIND_DATAW` | tools/rust/library/std/src/sys/fs/windows.rs:61 |
| `std::sys::fs::windows::File` | `.handle` | `sys::handle::Handle` | tools/rust/library/std/src/sys/fs/windows.rs:25 |
| `std::sys::fs::windows::FileAttr` | `.attributes` | `u32` | tools/rust/library/std/src/sys/fs/windows.rs:30 |
| `std::sys::fs::windows::FileAttr` | `.creation_time` | `sys::c::FILETIME` | tools/rust/library/std/src/sys/fs/windows.rs:31 |
| `std::sys::fs::windows::FileAttr` | `.last_access_time` | `sys::c::FILETIME` | tools/rust/library/std/src/sys/fs/windows.rs:32 |
| `std::sys::fs::windows::FileAttr` | `.last_write_time` | `sys::c::FILETIME` | tools/rust/library/std/src/sys/fs/windows.rs:33 |
| `std::sys::fs::windows::FileAttr` | `.change_time` | `Option<sys::c::FILETIME>` | tools/rust/library/std/src/sys/fs/windows.rs:34 |
| `std::sys::fs::windows::FileAttr` | `.file_size` | `u64` | tools/rust/library/std/src/sys/fs/windows.rs:35 |
| `std::sys::fs::windows::FileAttr` | `.reparse_tag` | `u32` | tools/rust/library/std/src/sys/fs/windows.rs:36 |
| `std::sys::fs::windows::FileAttr` | `.volume_serial_number` | `Option<u32>` | tools/rust/library/std/src/sys/fs/windows.rs:37 |
| `std::sys::fs::windows::FileAttr` | `.number_of_links` | `Option<u32>` | tools/rust/library/std/src/sys/fs/windows.rs:38 |
| `std::sys::fs::windows::FileAttr` | `.file_index` | `Option<u64>` | tools/rust/library/std/src/sys/fs/windows.rs:39 |
| `std::sys::fs::windows::FilePermissions` | `.attrs` | `u32` | tools/rust/library/std/src/sys/fs/windows.rs:87 |
| `std::sys::fs::windows::FileTimes` | `.accessed` | `Option<sys::c::FILETIME>` | tools/rust/library/std/src/sys/fs/windows.rs:92 |
| `std::sys::fs::windows::FileTimes` | `.modified` | `Option<sys::c::FILETIME>` | tools/rust/library/std/src/sys/fs/windows.rs:93 |
| `std::sys::fs::windows::FileTimes` | `.created` | `Option<sys::c::FILETIME>` | tools/rust/library/std/src/sys/fs/windows.rs:94 |
| `std::sys::fs::windows::FileType` | `.is_directory` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:44 |
| `std::sys::fs::windows::FileType` | `.is_symlink` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:45 |
| `std::sys::fs::windows::FindNextFileHandle` | `[0]` | `sys::c::HANDLE` | tools/rust/library/std/src/sys/fs/windows.rs:54 |
| `std::sys::fs::windows::OpenOptions` | `.read` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:70 |
| `std::sys::fs::windows::OpenOptions` | `.write` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:71 |
| `std::sys::fs::windows::OpenOptions` | `.append` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:72 |
| `std::sys::fs::windows::OpenOptions` | `.truncate` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:73 |
| `std::sys::fs::windows::OpenOptions` | `.create` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:74 |
| `std::sys::fs::windows::OpenOptions` | `.create_new` | `bool` | tools/rust/library/std/src/sys/fs/windows.rs:75 |
| `std::sys::fs::windows::OpenOptions` | `.custom_flags` | `u32` | tools/rust/library/std/src/sys/fs/windows.rs:77 |
| `std::sys::fs::windows::OpenOptions` | `.access_mode` | `Option<u32>` | tools/rust/library/std/src/sys/fs/windows.rs:78 |
| `std::sys::fs::windows::OpenOptions` | `.attributes` | `u32` | tools/rust/library/std/src/sys/fs/windows.rs:79 |
| `std::sys::fs::windows::OpenOptions` | `.share_mode` | `u32` | tools/rust/library/std/src/sys/fs/windows.rs:80 |
| `std::sys::fs::windows::OpenOptions` | `.security_qos_flags` | `u32` | tools/rust/library/std/src/sys/fs/windows.rs:81 |
| `std::sys::fs::windows::OpenOptions` | `.security_attributes` | `*mut sys::c::SECURITY_ATTRIBUTES` | tools/rust/library/std/src/sys/fs/windows.rs:82 |
| `std::sys::fs::windows::ReadDir` | `.handle` | `Option<std::sys::fs::windows::FindNextFileHandle>` | tools/rust/library/std/src/sys/fs/windows.rs:49 |
| `std::sys::fs::windows::ReadDir` | `.root` | `sync::Arc<std::path::PathBuf>` | tools/rust/library/std/src/sys/fs/windows.rs:50 |
| `std::sys::fs::windows::ReadDir` | `.first` | `Option<sys::c::WIN32_FIND_DATAW>` | tools/rust/library/std/src/sys/fs/windows.rs:51 |
| `std::sys::io::io_slice::iovec::IoSlice` | `.vec` | `sys::pal::abi::sockets::iovec` | tools/rust/library/std/src/sys/io/io_slice/iovec.rs:15 |
| `std::sys::io::io_slice::iovec::IoSlice` | `._p` | `marker::PhantomData<[u8]>` | tools/rust/library/std/src/sys/io/io_slice/iovec.rs:16 |
| `std::sys::io::io_slice::iovec::IoSliceMut` | `.vec` | `sys::pal::abi::sockets::iovec` | tools/rust/library/std/src/sys/io/io_slice/iovec.rs:48 |
| `std::sys::io::io_slice::iovec::IoSliceMut` | `._p` | `marker::PhantomData<[u8]>` | tools/rust/library/std/src/sys/io/io_slice/iovec.rs:49 |
| `std::sys::io::io_slice::unsupported::IoSlice` | `[0]` | `[u8]` | tools/rust/library/std/src/sys/io/io_slice/unsupported.rs:4 |
| `std::sys::io::io_slice::unsupported::IoSliceMut` | `[0]` | `[u8]` | tools/rust/library/std/src/sys/io/io_slice/unsupported.rs:23 |
| `std::sys::io::io_slice::wasi::IoSlice` | `.vec` | `wasi::Ciovec` | tools/rust/library/std/src/sys/io/io_slice/wasi.rs:7 |
| `std::sys::io::io_slice::wasi::IoSlice` | `._p` | `marker::PhantomData<[u8]>` | tools/rust/library/std/src/sys/io/io_slice/wasi.rs:8 |
| `std::sys::io::io_slice::wasi::IoSliceMut` | `.vec` | `wasi::Iovec` | tools/rust/library/std/src/sys/io/io_slice/wasi.rs:37 |
| `std::sys::io::io_slice::wasi::IoSliceMut` | `._p` | `marker::PhantomData<[u8]>` | tools/rust/library/std/src/sys/io/io_slice/wasi.rs:38 |
| `std::sys::io::io_slice::windows::IoSlice` | `.vec` | `sys::c::WSABUF` | tools/rust/library/std/src/sys/io/io_slice/windows.rs:8 |
| `std::sys::io::io_slice::windows::IoSlice` | `._p` | `marker::PhantomData<[u8]>` | tools/rust/library/std/src/sys/io/io_slice/windows.rs:9 |
| `std::sys::io::io_slice::windows::IoSliceMut` | `.vec` | `sys::c::WSABUF` | tools/rust/library/std/src/sys/io/io_slice/windows.rs:42 |
| `std::sys::io::io_slice::windows::IoSliceMut` | `._p` | `marker::PhantomData<[u8]>` | tools/rust/library/std/src/sys/io/io_slice/windows.rs:43 |
| `std::sys::net::connection::sgx::LookupHost` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/sgx.rs:468 |
| `std::sys::net::connection::sgx::NonIpSockAddr` | `.host` | `String` | tools/rust/library/std/src/sys/net/connection/sgx.rs:452 |
| `std::sys::net::connection::sgx::Socket` | `.inner` | `sync::Arc<sys::fd::FileDesc>` | tools/rust/library/std/src/sys/net/connection/sgx.rs:14 |
| `std::sys::net::connection::sgx::Socket` | `.local_addr` | `Option<String>` | tools/rust/library/std/src/sys/net/connection/sgx.rs:15 |
| `std::sys::net::connection::sgx::TcpListener` | `.inner` | `std::sys::net::connection::sgx::Socket` | tools/rust/library/std/src/sys/net/connection/sgx.rs:234 |
| `std::sys::net::connection::sgx::TcpStream` | `.inner` | `std::sys::net::connection::sgx::Socket` | tools/rust/library/std/src/sys/net/connection/sgx.rs:46 |
| `std::sys::net::connection::sgx::TcpStream` | `.peer_addr` | `Option<String>` | tools/rust/library/std/src/sys/net/connection/sgx.rs:47 |
| `std::sys::net::connection::sgx::UdpSocket` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/sgx.rs:316 |
| `std::sys::net::connection::socket::LookupHost` | `.original` | `*mut netc::addrinfo` | tools/rust/library/std/src/sys/net/connection/socket.rs:252 |
| `std::sys::net::connection::socket::LookupHost` | `.cur` | `*mut netc::addrinfo` | tools/rust/library/std/src/sys/net/connection/socket.rs:253 |
| `std::sys::net::connection::socket::LookupHost` | `.port` | `u16` | tools/rust/library/std/src/sys/net/connection/socket.rs:254 |
| `std::sys::net::connection::socket::TcpListener` | `.inner` | `Socket` | tools/rust/library/std/src/sys/net/connection/socket.rs:501 |
| `std::sys::net::connection::socket::TcpStream` | `.inner` | `Socket` | tools/rust/library/std/src/sys/net/connection/socket.rs:331 |
| `std::sys::net::connection::socket::UdpSocket` | `.inner` | `Socket` | tools/rust/library/std/src/sys/net/connection/socket.rs:626 |
| `std::sys::net::connection::socket::hermit::Socket` | `[0]` | `sys::fd::FileDesc` | tools/rust/library/std/src/sys/net/connection/socket/hermit.rs:37 |
| `std::sys::net::connection::socket::hermit::wrlen_t` | `(alias)` | `usize` | tools/rust/library/std/src/sys/net/connection/socket/hermit.rs:19 |
| `std::sys::net::connection::socket::solid::Socket` | `[0]` | `std::os::solid::io::OwnedFd` | tools/rust/library/std/src/sys/net/connection/socket/solid.rs:115 |
| `std::sys::net::connection::socket::solid::wrlen_t` | `(alias)` | `libc::size_t` | tools/rust/library/std/src/sys/net/connection/socket/solid.rs:19 |
| `std::sys::net::connection::socket::unix::Socket` | `[0]` | `sys::fd::FileDesc` | tools/rust/library/std/src/sys/net/connection/socket/unix.rs:30 |
| `std::sys::net::connection::socket::unix::wrlen_t` | `(alias)` | `libc::size_t` | tools/rust/library/std/src/sys/net/connection/socket/unix.rs:28 |
| `std::sys::net::connection::socket::wasip2::Socket` | `[0]` | `std::sys::net::connection::socket::wasip2::WasiSocket` | tools/rust/library/std/src/sys/net/connection/socket/wasip2.rs:74 |
| `std::sys::net::connection::socket::wasip2::WasiSocket` | `[0]` | `os::wasi::io::OwnedFd` | tools/rust/library/std/src/sys/net/connection/socket/wasip2.rs:72 |
| `std::sys::net::connection::socket::wasip2::wrlen_t` | `(alias)` | `libc::size_t` | tools/rust/library/std/src/sys/net/connection/socket/wasip2.rs:17 |
| `std::sys::net::connection::socket::windows::Socket` | `[0]` | `os::windows::io::OwnedSocket` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:115 |
| `std::sys::net::connection::socket::windows::netc::in6_addr` | `.s6_addr` | `[u8; 16]` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:57 |
| `std::sys::net::connection::socket::windows::netc::in_addr` | `.s_addr` | `u32` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:51 |
| `std::sys::net::connection::socket::windows::netc::ip_mreq` | `.imr_multiaddr` | `std::sys::net::connection::socket::windows::netc::in_addr` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:62 |
| `std::sys::net::connection::socket::windows::netc::ip_mreq` | `.imr_interface` | `std::sys::net::connection::socket::windows::netc::in_addr` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:63 |
| `std::sys::net::connection::socket::windows::netc::ipv6_mreq` | `.ipv6mr_multiaddr` | `std::sys::net::connection::socket::windows::netc::in6_addr` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:68 |
| `std::sys::net::connection::socket::windows::netc::ipv6_mreq` | `.ipv6mr_interface` | `core::ffi::c_uint` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:69 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in` | `.sin_family` | `sys::c::ADDRESS_FAMILY` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:75 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in` | `.sin_port` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:76 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in` | `.sin_addr` | `std::sys::net::connection::socket::windows::netc::in_addr` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:77 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in` | `.sin_zero` | `[core::ffi::c_char; 8]` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:78 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in6` | `.sin6_family` | `sys::c::ADDRESS_FAMILY` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:84 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in6` | `.sin6_port` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:85 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in6` | `.sin6_flowinfo` | `core::ffi::c_ulong` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:86 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in6` | `.sin6_addr` | `std::sys::net::connection::socket::windows::netc::in6_addr` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:87 |
| `std::sys::net::connection::socket::windows::netc::sockaddr_in6` | `.sin6_scope_id` | `core::ffi::c_ulong` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:88 |
| `std::sys::net::connection::socket::windows::netc::socklen_t` | `(alias)` | `core::ffi::c_int` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:40 |
| `std::sys::net::connection::socket::windows::wrlen_t` | `(alias)` | `i32` | tools/rust/library/std/src/sys/net/connection/socket/windows.rs:18 |
| `std::sys::net::connection::uefi::LookupHost` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/uefi/mod.rs:305 |
| `std::sys::net::connection::uefi::TcpListener` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/uefi/mod.rs:121 |
| `std::sys::net::connection::uefi::TcpStream` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/uefi/mod.rs:7 |
| `std::sys::net::connection::uefi::UdpSocket` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/uefi/mod.rs:171 |
| `std::sys::net::connection::unsupported::LookupHost` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/unsupported.rs:305 |
| `std::sys::net::connection::unsupported::TcpListener` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/unsupported.rs:121 |
| `std::sys::net::connection::unsupported::TcpStream` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/unsupported.rs:7 |
| `std::sys::net::connection::unsupported::UdpSocket` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/unsupported.rs:171 |
| `std::sys::net::connection::wasip1::LookupHost` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/net/connection/wasip1.rs:478 |
| `std::sys::net::connection::wasip1::Socket` | `[0]` | `sys::fd::WasiFd` | tools/rust/library/std/src/sys/net/connection/wasip1.rs:12 |
| `std::sys::net::connection::wasip1::TcpListener` | `.inner` | `std::sys::net::connection::wasip1::Socket` | tools/rust/library/std/src/sys/net/connection/wasip1.rs:211 |
| `std::sys::net::connection::wasip1::TcpStream` | `.inner` | `std::sys::net::connection::wasip1::Socket` | tools/rust/library/std/src/sys/net/connection/wasip1.rs:15 |
| `std::sys::net::connection::wasip1::UdpSocket` | `.inner` | `std::sys::net::connection::wasip1::Socket` | tools/rust/library/std/src/sys/net/connection/wasip1.rs:315 |
| `std::sys::net::connection::xous::ConnectRequest` | `.raw` | `[u8; 4096]` | tools/rust/library/std/src/sys/net/connection/xous/mod.rs:30 |
| `std::sys::net::connection::xous::GetAddress` | `.raw` | `[u8; 4096]` | tools/rust/library/std/src/sys/net/connection/xous/mod.rs:45 |
| `std::sys::net::connection::xous::ReceiveData` | `.raw` | `[u8; 4096]` | tools/rust/library/std/src/sys/net/connection/xous/mod.rs:40 |
| `std::sys::net::connection::xous::SendData` | `.raw` | `[u8; 4096]` | tools/rust/library/std/src/sys/net/connection/xous/mod.rs:35 |
| `std::sys::net::connection::xous::dns::DnsError` | `.code` | `u8` | tools/rust/library/std/src/sys/net/connection/xous/dns.rs:10 |
| `std::sys::net::connection::xous::dns::LookupHost` | `.data` | `std::sys::net::connection::xous::dns::LookupHostQuery` | tools/rust/library/std/src/sys/net/connection/xous/dns.rs:17 |
| `std::sys::net::connection::xous::dns::LookupHost` | `.port` | `u16` | tools/rust/library/std/src/sys/net/connection/xous/dns.rs:18 |
| `std::sys::net::connection::xous::dns::LookupHost` | `.offset` | `usize` | tools/rust/library/std/src/sys/net/connection/xous/dns.rs:19 |
| `std::sys::net::connection::xous::dns::LookupHost` | `.count` | `usize` | tools/rust/library/std/src/sys/net/connection/xous/dns.rs:20 |
| `std::sys::net::connection::xous::dns::LookupHostQuery` | `[0]` | `[u8; 4096]` | tools/rust/library/std/src/sys/net/connection/xous/dns.rs:14 |
| `std::sys::net::connection::xous::tcplistener::TcpListener` | `.fd` | `sync::Arc<core::sync::atomic::AtomicU16>` | tools/rust/library/std/src/sys/net/connection/xous/tcplistener.rs:21 |
| `std::sys::net::connection::xous::tcplistener::TcpListener` | `.local` | `core::net::socket_addr::SocketAddr` | tools/rust/library/std/src/sys/net/connection/xous/tcplistener.rs:22 |
| `std::sys::net::connection::xous::tcplistener::TcpListener` | `.handle_count` | `sync::Arc<core::sync::atomic::AtomicUsize>` | tools/rust/library/std/src/sys/net/connection/xous/tcplistener.rs:23 |
| `std::sys::net::connection::xous::tcplistener::TcpListener` | `.nonblocking` | `sync::Arc<core::sync::atomic::AtomicBool>` | tools/rust/library/std/src/sys/net/connection/xous/tcplistener.rs:24 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.fd` | `u16` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:27 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.local_port` | `u16` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:28 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.remote_port` | `u16` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:29 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.peer_addr` | `core::net::socket_addr::SocketAddr` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:30 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.read_timeout` | `sync::Arc<core::sync::atomic::AtomicU32>` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:32 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.write_timeout` | `sync::Arc<core::sync::atomic::AtomicU32>` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:34 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.handle_count` | `sync::Arc<core::sync::atomic::AtomicUsize>` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:35 |
| `std::sys::net::connection::xous::tcpstream::TcpStream` | `.nonblocking` | `sync::Arc<core::sync::atomic::AtomicBool>` | tools/rust/library/std/src/sys/net/connection/xous/tcpstream.rs:36 |
| `std::sys::net::connection::xous::udp::UdpSocket` | `.fd` | `u16` | tools/rust/library/std/src/sys/net/connection/xous/udp.rs:23 |
| `std::sys::net::connection::xous::udp::UdpSocket` | `.local` | `core::net::socket_addr::SocketAddr` | tools/rust/library/std/src/sys/net/connection/xous/udp.rs:24 |
| `std::sys::net::connection::xous::udp::UdpSocket` | `.remote` | `cell::Cell<Option<core::net::socket_addr::SocketAddr>>` | tools/rust/library/std/src/sys/net/connection/xous/udp.rs:25 |
| `std::sys::net::connection::xous::udp::UdpSocket` | `.read_timeout` | `cell::Cell<u64>` | tools/rust/library/std/src/sys/net/connection/xous/udp.rs:27 |
| `std::sys::net::connection::xous::udp::UdpSocket` | `.write_timeout` | `cell::Cell<u64>` | tools/rust/library/std/src/sys/net/connection/xous/udp.rs:29 |
| `std::sys::net::connection::xous::udp::UdpSocket` | `.handle_count` | `sync::Arc<core::sync::atomic::AtomicUsize>` | tools/rust/library/std/src/sys/net/connection/xous/udp.rs:30 |
| `std::sys::net::connection::xous::udp::UdpSocket` | `.nonblocking` | `cell::Cell<bool>` | tools/rust/library/std/src/sys/net/connection/xous/udp.rs:31 |
| `std::sys::os_str::bytes::Buf` | `.inner` | `Vec<u8>` | tools/rust/library/std/src/sys/os_str/bytes.rs:20 |
| `std::sys::os_str::bytes::Slice` | `.inner` | `[u8]` | tools/rust/library/std/src/sys/os_str/bytes.rs:25 |
| `std::sys::os_str::wtf8::Buf` | `.inner` | `std::sys_common::wtf8::Wtf8Buf` | tools/rust/library/std/src/sys/os_str/wtf8.rs:15 |
| `std::sys::os_str::wtf8::Slice` | `.inner` | `std::sys_common::wtf8::Wtf8` | tools/rust/library/std/src/sys/os_str/wtf8.rs:20 |
| `std::sys::pal::RawOsError` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/mod.rs:83 |
| `std::sys::pal::hermit::args::Args` | `.iter` | `vec::IntoIter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/hermit/args.rs:34 |
| `std::sys::pal::hermit::fd::FileDesc` | `.fd` | `os::hermit::io::OwnedFd` | tools/rust/library/std/src/sys/pal/hermit/fd.rs:16 |
| `std::sys::pal::hermit::futex::Futex` | `(alias)` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/hermit/futex.rs:7 |
| `std::sys::pal::hermit::futex::Primitive` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/hermit/futex.rs:9 |
| `std::sys::pal::hermit::futex::SmallFutex` | `(alias)` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/hermit/futex.rs:12 |
| `std::sys::pal::hermit::futex::SmallPrimitive` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/hermit/futex.rs:14 |
| `std::sys::pal::hermit::os::Env` | `.iter` | `vec::IntoIter<(std::ffi::os_str::OsString, std::ffi::os_str::OsString)>` | tools/rust/library/std/src/sys/pal/hermit/os.rs:110 |
| `std::sys::pal::hermit::os::EnvStrDebug` | `.slice` | `[(std::ffi::os_str::OsString, std::ffi::os_str::OsString)]` | tools/rust/library/std/src/sys/pal/hermit/os.rs:115 |
| `std::sys::pal::hermit::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/hermit/os.rs:30 |
| `std::sys::pal::hermit::os::SplitPaths` | `[1]` | `marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/hermit/os.rs:30 |
| `std::sys::pal::hermit::thread::Thread` | `.tid` | `std::sys::pal::hermit::thread::Tid` | tools/rust/library/std/src/sys/pal/hermit/thread.rs:13 |
| `std::sys::pal::hermit::thread::Tid` | `(alias)` | `os::hermit::hermit_abi::Tid` | tools/rust/library/std/src/sys/pal/hermit/thread.rs:10 |
| `std::sys::pal::hermit::time::Instant` | `[0]` | `std::sys::pal::hermit::time::Timespec` | tools/rust/library/std/src/sys/pal/hermit/time.rs:105 |
| `std::sys::pal::hermit::time::SystemTime` | `[0]` | `std::sys::pal::hermit::time::Timespec` | tools/rust/library/std/src/sys/pal/hermit/time.rs:200 |
| `std::sys::pal::hermit::time::Timespec` | `.t` | `os::hermit::hermit_abi::timespec` | tools/rust/library/std/src/sys/pal/hermit/time.rs:14 |
| `std::sys::pal::itron::abi::ATR` | `(alias)` | `std::sys::pal::itron::abi::uint_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:40 |
| `std::sys::pal::itron::abi::ER` | `(alias)` | `std::sys::pal::itron::abi::int_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:28 |
| `std::sys::pal::itron::abi::ER_ID` | `(alias)` | `std::sys::pal::itron::abi::int_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:31 |
| `std::sys::pal::itron::abi::EXINF` | `(alias)` | `isize` | tools/rust/library/std/src/sys/pal/itron/abi.rs:70 |
| `std::sys::pal::itron::abi::FLGPTN` | `(alias)` | `std::sys::pal::itron::abi::uint_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:51 |
| `std::sys::pal::itron::abi::ID` | `(alias)` | `std::sys::pal::itron::abi::int_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:7 |
| `std::sys::pal::itron::abi::MODE` | `(alias)` | `std::sys::pal::itron::abi::uint_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:34 |
| `std::sys::pal::itron::abi::PRI` | `(alias)` | `std::sys::pal::itron::abi::int_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:54 |
| `std::sys::pal::itron::abi::RELTIM` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/itron/abi.rs:13 |
| `std::sys::pal::itron::abi::SYSTIM` | `(alias)` | `u64` | tools/rust/library/std/src/sys/pal/itron/abi.rs:25 |
| `std::sys::pal::itron::abi::TASK` | `(alias)` | `Option<opaque(unsafe extern "C" fn (EXINF))>` | tools/rust/library/std/src/sys/pal/itron/abi.rs:73 |
| `std::sys::pal::itron::abi::TMO` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/itron/abi.rs:16 |
| `std::sys::pal::itron::abi::T_CFLG` | `.flgatr` | `std::sys::pal::itron::abi::ATR` | tools/rust/library/std/src/sys/pal/itron/abi.rs:113 |
| `std::sys::pal::itron::abi::T_CFLG` | `.iflgptn` | `std::sys::pal::itron::abi::FLGPTN` | tools/rust/library/std/src/sys/pal/itron/abi.rs:114 |
| `std::sys::pal::itron::abi::T_CMTX` | `.mtxatr` | `std::sys::pal::itron::abi::ATR` | tools/rust/library/std/src/sys/pal/itron/abi.rs:120 |
| `std::sys::pal::itron::abi::T_CMTX` | `.ceilpri` | `std::sys::pal::itron::abi::PRI` | tools/rust/library/std/src/sys/pal/itron/abi.rs:121 |
| `std::sys::pal::itron::abi::T_CSEM` | `.sematr` | `std::sys::pal::itron::abi::ATR` | tools/rust/library/std/src/sys/pal/itron/abi.rs:105 |
| `std::sys::pal::itron::abi::T_CSEM` | `.isemcnt` | `std::sys::pal::itron::abi::uint_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:106 |
| `std::sys::pal::itron::abi::T_CSEM` | `.maxsem` | `std::sys::pal::itron::abi::uint_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:107 |
| `std::sys::pal::itron::abi::T_CTSK` | `.tskatr` | `std::sys::pal::itron::abi::ATR` | tools/rust/library/std/src/sys/pal/itron/abi.rs:127 |
| `std::sys::pal::itron::abi::T_CTSK` | `.exinf` | `std::sys::pal::itron::abi::EXINF` | tools/rust/library/std/src/sys/pal/itron/abi.rs:128 |
| `std::sys::pal::itron::abi::T_CTSK` | `.task` | `std::sys::pal::itron::abi::TASK` | tools/rust/library/std/src/sys/pal/itron/abi.rs:129 |
| `std::sys::pal::itron::abi::T_CTSK` | `.itskpri` | `std::sys::pal::itron::abi::PRI` | tools/rust/library/std/src/sys/pal/itron/abi.rs:130 |
| `std::sys::pal::itron::abi::T_CTSK` | `.stksz` | `usize` | tools/rust/library/std/src/sys/pal/itron/abi.rs:131 |
| `std::sys::pal::itron::abi::T_CTSK` | `.stk` | `*mut u8` | tools/rust/library/std/src/sys/pal/itron/abi.rs:132 |
| `std::sys::pal::itron::abi::bool_t` | `(alias)` | `std::sys::pal::itron::abi::int_t` | tools/rust/library/std/src/sys/pal/itron/abi.rs:4 |
| `std::sys::pal::itron::abi::int_t` | `(alias)` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/itron/abi.rs:2 |
| `std::sys::pal::itron::abi::uint_t` | `(alias)` | `os::raw::c_uint` | tools/rust/library/std/src/sys/pal/itron/abi.rs:3 |
| `std::sys::pal::itron::error::ItronError` | `.er` | `std::sys::pal::itron::abi::ER` | tools/rust/library/std/src/sys/pal/itron/error.rs:8 |
| `std::sys::pal::itron::spin::SpinIdOnceCell` | `.id` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/pal/itron/spin.rs:53 |
| `std::sys::pal::itron::spin::SpinIdOnceCell` | `.spin` | `std::sys::pal::itron::spin::SpinMutex<()>` | tools/rust/library/std/src/sys/pal/itron/spin.rs:54 |
| `std::sys::pal::itron::spin::SpinIdOnceCell` | `.extra` | `cell::UnsafeCell<mem::MaybeUninit<$T>>` | tools/rust/library/std/src/sys/pal/itron/spin.rs:55 |
| `std::sys::pal::itron::spin::SpinMutex` | `.locked` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sys/pal/itron/spin.rs:9 |
| `std::sys::pal::itron::spin::SpinMutex` | `.data` | `cell::UnsafeCell<$T>` | tools/rust/library/std/src/sys/pal/itron/spin.rs:10 |
| `std::sys::pal::itron::thread::Thread` | `.p_inner` | `ptr::NonNull<std::sys::pal::itron::thread::ThreadInner>` | tools/rust/library/std/src/sys/pal/itron/thread.rs:17 |
| `std::sys::pal::itron::thread::Thread` | `.task` | `std::sys::pal::itron::abi::ID` | tools/rust/library/std/src/sys/pal/itron/thread.rs:20 |
| `std::sys::pal::itron::thread::ThreadInner` | `.start` | `cell::UnsafeCell<mem::ManuallyDrop<Box<opaque(dyn FnOnce ())>>>` | tools/rust/library/std/src/sys/pal/itron/thread.rs:34 |
| `std::sys::pal::itron::thread::ThreadInner` | `.lifecycle` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/pal/itron/thread.rs:67 |
| `std::sys::pal::itron::thread_parking::ThreadId` | `(alias)` | `std::sys::pal::itron::abi::ID` | tools/rust/library/std/src/sys/pal/itron/thread_parking.rs:6 |
| `std::sys::pal::itron::time::Instant` | `[0]` | `std::sys::pal::itron::abi::SYSTIM` | tools/rust/library/std/src/sys/pal/itron/time.rs:7 |
| `std::sys::pal::sgx::abi::EntryReturn` | `[0]` | `u64` | tools/rust/library/std/src/sys/pal/sgx/abi/mod.rs:23 |
| `std::sys::pal::sgx::abi::EntryReturn` | `[1]` | `u64` | tools/rust/library/std/src/sys/pal/sgx/abi/mod.rs:23 |
| `std::sys::pal::sgx::abi::panic::SgxPanicOutput` | `[0]` | `Option<std::sys::pal::sgx::abi::usercalls::alloc::UserRef<[u8]>>` | tools/rust/library/std/src/sys/pal/sgx/abi/panic.rs:10 |
| `std::sys::pal::sgx::abi::reloc::Rela` | `.offset` | `$T` | tools/rust/library/std/src/sys/pal/sgx/abi/reloc.rs:8 |
| `std::sys::pal::sgx::abi::reloc::Rela` | `.info` | `$T` | tools/rust/library/std/src/sys/pal/sgx/abi/reloc.rs:9 |
| `std::sys::pal::sgx::abi::reloc::Rela` | `.addend` | `$T` | tools/rust/library/std/src/sys/pal/sgx/abi/reloc.rs:10 |
| `std::sys::pal::sgx::abi::tls::ActiveTls` | `.tls` | `std::sys::pal::sgx::abi::tls::Tls` | tools/rust/library/std/src/sys/pal/sgx/abi/tls/mod.rs:58 |
| `std::sys::pal::sgx::abi::tls::Key` | `[0]` | `core::num::nonzero::NonZero<usize>` | tools/rust/library/std/src/sys/pal/sgx/abi/tls/mod.rs:32 |
| `std::sys::pal::sgx::abi::tls::Tls` | `.data` | `[cell::Cell<*mut u8>; TLS_KEYS]` | tools/rust/library/std/src/sys/pal/sgx/abi/tls/mod.rs:54 |
| `std::sys::pal::sgx::abi::tls::sync_bitset::SyncBitset` | `[0]` | `[sync::atomic::AtomicUsize; TLS_KEYS_BITSET_SIZE]` | tools/rust/library/std/src/sys/pal/sgx/abi/tls/sync_bitset.rs:10 |
| `std::sys::pal::sgx::abi::tls::sync_bitset::SyncBitsetIter` | `.iter` | `iter::Peekable<iter::Enumerate<slice::Iter<sync::atomic::AtomicUsize>>>` | tools/rust/library/std/src/sys/pal/sgx/abi/tls/sync_bitset.rs:61 |
| `std::sys::pal::sgx::abi::tls::sync_bitset::SyncBitsetIter` | `.elem_idx` | `usize` | tools/rust/library/std/src/sys/pal/sgx/abi/tls/sync_bitset.rs:62 |
| `std::sys::pal::sgx::abi::usercalls::alloc::Iter` | `[0]` | `slice::Iter<$T>` | tools/rust/library/std/src/sys/pal/sgx/abi/usercalls/alloc.rs:717 |
| `std::sys::pal::sgx::abi::usercalls::alloc::IterMut` | `[0]` | `slice::IterMut<$T>` | tools/rust/library/std/src/sys/pal/sgx/abi/usercalls/alloc.rs:733 |
| `std::sys::pal::sgx::abi::usercalls::alloc::User` | `[0]` | `ptr::NonNull<std::sys::pal::sgx::abi::usercalls::alloc::UserRef<$T>>` | tools/rust/library/std/src/sys/pal/sgx/abi/usercalls/alloc.rs:186 |
| `std::sys::pal::sgx::abi::usercalls::alloc::UserRef` | `[0]` | `cell::UnsafeCell<$T>` | tools/rust/library/std/src/sys/pal/sgx/abi/usercalls/alloc.rs:179 |
| `std::sys::pal::sgx::abi::usercalls::raw::Register` | `(alias)` | `u64` | tools/rust/library/std/src/sys/pal/sgx/abi/usercalls/raw.rs:43 |
| `std::sys::pal::sgx::abi::usercalls::raw::UsercallReturn` | `[0]` | `u64` | tools/rust/library/std/src/sys/pal/sgx/abi/usercalls/raw.rs:10 |
| `std::sys::pal::sgx::abi::usercalls::raw::UsercallReturn` | `[1]` | `u64` | tools/rust/library/std/src/sys/pal/sgx/abi/usercalls/raw.rs:10 |
| `std::sys::pal::sgx::args::Args` | `[0]` | `slice::Iter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/sgx/args.rs:31 |
| `std::sys::pal::sgx::args::ArgsStore` | `(alias)` | `Vec<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/sgx/args.rs:12 |
| `std::sys::pal::sgx::fd::FileDesc` | `.fd` | `fortanix_sgx_abi::Fd` | tools/rust/library/std/src/sys/pal/sgx/fd.rs:10 |
| `std::sys::pal::sgx::os::Env` | `.iter` | `vec::IntoIter<(std::ffi::os_str::OsString, std::ffi::os_str::OsString)>` | tools/rust/library/std/src/sys/pal/sgx/os.rs:96 |
| `std::sys::pal::sgx::os::EnvStore` | `(alias)` | `std::sync::poison::mutex::Mutex<collections::HashMap<std::ffi::os_str::OsString, std::ffi::os_str::OsString>>` | tools/rust/library/std/src/sys/pal/sgx/os.rs:82 |
| `std::sys::pal::sgx::os::EnvStrDebug` | `.slice` | `[(std::ffi::os_str::OsString, std::ffi::os_str::OsString)]` | tools/rust/library/std/src/sys/pal/sgx/os.rs:101 |
| `std::sys::pal::sgx::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/sgx/os.rs:35 |
| `std::sys::pal::sgx::os::SplitPaths` | `[1]` | `marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/sgx/os.rs:35 |
| `std::sys::pal::sgx::thread::Thread` | `[0]` | `std::sys::pal::sgx::thread::task_queue::JoinHandle` | tools/rust/library/std/src/sys/pal/sgx/thread.rs:10 |
| `std::sys::pal::sgx::thread::task_queue::JoinHandle` | `(alias)` | `std::sys::pal::sgx::thread::wait_notify::Waiter` | tools/rust/library/std/src/sys/pal/sgx/thread.rs:20 |
| `std::sys::pal::sgx::thread::task_queue::JoinNotifier` | `[0]` | `Option<std::sys::pal::sgx::thread::wait_notify::Notifier>` | tools/rust/library/std/src/sys/pal/sgx/thread.rs:22 |
| `std::sys::pal::sgx::thread::task_queue::Task` | `.p` | `Box<opaque(dyn FnOnce () + Send)>` | tools/rust/library/std/src/sys/pal/sgx/thread.rs:31 |
| `std::sys::pal::sgx::thread::task_queue::Task` | `.done` | `std::sys::pal::sgx::thread::task_queue::JoinNotifier` | tools/rust/library/std/src/sys/pal/sgx/thread.rs:32 |
| `std::sys::pal::sgx::thread::wait_notify::Notifier` | `[0]` | `sync::Arc<sys::sync::Parker>` | tools/rust/library/std/src/sys/pal/sgx/thread.rs:66 |
| `std::sys::pal::sgx::thread::wait_notify::Waiter` | `[0]` | `sync::Arc<sys::sync::Parker>` | tools/rust/library/std/src/sys/pal/sgx/thread.rs:77 |
| `std::sys::pal::sgx::thread_parking::ThreadId` | `(alias)` | `fortanix_sgx_abi::Tcs` | tools/rust/library/std/src/sys/pal/sgx/thread_parking.rs:7 |
| `std::sys::pal::sgx::time::Instant` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/sgx/time.rs:5 |
| `std::sys::pal::sgx::time::SystemTime` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/sgx/time.rs:8 |
| `std::sys::pal::sgx::waitqueue::NotifiedTcs` | `.Single` | `std::sys::pal::sgx::waitqueue::NotifiedTcs::Single` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:64 |
| `std::sys::pal::sgx::waitqueue::NotifiedTcs` | `.All` | `std::sys::pal::sgx::waitqueue::NotifiedTcs::All` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:65 |
| `std::sys::pal::sgx::waitqueue::NotifiedTcs::All` | `._count` | `core::num::nonzero::NonZero<usize>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:65 |
| `std::sys::pal::sgx::waitqueue::NotifiedTcs::Single` | `[0]` | `fortanix_sgx_abi::Tcs` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:64 |
| `std::sys::pal::sgx::waitqueue::WaitEntry` | `.tcs` | `fortanix_sgx_abi::Tcs` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:32 |
| `std::sys::pal::sgx::waitqueue::WaitEntry` | `.wake` | `bool` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:34 |
| `std::sys::pal::sgx::waitqueue::WaitGuard` | `.mutex_guard` | `Option<std::sys::pal::sgx::waitqueue::spin_mutex::SpinMutexGuard<std::sys::pal::sgx::waitqueue::WaitVariable<$T>>>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:71 |
| `std::sys::pal::sgx::waitqueue::WaitGuard` | `.notified_tcs` | `std::sys::pal::sgx::waitqueue::NotifiedTcs` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:72 |
| `std::sys::pal::sgx::waitqueue::WaitQueue` | `.inner` | `std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeList<std::sys::pal::sgx::waitqueue::spin_mutex::SpinMutex<std::sys::pal::sgx::waitqueue::WaitEntry>>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:85 |
| `std::sys::pal::sgx::waitqueue::WaitVariable` | `.queue` | `std::sys::pal::sgx::waitqueue::WaitQueue` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:44 |
| `std::sys::pal::sgx::waitqueue::WaitVariable` | `.lock` | `$T` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/mod.rs:45 |
| `std::sys::pal::sgx::waitqueue::spin_mutex::SpinMutex` | `.value` | `cell::UnsafeCell<$T>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/spin_mutex.rs:14 |
| `std::sys::pal::sgx::waitqueue::spin_mutex::SpinMutex` | `.lock` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/spin_mutex.rs:15 |
| `std::sys::pal::sgx::waitqueue::spin_mutex::SpinMutexGuard` | `.mutex` | `std::sys::pal::sgx::waitqueue::spin_mutex::SpinMutex<$T>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/spin_mutex.rs:22 |
| `std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeList` | `.head_tail` | `ptr::NonNull<std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeListEntry<$T>>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/unsafe_list.rs:28 |
| `std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeList` | `.head_tail_entry` | `Option<std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeListEntry<$T>>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/unsafe_list.rs:29 |
| `std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeListEntry` | `.next` | `ptr::NonNull<std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeListEntry<$T>>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/unsafe_list.rs:11 |
| `std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeListEntry` | `.prev` | `ptr::NonNull<std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeListEntry<$T>>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/unsafe_list.rs:12 |
| `std::sys::pal::sgx::waitqueue::unsafe_list::UnsafeListEntry` | `.value` | `Option<$T>` | tools/rust/library/std/src/sys/pal/sgx/waitqueue/unsafe_list.rs:13 |
| `std::sys::pal::solid::abi::SOLID_RTC_TIME` | `.tm_sec` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/mod.rs:27 |
| `std::sys::pal::solid::abi::SOLID_RTC_TIME` | `.tm_min` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/mod.rs:28 |
| `std::sys::pal::solid::abi::SOLID_RTC_TIME` | `.tm_hour` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/mod.rs:29 |
| `std::sys::pal::solid::abi::SOLID_RTC_TIME` | `.tm_mday` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/mod.rs:30 |
| `std::sys::pal::solid::abi::SOLID_RTC_TIME` | `.tm_mon` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/mod.rs:31 |
| `std::sys::pal::solid::abi::SOLID_RTC_TIME` | `.tm_year` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/mod.rs:32 |
| `std::sys::pal::solid::abi::SOLID_RTC_TIME` | `.tm_wday` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/mod.rs:33 |
| `std::sys::pal::solid::abi::fs::S_DIR` | `(alias)` | `os::raw::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/fs.rs:32 |
| `std::sys::pal::solid::abi::fs::dirent` | `.d_ino` | `libc::ino_t` | tools/rust/library/std/src/sys/pal/solid/abi/fs.rs:17 |
| `std::sys::pal::solid::abi::fs::dirent` | `.d_type` | `os::raw::c_uchar` | tools/rust/library/std/src/sys/pal/solid/abi/fs.rs:18 |
| `std::sys::pal::solid::abi::fs::dirent` | `.d_name` | `[os::raw::c_char; 256usize]` | tools/rust/library/std/src/sys/pal/solid/abi/fs.rs:19 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_flags` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:127 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_family` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:128 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_socktype` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:129 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_protocol` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:130 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_addrlen` | `std::sys::pal::solid::abi::sockets::socklen_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:131 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_addr` | `*mut std::sys::pal::solid::abi::sockets::sockaddr` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:132 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_canonname` | `*mut os::raw::c_char` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:133 |
| `std::sys::pal::solid::abi::sockets::addrinfo` | `.ai_next` | `*mut std::sys::pal::solid::abi::sockets::addrinfo` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:134 |
| `std::sys::pal::solid::abi::sockets::fd_set` | `.num_fds` | `usize` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:157 |
| `std::sys::pal::solid::abi::sockets::fd_set` | `.fds` | `[libc::c_int; SOLID_NET_FD_SETSIZE]` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:158 |
| `std::sys::pal::solid::abi::sockets::in6_addr` | `.s6_addr` | `[u8; 16]` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:57 |
| `std::sys::pal::solid::abi::sockets::in_addr` | `.s_addr` | `std::sys::pal::solid::abi::sockets::in_addr_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:51 |
| `std::sys::pal::solid::abi::sockets::in_addr_t` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:45 |
| `std::sys::pal::solid::abi::sockets::in_port_t` | `(alias)` | `u16` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:46 |
| `std::sys::pal::solid::abi::sockets::iovec` | `.iov_base` | `*mut os::raw::c_void` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:147 |
| `std::sys::pal::solid::abi::sockets::iovec` | `.iov_len` | `usize` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:148 |
| `std::sys::pal::solid::abi::sockets::ip_mreq` | `.imr_multiaddr` | `std::sys::pal::solid::abi::sockets::in_addr` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:63 |
| `std::sys::pal::solid::abi::sockets::ip_mreq` | `.imr_interface` | `std::sys::pal::solid::abi::sockets::in_addr` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:64 |
| `std::sys::pal::solid::abi::sockets::ipv6_mreq` | `.ipv6mr_multiaddr` | `std::sys::pal::solid::abi::sockets::in6_addr` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:70 |
| `std::sys::pal::solid::abi::sockets::ipv6_mreq` | `.ipv6mr_interface` | `os::raw::c_uint` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:71 |
| `std::sys::pal::solid::abi::sockets::linger` | `.l_onoff` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:140 |
| `std::sys::pal::solid::abi::sockets::linger` | `.l_linger` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:141 |
| `std::sys::pal::solid::abi::sockets::msghdr` | `.msg_name` | `*mut os::raw::c_void` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:77 |
| `std::sys::pal::solid::abi::sockets::msghdr` | `.msg_namelen` | `std::sys::pal::solid::abi::sockets::socklen_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:78 |
| `std::sys::pal::solid::abi::sockets::msghdr` | `.msg_iov` | `*mut std::sys::pal::solid::abi::sockets::iovec` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:79 |
| `std::sys::pal::solid::abi::sockets::msghdr` | `.msg_iovlen` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:80 |
| `std::sys::pal::solid::abi::sockets::msghdr` | `.msg_control` | `*mut os::raw::c_void` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:81 |
| `std::sys::pal::solid::abi::sockets::msghdr` | `.msg_controllen` | `std::sys::pal::solid::abi::sockets::socklen_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:82 |
| `std::sys::pal::solid::abi::sockets::msghdr` | `.msg_flags` | `libc::c_int` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:83 |
| `std::sys::pal::solid::abi::sockets::sa_family_t` | `(alias)` | `u8` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:43 |
| `std::sys::pal::solid::abi::sockets::sockaddr` | `.sa_len` | `u8` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:89 |
| `std::sys::pal::solid::abi::sockets::sockaddr` | `.sa_family` | `std::sys::pal::solid::abi::sockets::sa_family_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:90 |
| `std::sys::pal::solid::abi::sockets::sockaddr` | `.sa_data` | `[os::raw::c_char; 14usize]` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:91 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in` | `.sin_len` | `u8` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:97 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in` | `.sin_family` | `std::sys::pal::solid::abi::sockets::sa_family_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:98 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in` | `.sin_port` | `std::sys::pal::solid::abi::sockets::in_port_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:99 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in` | `.sin_addr` | `std::sys::pal::solid::abi::sockets::in_addr` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:100 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in` | `.sin_zero` | `[os::raw::c_char; 8usize]` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:101 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in6` | `.sin6_len` | `u8` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:107 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in6` | `.sin6_family` | `std::sys::pal::solid::abi::sockets::sa_family_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:108 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in6` | `.sin6_port` | `std::sys::pal::solid::abi::sockets::in_port_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:109 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in6` | `.sin6_flowinfo` | `u32` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:110 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in6` | `.sin6_addr` | `std::sys::pal::solid::abi::sockets::in6_addr` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:111 |
| `std::sys::pal::solid::abi::sockets::sockaddr_in6` | `.sin6_scope_id` | `u32` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:112 |
| `std::sys::pal::solid::abi::sockets::sockaddr_storage` | `.s2_len` | `u8` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:118 |
| `std::sys::pal::solid::abi::sockets::sockaddr_storage` | `.ss_family` | `std::sys::pal::solid::abi::sockets::sa_family_t` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:119 |
| `std::sys::pal::solid::abi::sockets::sockaddr_storage` | `.s2_data1` | `[os::raw::c_char; 2usize]` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:120 |
| `std::sys::pal::solid::abi::sockets::sockaddr_storage` | `.s2_data2` | `[u32; 3usize]` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:121 |
| `std::sys::pal::solid::abi::sockets::socklen_t` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/solid/abi/sockets.rs:44 |
| `std::sys::pal::solid::os::Env` | `.iter` | `vec::IntoIter<(std::ffi::os_str::OsString, std::ffi::os_str::OsString)>` | tools/rust/library/std/src/sys/pal/solid/os.rs:85 |
| `std::sys::pal::solid::os::EnvStrDebug` | `.slice` | `[(std::ffi::os_str::OsString, std::ffi::os_str::OsString)]` | tools/rust/library/std/src/sys/pal/solid/os.rs:90 |
| `std::sys::pal::solid::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/solid/os.rs:37 |
| `std::sys::pal::solid::time::SystemTime` | `[0]` | `abi::time_t` | tools/rust/library/std/src/sys/pal/solid/time.rs:8 |
| `std::sys::pal::teeos::os::Env` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/teeos/os.rs:76 |
| `std::sys::pal::teeos::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/teeos/os.rs:35 |
| `std::sys::pal::teeos::os::SplitPaths` | `[1]` | `core::marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/teeos/os.rs:35 |
| `std::sys::pal::teeos::thread::Thread` | `.id` | `libc::pthread_t` | tools/rust/library/std/src/sys/pal/teeos/thread.rs:11 |
| `std::sys::pal::uefi::RawOsError` | `(alias)` | `usize` | tools/rust/library/std/src/sys/pal/uefi/mod.rs:28 |
| `std::sys::pal::uefi::args::Args` | `.parsed_args_list` | `vec::IntoIter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/uefi/args.rs:10 |
| `std::sys::pal::uefi::helpers::BootInstallMultipleProtocolInterfaces` | `(alias)` | `opaque(unsafe extern "efiapi" fn (_ : * mut r_efi :: efi :: Handle , _ : ...) -> r_efi :: efi :: Status)` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:28 |
| `std::sys::pal::uefi::helpers::BootUninstallMultipleProtocolInterfaces` | `(alias)` | `opaque(unsafe extern "efiapi" fn (_ : r_efi :: efi :: Handle , _ : ...) -> r_efi :: efi :: Status)` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:31 |
| `std::sys::pal::uefi::helpers::BorrowedDevicePath` | `.protocol` | `ptr::NonNull<r_efi::protocols::device_path::Protocol>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:331 |
| `std::sys::pal::uefi::helpers::BorrowedDevicePath` | `.phantom` | `marker::PhantomData<r_efi::protocols::device_path::Protocol>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:332 |
| `std::sys::pal::uefi::helpers::DevicePathIterator` | `[0]` | `Option<std::sys::pal::uefi::helpers::DevicePathNode>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:358 |
| `std::sys::pal::uefi::helpers::DevicePathNode` | `.protocol` | `ptr::NonNull<r_efi::protocols::device_path::Protocol>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:381 |
| `std::sys::pal::uefi::helpers::DevicePathNode` | `.phantom` | `marker::PhantomData<r_efi::protocols::device_path::Protocol>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:382 |
| `std::sys::pal::uefi::helpers::OwnedDevicePath` | `[0]` | `ptr::NonNull<r_efi::protocols::device_path::Protocol>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:248 |
| `std::sys::pal::uefi::helpers::OwnedEvent` | `[0]` | `ptr::NonNull<core::ffi::c_void>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:707 |
| `std::sys::pal::uefi::helpers::OwnedProtocol` | `.guid` | `r_efi::efi::Guid` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:477 |
| `std::sys::pal::uefi::helpers::OwnedProtocol` | `.handle` | `ptr::NonNull<core::ffi::c_void>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:478 |
| `std::sys::pal::uefi::helpers::OwnedProtocol` | `.protocol` | `*mut $T` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:479 |
| `std::sys::pal::uefi::helpers::OwnedTable` | `.layout` | `alloc::Layout` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:552 |
| `std::sys::pal::uefi::helpers::OwnedTable` | `.ptr` | `*mut $T` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:553 |
| `std::sys::pal::uefi::helpers::ServiceProtocol` | `.service_guid` | `r_efi::efi::Guid` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:650 |
| `std::sys::pal::uefi::helpers::ServiceProtocol` | `.handle` | `ptr::NonNull<core::ffi::c_void>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:651 |
| `std::sys::pal::uefi::helpers::ServiceProtocol` | `.child_handle` | `ptr::NonNull<core::ffi::c_void>` | tools/rust/library/std/src/sys/pal/uefi/helpers.rs:652 |
| `std::sys::pal::uefi::os::Env` | `[0]` | `vec::IntoIter<(std::ffi::os_str::OsString, std::ffi::os_str::OsString)>` | tools/rust/library/std/src/sys/pal/uefi/os.rs:148 |
| `std::sys::pal::uefi::os::EnvStrDebug` | `.iter` | `[(std::ffi::os_str::OsString, std::ffi::os_str::OsString)]` | tools/rust/library/std/src/sys/pal/uefi/os.rs:135 |
| `std::sys::pal::uefi::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/uefi/os.rs:95 |
| `std::sys::pal::uefi::os::SplitPaths` | `[1]` | `marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/uefi/os.rs:95 |
| `std::sys::pal::uefi::thread::Thread` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/uefi/thread.rs:8 |
| `std::sys::pal::uefi::time::Instant` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/uefi/time.rs:8 |
| `std::sys::pal::uefi::time::SystemTime` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/uefi/time.rs:11 |
| `std::sys::pal::unix::args::Args` | `.iter` | `vec::IntoIter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/unix/args.rs:62 |
| `std::sys::pal::unix::fd::FileDesc` | `[0]` | `os::unix::io::OwnedFd` | tools/rust/library/std/src/sys/pal/unix/fd.rs:28 |
| `std::sys::pal::unix::fuchsia::zx_handle_t` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/unix/fuchsia.rs:9 |
| `std::sys::pal::unix::fuchsia::zx_rights_t` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/unix/fuchsia.rs:11 |
| `std::sys::pal::unix::fuchsia::zx_signals_t` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/unix/fuchsia.rs:19 |
| `std::sys::pal::unix::fuchsia::zx_status_t` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/unix/fuchsia.rs:12 |
| `std::sys::pal::unix::fuchsia::zx_time_t` | `(alias)` | `i64` | tools/rust/library/std/src/sys/pal/unix/fuchsia.rs:16 |
| `std::sys::pal::unix::fuchsia::zx_vaddr_t` | `(alias)` | `usize` | tools/rust/library/std/src/sys/pal/unix/fuchsia.rs:10 |
| `std::sys::pal::unix::futex::Futex` | `(alias)` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/unix/futex.rs:15 |
| `std::sys::pal::unix::futex::Primitive` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/unix/futex.rs:17 |
| `std::sys::pal::unix::futex::SmallFutex` | `(alias)` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/unix/futex.rs:20 |
| `std::sys::pal::unix::futex::SmallPrimitive` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/unix/futex.rs:22 |
| `std::sys::pal::unix::futex::zircon::zx_futex_t` | `(alias)` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/unix/futex.rs:254 |
| `std::sys::pal::unix::futex::zircon::zx_handle_t` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/unix/futex.rs:255 |
| `std::sys::pal::unix::futex::zircon::zx_status_t` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/unix/futex.rs:256 |
| `std::sys::pal::unix::futex::zircon::zx_time_t` | `(alias)` | `i64` | tools/rust/library/std/src/sys/pal/unix/futex.rs:257 |
| `std::sys::pal::unix::kernel_copy::Copier` | `.read` | `$R` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:174 |
| `std::sys::pal::unix::kernel_copy::Copier` | `.write` | `$W` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:175 |
| `std::sys::pal::unix::kernel_copy::CopyParams` | `[0]` | `std::sys::pal::unix::kernel_copy::FdMeta` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:171 |
| `std::sys::pal::unix::kernel_copy::CopyParams` | `[1]` | `Option<os::unix::io::RawFd>` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:171 |
| `std::sys::pal::unix::kernel_copy::CopyResult` | `.Ended` | `std::sys::pal::unix::kernel_copy::CopyResult::Ended` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:562 |
| `std::sys::pal::unix::kernel_copy::CopyResult` | `.Error` | `std::sys::pal::unix::kernel_copy::CopyResult::Error` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:563 |
| `std::sys::pal::unix::kernel_copy::CopyResult` | `.Fallback` | `std::sys::pal::unix::kernel_copy::CopyResult::Fallback` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:564 |
| `std::sys::pal::unix::kernel_copy::CopyResult::Ended` | `[0]` | `u64` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:562 |
| `std::sys::pal::unix::kernel_copy::CopyResult::Error` | `[0]` | `std::io::error::Error` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:563 |
| `std::sys::pal::unix::kernel_copy::CopyResult::Error` | `[1]` | `u64` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:563 |
| `std::sys::pal::unix::kernel_copy::CopyResult::Fallback` | `[0]` | `u64` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:564 |
| `std::sys::pal::unix::kernel_copy::FdMeta` | `.Metadata` | `std::sys::pal::unix::kernel_copy::FdMeta::Metadata` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:87 |
| `std::sys::pal::unix::kernel_copy::FdMeta::Metadata` | `[0]` | `std::fs::Metadata` | tools/rust/library/std/src/sys/pal/unix/kernel_copy.rs:87 |
| `std::sys::pal::unix::linux::pidfd::PidFd` | `[0]` | `std::sys::pal::unix::fd::FileDesc` | tools/rust/library/std/src/sys/pal/unix/linux/pidfd.rs:12 |
| `std::sys::pal::unix::os::Env` | `.iter` | `vec::IntoIter<(std::ffi::os_str::OsString, std::ffi::os_str::OsString)>` | tools/rust/library/std/src/sys/pal/unix/os.rs:548 |
| `std::sys::pal::unix::os::EnvStrDebug` | `.slice` | `[(std::ffi::os_str::OsString, std::ffi::os_str::OsString)]` | tools/rust/library/std/src/sys/pal/unix/os.rs:553 |
| `std::sys::pal::unix::os::SplitPaths` | `.iter` | `iter::Map<slice::Split<u8, opaque(fn (& u8) -> bool)>, opaque(fn (& 'a [u8]) -> PathBuf)>` | tools/rust/library/std/src/sys/pal/unix/os.rs:194 |
| `std::sys::pal::unix::pipe::AnonPipe` | `[0]` | `sys::fd::FileDesc` | tools/rust/library/std/src/sys/pal/unix/pipe.rs:13 |
| `std::sys::pal::unix::stack_overflow::Handler` | `.data` | `*mut libc::c_void` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:7 |
| `std::sys::pal::unix::stack_overflow::imp::c::BOOL` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:608 |
| `std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_POINTERS` | `.ExceptionRecord` | `*mut std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:624 |
| `std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | `.ExceptionCode` | `std::sys::pal::unix::stack_overflow::imp::c::NTSTATUS` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:631 |
| `std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | `.ExceptionFlags` | `u32` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:632 |
| `std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | `.ExceptionRecord` | `*mut std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:633 |
| `std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | `.ExceptionAddress` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:634 |
| `std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | `.NumberParameters` | `u32` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:635 |
| `std::sys::pal::unix::stack_overflow::imp::c::EXCEPTION_RECORD` | `.ExceptionInformation` | `[usize; 15]` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:636 |
| `std::sys::pal::unix::stack_overflow::imp::c::NTSTATUS` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:607 |
| `std::sys::pal::unix::stack_overflow::imp::c::PVECTORED_EXCEPTION_HANDLER` | `(alias)` | `Option<opaque(unsafe extern "system" fn (exceptioninfo : * mut EXCEPTION_POINTERS) -> i32)>` | tools/rust/library/std/src/sys/pal/unix/stack_overflow.rs:605 |
| `std::sys::pal::unix::sync::condvar::Condvar` | `.inner` | `cell::UnsafeCell<libc::pthread_cond_t>` | tools/rust/library/std/src/sys/pal/unix/sync/condvar.rs:12 |
| `std::sys::pal::unix::sync::mutex::AttrGuard` | `[0]` | `mem::MaybeUninit<libc::pthread_mutexattr_t>` | tools/rust/library/std/src/sys/pal/unix/sync/mutex.rs:126 |
| `std::sys::pal::unix::sync::mutex::Mutex` | `.inner` | `cell::UnsafeCell<libc::pthread_mutex_t>` | tools/rust/library/std/src/sys/pal/unix/sync/mutex.rs:8 |
| `std::sys::pal::unix::thread::Thread` | `.id` | `libc::pthread_t` | tools/rust/library/std/src/sys/pal/unix/thread.rs:38 |
| `std::sys::pal::unix::thread::zircon::zx_handle_t` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/unix/thread.rs:22 |
| `std::sys::pal::unix::thread::zircon::zx_status_t` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/unix/thread.rs:23 |
| `std::sys::pal::unix::thread_parking::ThreadId` | `(alias)` | `libc::lwpid_t` | tools/rust/library/std/src/sys/pal/unix/thread_parking.rs:23 |
| `std::sys::pal::unix::time::Instant` | `.t` | `std::sys::pal::unix::time::Timespec` | tools/rust/library/std/src/sys/pal/unix/time.rs:266 |
| `std::sys::pal::unix::time::SystemTime` | `.t` | `std::sys::pal::unix::time::Timespec` | tools/rust/library/std/src/sys/pal/unix/time.rs:22 |
| `std::sys::pal::unix::time::Timespec` | `.tv_sec` | `i64` | tools/rust/library/std/src/sys/pal/unix/time.rs:27 |
| `std::sys::pal::unix::time::Timespec` | `.tv_nsec` | `core::num::niche_types::Nanoseconds` | tools/rust/library/std/src/sys/pal/unix/time.rs:28 |
| `std::sys::pal::unix::time::__timespec64` | `.tv_sec` | `i64` | tools/rust/library/std/src/sys/pal/unix/time.rs:244 |
| `std::sys::pal::unix::time::__timespec64` | `._padding` | `i32` | tools/rust/library/std/src/sys/pal/unix/time.rs:246 |
| `std::sys::pal::unix::time::__timespec64` | `.tv_nsec` | `i32` | tools/rust/library/std/src/sys/pal/unix/time.rs:247 |
| `std::sys::pal::unix::time::__timespec64` | `._padding` | `i32` | tools/rust/library/std/src/sys/pal/unix/time.rs:249 |
| `std::sys::pal::unix::weak::DlsymWeak` | `.name` | `str` | tools/rust/library/std/src/sys/pal/unix/weak.rs:75 |
| `std::sys::pal::unix::weak::DlsymWeak` | `.func` | `sync::atomic::AtomicPtr<libc::c_void>` | tools/rust/library/std/src/sys/pal/unix/weak.rs:76 |
| `std::sys::pal::unix::weak::DlsymWeak` | `._marker` | `marker::PhantomData<$F>` | tools/rust/library/std/src/sys/pal/unix/weak.rs:77 |
| `std::sys::pal::unix::weak::ExternWeak` | `.weak_ptr` | `Option<$F>` | tools/rust/library/std/src/sys/pal/unix/weak.rs:49 |
| `std::sys::pal::unsupported::os::Env` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/unsupported/os.rs:65 |
| `std::sys::pal::unsupported::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/unsupported/os.rs:24 |
| `std::sys::pal::unsupported::os::SplitPaths` | `[1]` | `marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/unsupported/os.rs:24 |
| `std::sys::pal::unsupported::pipe::AnonPipe` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/unsupported/pipe.rs:5 |
| `std::sys::pal::unsupported::thread::Thread` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/unsupported/thread.rs:7 |
| `std::sys::pal::unsupported::time::Instant` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/unsupported/time.rs:4 |
| `std::sys::pal::unsupported::time::SystemTime` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/unsupported/time.rs:7 |
| `std::sys::pal::wasi::args::Args` | `.iter` | `vec::IntoIter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/wasi/args.rs:8 |
| `std::sys::pal::wasi::fd::WasiFd` | `.fd` | `os::wasi::io::OwnedFd` | tools/rust/library/std/src/sys/pal/wasi/fd.rs:13 |
| `std::sys::pal::wasi::os::Env` | `.iter` | `vec::IntoIter<(std::ffi::os_str::OsString, std::ffi::os_str::OsString)>` | tools/rust/library/std/src/sys/pal/wasi/os.rs:145 |
| `std::sys::pal::wasi::os::EnvStrDebug` | `.slice` | `[(std::ffi::os_str::OsString, std::ffi::os_str::OsString)]` | tools/rust/library/std/src/sys/pal/wasi/os.rs:150 |
| `std::sys::pal::wasi::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/wasi/os.rs:103 |
| `std::sys::pal::wasi::os::SplitPaths` | `[1]` | `marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/wasi/os.rs:103 |
| `std::sys::pal::wasi::time::Instant` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/wasi/time.rs:6 |
| `std::sys::pal::wasi::time::SystemTime` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/wasi/time.rs:9 |
| `std::sys::pal::wasm::atomics::futex::Futex` | `(alias)` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/wasm/atomics/futex.rs:10 |
| `std::sys::pal::wasm::atomics::futex::Primitive` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/wasm/atomics/futex.rs:12 |
| `std::sys::pal::wasm::atomics::futex::SmallFutex` | `(alias)` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/wasm/atomics/futex.rs:15 |
| `std::sys::pal::wasm::atomics::futex::SmallPrimitive` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/wasm/atomics/futex.rs:17 |
| `std::sys::pal::wasm::atomics::thread::Thread` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/wasm/atomics/thread.rs:7 |
| `std::sys::pal::wasm::atomics::thread::guard::Guard` | `(alias)` | `opaque(!)` | tools/rust/library/std/src/sys/pal/wasm/atomics/thread.rs:52 |
| `std::sys::pal::windows::Align8` | `[0]` | `$T` | tools/rust/library/std/src/sys/pal/windows/mod.rs:363 |
| `std::sys::pal::windows::api::WinError` | `.code` | `u32` | tools/rust/library/std/src/sys/pal/windows/api.rs:253 |
| `std::sys::pal::windows::args::Arg` | `.Regular` | `std::sys::pal::windows::args::Arg::Regular` | tools/rust/library/std/src/sys/pal/windows/args.rs:191 |
| `std::sys::pal::windows::args::Arg` | `.Raw` | `std::sys::pal::windows::args::Arg::Raw` | tools/rust/library/std/src/sys/pal/windows/args.rs:193 |
| `std::sys::pal::windows::args::Arg::Raw` | `[0]` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/sys/pal/windows/args.rs:193 |
| `std::sys::pal::windows::args::Arg::Regular` | `[0]` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/sys/pal/windows/args.rs:191 |
| `std::sys::pal::windows::args::Args` | `.parsed_args_list` | `vec::IntoIter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/windows/args.rs:157 |
| `std::sys::pal::windows::c::MOUNT_POINT_REPARSE_BUFFER` | `.SubstituteNameOffset` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:97 |
| `std::sys::pal::windows::c::MOUNT_POINT_REPARSE_BUFFER` | `.SubstituteNameLength` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:98 |
| `std::sys::pal::windows::c::MOUNT_POINT_REPARSE_BUFFER` | `.PrintNameOffset` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:99 |
| `std::sys::pal::windows::c::MOUNT_POINT_REPARSE_BUFFER` | `.PrintNameLength` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:100 |
| `std::sys::pal::windows::c::MOUNT_POINT_REPARSE_BUFFER` | `.PathBuffer` | `std::sys::pal::windows::c::WCHAR` | tools/rust/library/std/src/sys/pal/windows/c.rs:101 |
| `std::sys::pal::windows::c::REPARSE_DATA_BUFFER` | `.ReparseTag` | `core::ffi::c_uint` | tools/rust/library/std/src/sys/pal/windows/c.rs:77 |
| `std::sys::pal::windows::c::REPARSE_DATA_BUFFER` | `.ReparseDataLength` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:78 |
| `std::sys::pal::windows::c::REPARSE_DATA_BUFFER` | `.Reserved` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:79 |
| `std::sys::pal::windows::c::REPARSE_DATA_BUFFER` | `.rest` | `()` | tools/rust/library/std/src/sys/pal/windows/c.rs:80 |
| `std::sys::pal::windows::c::SYMBOLIC_LINK_REPARSE_BUFFER` | `.SubstituteNameOffset` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:87 |
| `std::sys::pal::windows::c::SYMBOLIC_LINK_REPARSE_BUFFER` | `.SubstituteNameLength` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:88 |
| `std::sys::pal::windows::c::SYMBOLIC_LINK_REPARSE_BUFFER` | `.PrintNameOffset` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:89 |
| `std::sys::pal::windows::c::SYMBOLIC_LINK_REPARSE_BUFFER` | `.PrintNameLength` | `core::ffi::c_ushort` | tools/rust/library/std/src/sys/pal/windows/c.rs:90 |
| `std::sys::pal::windows::c::SYMBOLIC_LINK_REPARSE_BUFFER` | `.Flags` | `core::ffi::c_ulong` | tools/rust/library/std/src/sys/pal/windows/c.rs:91 |
| `std::sys::pal::windows::c::SYMBOLIC_LINK_REPARSE_BUFFER` | `.PathBuffer` | `std::sys::pal::windows::c::WCHAR` | tools/rust/library/std/src/sys/pal/windows/c.rs:92 |
| `std::sys::pal::windows::c::WCHAR` | `(alias)` | `u16` | tools/rust/library/std/src/sys/pal/windows/c.rs:14 |
| `std::sys::pal::windows::c::windows_sys::ACL` | `.AclRevision` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:146 |
| `std::sys::pal::windows::c::windows_sys::ACL` | `.Sbz1` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:147 |
| `std::sys::pal::windows::c::windows_sys::ACL` | `.AclSize` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:148 |
| `std::sys::pal::windows::c::windows_sys::ACL` | `.AceCount` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:149 |
| `std::sys::pal::windows::c::windows_sys::ACL` | `.Sbz2` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:150 |
| `std::sys::pal::windows::c::windows_sys::ADDRESS_FAMILY` | `(alias)` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:152 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_flags` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:156 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_family` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:157 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_socktype` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:158 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_protocol` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:159 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_addrlen` | `usize` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:160 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_canonname` | `std::sys::pal::windows::c::windows_sys::PSTR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:161 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_addr` | `*mut std::sys::pal::windows::c::windows_sys::SOCKADDR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:162 |
| `std::sys::pal::windows::c::windows_sys::ADDRINFOA` | `.ai_next` | `*mut std::sys::pal::windows::c::windows_sys::ADDRINFOA` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:163 |
| `std::sys::pal::windows::c::windows_sys::ARM64_NT_NEON128_0` | `.Low` | `u64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:182 |
| `std::sys::pal::windows::c::windows_sys::ARM64_NT_NEON128_0` | `.High` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:183 |
| `std::sys::pal::windows::c::windows_sys::BOOL` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:186 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.dwFileAttributes` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:190 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.ftCreationTime` | `std::sys::pal::windows::c::windows_sys::FILETIME` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:191 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.ftLastAccessTime` | `std::sys::pal::windows::c::windows_sys::FILETIME` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:192 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.ftLastWriteTime` | `std::sys::pal::windows::c::windows_sys::FILETIME` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:193 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.dwVolumeSerialNumber` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:194 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.nFileSizeHigh` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:195 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.nFileSizeLow` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:196 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.nNumberOfLinks` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:197 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.nFileIndexHigh` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:198 |
| `std::sys::pal::windows::c::windows_sys::BY_HANDLE_FILE_INFORMATION` | `.nFileIndexLow` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:199 |
| `std::sys::pal::windows::c::windows_sys::COMPARESTRING_RESULT` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:203 |
| `std::sys::pal::windows::c::windows_sys::CONDITION_VARIABLE` | `.Ptr` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:207 |
| `std::sys::pal::windows::c::windows_sys::CONSOLE_MODE` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:209 |
| `std::sys::pal::windows::c::windows_sys::CONSOLE_READCONSOLE_CONTROL` | `.nLength` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:213 |
| `std::sys::pal::windows::c::windows_sys::CONSOLE_READCONSOLE_CONTROL` | `.nInitialChars` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:214 |
| `std::sys::pal::windows::c::windows_sys::CONSOLE_READCONSOLE_CONTROL` | `.dwCtrlWakeupMask` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:215 |
| `std::sys::pal::windows::c::windows_sys::CONSOLE_READCONSOLE_CONTROL` | `.dwControlKeyState` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:216 |
| `std::sys::pal::windows::c::windows_sys::CONTEXT_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:389 |
| `std::sys::pal::windows::c::windows_sys::COPYFILE_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:390 |
| `std::sys::pal::windows::c::windows_sys::COPYPROGRESSROUTINE_PROGRESS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:391 |
| `std::sys::pal::windows::c::windows_sys::DUPLICATE_HANDLE_OPTIONS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:599 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_DISPOSITION` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2300 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_POINTERS` | `.ExceptionRecord` | `*mut std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2305 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_POINTERS` | `.ContextRecord` | `*mut std::sys::pal::windows::c::windows_sys::CONTEXT` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2306 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | `.ExceptionCode` | `std::sys::pal::windows::c::windows_sys::NTSTATUS` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2311 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | `.ExceptionFlags` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2312 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | `.ExceptionRecord` | `*mut std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2313 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | `.ExceptionAddress` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2314 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | `.NumberParameters` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2315 |
| `std::sys::pal::windows::c::windows_sys::EXCEPTION_RECORD` | `.ExceptionInformation` | `[usize; 15]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2316 |
| `std::sys::pal::windows::c::windows_sys::FACILITY_CODE` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2325 |
| `std::sys::pal::windows::c::windows_sys::FARPROC` | `(alias)` | `Option<opaque(unsafe extern "system" fn () -> isize)>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2328 |
| `std::sys::pal::windows::c::windows_sys::FD_SET` | `.fd_count` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2333 |
| `std::sys::pal::windows::c::windows_sys::FD_SET` | `.fd_array` | `[std::sys::pal::windows::c::windows_sys::SOCKET; 64]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2334 |
| `std::sys::pal::windows::c::windows_sys::FILETIME` | `.dwLowDateTime` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2339 |
| `std::sys::pal::windows::c::windows_sys::FILETIME` | `.dwHighDateTime` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2340 |
| `std::sys::pal::windows::c::windows_sys::FILE_ACCESS_RIGHTS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2342 |
| `std::sys::pal::windows::c::windows_sys::FILE_ALLOCATION_INFO` | `.AllocationSize` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2348 |
| `std::sys::pal::windows::c::windows_sys::FILE_ATTRIBUTE_TAG_INFO` | `.FileAttributes` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2374 |
| `std::sys::pal::windows::c::windows_sys::FILE_ATTRIBUTE_TAG_INFO` | `.ReparseTag` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2375 |
| `std::sys::pal::windows::c::windows_sys::FILE_BASIC_INFO` | `.CreationTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2383 |
| `std::sys::pal::windows::c::windows_sys::FILE_BASIC_INFO` | `.LastAccessTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2384 |
| `std::sys::pal::windows::c::windows_sys::FILE_BASIC_INFO` | `.LastWriteTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2385 |
| `std::sys::pal::windows::c::windows_sys::FILE_BASIC_INFO` | `.ChangeTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2386 |
| `std::sys::pal::windows::c::windows_sys::FILE_BASIC_INFO` | `.FileAttributes` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2387 |
| `std::sys::pal::windows::c::windows_sys::FILE_CREATION_DISPOSITION` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2395 |
| `std::sys::pal::windows::c::windows_sys::FILE_DISPOSITION_INFO` | `.DeleteFile` | `bool` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2410 |
| `std::sys::pal::windows::c::windows_sys::FILE_DISPOSITION_INFO_EX` | `.Flags` | `std::sys::pal::windows::c::windows_sys::FILE_DISPOSITION_INFO_EX_FLAGS` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2415 |
| `std::sys::pal::windows::c::windows_sys::FILE_DISPOSITION_INFO_EX_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2417 |
| `std::sys::pal::windows::c::windows_sys::FILE_END_OF_FILE_INFO` | `.EndOfFile` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2422 |
| `std::sys::pal::windows::c::windows_sys::FILE_FLAGS_AND_ATTRIBUTES` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2425 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.NextEntryOffset` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2444 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.FileIndex` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2445 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.CreationTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2446 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.LastAccessTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2447 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.LastWriteTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2448 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.ChangeTime` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2449 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.EndOfFile` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2450 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.AllocationSize` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2451 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.FileAttributes` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2452 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.FileNameLength` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2453 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.EaSize` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2454 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.ShortNameLength` | `i8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2455 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.ShortName` | `[u16; 12]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2456 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.FileId` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2457 |
| `std::sys::pal::windows::c::windows_sys::FILE_ID_BOTH_DIR_INFO` | `.FileName` | `[u16; 1]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2458 |
| `std::sys::pal::windows::c::windows_sys::FILE_INFO_BY_HANDLE_CLASS` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2460 |
| `std::sys::pal::windows::c::windows_sys::FILE_IO_PRIORITY_HINT_INFO` | `.PriorityHint` | `std::sys::pal::windows::c::windows_sys::PRIORITY_HINT` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2464 |
| `std::sys::pal::windows::c::windows_sys::FILE_RENAME_INFO` | `.Anonymous` | `FILE_RENAME_INFO_0` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2492 |
| `std::sys::pal::windows::c::windows_sys::FILE_RENAME_INFO` | `.RootDirectory` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2493 |
| `std::sys::pal::windows::c::windows_sys::FILE_RENAME_INFO` | `.FileNameLength` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2494 |
| `std::sys::pal::windows::c::windows_sys::FILE_RENAME_INFO` | `.FileName` | `[u16; 1]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2495 |
| `std::sys::pal::windows::c::windows_sys::FILE_SHARE_MODE` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2507 |
| `std::sys::pal::windows::c::windows_sys::FILE_STANDARD_INFO` | `.AllocationSize` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2514 |
| `std::sys::pal::windows::c::windows_sys::FILE_STANDARD_INFO` | `.EndOfFile` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2515 |
| `std::sys::pal::windows::c::windows_sys::FILE_STANDARD_INFO` | `.NumberOfLinks` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2516 |
| `std::sys::pal::windows::c::windows_sys::FILE_STANDARD_INFO` | `.DeletePending` | `bool` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2517 |
| `std::sys::pal::windows::c::windows_sys::FILE_STANDARD_INFO` | `.Directory` | `bool` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2518 |
| `std::sys::pal::windows::c::windows_sys::FILE_TYPE` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2524 |
| `std::sys::pal::windows::c::windows_sys::FINDEX_INFO_LEVELS` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2534 |
| `std::sys::pal::windows::c::windows_sys::FINDEX_SEARCH_OPS` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2535 |
| `std::sys::pal::windows::c::windows_sys::FIND_FIRST_EX_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2536 |
| `std::sys::pal::windows::c::windows_sys::FORMAT_MESSAGE_OPTIONS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2572 |
| `std::sys::pal::windows::c::windows_sys::GENERIC_ACCESS_RIGHTS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2603 |
| `std::sys::pal::windows::c::windows_sys::GETFINALPATHNAMEBYHANDLE_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2608 |
| `std::sys::pal::windows::c::windows_sys::GUID` | `.data1` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2612 |
| `std::sys::pal::windows::c::windows_sys::GUID` | `.data2` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2613 |
| `std::sys::pal::windows::c::windows_sys::GUID` | `.data3` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2614 |
| `std::sys::pal::windows::c::windows_sys::GUID` | `.data4` | `[u8; 8]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2615 |
| `std::sys::pal::windows::c::windows_sys::HANDLE` | `(alias)` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2627 |
| `std::sys::pal::windows::c::windows_sys::HANDLE_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2628 |
| `std::sys::pal::windows::c::windows_sys::HINSTANCE` | `(alias)` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2632 |
| `std::sys::pal::windows::c::windows_sys::HLOCAL` | `(alias)` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2633 |
| `std::sys::pal::windows::c::windows_sys::HMODULE` | `(alias)` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2634 |
| `std::sys::pal::windows::c::windows_sys::HRESULT` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2635 |
| `std::sys::pal::windows::c::windows_sys::IN6_ADDR` | `.u` | `IN6_ADDR_0` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2640 |
| `std::sys::pal::windows::c::windows_sys::IN_ADDR` | `.S_un` | `IN_ADDR_0` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2662 |
| `std::sys::pal::windows::c::windows_sys::IN_ADDR_0_0` | `.s_b1` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2674 |
| `std::sys::pal::windows::c::windows_sys::IN_ADDR_0_0` | `.s_b2` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2675 |
| `std::sys::pal::windows::c::windows_sys::IN_ADDR_0_0` | `.s_b3` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2676 |
| `std::sys::pal::windows::c::windows_sys::IN_ADDR_0_0` | `.s_b4` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2677 |
| `std::sys::pal::windows::c::windows_sys::IN_ADDR_0_1` | `.s_w1` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2682 |
| `std::sys::pal::windows::c::windows_sys::IN_ADDR_0_1` | `.s_w2` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2683 |
| `std::sys::pal::windows::c::windows_sys::IO_STATUS_BLOCK` | `.Anonymous` | `IO_STATUS_BLOCK_0` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2690 |
| `std::sys::pal::windows::c::windows_sys::IO_STATUS_BLOCK` | `.Information` | `usize` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2691 |
| `std::sys::pal::windows::c::windows_sys::IPPROTO` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2699 |
| `std::sys::pal::windows::c::windows_sys::IPV6_MREQ` | `.ipv6mr_multiaddr` | `std::sys::pal::windows::c::windows_sys::IN6_ADDR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2742 |
| `std::sys::pal::windows::c::windows_sys::IPV6_MREQ` | `.ipv6mr_interface` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2743 |
| `std::sys::pal::windows::c::windows_sys::IP_MREQ` | `.imr_multiaddr` | `std::sys::pal::windows::c::windows_sys::IN_ADDR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2752 |
| `std::sys::pal::windows::c::windows_sys::IP_MREQ` | `.imr_interface` | `std::sys::pal::windows::c::windows_sys::IN_ADDR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2753 |
| `std::sys::pal::windows::c::windows_sys::LINGER` | `.l_onoff` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2761 |
| `std::sys::pal::windows::c::windows_sys::LINGER` | `.l_linger` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2762 |
| `std::sys::pal::windows::c::windows_sys::LOCK_FILE_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2766 |
| `std::sys::pal::windows::c::windows_sys::LPOVERLAPPED_COMPLETION_ROUTINE` | `(alias)` | `Option<opaque(unsafe extern "system" fn (dwerrorcode : u32 , dwnumberofbytestransfered : u32 , lpoverlapped : * mut OVERLAPPED ,))>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2767 |
| `std::sys::pal::windows::c::windows_sys::LPPROC_THREAD_ATTRIBUTE_LIST` | `(alias)` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2774 |
| `std::sys::pal::windows::c::windows_sys::LPPROGRESS_ROUTINE` | `(alias)` | `Option<opaque(unsafe extern "system" fn (totalfilesize : i64 , totalbytestransferred : i64 , streamsize : i64 , streambytestransferred : i64 , dwstreamnumber : u32 , dwcallbackreason : LPPROGRESS_ROUTINE_CALLBACK_REASON , hsourcefile : HANDLE , hdestinationfile : HANDLE , lpdata : * const core :: ffi :: c_void ,) -> COPYPROGRESSROUTINE_PROGRESS)>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2775 |
| `std::sys::pal::windows::c::windows_sys::LPPROGRESS_ROUTINE_CALLBACK_REASON` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2788 |
| `std::sys::pal::windows::c::windows_sys::LPTHREAD_START_ROUTINE` | `(alias)` | `Option<opaque(unsafe extern "system" fn (lpthreadparameter : * mut core :: ffi :: c_void) -> u32)>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2789 |
| `std::sys::pal::windows::c::windows_sys::LPWSAOVERLAPPED_COMPLETION_ROUTINE` | `(alias)` | `Option<opaque(unsafe extern "system" fn (dwerror : u32 , cbtransferred : u32 , lpoverlapped : * mut OVERLAPPED , dwflags : u32 ,))>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2791 |
| `std::sys::pal::windows::c::windows_sys::M128A` | `.Low` | `u64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2802 |
| `std::sys::pal::windows::c::windows_sys::M128A` | `.High` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2803 |
| `std::sys::pal::windows::c::windows_sys::MOVE_FILE_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2817 |
| `std::sys::pal::windows::c::windows_sys::MULTI_BYTE_TO_WIDE_CHAR_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2823 |
| `std::sys::pal::windows::c::windows_sys::NAMED_PIPE_MODE` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2825 |
| `std::sys::pal::windows::c::windows_sys::NTCREATEFILE_CREATE_DISPOSITION` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2828 |
| `std::sys::pal::windows::c::windows_sys::NTCREATEFILE_CREATE_OPTIONS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2829 |
| `std::sys::pal::windows::c::windows_sys::NTSTATUS` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2830 |
| `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTES` | `.Length` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2834 |
| `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTES` | `.RootDirectory` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2835 |
| `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTES` | `.ObjectName` | `*const std::sys::pal::windows::c::windows_sys::UNICODE_STRING` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2836 |
| `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTES` | `.Attributes` | `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTE_FLAGS` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2837 |
| `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTES` | `.SecurityDescriptor` | `*const std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2838 |
| `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTES` | `.SecurityQualityOfService` | `*const std::sys::pal::windows::c::windows_sys::SECURITY_QUALITY_OF_SERVICE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2839 |
| `std::sys::pal::windows::c::windows_sys::OBJECT_ATTRIBUTE_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2841 |
| `std::sys::pal::windows::c::windows_sys::OVERLAPPED` | `.Internal` | `usize` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2848 |
| `std::sys::pal::windows::c::windows_sys::OVERLAPPED` | `.InternalHigh` | `usize` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2849 |
| `std::sys::pal::windows::c::windows_sys::OVERLAPPED` | `.Anonymous` | `OVERLAPPED_0` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2850 |
| `std::sys::pal::windows::c::windows_sys::OVERLAPPED` | `.hEvent` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2851 |
| `std::sys::pal::windows::c::windows_sys::OVERLAPPED_0_0` | `.Offset` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2862 |
| `std::sys::pal::windows::c::windows_sys::OVERLAPPED_0_0` | `.OffsetHigh` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2863 |
| `std::sys::pal::windows::c::windows_sys::PCSTR` | `(alias)` | `*const u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2865 |
| `std::sys::pal::windows::c::windows_sys::PCWSTR` | `(alias)` | `*const u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2866 |
| `std::sys::pal::windows::c::windows_sys::PIO_APC_ROUTINE` | `(alias)` | `Option<opaque(unsafe extern "system" fn (apccontext : * mut core :: ffi :: c_void , iostatusblock : * mut IO_STATUS_BLOCK , reserved : u32 ,))>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2867 |
| `std::sys::pal::windows::c::windows_sys::PRIORITY_HINT` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2887 |
| `std::sys::pal::windows::c::windows_sys::PROCESSOR_ARCHITECTURE` | `(alias)` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2888 |
| `std::sys::pal::windows::c::windows_sys::PROCESS_CREATION_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2889 |
| `std::sys::pal::windows::c::windows_sys::PROCESS_INFORMATION` | `.hProcess` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2893 |
| `std::sys::pal::windows::c::windows_sys::PROCESS_INFORMATION` | `.hThread` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2894 |
| `std::sys::pal::windows::c::windows_sys::PROCESS_INFORMATION` | `.dwProcessId` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2895 |
| `std::sys::pal::windows::c::windows_sys::PROCESS_INFORMATION` | `.dwThreadId` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2896 |
| `std::sys::pal::windows::c::windows_sys::PSID` | `(alias)` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2904 |
| `std::sys::pal::windows::c::windows_sys::PSTR` | `(alias)` | `*mut u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2905 |
| `std::sys::pal::windows::c::windows_sys::PTIMERAPCROUTINE` | `(alias)` | `Option<opaque(unsafe extern "system" fn (lpargtocompletionroutine : * const core :: ffi :: c_void , dwtimerlowvalue : u32 , dwtimerhighvalue : u32 ,))>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2906 |
| `std::sys::pal::windows::c::windows_sys::PVECTORED_EXCEPTION_HANDLER` | `(alias)` | `Option<opaque(unsafe extern "system" fn (exceptioninfo : * mut EXCEPTION_POINTERS) -> i32)>` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2913 |
| `std::sys::pal::windows::c::windows_sys::PWSTR` | `(alias)` | `*mut u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2915 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_ATTRIBUTES` | `.nLength` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2925 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_ATTRIBUTES` | `.lpSecurityDescriptor` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2926 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_ATTRIBUTES` | `.bInheritHandle` | `std::sys::pal::windows::c::windows_sys::BOOL` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2927 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | `.Revision` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2934 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | `.Sbz1` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2935 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | `.Control` | `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR_CONTROL` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2936 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | `.Owner` | `std::sys::pal::windows::c::windows_sys::PSID` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2937 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | `.Group` | `std::sys::pal::windows::c::windows_sys::PSID` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2938 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | `.Sacl` | `*mut std::sys::pal::windows::c::windows_sys::ACL` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2939 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR` | `.Dacl` | `*mut std::sys::pal::windows::c::windows_sys::ACL` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2940 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_DESCRIPTOR_CONTROL` | `(alias)` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2942 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_IMPERSONATION_LEVEL` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2946 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_QUALITY_OF_SERVICE` | `.Length` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2950 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_QUALITY_OF_SERVICE` | `.ImpersonationLevel` | `std::sys::pal::windows::c::windows_sys::SECURITY_IMPERSONATION_LEVEL` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2951 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_QUALITY_OF_SERVICE` | `.ContextTrackingMode` | `u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2952 |
| `std::sys::pal::windows::c::windows_sys::SECURITY_QUALITY_OF_SERVICE` | `.EffectiveOnly` | `bool` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2953 |
| `std::sys::pal::windows::c::windows_sys::SEND_RECV_FLAGS` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2957 |
| `std::sys::pal::windows::c::windows_sys::SET_FILE_POINTER_MOVE_METHOD` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2958 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR` | `.sa_family` | `std::sys::pal::windows::c::windows_sys::ADDRESS_FAMILY` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2962 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR` | `.sa_data` | `[i8; 14]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2963 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR_STORAGE` | `.ss_family` | `std::sys::pal::windows::c::windows_sys::ADDRESS_FAMILY` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2968 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR_STORAGE` | `.__ss_pad1` | `[i8; 6]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2969 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR_STORAGE` | `.__ss_align` | `i64` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2970 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR_STORAGE` | `.__ss_pad2` | `[i8; 112]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2971 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR_UN` | `.sun_family` | `std::sys::pal::windows::c::windows_sys::ADDRESS_FAMILY` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2976 |
| `std::sys::pal::windows::c::windows_sys::SOCKADDR_UN` | `.sun_path` | `[i8; 108]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2977 |
| `std::sys::pal::windows::c::windows_sys::SOCKET` | `(alias)` | `usize` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2979 |
| `std::sys::pal::windows::c::windows_sys::SRWLOCK` | `.Ptr` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:2996 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOEXW` | `.StartupInfo` | `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3021 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOEXW` | `.lpAttributeList` | `std::sys::pal::windows::c::windows_sys::LPPROC_THREAD_ATTRIBUTE_LIST` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3022 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.cb` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3027 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.lpReserved` | `std::sys::pal::windows::c::windows_sys::PWSTR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3028 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.lpDesktop` | `std::sys::pal::windows::c::windows_sys::PWSTR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3029 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.lpTitle` | `std::sys::pal::windows::c::windows_sys::PWSTR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3030 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwX` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3031 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwY` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3032 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwXSize` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3033 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwYSize` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3034 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwXCountChars` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3035 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwYCountChars` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3036 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwFillAttribute` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3037 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.dwFlags` | `std::sys::pal::windows::c::windows_sys::STARTUPINFOW_FLAGS` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3038 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.wShowWindow` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3039 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.cbReserved2` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3040 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.lpReserved2` | `*mut u8` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3041 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.hStdInput` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3042 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.hStdOutput` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3043 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW` | `.hStdError` | `std::sys::pal::windows::c::windows_sys::HANDLE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3044 |
| `std::sys::pal::windows::c::windows_sys::STARTUPINFOW_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3046 |
| `std::sys::pal::windows::c::windows_sys::STD_HANDLE` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3058 |
| `std::sys::pal::windows::c::windows_sys::SYMBOLIC_LINK_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3061 |
| `std::sys::pal::windows::c::windows_sys::SYNCHRONIZATION_ACCESS_RIGHTS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3065 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.Anonymous` | `SYSTEM_INFO_0` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3070 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.dwPageSize` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3071 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.lpMinimumApplicationAddress` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3072 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.lpMaximumApplicationAddress` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3073 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.dwActiveProcessorMask` | `usize` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3074 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.dwNumberOfProcessors` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3075 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.dwProcessorType` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3076 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.dwAllocationGranularity` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3077 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.wProcessorLevel` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3078 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO` | `.wProcessorRevision` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3079 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO_0_0` | `.wProcessorArchitecture` | `std::sys::pal::windows::c::windows_sys::PROCESSOR_ARCHITECTURE` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3090 |
| `std::sys::pal::windows::c::windows_sys::SYSTEM_INFO_0_0` | `.wReserved` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3091 |
| `std::sys::pal::windows::c::windows_sys::THREAD_CREATION_FLAGS` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3096 |
| `std::sys::pal::windows::c::windows_sys::TIMEVAL` | `.tv_sec` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3102 |
| `std::sys::pal::windows::c::windows_sys::TIMEVAL` | `.tv_usec` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3103 |
| `std::sys::pal::windows::c::windows_sys::TOKEN_ACCESS_MASK` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3106 |
| `std::sys::pal::windows::c::windows_sys::UNICODE_STRING` | `.Length` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3133 |
| `std::sys::pal::windows::c::windows_sys::UNICODE_STRING` | `.MaximumLength` | `u16` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3134 |
| `std::sys::pal::windows::c::windows_sys::UNICODE_STRING` | `.Buffer` | `std::sys::pal::windows::c::windows_sys::PWSTR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3135 |
| `std::sys::pal::windows::c::windows_sys::WAIT_EVENT` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3142 |
| `std::sys::pal::windows::c::windows_sys::WIN32_ERROR` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3148 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.dwFileAttributes` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3152 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.ftCreationTime` | `std::sys::pal::windows::c::windows_sys::FILETIME` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3153 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.ftLastAccessTime` | `std::sys::pal::windows::c::windows_sys::FILETIME` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3154 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.ftLastWriteTime` | `std::sys::pal::windows::c::windows_sys::FILETIME` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3155 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.nFileSizeHigh` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3156 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.nFileSizeLow` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3157 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.dwReserved0` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3158 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.dwReserved1` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3159 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.cFileName` | `[u16; 260]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3160 |
| `std::sys::pal::windows::c::windows_sys::WIN32_FIND_DATAW` | `.cAlternateFileName` | `[u16; 14]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3161 |
| `std::sys::pal::windows::c::windows_sys::WINSOCK_SHUTDOWN_HOW` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3163 |
| `std::sys::pal::windows::c::windows_sys::WINSOCK_SOCKET_TYPE` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3164 |
| `std::sys::pal::windows::c::windows_sys::WSABUF` | `.len` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3171 |
| `std::sys::pal::windows::c::windows_sys::WSABUF` | `.buf` | `std::sys::pal::windows::c::windows_sys::PSTR` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3172 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOLCHAIN` | `.ChainLen` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3255 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOLCHAIN` | `.ChainEntries` | `[u32; 7]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3256 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwServiceFlags1` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3261 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwServiceFlags2` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3262 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwServiceFlags3` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3263 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwServiceFlags4` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3264 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwProviderFlags` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3265 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.ProviderId` | `std::sys::pal::windows::c::windows_sys::GUID` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3266 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwCatalogEntryId` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3267 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.ProtocolChain` | `std::sys::pal::windows::c::windows_sys::WSAPROTOCOLCHAIN` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3268 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iVersion` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3269 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iAddressFamily` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3270 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iMaxSockAddr` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3271 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iMinSockAddr` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3272 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iSocketType` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3273 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iProtocol` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3274 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iProtocolMaxOffset` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3275 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iNetworkByteOrder` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3276 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.iSecurityScheme` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3277 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwMessageSize` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3278 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.dwProviderReserved` | `u32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3279 |
| `std::sys::pal::windows::c::windows_sys::WSAPROTOCOL_INFOW` | `.szProtocol` | `[u16; 256]` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3280 |
| `std::sys::pal::windows::c::windows_sys::WSA_ERROR` | `(alias)` | `i32` | tools/rust/library/std/src/sys/pal/windows/c/windows_sys.rs:3288 |
| `std::sys::pal::windows::compat::Module` | `[0]` | `ptr::NonNull<core::ffi::c_void>` | tools/rust/library/std/src/sys/pal/windows/compat.rs:105 |
| `std::sys::pal::windows::futex::Futex` | `(alias)` | `core::sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/pal/windows/futex.rs:13 |
| `std::sys::pal::windows::futex::Primitive` | `(alias)` | `u32` | tools/rust/library/std/src/sys/pal/windows/futex.rs:15 |
| `std::sys::pal::windows::futex::SmallFutex` | `(alias)` | `core::sync::atomic::AtomicU8` | tools/rust/library/std/src/sys/pal/windows/futex.rs:18 |
| `std::sys::pal::windows::futex::SmallPrimitive` | `(alias)` | `u8` | tools/rust/library/std/src/sys/pal/windows/futex.rs:20 |
| `std::sys::pal::windows::handle::Handle` | `[0]` | `os::windows::io::OwnedHandle` | tools/rust/library/std/src/sys/pal/windows/handle.rs:20 |
| `std::sys::pal::windows::os::Env` | `.base` | `*mut sys::c::WCHAR` | tools/rust/library/std/src/sys/pal/windows/os.rs:80 |
| `std::sys::pal::windows::os::Env` | `.iter` | `std::sys::pal::windows::os::EnvIterator` | tools/rust/library/std/src/sys/pal/windows/os.rs:81 |
| `std::sys::pal::windows::os::EnvIterator` | `[0]` | `*mut sys::c::WCHAR` | tools/rust/library/std/src/sys/pal/windows/os.rs:125 |
| `std::sys::pal::windows::os::EnvStrDebug` | `.iter` | `std::sys::pal::windows::os::EnvIterator` | tools/rust/library/std/src/sys/pal/windows/os.rs:86 |
| `std::sys::pal::windows::os::SplitPaths` | `.data` | `std::sys_common::wtf8::EncodeWide` | tools/rust/library/std/src/sys/pal/windows/os.rs:182 |
| `std::sys::pal::windows::os::SplitPaths` | `.must_yield` | `bool` | tools/rust/library/std/src/sys/pal/windows/os.rs:183 |
| `std::sys::pal::windows::pipe::AnonPipe` | `.inner` | `sys::handle::Handle` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:20 |
| `std::sys::pal::windows::pipe::AsyncPipe` | `.pipe` | `sys::handle::Handle` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:424 |
| `std::sys::pal::windows::pipe::AsyncPipe` | `.event` | `sys::handle::Handle` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:425 |
| `std::sys::pal::windows::pipe::AsyncPipe` | `.overlapped` | `Box<sys::c::OVERLAPPED>` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:426 |
| `std::sys::pal::windows::pipe::AsyncPipe` | `.dst` | `Vec<u8>` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:427 |
| `std::sys::pal::windows::pipe::AsyncPipe` | `.state` | `std::sys::pal::windows::pipe::State` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:428 |
| `std::sys::pal::windows::pipe::Pipes` | `.ours` | `std::sys::pal::windows::pipe::AnonPipe` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:36 |
| `std::sys::pal::windows::pipe::Pipes` | `.theirs` | `std::sys::pal::windows::pipe::AnonPipe` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:37 |
| `std::sys::pal::windows::pipe::State` | `.Read` | `std::sys::pal::windows::pipe::State::Read` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:435 |
| `std::sys::pal::windows::pipe::State::Read` | `[0]` | `usize` | tools/rust/library/std/src/sys/pal/windows/pipe.rs:435 |
| `std::sys::pal::windows::thread::Thread` | `.handle` | `sys::handle::Handle` | tools/rust/library/std/src/sys/pal/windows/thread.rs:17 |
| `std::sys::pal::windows::time::Instant` | `.t` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/windows/time.rs:18 |
| `std::sys::pal::windows::time::SystemTime` | `.t` | `sys::c::FILETIME` | tools/rust/library/std/src/sys/pal/windows/time.rs:23 |
| `std::sys::pal::windows::time::WaitableTimer` | `.handle` | `sys::c::HANDLE` | tools/rust/library/std/src/sys/pal/windows/time.rs:228 |
| `std::sys::pal::windows::time::perf_counter::PerformanceCounterInstant` | `.ts` | `i64` | tools/rust/library/std/src/sys/pal/windows/time.rs:173 |
| `std::sys::pal::xous::args::Args` | `.parsed_args_list` | `vec::IntoIter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/pal/xous/args.rs:7 |
| `std::sys::pal::xous::os::Env` | `.iter` | `vec::IntoIter<(std::ffi::os_str::OsString, std::ffi::os_str::OsString)>` | tools/rust/library/std/src/sys/pal/xous/os.rs:164 |
| `std::sys::pal::xous::os::EnvStore` | `(alias)` | `std::sync::poison::mutex::Mutex<collections::HashMap<std::ffi::os_str::OsString, std::ffi::os_str::OsString>>` | tools/rust/library/std/src/sys/pal/xous/os.rs:142 |
| `std::sys::pal::xous::os::EnvStrDebug` | `.slice` | `[(std::ffi::os_str::OsString, std::ffi::os_str::OsString)]` | tools/rust/library/std/src/sys/pal/xous/os.rs:169 |
| `std::sys::pal::xous::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/xous/os.rs:93 |
| `std::sys::pal::xous::os::SplitPaths` | `[1]` | `marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/xous/os.rs:93 |
| `std::sys::pal::xous::os::params::ApplicationParameter` | `.data` | `[u8]` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:151 |
| `std::sys::pal::xous::os::params::ApplicationParameter` | `.magic` | `[u8; 4]` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:152 |
| `std::sys::pal::xous::os::params::ApplicationParameters` | `.data` | `[u8]` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:87 |
| `std::sys::pal::xous::os::params::ApplicationParameters` | `.offset` | `usize` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:88 |
| `std::sys::pal::xous::os::params::ApplicationParameters` | `._entries` | `usize` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:89 |
| `std::sys::pal::xous::os::params::ArgumentEntry` | `.value` | `str` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:237 |
| `std::sys::pal::xous::os::params::ArgumentList` | `.data` | `[u8]` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:217 |
| `std::sys::pal::xous::os::params::ArgumentList` | `._count` | `usize` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:218 |
| `std::sys::pal::xous::os::params::ArgumentList` | `.offset` | `usize` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:219 |
| `std::sys::pal::xous::os::params::EnvironmentBlock` | `._count` | `usize` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:158 |
| `std::sys::pal::xous::os::params::EnvironmentBlock` | `.data` | `[u8]` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:159 |
| `std::sys::pal::xous::os::params::EnvironmentBlock` | `.offset` | `usize` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:160 |
| `std::sys::pal::xous::os::params::EnvironmentEntry` | `.key` | `str` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:178 |
| `std::sys::pal::xous::os::params::EnvironmentEntry` | `.value` | `str` | tools/rust/library/std/src/sys/pal/xous/os/params.rs:179 |
| `std::sys::pal::xous::thread::Thread` | `.tid` | `os::xous::ffi::ThreadId` | tools/rust/library/std/src/sys/pal/xous/thread.rs:14 |
| `std::sys::pal::xous::time::Instant` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/xous/time.rs:8 |
| `std::sys::pal::xous::time::SystemTime` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/sys/pal/xous/time.rs:11 |
| `std::sys::pal::zkvm::args::Args` | `.i_forward` | `usize` | tools/rust/library/std/src/sys/pal/zkvm/args.rs:8 |
| `std::sys::pal::zkvm::args::Args` | `.i_back` | `usize` | tools/rust/library/std/src/sys/pal/zkvm/args.rs:9 |
| `std::sys::pal::zkvm::args::Args` | `.count` | `usize` | tools/rust/library/std/src/sys/pal/zkvm/args.rs:10 |
| `std::sys::pal::zkvm::os::Env` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/zkvm/os.rs:67 |
| `std::sys::pal::zkvm::os::SplitPaths` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/pal/zkvm/os.rs:26 |
| `std::sys::pal::zkvm::os::SplitPaths` | `[1]` | `marker::PhantomData<()>` | tools/rust/library/std/src/sys/pal/zkvm/os.rs:26 |
| `std::sys::path::windows::PrefixParser` | `.path` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/sys/path/windows.rs:47 |
| `std::sys::path::windows::PrefixParser` | `.prefix` | `[u8; LEN]` | tools/rust/library/std/src/sys/path/windows.rs:48 |
| `std::sys::path::windows::PrefixParserSlice` | `.path` | `std::ffi::os_str::OsStr` | tools/rust/library/std/src/sys/path/windows.rs:76 |
| `std::sys::path::windows::PrefixParserSlice` | `.prefix` | `[u8]` | tools/rust/library/std/src/sys/path/windows.rs:77 |
| `std::sys::path::windows::PrefixParserSlice` | `.index` | `usize` | tools/rust/library/std/src/sys/path/windows.rs:78 |
| `std::sys::personality::dwarf::DwarfReader` | `.ptr` | `*const u8` | tools/rust/library/std/src/sys/personality/dwarf/mod.rs:16 |
| `std::sys::personality::dwarf::eh::EHAction` | `.Cleanup` | `std::sys::personality::dwarf::eh::EHAction::Cleanup` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:51 |
| `std::sys::personality::dwarf::eh::EHAction` | `.Catch` | `std::sys::personality::dwarf::eh::EHAction::Catch` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:52 |
| `std::sys::personality::dwarf::eh::EHAction` | `.Filter` | `std::sys::personality::dwarf::eh::EHAction::Filter` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:53 |
| `std::sys::personality::dwarf::eh::EHAction::Catch` | `[0]` | `std::sys::personality::dwarf::eh::LPad` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:52 |
| `std::sys::personality::dwarf::eh::EHAction::Cleanup` | `[0]` | `std::sys::personality::dwarf::eh::LPad` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:51 |
| `std::sys::personality::dwarf::eh::EHAction::Filter` | `[0]` | `std::sys::personality::dwarf::eh::LPad` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:53 |
| `std::sys::personality::dwarf::eh::EHContext` | `.ip` | `*const u8` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:41 |
| `std::sys::personality::dwarf::eh::EHContext` | `.func_start` | `*const u8` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:42 |
| `std::sys::personality::dwarf::eh::EHContext` | `.get_text_start` | `opaque(dyn Fn () -> * const u8)` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:43 |
| `std::sys::personality::dwarf::eh::EHContext` | `.get_data_start` | `opaque(dyn Fn () -> * const u8)` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:44 |
| `std::sys::personality::dwarf::eh::LPad` | `(alias)` | `*const u8` | tools/rust/library/std/src/sys/personality/dwarf/eh.rs:48 |
| `std::sys::process::uefi::Command` | `.prog` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/sys/process/uefi.rs:22 |
| `std::sys::process::uefi::Command` | `.args` | `Vec<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/process/uefi.rs:23 |
| `std::sys::process::uefi::Command` | `.stdout` | `Option<std::sys::process::uefi::Stdio>` | tools/rust/library/std/src/sys/process/uefi.rs:24 |
| `std::sys::process::uefi::Command` | `.stderr` | `Option<std::sys::process::uefi::Stdio>` | tools/rust/library/std/src/sys/process/uefi.rs:25 |
| `std::sys::process::uefi::Command` | `.env` | `std::sys_common::process::CommandEnv` | tools/rust/library/std/src/sys/process/uefi.rs:26 |
| `std::sys::process::uefi::CommandArgs` | `.iter` | `slice::Iter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/process/uefi.rs:304 |
| `std::sys::process::uefi::ExitCode` | `[0]` | `bool` | tools/rust/library/std/src/sys/process/uefi.rs:263 |
| `std::sys::process::uefi::ExitStatus` | `[0]` | `r_efi::efi::Status` | tools/rust/library/std/src/sys/process/uefi.rs:215 |
| `std::sys::process::uefi::ExitStatusError` | `[0]` | `r_efi::efi::Status` | tools/rust/library/std/src/sys/process/uefi.rs:241 |
| `std::sys::process::uefi::Process` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/process/uefi.rs:283 |
| `std::sys::process::uefi::StdioPipes` | `.stdin` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/uefi.rs:32 |
| `std::sys::process::uefi::StdioPipes` | `.stdout` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/uefi.rs:33 |
| `std::sys::process::uefi::StdioPipes` | `.stderr` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/uefi.rs:34 |
| `std::sys::process::uefi::uefi_command_internal::Image` | `.handle` | `ptr::NonNull<core::ffi::c_void>` | tools/rust/library/std/src/sys/process/uefi.rs:350 |
| `std::sys::process::uefi::uefi_command_internal::Image` | `.stdout` | `Option<sys::pal::helpers::OwnedProtocol<std::sys::process::uefi::uefi_command_internal::PipeProtocol>>` | tools/rust/library/std/src/sys/process/uefi.rs:351 |
| `std::sys::process::uefi::uefi_command_internal::Image` | `.stderr` | `Option<sys::pal::helpers::OwnedProtocol<std::sys::process::uefi::uefi_command_internal::PipeProtocol>>` | tools/rust/library/std/src/sys/process/uefi.rs:352 |
| `std::sys::process::uefi::uefi_command_internal::Image` | `.st` | `sys::pal::helpers::OwnedTable<r_efi::efi::SystemTable>` | tools/rust/library/std/src/sys/process/uefi.rs:353 |
| `std::sys::process::uefi::uefi_command_internal::Image` | `.args` | `Option<(*mut u16, usize)>` | tools/rust/library/std/src/sys/process/uefi.rs:354 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.reset` | `r_efi::protocols::simple_text_output::ProtocolReset` | tools/rust/library/std/src/sys/process/uefi.rs:550 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.output_string` | `r_efi::protocols::simple_text_output::ProtocolOutputString` | tools/rust/library/std/src/sys/process/uefi.rs:551 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.test_string` | `r_efi::protocols::simple_text_output::ProtocolTestString` | tools/rust/library/std/src/sys/process/uefi.rs:552 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.query_mode` | `r_efi::protocols::simple_text_output::ProtocolQueryMode` | tools/rust/library/std/src/sys/process/uefi.rs:553 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.set_mode` | `r_efi::protocols::simple_text_output::ProtocolSetMode` | tools/rust/library/std/src/sys/process/uefi.rs:554 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.set_attribute` | `r_efi::protocols::simple_text_output::ProtocolSetAttribute` | tools/rust/library/std/src/sys/process/uefi.rs:555 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.clear_screen` | `r_efi::protocols::simple_text_output::ProtocolClearScreen` | tools/rust/library/std/src/sys/process/uefi.rs:556 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.set_cursor_position` | `r_efi::protocols::simple_text_output::ProtocolSetCursorPosition` | tools/rust/library/std/src/sys/process/uefi.rs:557 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.enable_cursor` | `r_efi::protocols::simple_text_output::ProtocolEnableCursor` | tools/rust/library/std/src/sys/process/uefi.rs:558 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `.mode` | `*mut r_efi::protocols::simple_text_output::Mode` | tools/rust/library/std/src/sys/process/uefi.rs:559 |
| `std::sys::process::uefi::uefi_command_internal::PipeProtocol` | `._buffer` | `Vec<u16>` | tools/rust/library/std/src/sys/process/uefi.rs:560 |
| `std::sys::process::unix::common::Argv` | `[0]` | `Vec<*const libc::c_char>` | tools/rust/library/std/src/sys/process/unix/common.rs:105 |
| `std::sys::process::unix::common::CStringArray` | `.items` | `Vec<alloc::ffi::c_str::CString>` | tools/rust/library/std/src/sys/process/unix/common.rs:397 |
| `std::sys::process::unix::common::CStringArray` | `.ptrs` | `Vec<*const libc::c_char>` | tools/rust/library/std/src/sys/process/unix/common.rs:398 |
| `std::sys::process::unix::common::ChildPipes` | `.stdin` | `std::sys::process::unix::common::ChildStdio` | tools/rust/library/std/src/sys/process/unix/common.rs:124 |
| `std::sys::process::unix::common::ChildPipes` | `.stdout` | `std::sys::process::unix::common::ChildStdio` | tools/rust/library/std/src/sys/process/unix/common.rs:125 |
| `std::sys::process::unix::common::ChildPipes` | `.stderr` | `std::sys::process::unix::common::ChildStdio` | tools/rust/library/std/src/sys/process/unix/common.rs:126 |
| `std::sys::process::unix::common::ChildStdio` | `.Explicit` | `std::sys::process::unix::common::ChildStdio::Explicit` | tools/rust/library/std/src/sys/process/unix/common.rs:131 |
| `std::sys::process::unix::common::ChildStdio` | `.Owned` | `std::sys::process::unix::common::ChildStdio::Owned` | tools/rust/library/std/src/sys/process/unix/common.rs:132 |
| `std::sys::process::unix::common::ChildStdio::Explicit` | `[0]` | `libc::c_int` | tools/rust/library/std/src/sys/process/unix/common.rs:131 |
| `std::sys::process::unix::common::ChildStdio::Owned` | `[0]` | `sys::fd::FileDesc` | tools/rust/library/std/src/sys/process/unix/common.rs:132 |
| `std::sys::process::unix::common::Command` | `.program` | `alloc::ffi::c_str::CString` | tools/rust/library/std/src/sys/process/unix/common.rs:79 |
| `std::sys::process::unix::common::Command` | `.args` | `Vec<alloc::ffi::c_str::CString>` | tools/rust/library/std/src/sys/process/unix/common.rs:80 |
| `std::sys::process::unix::common::Command` | `.argv` | `std::sys::process::unix::common::Argv` | tools/rust/library/std/src/sys/process/unix/common.rs:86 |
| `std::sys::process::unix::common::Command` | `.env` | `std::sys_common::process::CommandEnv` | tools/rust/library/std/src/sys/process/unix/common.rs:87 |
| `std::sys::process::unix::common::Command` | `.program_kind` | `std::sys::process::unix::common::ProgramKind` | tools/rust/library/std/src/sys/process/unix/common.rs:89 |
| `std::sys::process::unix::common::Command` | `.cwd` | `Option<alloc::ffi::c_str::CString>` | tools/rust/library/std/src/sys/process/unix/common.rs:90 |
| `std::sys::process::unix::common::Command` | `.uid` | `Option<libc::uid_t>` | tools/rust/library/std/src/sys/process/unix/common.rs:91 |
| `std::sys::process::unix::common::Command` | `.gid` | `Option<libc::gid_t>` | tools/rust/library/std/src/sys/process/unix/common.rs:92 |
| `std::sys::process::unix::common::Command` | `.saw_nul` | `bool` | tools/rust/library/std/src/sys/process/unix/common.rs:93 |
| `std::sys::process::unix::common::Command` | `.closures` | `Vec<Box<opaque(dyn FnMut () -> io :: Result < () > + Send + Sync)>>` | tools/rust/library/std/src/sys/process/unix/common.rs:94 |
| `std::sys::process::unix::common::Command` | `.groups` | `Option<Box<[libc::gid_t]>>` | tools/rust/library/std/src/sys/process/unix/common.rs:95 |
| `std::sys::process::unix::common::Command` | `.stdin` | `Option<std::sys::process::unix::common::Stdio>` | tools/rust/library/std/src/sys/process/unix/common.rs:96 |
| `std::sys::process::unix::common::Command` | `.stdout` | `Option<std::sys::process::unix::common::Stdio>` | tools/rust/library/std/src/sys/process/unix/common.rs:97 |
| `std::sys::process::unix::common::Command` | `.stderr` | `Option<std::sys::process::unix::common::Stdio>` | tools/rust/library/std/src/sys/process/unix/common.rs:98 |
| `std::sys::process::unix::common::Command` | `.create_pidfd` | `bool` | tools/rust/library/std/src/sys/process/unix/common.rs:100 |
| `std::sys::process::unix::common::Command` | `.pgroup` | `Option<libc::pid_t>` | tools/rust/library/std/src/sys/process/unix/common.rs:101 |
| `std::sys::process::unix::common::CommandArgs` | `.iter` | `slice::Iter<alloc::ffi::c_str::CString>` | tools/rust/library/std/src/sys/process/unix/common.rs:648 |
| `std::sys::process::unix::common::ExitCode` | `[0]` | `u8` | tools/rust/library/std/src/sys/process/unix/common.rs:623 |
| `std::sys::process::unix::common::Stdio` | `.Fd` | `std::sys::process::unix::common::Stdio::Fd` | tools/rust/library/std/src/sys/process/unix/common.rs:145 |
| `std::sys::process::unix::common::Stdio` | `.StaticFd` | `std::sys::process::unix::common::Stdio::StaticFd` | tools/rust/library/std/src/sys/process/unix/common.rs:146 |
| `std::sys::process::unix::common::Stdio::Fd` | `[0]` | `sys::fd::FileDesc` | tools/rust/library/std/src/sys/process/unix/common.rs:145 |
| `std::sys::process::unix::common::Stdio::StaticFd` | `[0]` | `BorrowedFd` | tools/rust/library/std/src/sys/process/unix/common.rs:146 |
| `std::sys::process::unix::common::StdioPipes` | `.stdin` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/unix/common.rs:115 |
| `std::sys::process::unix::common::StdioPipes` | `.stdout` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/unix/common.rs:116 |
| `std::sys::process::unix::common::StdioPipes` | `.stderr` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/unix/common.rs:117 |
| `std::sys::process::unix::fuchsia::ExitStatus` | `[0]` | `i64` | tools/rust/library/std/src/sys/process/unix/fuchsia.rs:227 |
| `std::sys::process::unix::fuchsia::ExitStatusError` | `[0]` | `core::num::nonzero::NonZero<i64>` | tools/rust/library/std/src/sys/process/unix/fuchsia.rs:305 |
| `std::sys::process::unix::fuchsia::Process` | `.handle` | `Handle` | tools/rust/library/std/src/sys/process/unix/fuchsia.rs:144 |
| `std::sys::process::unix::unix::ExitStatus` | `[0]` | `libc::c_int` | tools/rust/library/std/src/sys/process/unix/unix.rs:1018 |
| `std::sys::process::unix::unix::ExitStatusError` | `[0]` | `core::num::nonzero::NonZero<libc::c_int>` | tools/rust/library/std/src/sys/process/unix/unix.rs:1213 |
| `std::sys::process::unix::unix::Process` | `.pid` | `libc::pid_t` | tools/rust/library/std/src/sys/process/unix/unix.rs:923 |
| `std::sys::process::unix::unix::Process` | `.status` | `Option<std::sys::process::unix::unix::ExitStatus>` | tools/rust/library/std/src/sys/process/unix/unix.rs:924 |
| `std::sys::process::unix::unix::Process` | `.pidfd` | `Option<sys::pal::linux::pidfd::PidFd>` | tools/rust/library/std/src/sys/process/unix/unix.rs:930 |
| `std::sys::process::unix::unsupported::ExitStatusError` | `[0]` | `core::num::nonzero::NonZero<libc::c_int>` | tools/rust/library/std/src/sys/process/unix/unsupported.rs:60 |
| `std::sys::process::unix::unsupported::Process` | `._handle` | `libc::pid_t` | tools/rust/library/std/src/sys/process/unix/unsupported.rs:35 |
| `std::sys::process::unix::unsupported::wait_status::ExitStatus` | `.wait_status` | `ffi::c_int` | tools/rust/library/std/src/sys/process/unix/unsupported/wait_status.rs:19 |
| `std::sys::process::unix::vxworks::ExitStatus` | `[0]` | `opaque(c_int)` | tools/rust/library/std/src/sys/process/unix/vxworks.rs:187 |
| `std::sys::process::unix::vxworks::ExitStatusError` | `[0]` | `core::num::nonzero::NonZero<opaque(c_int)>` | tools/rust/library/std/src/sys/process/unix/vxworks.rs:256 |
| `std::sys::process::unix::vxworks::Process` | `.pid` | `opaque(RTP_ID)` | tools/rust/library/std/src/sys/process/unix/vxworks.rs:139 |
| `std::sys::process::unix::vxworks::Process` | `.status` | `Option<std::sys::process::unix::vxworks::ExitStatus>` | tools/rust/library/std/src/sys/process/unix/vxworks.rs:140 |
| `std::sys::process::unsupported::Command` | `.program` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/sys/process/unsupported.rs:16 |
| `std::sys::process::unsupported::Command` | `.args` | `Vec<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/process/unsupported.rs:17 |
| `std::sys::process::unsupported::Command` | `.env` | `std::sys_common::process::CommandEnv` | tools/rust/library/std/src/sys/process/unsupported.rs:18 |
| `std::sys::process::unsupported::Command` | `.cwd` | `Option<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/process/unsupported.rs:20 |
| `std::sys::process::unsupported::Command` | `.stdin` | `Option<std::sys::process::unsupported::Stdio>` | tools/rust/library/std/src/sys/process/unsupported.rs:21 |
| `std::sys::process::unsupported::Command` | `.stdout` | `Option<std::sys::process::unsupported::Stdio>` | tools/rust/library/std/src/sys/process/unsupported.rs:22 |
| `std::sys::process::unsupported::Command` | `.stderr` | `Option<std::sys::process::unsupported::Stdio>` | tools/rust/library/std/src/sys/process/unsupported.rs:23 |
| `std::sys::process::unsupported::CommandArgs` | `.iter` | `slice::Iter<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/process/unsupported.rs:296 |
| `std::sys::process::unsupported::ExitCode` | `[0]` | `u8` | tools/rust/library/std/src/sys/process/unsupported.rs:258 |
| `std::sys::process::unsupported::ExitStatusError` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/process/unsupported.rs:221 |
| `std::sys::process::unsupported::Process` | `[0]` | `opaque(!)` | tools/rust/library/std/src/sys/process/unsupported.rs:275 |
| `std::sys::process::unsupported::Stdio` | `.InheritFile` | `std::sys::process::unsupported::Stdio::InheritFile` | tools/rust/library/std/src/sys/process/unsupported.rs:42 |
| `std::sys::process::unsupported::Stdio::InheritFile` | `[0]` | `sys::fs::File` | tools/rust/library/std/src/sys/process/unsupported.rs:42 |
| `std::sys::process::unsupported::StdioPipes` | `.stdin` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/unsupported.rs:29 |
| `std::sys::process::unsupported::StdioPipes` | `.stdout` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/unsupported.rs:30 |
| `std::sys::process::unsupported::StdioPipes` | `.stderr` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/unsupported.rs:31 |
| `std::sys::process::windows::Command` | `.program` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/sys/process/windows.rs:147 |
| `std::sys::process::windows::Command` | `.args` | `Vec<sys::args::Arg>` | tools/rust/library/std/src/sys/process/windows.rs:148 |
| `std::sys::process::windows::Command` | `.env` | `std::sys_common::process::CommandEnv` | tools/rust/library/std/src/sys/process/windows.rs:149 |
| `std::sys::process::windows::Command` | `.cwd` | `Option<std::ffi::os_str::OsString>` | tools/rust/library/std/src/sys/process/windows.rs:150 |
| `std::sys::process::windows::Command` | `.flags` | `u32` | tools/rust/library/std/src/sys/process/windows.rs:151 |
| `std::sys::process::windows::Command` | `.show_window` | `Option<u16>` | tools/rust/library/std/src/sys/process/windows.rs:152 |
| `std::sys::process::windows::Command` | `.detach` | `bool` | tools/rust/library/std/src/sys/process/windows.rs:153 |
| `std::sys::process::windows::Command` | `.stdin` | `Option<std::sys::process::windows::Stdio>` | tools/rust/library/std/src/sys/process/windows.rs:154 |
| `std::sys::process::windows::Command` | `.stdout` | `Option<std::sys::process::windows::Stdio>` | tools/rust/library/std/src/sys/process/windows.rs:155 |
| `std::sys::process::windows::Command` | `.stderr` | `Option<std::sys::process::windows::Stdio>` | tools/rust/library/std/src/sys/process/windows.rs:156 |
| `std::sys::process::windows::Command` | `.force_quotes_enabled` | `bool` | tools/rust/library/std/src/sys/process/windows.rs:157 |
| `std::sys::process::windows::CommandArgs` | `.iter` | `slice::Iter<sys::args::Arg>` | tools/rust/library/std/src/sys/process/windows.rs:892 |
| `std::sys::process::windows::EnvKey` | `.os_string` | `std::ffi::os_str::OsString` | tools/rust/library/std/src/sys/process/windows.rs:37 |
| `std::sys::process::windows::EnvKey` | `.utf16` | `Vec<u16>` | tools/rust/library/std/src/sys/process/windows.rs:42 |
| `std::sys::process::windows::ExitCode` | `[0]` | `u32` | tools/rust/library/std/src/sys/process/windows.rs:767 |
| `std::sys::process::windows::ExitStatus` | `[0]` | `u32` | tools/rust/library/std/src/sys/process/windows.rs:715 |
| `std::sys::process::windows::ExitStatusError` | `[0]` | `core::num::nonzero::NonZero<u32>` | tools/rust/library/std/src/sys/process/windows.rs:752 |
| `std::sys::process::windows::Process` | `.handle` | `sys::handle::Handle` | tools/rust/library/std/src/sys/process/windows.rs:651 |
| `std::sys::process::windows::Process` | `.main_thread_handle` | `sys::handle::Handle` | tools/rust/library/std/src/sys/process/windows.rs:652 |
| `std::sys::process::windows::Stdio` | `.InheritSpecific` | `std::sys::process::windows::Stdio::InheritSpecific` | tools/rust/library/std/src/sys/process/windows.rs:162 |
| `std::sys::process::windows::Stdio` | `.Pipe` | `std::sys::process::windows::Stdio::Pipe` | tools/rust/library/std/src/sys/process/windows.rs:165 |
| `std::sys::process::windows::Stdio` | `.Handle` | `std::sys::process::windows::Stdio::Handle` | tools/rust/library/std/src/sys/process/windows.rs:166 |
| `std::sys::process::windows::Stdio::Handle` | `[0]` | `sys::handle::Handle` | tools/rust/library/std/src/sys/process/windows.rs:166 |
| `std::sys::process::windows::Stdio::InheritSpecific` | `.from_stdio_id` | `u32` | tools/rust/library/std/src/sys/process/windows.rs:162 |
| `std::sys::process::windows::Stdio::Pipe` | `[0]` | `sys::pipe::AnonPipe` | tools/rust/library/std/src/sys/process/windows.rs:165 |
| `std::sys::process::windows::StdioPipes` | `.stdin` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/windows.rs:170 |
| `std::sys::process::windows::StdioPipes` | `.stdout` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/windows.rs:171 |
| `std::sys::process::windows::StdioPipes` | `.stderr` | `Option<sys::pipe::AnonPipe>` | tools/rust/library/std/src/sys/process/windows.rs:172 |
| `std::sys::stdio::solid::Stderr` | `(alias)` | `std::sys::stdio::solid::Stdout` | tools/rust/library/std/src/sys/stdio/solid.rs:10 |
| `std::sys::stdio::solid::Stdin` | `(alias)` | `unsupported_stdio::Stdin` | tools/rust/library/std/src/sys/stdio/solid.rs:8 |
| `std::sys::stdio::teeos::Stderr` | `(alias)` | `std::sys::stdio::teeos::Stdout` | tools/rust/library/std/src/sys/stdio/teeos.rs:13 |
| `std::sys::stdio::teeos::Stdin` | `(alias)` | `unsupported_stdio::Stdin` | tools/rust/library/std/src/sys/stdio/teeos.rs:11 |
| `std::sys::stdio::uefi::IncompleteUtf8` | `.bytes` | `[u8; 4]` | tools/rust/library/std/src/sys/stdio/uefi.rs:13 |
| `std::sys::stdio::uefi::IncompleteUtf8` | `.len` | `u8` | tools/rust/library/std/src/sys/stdio/uefi.rs:14 |
| `std::sys::stdio::uefi::Stdin` | `.surrogate` | `Option<u16>` | tools/rust/library/std/src/sys/stdio/uefi.rs:8 |
| `std::sys::stdio::uefi::Stdin` | `.incomplete_utf8` | `std::sys::stdio::uefi::IncompleteUtf8` | tools/rust/library/std/src/sys/stdio/uefi.rs:9 |
| `std::sys::stdio::unsupported::Stderr` | `(alias)` | `std::sys::stdio::unsupported::Stdout` | tools/rust/library/std/src/sys/stdio/unsupported.rs:5 |
| `std::sys::stdio::windows::IncompleteUtf8` | `.bytes` | `[u8; 4]` | tools/rust/library/std/src/sys/stdio/windows.rs:32 |
| `std::sys::stdio::windows::IncompleteUtf8` | `.len` | `u8` | tools/rust/library/std/src/sys/stdio/windows.rs:33 |
| `std::sys::stdio::windows::Stderr` | `.incomplete_utf8` | `std::sys::stdio::windows::IncompleteUtf8` | tools/rust/library/std/src/sys/stdio/windows.rs:28 |
| `std::sys::stdio::windows::Stdin` | `.surrogate` | `u16` | tools/rust/library/std/src/sys/stdio/windows.rs:19 |
| `std::sys::stdio::windows::Stdin` | `.incomplete_utf8` | `std::sys::stdio::windows::IncompleteUtf8` | tools/rust/library/std/src/sys/stdio/windows.rs:20 |
| `std::sys::stdio::windows::Stdout` | `.incomplete_utf8` | `std::sys::stdio::windows::IncompleteUtf8` | tools/rust/library/std/src/sys/stdio/windows.rs:24 |
| `std::sys::stdio::xous::PanicWriter` | `.log` | `os::xous::ffi::Connection` | tools/rust/library/std/src/sys/stdio/xous.rs:75 |
| `std::sys::stdio::xous::PanicWriter` | `.gfx` | `Option<os::xous::ffi::Connection>` | tools/rust/library/std/src/sys/stdio/xous.rs:76 |
| `std::sys::stdio::xous::Stdin` | `(alias)` | `unsupported_stdio::Stdin` | tools/rust/library/std/src/sys/stdio/xous.rs:9 |
| `std::sys::sync::condvar::futex::Condvar` | `.futex` | `sys::futex::Futex` | tools/rust/library/std/src/sys/sync/condvar/futex.rs:10 |
| `std::sys::sync::condvar::itron::Condvar` | `.waiters` | `std::sys::pal::itron::spin::SpinMutex<std::sys::sync::condvar::itron::waiter_queue::WaiterQueue>` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:16 |
| `std::sys::sync::condvar::itron::waiter_queue::ListHead` | `.first` | `NonNull<std::sys::sync::condvar::itron::waiter_queue::Waiter>` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:133 |
| `std::sys::sync::condvar::itron::waiter_queue::ListHead` | `.last` | `NonNull<std::sys::sync::condvar::itron::waiter_queue::Waiter>` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:134 |
| `std::sys::sync::condvar::itron::waiter_queue::Waiter` | `.task` | `abi::ID` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:144 |
| `std::sys::sync::condvar::itron::waiter_queue::Waiter` | `.priority` | `abi::PRI` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:145 |
| `std::sys::sync::condvar::itron::waiter_queue::Waiter` | `.prev` | `Option<NonNull<std::sys::sync::condvar::itron::waiter_queue::Waiter>>` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:146 |
| `std::sys::sync::condvar::itron::waiter_queue::Waiter` | `.next` | `Option<NonNull<std::sys::sync::condvar::itron::waiter_queue::Waiter>>` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:147 |
| `std::sys::sync::condvar::itron::waiter_queue::WaiterQueue` | `.head` | `Option<std::sys::sync::condvar::itron::waiter_queue::ListHead>` | tools/rust/library/std/src/sys/sync/condvar/itron.rs:128 |
| `std::sys::sync::condvar::pthread::Condvar` | `.cvar` | `std::sys::sync::once_box::OnceBox<sys::pal::sync::Condvar>` | tools/rust/library/std/src/sys/sync/condvar/pthread.rs:12 |
| `std::sys::sync::condvar::pthread::Condvar` | `.mutex` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/sync/condvar/pthread.rs:13 |
| `std::sys::sync::condvar::sgx::Condvar` | `.inner` | `std::sys::sync::once_box::OnceBox<sys::pal::waitqueue::SpinMutex<sys::pal::waitqueue::WaitVariable<()>>>` | tools/rust/library/std/src/sys/sync/condvar/sgx.rs:7 |
| `std::sys::sync::condvar::windows7::Condvar` | `.inner` | `cell::UnsafeCell<sys::c::CONDITION_VARIABLE>` | tools/rust/library/std/src/sys/sync/condvar/windows7.rs:7 |
| `std::sys::sync::condvar::xous::Condvar` | `.counter` | `core::sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/sync/condvar/xous.rs:14 |
| `std::sys::sync::condvar::xous::Condvar` | `.timed_out` | `core::sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/sync/condvar/xous.rs:15 |
| `std::sys::sync::mutex::fuchsia::Mutex` | `.futex` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/sync/mutex/fuchsia.rs:55 |
| `std::sys::sync::mutex::futex::Futex` | `(alias)` | `sys::futex::SmallFutex` | tools/rust/library/std/src/sys/sync/mutex/futex.rs:4 |
| `std::sys::sync::mutex::futex::Mutex` | `.futex` | `std::sys::sync::mutex::futex::Futex` | tools/rust/library/std/src/sys/sync/mutex/futex.rs:8 |
| `std::sys::sync::mutex::futex::State` | `(alias)` | `sys::futex::SmallPrimitive` | tools/rust/library/std/src/sys/sync/mutex/futex.rs:5 |
| `std::sys::sync::mutex::itron::Mutex` | `.mtx` | `std::sys::pal::itron::spin::SpinIdOnceCell<()>` | tools/rust/library/std/src/sys/sync/mutex/itron.rs:11 |
| `std::sys::sync::mutex::no_threads::Mutex` | `.locked` | `cell::Cell<bool>` | tools/rust/library/std/src/sys/sync/mutex/no_threads.rs:5 |
| `std::sys::sync::mutex::pthread::Mutex` | `.pal` | `std::sys::sync::once_box::OnceBox<sys::pal::sync::Mutex>` | tools/rust/library/std/src/sys/sync/mutex/pthread.rs:9 |
| `std::sys::sync::mutex::sgx::Mutex` | `.inner` | `std::sys::sync::once_box::OnceBox<sys::pal::waitqueue::SpinMutex<sys::pal::waitqueue::WaitVariable<bool>>>` | tools/rust/library/std/src/sys/sync/mutex/sgx.rs:6 |
| `std::sys::sync::mutex::windows7::Mutex` | `.srwlock` | `cell::UnsafeCell<sys::c::SRWLOCK>` | tools/rust/library/std/src/sys/sync/mutex/windows7.rs:21 |
| `std::sys::sync::mutex::xous::Mutex` | `.locked` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/sync/mutex/xous.rs:17 |
| `std::sys::sync::mutex::xous::Mutex` | `.contended` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sys/sync/mutex/xous.rs:22 |
| `std::sys::sync::once::futex::CompletionGuard` | `.state_and_queued` | `sys::futex::Futex` | tools/rust/library/std/src/sys/sync/once/futex.rs:51 |
| `std::sys::sync::once::futex::CompletionGuard` | `.set_state_on_drop_to` | `sys::futex::Primitive` | tools/rust/library/std/src/sys/sync/once/futex.rs:52 |
| `std::sys::sync::once::futex::Once` | `.state_and_queued` | `sys::futex::Futex` | tools/rust/library/std/src/sys/sync/once/futex.rs:67 |
| `std::sys::sync::once::futex::OnceState` | `.poisoned` | `bool` | tools/rust/library/std/src/sys/sync/once/futex.rs:34 |
| `std::sys::sync::once::futex::OnceState` | `.set_state_to` | `cell::Cell<sys::futex::Primitive>` | tools/rust/library/std/src/sys/sync/once/futex.rs:35 |
| `std::sys::sync::once::no_threads::CompletionGuard` | `.state` | `cell::Cell<std::sys::sync::once::no_threads::State>` | tools/rust/library/std/src/sys/sync/once/no_threads.rs:23 |
| `std::sys::sync::once::no_threads::CompletionGuard` | `.set_state_on_drop_to` | `std::sys::sync::once::no_threads::State` | tools/rust/library/std/src/sys/sync/once/no_threads.rs:24 |
| `std::sys::sync::once::no_threads::Once` | `.state` | `cell::Cell<std::sys::sync::once::no_threads::State>` | tools/rust/library/std/src/sys/sync/once/no_threads.rs:6 |
| `std::sys::sync::once::no_threads::OnceState` | `.poisoned` | `bool` | tools/rust/library/std/src/sys/sync/once/no_threads.rs:10 |
| `std::sys::sync::once::no_threads::OnceState` | `.set_state_to` | `cell::Cell<std::sys::sync::once::no_threads::State>` | tools/rust/library/std/src/sys/sync/once/no_threads.rs:11 |
| `std::sys::sync::once::queue::Once` | `.state_and_queue` | `sync::atomic::AtomicPtr<()>` | tools/rust/library/std/src/sys/sync/once/queue.rs:68 |
| `std::sys::sync::once::queue::OnceState` | `.poisoned` | `bool` | tools/rust/library/std/src/sys/sync/once/queue.rs:72 |
| `std::sys::sync::once::queue::OnceState` | `.set_state_on_drop_to` | `cell::Cell<std::sys::sync::once::queue::StateAndQueue>` | tools/rust/library/std/src/sys/sync/once/queue.rs:73 |
| `std::sys::sync::once::queue::StateAndQueue` | `(alias)` | `*mut ()` | tools/rust/library/std/src/sys/sync/once/queue.rs:65 |
| `std::sys::sync::once::queue::Waiter` | `.thread` | `std::thread::Thread` | tools/rust/library/std/src/sys/sync/once/queue.rs:96 |
| `std::sys::sync::once::queue::Waiter` | `.signaled` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/sys/sync/once/queue.rs:97 |
| `std::sys::sync::once::queue::Waiter` | `.next` | `cell::Cell<*const std::sys::sync::once::queue::Waiter>` | tools/rust/library/std/src/sys/sync/once/queue.rs:98 |
| `std::sys::sync::once::queue::WaiterQueue` | `.state_and_queue` | `sync::atomic::AtomicPtr<()>` | tools/rust/library/std/src/sys/sync/once/queue.rs:105 |
| `std::sys::sync::once::queue::WaiterQueue` | `.set_state_on_drop_to` | `std::sys::sync::once::queue::StateAndQueue` | tools/rust/library/std/src/sys/sync/once/queue.rs:106 |
| `std::sys::sync::once_box::OnceBox` | `.ptr` | `sync::atomic::AtomicPtr<$T>` | tools/rust/library/std/src/sys/sync/once_box.rs:15 |
| `std::sys::sync::rwlock::futex::RwLock` | `.state` | `sys::futex::Futex` | tools/rust/library/std/src/sys/sync/rwlock/futex.rs:12 |
| `std::sys::sync::rwlock::futex::RwLock` | `.writer_notify` | `sys::futex::Futex` | tools/rust/library/std/src/sys/sync/rwlock/futex.rs:15 |
| `std::sys::sync::rwlock::no_threads::RwLock` | `.mode` | `cell::Cell<isize>` | tools/rust/library/std/src/sys/sync/rwlock/no_threads.rs:5 |
| `std::sys::sync::rwlock::queue::AtomicState` | `(alias)` | `sync::atomic::AtomicPtr<()>` | tools/rust/library/std/src/sys/sync/rwlock/queue.rs:124 |
| `std::sys::sync::rwlock::queue::State` | `(alias)` | `*mut ()` | tools/rust/library/std/src/sys/sync/rwlock/queue.rs:126 |
| `std::sys::sync::rwlock::solid::RwLock` | `.rwl` | `std::sys::pal::itron::spin::SpinIdOnceCell<()>` | tools/rust/library/std/src/sys/sync/rwlock/solid.rs:10 |
| `std::sys::sync::rwlock::teeos::RwLock` | `.inner` | `sys::sync::mutex::Mutex` | tools/rust/library/std/src/sys/sync/rwlock/teeos.rs:6 |
| `std::sys::sync::thread_parking::darwin::Parker` | `.semaphore` | `std::sys::sync::thread_parking::darwin::dispatch_semaphore_t` | tools/rust/library/std/src/sys/sync/thread_parking/darwin.rs:40 |
| `std::sys::sync::thread_parking::darwin::Parker` | `.state` | `sync::atomic::AtomicI8` | tools/rust/library/std/src/sys/sync/thread_parking/darwin.rs:41 |
| `std::sys::sync::thread_parking::darwin::dispatch_semaphore_t` | `(alias)` | `*mut core::ffi::c_void` | tools/rust/library/std/src/sys/sync/thread_parking/darwin.rs:20 |
| `std::sys::sync::thread_parking::darwin::dispatch_time_t` | `(alias)` | `u64` | tools/rust/library/std/src/sys/sync/thread_parking/darwin.rs:21 |
| `std::sys::sync::thread_parking::futex::Futex` | `(alias)` | `sys::futex::SmallFutex` | tools/rust/library/std/src/sys/sync/thread_parking/futex.rs:7 |
| `std::sys::sync::thread_parking::futex::Parker` | `.state` | `std::sys::sync::thread_parking::futex::Futex` | tools/rust/library/std/src/sys/sync/thread_parking/futex.rs:15 |
| `std::sys::sync::thread_parking::futex::State` | `(alias)` | `sys::futex::SmallPrimitive` | tools/rust/library/std/src/sys/sync/thread_parking/futex.rs:8 |
| `std::sys::sync::thread_parking::id::Parker` | `.state` | `sync::atomic::AtomicI8` | tools/rust/library/std/src/sys/sync/thread_parking/id.rs:18 |
| `std::sys::sync::thread_parking::id::Parker` | `.tid` | `cell::UnsafeCell<Option<sys::thread_parking::ThreadId>>` | tools/rust/library/std/src/sys/sync/thread_parking/id.rs:19 |
| `std::sys::sync::thread_parking::pthread::Parker` | `.state` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/sync/thread_parking/pthread.rs:14 |
| `std::sys::sync::thread_parking::pthread::Parker` | `.lock` | `sys::pal::sync::Mutex` | tools/rust/library/std/src/sys/sync/thread_parking/pthread.rs:15 |
| `std::sys::sync::thread_parking::pthread::Parker` | `.cvar` | `sys::pal::sync::Condvar` | tools/rust/library/std/src/sys/sync/thread_parking/pthread.rs:16 |
| `std::sys::sync::thread_parking::windows7::Parker` | `.state` | `sync::atomic::AtomicI8` | tools/rust/library/std/src/sys/sync/thread_parking/windows7.rs:69 |
| `std::sys::sync::thread_parking::xous::Parker` | `.state` | `sync::atomic::AtomicI8` | tools/rust/library/std/src/sys/sync/thread_parking/xous.rs:14 |
| `std::sys::thread_local::key::racy::LazyKey` | `.key` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/sys/thread_local/key/racy.rs:17 |
| `std::sys::thread_local::key::racy::LazyKey` | `.dtor` | `Option<opaque(unsafe extern "C" fn (* mut u8))>` | tools/rust/library/std/src/sys/thread_local/key/racy.rs:19 |
| `std::sys::thread_local::key::sgx::Key` | `(alias)` | `usize` | tools/rust/library/std/src/sys/thread_local/key/sgx.rs:3 |
| `std::sys::thread_local::key::unix::Key` | `(alias)` | `std::sys::thread_local::key::unix::libc::pthread_key_t` | tools/rust/library/std/src/sys/thread_local/key/unix.rs:23 |
| `std::sys::thread_local::key::unix::libc::pthread_key_t` | `(alias)` | `ffi::c_uint` | tools/rust/library/std/src/sys/thread_local/key/unix.rs:9 |
| `std::sys::thread_local::key::windows::Dtor` | `(alias)` | `opaque(unsafe extern "C" fn (* mut u8))` | tools/rust/library/std/src/sys/thread_local/key/windows.rs:35 |
| `std::sys::thread_local::key::windows::Key` | `(alias)` | `u32` | tools/rust/library/std/src/sys/thread_local/key/windows.rs:34 |
| `std::sys::thread_local::key::windows::LazyKey` | `.key` | `sync::atomic::AtomicU32` | tools/rust/library/std/src/sys/thread_local/key/windows.rs:41 |
| `std::sys::thread_local::key::windows::LazyKey` | `.dtor` | `Option<std::sys::thread_local::key::windows::Dtor>` | tools/rust/library/std/src/sys/thread_local/key/windows.rs:42 |
| `std::sys::thread_local::key::windows::LazyKey` | `.next` | `sync::atomic::AtomicPtr<std::sys::thread_local::key::windows::LazyKey>` | tools/rust/library/std/src/sys/thread_local/key/windows.rs:43 |
| `std::sys::thread_local::key::windows::LazyKey` | `.once` | `cell::UnsafeCell<sys::c::INIT_ONCE>` | tools/rust/library/std/src/sys/thread_local/key/windows.rs:47 |
| `std::sys::thread_local::key::xous::Dtor` | `(alias)` | `opaque(unsafe extern "C" fn (* mut u8))` | tools/rust/library/std/src/sys/thread_local/key/xous.rs:48 |
| `std::sys::thread_local::key::xous::Key` | `(alias)` | `usize` | tools/rust/library/std/src/sys/thread_local/key/xous.rs:47 |
| `std::sys::thread_local::key::xous::Node` | `.dtor` | `std::sys::thread_local::key::xous::Dtor` | tools/rust/library/std/src/sys/thread_local/key/xous.rs:148 |
| `std::sys::thread_local::key::xous::Node` | `.key` | `std::sys::thread_local::key::xous::Key` | tools/rust/library/std/src/sys/thread_local/key/xous.rs:149 |
| `std::sys::thread_local::key::xous::Node` | `.next` | `*mut std::sys::thread_local::key::xous::Node` | tools/rust/library/std/src/sys/thread_local/key/xous.rs:150 |
| `std::sys::thread_local::native::LocalPointer` | `.p` | `cell::Cell<*mut ()>` | tools/rust/library/std/src/sys/thread_local/native/mod.rs:124 |
| `std::sys::thread_local::native::eager::Storage` | `.state` | `cell::Cell<std::sys::thread_local::native::eager::State>` | tools/rust/library/std/src/sys/thread_local/native/eager.rs:14 |
| `std::sys::thread_local::native::eager::Storage` | `.val` | `cell::UnsafeCell<$T>` | tools/rust/library/std/src/sys/thread_local/native/eager.rs:15 |
| `std::sys::thread_local::native::lazy::State` | `.Alive` | `std::sys::thread_local::native::lazy::State::Alive<$T, $D>` | tools/rust/library/std/src/sys/thread_local/native/lazy.rs:24 |
| `std::sys::thread_local::native::lazy::State` | `.Destroyed` | `std::sys::thread_local::native::lazy::State::Destroyed<$T, $D>` | tools/rust/library/std/src/sys/thread_local/native/lazy.rs:25 |
| `std::sys::thread_local::native::lazy::State::Alive` | `[0]` | `$T` | tools/rust/library/std/src/sys/thread_local/native/lazy.rs:24 |
| `std::sys::thread_local::native::lazy::State::Destroyed` | `[0]` | `$D` | tools/rust/library/std/src/sys/thread_local/native/lazy.rs:25 |
| `std::sys::thread_local::native::lazy::Storage` | `.state` | `cell::UnsafeCell<std::sys::thread_local::native::lazy::State<$T, $D>>` | tools/rust/library/std/src/sys/thread_local/native/lazy.rs:30 |
| `std::sys::thread_local::os::LocalPointer` | `.key` | `key::LazyKey` | tools/rust/library/std/src/sys/thread_local/os.rs:159 |
| `std::sys::thread_local::os::Storage` | `.key` | `key::LazyKey` | tools/rust/library/std/src/sys/thread_local/os.rs:48 |
| `std::sys::thread_local::os::Storage` | `.marker` | `marker::PhantomData<cell::Cell<$T>>` | tools/rust/library/std/src/sys/thread_local/os.rs:49 |
| `std::sys::thread_local::os::Value` | `.value` | `$T` | tools/rust/library/std/src/sys/thread_local/os.rs:55 |
| `std::sys::thread_local::os::Value` | `.key` | `key::Key` | tools/rust/library/std/src/sys/thread_local/os.rs:57 |
| `std::sys::thread_local::statik::EagerStorage` | `.value` | `$T` | tools/rust/library/std/src/sys/thread_local/statik.rs:50 |
| `std::sys::thread_local::statik::LazyStorage` | `.value` | `cell::UnsafeCell<Option<$T>>` | tools/rust/library/std/src/sys/thread_local/statik.rs:58 |
| `std::sys::thread_local::statik::LocalPointer` | `.p` | `cell::Cell<*mut ()>` | tools/rust/library/std/src/sys/thread_local/statik.rs:107 |
| `std::sys_common::process::CommandEnv` | `.clear` | `bool` | tools/rust/library/std/src/sys_common/process.rs:13 |
| `std::sys_common::process::CommandEnv` | `.saw_path` | `bool` | tools/rust/library/std/src/sys_common/process.rs:14 |
| `std::sys_common::process::CommandEnv` | `.vars` | `collections::BTreeMap<sys::process::EnvKey, Option<std::ffi::os_str::OsString>>` | tools/rust/library/std/src/sys_common/process.rs:15 |
| `std::sys_common::process::CommandEnvs` | `.iter` | `collections::btree_map::Iter<sys::process::EnvKey, Option<std::ffi::os_str::OsString>>` | tools/rust/library/std/src/sys_common/process.rs:104 |
| `std::sys_common::wstr::WStrUnits` | `.lpwstr` | `ptr::NonNull<u16>` | tools/rust/library/std/src/sys_common/wstr.rs:12 |
| `std::sys_common::wstr::WStrUnits` | `.lifetime` | `marker::PhantomData<[u16]>` | tools/rust/library/std/src/sys_common/wstr.rs:14 |
| `std::sys_common::wtf8::CodePoint` | `.value` | `u32` | tools/rust/library/std/src/sys_common/wtf8.rs:43 |
| `std::sys_common::wtf8::EncodeWide` | `.code_points` | `std::sys_common::wtf8::Wtf8CodePoints` | tools/rust/library/std/src/sys_common/wtf8.rs:1011 |
| `std::sys_common::wtf8::EncodeWide` | `.extra` | `u16` | tools/rust/library/std/src/sys_common/wtf8.rs:1012 |
| `std::sys_common::wtf8::Wtf8` | `.bytes` | `[u8]` | tools/rust/library/std/src/sys_common/wtf8.rs:551 |
| `std::sys_common::wtf8::Wtf8Buf` | `.bytes` | `Vec<u8>` | tools/rust/library/std/src/sys_common/wtf8.rs:134 |
| `std::sys_common::wtf8::Wtf8Buf` | `.is_known_utf8` | `bool` | tools/rust/library/std/src/sys_common/wtf8.rs:142 |
| `std::sys_common::wtf8::Wtf8CodePoints` | `.bytes` | `slice::Iter<u8>` | tools/rust/library/std/src/sys_common/wtf8.rs:988 |
| `std::test_helpers::TempDir` | `[0]` | `std::path::PathBuf` | tools/rust/library/std/src/test_helpers.rs:30 |
| `std::thread::Builder` | `.name` | `Option<String>` | tools/rust/library/std/src/thread/mod.rs:262 |
| `std::thread::Builder` | `.stack_size` | `Option<usize>` | tools/rust/library/std/src/thread/mod.rs:264 |
| `std::thread::Builder` | `.no_hooks` | `bool` | tools/rust/library/std/src/thread/mod.rs:266 |
| `std::thread::Inner` | `.name` | `Option<std::thread::thread_name_string::ThreadNameString>` | tools/rust/library/std/src/thread/mod.rs:1384 |
| `std::thread::Inner` | `.id` | `std::thread::ThreadId` | tools/rust/library/std/src/thread/mod.rs:1385 |
| `std::thread::Inner` | `.parker` | `sys::sync::Parker` | tools/rust/library/std/src/thread/mod.rs:1386 |
| `std::thread::JoinHandle` | `[0]` | `std::thread::JoinInner<$T>` | tools/rust/library/std/src/thread/mod.rs:1819 |
| `std::thread::JoinInner` | `.native` | `sys::thread::Thread` | tools/rust/library/std/src/thread/mod.rs:1734 |
| `std::thread::JoinInner` | `.thread` | `std::thread::Thread` | tools/rust/library/std/src/thread/mod.rs:1735 |
| `std::thread::JoinInner` | `.packet` | `sync::Arc<std::thread::Packet<$T>>` | tools/rust/library/std/src/thread/mod.rs:1736 |
| `std::thread::Packet` | `.scope` | `Option<sync::Arc<std::thread::scoped::ScopeData>>` | tools/rust/library/std/src/thread/mod.rs:1689 |
| `std::thread::Packet` | `.result` | `cell::UnsafeCell<Option<std::thread::Result<$T>>>` | tools/rust/library/std/src/thread/mod.rs:1690 |
| `std::thread::Packet` | `._marker` | `marker::PhantomData<Option<std::thread::scoped::ScopeData>>` | tools/rust/library/std/src/thread/mod.rs:1691 |
| `std::thread::Result` | `(alias)` | `result::Result<$T, Box<opaque(dyn Any + Send + 'static)>>` | tools/rust/library/std/src/thread/mod.rs:1679 |
| `std::thread::Thread` | `.inner` | `pin::Pin<sync::Arc<std::thread::Inner>>` | tools/rust/library/std/src/thread/mod.rs:1416 |
| `std::thread::ThreadId` | `[0]` | `core::num::nonzero::NonZero<u64>` | tools/rust/library/std/src/thread/mod.rs:1186 |
| `std::thread::local::LocalKey` | `.inner` | `opaque(fn (Option < & mut Option < T > >) -> * const T)` | tools/rust/library/std/src/thread/local.rs:121 |
| `std::thread::scoped::Scope` | `.data` | `sync::Arc<std::thread::scoped::ScopeData>` | tools/rust/library/std/src/thread/scoped.rs:13 |
| `std::thread::scoped::Scope` | `.scope` | `marker::PhantomData<()>` | tools/rust/library/std/src/thread/scoped.rs:27 |
| `std::thread::scoped::Scope` | `.env` | `marker::PhantomData<()>` | tools/rust/library/std/src/thread/scoped.rs:28 |
| `std::thread::scoped::ScopeData` | `.num_running_threads` | `sync::atomic::AtomicUsize` | tools/rust/library/std/src/thread/scoped.rs:38 |
| `std::thread::scoped::ScopeData` | `.a_thread_panicked` | `sync::atomic::AtomicBool` | tools/rust/library/std/src/thread/scoped.rs:39 |
| `std::thread::scoped::ScopeData` | `.main_thread` | `std::thread::Thread` | tools/rust/library/std/src/thread/scoped.rs:40 |
| `std::thread::scoped::ScopedJoinHandle` | `[0]` | `std::thread::JoinInner<$T>` | tools/rust/library/std/src/thread/scoped.rs:35 |
| `std::thread::spawnhook::ChildSpawnHooks` | `.hooks` | `std::thread::spawnhook::SpawnHooks` | tools/rust/library/std/src/thread/spawnhook.rs:141 |
| `std::thread::spawnhook::ChildSpawnHooks` | `.to_run` | `Vec<Box<opaque(dyn FnOnce () + Send)>>` | tools/rust/library/std/src/thread/spawnhook.rs:142 |
| `std::thread::spawnhook::SpawnHook` | `.hook` | `Box<opaque(dyn Send + Sync + Fn (& Thread) -> Box < dyn Send + FnOnce () >)>` | tools/rust/library/std/src/thread/spawnhook.rs:32 |
| `std::thread::spawnhook::SpawnHook` | `.next` | `Option<sync::Arc<std::thread::spawnhook::SpawnHook>>` | tools/rust/library/std/src/thread/spawnhook.rs:33 |
| `std::thread::spawnhook::SpawnHooks` | `.first` | `Option<sync::Arc<std::thread::spawnhook::SpawnHook>>` | tools/rust/library/std/src/thread/spawnhook.rs:17 |
| `std::thread::thread_name_string::ThreadNameString` | `.inner` | `alloc::ffi::c_str::CString` | tools/rust/library/std/src/thread/mod.rs:1264 |
| `std::time::Instant` | `[0]` | `sys::time::Instant` | tools/rust/library/std/src/time.rs:158 |
| `std::time::SystemTime` | `[0]` | `sys::time::SystemTime` | tools/rust/library/std/src/time.rs:250 |
| `std::time::SystemTimeError` | `[0]` | `core::time::Duration` | tools/rust/library/std/src/time.rs:272 |

844 files; 1800 declarations/payload sorts; 43 ambiguous declarations; 0 declaration diagnostics; 1144 unexpanded item macros.
