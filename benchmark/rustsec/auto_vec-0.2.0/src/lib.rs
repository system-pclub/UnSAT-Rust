/*!
# AutoVec
Vec that automatically remove the child when the child is being dropped.  
**This crate is still in development! If you discovered any bugs or problems, please report them on [github issues](https://github.com/lluvz/AutoVec/), which will be much appreciated.** 
## Purpose of this crate
The purpose of this crate is create a container to gather variables to be read or processed collectively, while still allowing individual variables to be mutated freely.  
This means that the container need to store raw pointers rather than references. However, if the child variables are moved or dropped, the raw pointers will pointer to invalid datas. Thus, the dropped child need to be removed from the vector, and its place in the memory cannot be changed.  
Luckily, Rust provide the `Drop` trait which allows to define a `drop()` callback to be called when the variable is dropped, which can be used to inform the container to remove the dropped child.  
Rust also provide `pin` to prevent the child from begin moved thus changing its position in the memory.  
Also note that mutating the child itself, e.g. `child1 = child2` will cause child1 to be dropped thus removing it from the vec. This is needed for the reason that the above operation will change the pointer to its container.

# Example
```
use auto_vec::*;
let mut t1 = AutoChild::new(0);
let mut t2 = AutoChild::new(1);
let mut v = AutoVec::new();
v.add(&mut t1);
v.add(&mut t2);
println!("{:?}", v);
drop(t1);
println!("{:?}", v);
// t1 has been automatically removed from the vector
```
 */
use std::ops::Deref;
use std::ops::DerefMut;
use std::pin::Pin;
/**
## Example
```
use auto_vec::*;
fn main() {
    let mut t1 = AutoChild::new(0);
    let mut t2 = AutoChild::new(1);
    let mut v = AutoVec::new();
    v.add(&mut t1);
    v.add(&mut t2);
    for i in &mut v {
        *i = 3;
    }
    println!("{:?}", v);
}
```
 */
mod deref; pub use deref::*;
mod index; pub use index::*;
mod iter; pub use iter::*;
#[derive(Debug)]
pub struct AutoVec<T> {
    raw: Pin<Box<RawAutoVec<T>>>
}
#[derive(Debug)]
pub struct RawAutoVec<T> {
    children: Vec<*const RawAutoChild<T>>,
}
#[derive(Debug)]
pub struct RawAutoChild<T> {
    parent: *const RawAutoVec<T>,
    index: usize,
    pub child: T,
}
#[derive(Debug)]
pub struct AutoChild<T> {
    raw: Pin<Box<RawAutoChild<T>>>
}
impl<T> AutoChild<T> {
    pub fn new(child: T) -> Self {
        Self {
            raw: Box::pin(
                RawAutoChild {
                    parent: 0 as _, index: 0,
                    child,
                }
            )
        }
    }
}
impl<T> Drop for AutoChild<T> {
    fn drop(&mut self) {
        if self.parent!= 0 as _ {
            unsafe {&mut*(self.parent as *mut RawAutoVec<T>)}.called_remove(self);
        }
    }
}
impl<T> Drop for AutoVec<T> {
    fn drop(&mut self) {
        for i in &self.children {
            unsafe {&mut*(*i as *mut RawAutoChild<T>)}.parent = 0 as _;
        }
    }
}
impl<T> RawAutoVec<T> {
    fn len(&self) -> usize{
        self.children.len()
    }
    #[inline]
    fn called_remove(&mut self, child: &RawAutoChild<T>) {
        self.children.swap_remove(child.index);
    }
}
impl<T> AutoVec<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            raw: Box::pin(
                RawAutoVec {
                    children: Vec::new(),
                }
            )
        }
    }
    /// Moves all the elements of other into self, leaving other empty.
    /// See [`Vec::append()`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.append) for more details.
    pub fn append(&mut self, other: &mut AutoVec<T>) {
        let other = &mut other.raw as &mut RawAutoVec<T>;
        for i in 0..other.len() {
            let child = unsafe {&mut*(other.children[i] as *mut RawAutoChild<T>)};
            child.parent = &self.raw as &RawAutoVec<T> as _;
            child.index = i + self.len();
        }
        self.children.append(&mut other.children);
    }
    /// If the child is already in the vec, it will not be added a second time.
    /// A normal child cannot be added to multiple containers, adding it to another vec will remove it from the previous one.
    pub fn add(&mut self, child: &mut RawAutoChild<T>) {
        if child.parent == 0 as _ {
            child.parent = &self.raw as &RawAutoVec<T> as _;
            child.index = self.children.len();
            self.children.push(child as *const _ as *const RawAutoChild<T>);
        } else {
            if child.parent != &self.raw as &RawAutoVec<T> as _ {
                unsafe {(child.parent as *mut AutoVec<T>).as_mut().unwrap().called_remove(&child)};
                child.parent = &self.raw as &RawAutoVec<T> as _;
                child.index = self.children.len();
                self.children.push(child as *const _ as *const RawAutoChild<T>);
            }
        }
    }
    /// Using [`Vec::swap_remove()`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.swap_remove)
    pub fn remove(&mut self, child: &mut RawAutoChild<T>) {
        self.children.swap_remove(child.index);
        child.parent = 0 as _;
    }
    pub fn clear(&mut self) {
        for i in 0..self.children.len() {
            unsafe {(&mut*(self.children[i] as *mut RawAutoChild<T>)).parent = 0 as _};
        }
        self.children.clear();
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.children.len()
    }
    /// Reexport [`Vec::shrink_to()`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.shrink_to)
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.children.shrink_to(min_capacity);
    }
    /// Reexport [`Vec::shrink_to_fit()`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.shrink_to_fit)
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.children.shrink_to_fit();
    }
    /// See [`Vec::with_capacity()`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.with_capacity) for more details.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            raw: Box::pin(
                RawAutoVec {
                    children: Vec::with_capacity(capacity)
                }
            )
        }
    }
}