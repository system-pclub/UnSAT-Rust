
use std::any::{TypeId, Any};

use crate::anon::Anon;
use crate::iter::{
    AnonIter, 
    AnonIterMut
};

/// An Anonymously typed Vector.
/// 
/// Internally, AnonVec is a Vec<u8>. 
/// When pushing to an AnonVec, `T` is converted to `*const u8`.
/// When getting from an AnonVec, `*const u8` is converted to `T`.
/// 
/// ## Usage
/// 
/// Anon Vec is intended for use in data systems where the type or size of the values
/// stored cannot be known at compile-time.  It is a more lax approach to `Box<dyn Any>`.
/// ```
/// use anon_vec::AnonVec;
/// 
/// let mut anon = AnonVec::new::<i32>();
/// anon.push::<i32>(5);
/// anon.push::<i32>(10);
/// anon.push::<i32>(15);
/// 
/// let x = anon.get_ref::<i32>(1);
/// ```
/// `AnonVec` can also work with `Anon` values. 
/// ```
/// use anon_vec::{AnonVec, Anon};
/// use std::mem::size_of;
/// use std::any::TypeId;
/// 
/// // Create AnonVec using the size and typeid.
/// let mut vec = AnonVec::from_size(size_of::<i32>(), TypeId::of::<i32>());
/// vec.push_anon(Anon::new::<i32>(5));
/// vec.push_anon(Anon::new::<i32>(10));
/// vec.push_anon(Anon::new::<i32>(15));
/// 
/// // move index 1 out and into `anon`.
/// let anon: Anon = vec.remove_get_anon(1);
/// 
/// let x: &i32 = anon.cast_ref::<i32>();
/// ```
pub struct AnonVec {
    /// Vec<T>, represented as Vec<u8>. 
    inner: Vec<u8>,
    /// The `std::mem::size_of` each element.
    size: usize,
    /// The length of the vector, in terms of `inner.len() / size.`
    len: usize,
    /// The TypeId of this AnonVec. 
    typeid: TypeId,
}

impl AnonVec {

    // --- // Constructors // --- //

    /// Creates a new Anonymously Typed Vector in-place.
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::AnonVec;
    /// 
    /// let mut anon = AnonVec::new::<i32>();
    /// anon.push::<i32>(5);
    /// ```
    pub fn new<T>() -> Self 
    where
        T: Any + 'static,
    {
        Self {
            inner: Vec::new(),
            size: std::mem::size_of::<T>(),
            len: 0,
            typeid: TypeId::of::<T>(),
        }
    }

    /// Creates a new Anonymously Typed Vector using the 
    /// size and TypeId of the value to be stored.
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::AnonVec;
    /// use std::mem::size_of;
    /// use std::any::TypeId;
    /// 
    /// let mut anon = AnonVec::from_size(size_of::<i32>(), TypeId::of::<i32>());
    /// anon.push::<i32>(5);
    /// ```
    pub fn from_size(size: usize, typeid: TypeId) -> Self {
        Self {
            inner: Vec::new(),
            size,
            len: 0,
            typeid,
        }
    }

    /// Creates an Uninitialized Anonymously Typed Vector
    /// 
    /// MUST be initialized before access by calling init::<T>. 
    /// If you can't call init::<T>, call init_size instead.
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::AnonVec;
    /// 
    /// let mut vec = AnonVec::uninit();
    /// 
    /// if vec.is_uninit() {
    ///     vec.init::<i32>();
    /// }
    /// 
    /// // do stuff with anon_vec
    /// ```
    pub fn uninit() -> Self {
        Self {
            inner: Vec::new(),
            size: 0,
            len: 0,
            typeid: TypeId::of::<i32>(),
        }
    }

    /// Initializes a previously uninitialized AnonVec.
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::AnonVec;
    /// 
    /// let mut vec = AnonVec::uninit();
    /// 
    /// if vec.is_uninit() {
    ///     vec.init::<i32>();
    /// }
    /// 
    /// // do stuff with anon_vec
    /// ```
    pub fn init<T>(&mut self) 
    where
        T: Any + 'static,
    {
        self.size = std::mem::size_of::<T>();
        self.typeid = TypeId::of::<T>();
    }

