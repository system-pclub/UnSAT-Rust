# Anonymous Vector

Anonymous Types and Anonymously Typed Vectors for Dynamic Type Emulation in Rust.

anon_vec is essentially an easier-to-use replacement for Box<dyn Any>, without 
the safety checks that come along with Box<dyn Any>.  It was created for strata-ecs
with the goal of storing Components in AnonVecs. 

## Memory Safety

Usage of Anon, AnonVec, and AnonVec Iterators is inherently unsafe and should be used with
caution.  Attempting to access these types incorrectly can result in data leaks, incorrect
values, and invalid memory access, leading to catastrophic errors. 

Although it may sometime in the future, anon_vec does not currently implement
any error checking.  You are expected to make good decisions about managing the unsafety
of these types. 

## Usage

Use this Crate if you need Dynamic Typing and are ok with some unsafe code. 

anon_vec supports Anonymous single-types called [`Anon`]. These types are
useful for moving data around in your engine, and the inner value can only be accessed
in a scope where the type is known (`T`).

```rs
use anon_vec::Anon;
use std::mem::size_of;
use std::any::TypeId;

// you can create anons from existing values...
let x: i32 = 5;
let mut anon1 = Anon::new::<i32>(x);

// the value within can then be accessed.
let y: &i32 = anon1.cast_ref::<i32>();
let z: &i32 = anon1.cast_ref::<i32>();

// Anons can also be uninitialized.
let mut anon2 = Anon::uninit();

if anon2.is_uninit() {
    anon2.init::<i32>(5); 
} else {
    // do something to the value
}
```

anon_vec supports Anonymous Vectors called [`AnonVec`]. This type is useful when you
want to store a Vec<T>, but you don't know T at compile-time. AnonVec converts T to u8, allowing 
for sequential storage of T with minimal heap allocation. AnonVec can be accessed either with T,
or by converting the value to Anon. 

```rs
use anon_vec::AnonVec;
use anon_vec::Anon;
use std::mem::size_of;
use std::any::TypeId;

// You can create AnonVecs from a T...
let mut anon1 = AnonVec::new::<i32>();

// ...or from a size.
let mut anon2 = AnonVec::from_size(size_of::<i32>(), TypeId::of::<i32>());

// values can be pushed to the vec.
anon1.push::<i32>(5);
anon1.push::<i32>(5);

// and removed.
anon1.remove(1);

// AnonVecs can also be uninitialized
let mut anon3 = Anon::uninit();

if anon3.is_uninit() {
    anon3.init::<i32>(5);
} else {
    // do something to the value.
}
```

anon_vec supports typed iterators over [`AnonVec`], and chaining iterators together.
This is particularly useful for ECS, and the motivation behind why chaining iterators were included.

['AnonIter'] can be created with AnonVec::iter::<T>().  It takes a pointer to the inner value of
[`AnonIter`] and is not lifetime-checked. If the Iter outlives its parent AnonVec, iterating it will
access invalid data.

[`AnonIterMut`] is the same as AnonIter, and can be created with AnonVec::iter_mut::<T>(). 

[`AnonChain`] and [`AnonChainMut`] use the push method to construct a chain of iterators that can be
iterated as one. 