    /// Initializes a previously uninitialized AnonVec.
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::AnonVec;
    /// use std::mem::size_of;
    /// use std::any::TypeId;
    /// 
    /// let mut vec = AnonVec::uninit();
    /// 
    /// if vec.is_uninit() {
    ///     vec.init_size(size_of::<i32>(), TypeId::of::<i32>());
    /// }
    /// 
    /// // do stuff with anon_vec
    /// ```
    pub fn init_size(&mut self, size: usize, typeid: TypeId) {
        self.size = size;
        self.typeid = typeid;
    }

    // --- // Accessors // --- //

    /// The TypeId associated with this AnonVec.
    pub fn typeid(&self) -> TypeId {
        self.typeid
    }

    /// The size, in bytes, each element of this AnonVec holds.
    pub fn size(&self) -> usize {
        self.size
    }

    /// The number of elements this AnonVec holds. (as T)
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether or not this AnonVec has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Whether or not the size of this AnonVec is 0.
    pub fn is_uninit(&self) -> bool {
        self.size == 0
    }

    /// Get a reference to the interior value at index as T.
    pub fn get_ref<T>(&self, index: usize) -> &T 
    where
        T: Any + 'static,
    {   
        let ptr = self.inner.as_ptr() as *const T;
        unsafe { &*(ptr.add(index)) }
    } 

    /// Get a mutable reference to the interior value at index as T.
    pub fn get_mut<T>(&mut self, index: usize) -> &mut T
    where
        T: Any + 'static,
    {
        let ptr = self.inner.as_mut_ptr() as *mut T;
        unsafe { &mut *(ptr.add(index)) }
    }

    /// Reserves `additional` number of BYTES. 
    /// If you want to reserve size_of::<T>, use `reserve` instead.
    /// 
    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Vec<T>`. The collection may reserve more space to
    /// speculatively avoid frequent reallocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1];
    /// vec.reserve(10);
    /// assert!(vec.capacity() >= 11);
    /// ```
    pub fn reserve_bytes(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Vec<T>`. The collection may reserve more space to
    /// speculatively avoid frequent reallocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1];
    /// vec.reserve(10);
    /// assert!(vec.capacity() >= 11);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        if !self.is_uninit() {
            self.inner.reserve(additional * self.size);
        }
    }

    // --- // Operators // -- //

    /// Appends an element to the back of this AnonVec.
    pub fn push<T>(&mut self, val: T)
    where
        T: Any + 'static,
    {
        let v = &val as *const T as *const u8;
        for i in 0..self.size {
            unsafe {
                self.inner.push(*(v.add(i)))
            }
        }
        self.len += 1;
    }

    /// Appends an anonymous element to the back of this AnonVec.
    pub fn push_anon(&mut self, anon: Anon) {
        let v = anon.inner();
        for _ in 0..self.size {
            self.inner.extend(v.iter());
        }
        self.len += 1;
    }

    /// Inserts an element at `index`, moving all elements after it to the right.
    pub fn insert<T>(&mut self, val: T, index: usize) {
        let v = &val as *const T as *const u8;
        let index = index * self.size;

        for i in (0..self.size).rev() {
            unsafe {
                self.inner.insert(index, *(v.add(i)))
            }
        }
        self.len += 1;
    }

    /// Inserts an anonymous element at `index`, moving all elements after it to the right.
    pub fn insert_anon(&mut self, anon: Anon, index: usize) {
        let v = anon.inner();
        let index = index * self.size;

        for i in (0..self.size).rev() {
            self.inner.insert(index, v[i])
        }
        self.len += 1;
    }

    /// Removes an element at `index`. 
    pub fn remove(&mut self, index: usize) {
        let index = index * self.size;
        for i in (index..index + self.size).rev() {
            self.inner.remove(i);
        }
        self.len -= 1;
    }

    /// Removes and returns the element at `index`. 
    pub fn remove_get<T>(&mut self, index: usize) -> T
    where
        T: Any + Clone + 'static,
    {
        let ptr = self.inner.as_mut_ptr() as *mut T;
        let out = unsafe { &*(ptr.add(index)) }.clone();

        let index = index * self.size;
        for i in (index..index + self.size).rev() {
            self.inner.remove(i);
        }
        self.len -= 1;
        out
    }

    /// Removes and returns the element at `index` as an anonymous type.
    pub fn remove_get_anon(&mut self, index: usize) -> Anon {
        let ptr = self.inner.as_ptr();
        let out = Anon::from_ptr(ptr, self.size, self.typeid);

        let index = index * self.size;
        for i in (index..index + self.size).rev() {
            self.inner.remove(i);
        }
        self.len -= 1;
        out
    }

    /// Pops off and returns the last element in the Vec.
    pub fn pop<T>(&mut self) -> Option<T> 
    where
        T: Any + Clone + 'static,
    {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.remove_get::<T>(self.len() - 1))
        }
    }

    /// Pops off and returns the last element in the Vec as an Anon. 
    pub fn pop_anon(&mut self) -> Option<Anon> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.remove_get_anon(self.len() - 1))
        }
    }

    /// Remove the last element after copying it into `index`. 
    /// MUCH faster than `remove`, in certain situations. 
    pub fn remove_swap(&mut self, index: usize) {
        if index == self.len - 1 {
            for _ in 0..self.size {
                self.inner.pop();
            }
        } else {
            let index = self.size * index;
            for i in (0..self.size).rev() {
                self.inner[index + i] = self.inner.pop().unwrap()
            }
        }
        self.len -= 1;
    }

    /// Immutably Iterate over this AnonVec as T.
    pub fn iter<T>(&self) -> AnonIter<T> {
        AnonIter {
            data: self.inner.as_ptr() as *const T,
            curr: 0,
            len: self.inner.len(),
        }
    } 

    /// Mutably Iterate over this AnonVec as T. 
    pub fn iter_mut<T>(&mut self) -> AnonIterMut<T> {
        AnonIterMut {
            data: self.inner.as_mut_ptr() as *mut T,
            curr: 0,
            len: self.inner.len(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::any::TypeId;

    use crate::vec::AnonVec;

    const THING: Thing = Thing { a: 1, b: 2, c: 3 };

    #[repr(C)]
    #[derive(PartialEq, Debug, Clone)]
    struct Thing {
        pub a: i32,
        pub b: i32,
        pub c: i32,
    }

    impl Thing {
        fn sum(&self) -> i32 {
            self.a + self.b + self.c
        }
    }

    #[test]
    fn new() {
        let mut anon = AnonVec::new::<Thing>();

        {
            anon.push::<Thing>(THING);
            anon.push::<Thing>(THING);
            anon.push::<Thing>(THING);
        }

        let t1 = anon.get_ref::<Thing>(0);
        let t2 = anon.get_ref::<Thing>(1);
        let t3 = anon.get_ref::<Thing>(2);

        let v = t1.sum() + t2.sum() + t3.sum();

        assert_eq!(v, 18);
    }

    #[test]
    fn from_size() {
        let mut anon = AnonVec::from_size(std::mem::size_of::<Thing>(), TypeId::of::<Thing>());

        {
            anon.push::<Thing>(THING);
            anon.push::<Thing>(THING);
            anon.push::<Thing>(THING);
        }

        let t1 = anon.get_ref::<Thing>(0);
        let t2 = anon.get_ref::<Thing>(1);
        let t3 = anon.get_ref::<Thing>(2);

        let v = t1.sum() + t2.sum() + t3.sum();

        assert_eq!(v, 18);
    }

    #[test]
    fn uninit_init() {
        let mut anon = AnonVec::uninit();

        {
            anon.init::<Thing>();
            anon.push::<Thing>(THING);
            anon.push::<Thing>(THING);
            anon.push::<Thing>(THING);
        }

        let t1 = anon.get_ref::<Thing>(0);
        let t2 = anon.get_ref::<Thing>(1);
        let t3 = anon.get_ref::<Thing>(2);

        let v = t1.sum() + t2.sum() + t3.sum();

        assert_eq!(v, 18);
    }
}