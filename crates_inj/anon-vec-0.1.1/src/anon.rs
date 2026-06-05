
use std::any::{TypeId, Any};

/// ## Anonymous Type 
/// 
/// Emulates dynamic typing for efficient, albeit unsafe, data storage.  
///
/// Internally, Anon is a `Vec<u8>`. It works by casting a type to 
/// a slice and adding it to the vector.  The inner value can then be downcasted to either
/// a &mut T or a &T. 
/// 
/// ## Safety Disclaimer
/// 
/// Use of Anon is inherently unsafe! Attempting to access and modify the data CAN
/// cause major problems.  Be careful!
/// 
/// Any structs that are allocated to Anon should have `#[repr(C)]` to ensure
/// consistent alignment.  Failure to do so may cause the data to be come garbled. 
/// 
/// ## Usage
/// 
/// Anon provides methods for construction either in-place with a value or 
/// uninit for later initialization.
/// 
/// ```
/// use anon_vec::Anon;
/// 
/// // in-place construction
/// let x: i32 = 5;
/// let mut anon = Anon::new::<i32>(x);
/// 
/// // access the inner value with `cast_ref` and `cast_mut`
/// let v: &i32 = anon.cast_ref::<i32>();
/// let v: &mut i32 = anon.cast_mut::<i32>();
/// 
/// // uninit construction, to be initialized later. 
/// let mut anon = Anon::uninit();
/// 
/// ```
pub struct Anon {
    /// The Value, represented as a `Vec<u8>`.
    /// 
    /// To make this possible, cast the value to a slice, then
    /// copy it into `inner`.  This is very unsafe, but it allows
    /// for dynamic typing in data storage systems like game engines.
    inner: Vec<u8>,

    /// The TypeId of the value stored in this Anon. 
    /// Can be used to check for cast validity.
    typeid: TypeId,
}

impl Anon {

    // --- // Initializers // --- //

    /// Creates a new Anonymous Type in-place.
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::Anon;
    /// 
    /// let x: i32 = 5;
    /// let anon = Anon::new::<i32>(x);
    /// ```
    /// ## Memory Safety
    /// 
    /// Anon will consume the value on `Anon::init` or `Anon::new`, so the value
    /// will inherit the lifetime of Anon, allowing you to store `Vec<T>` as anon safely. 
    pub fn new<T>(val: T) -> Self 
    where
        T: Any + 'static,
    {
        let mut inner: Vec<u8> = (0..std::mem::size_of::<T>()).map(|_| 0).collect();

        let ptr = inner.as_mut_ptr() as *mut T;
        unsafe { *(ptr) = val; }

        Self {
            inner, typeid: TypeId::of::<T>(),
        }
    }

    /// Creates a new Anonymous Type in-place from a *const u8 which represents T.
    pub fn from_ptr(ptr: *const u8, size: usize, typeid: TypeId) -> Self {
        Self {
            inner: Vec::from_iter((0..size).map(|i| unsafe { *(ptr.add(i)) })),
            typeid,
        }
    }

    /// Creates a new, uninitialized Anonymous Type. 
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::Anon;
    /// 
    /// // declare the wrapper
    /// let mut anon = Anon::uninit();
    /// 
    /// // initialize later
    /// let x: i32 = 5;
    /// anon.init::<i32>(x);
    /// ```
    /// 
    /// ## Memory Safety
    /// 
    /// Anon will consume the value on `Anon::init` or `Anon::new`, so the value
    /// will inherit the lifetime of Anon, allowing you to store `Vec<T>` as anon safely. 
    /// 
    /// If you try and access an uninitialized Anon, you will access memory incorrectly.
    pub fn uninit() -> Self {
        Self {
            inner: Vec::new(),
            typeid: TypeId::of::<i32>(),
        }
    }

    /// Initializes an Anon created with Anon::uninit().
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::Anon;
    /// 
    /// // declare the wrapper
    /// let mut anon = Anon::uninit();
    /// 
    /// // initialize later
    /// let x: i32 = 5;
    /// anon.init::<i32>(x);
    /// ```
    /// 
    /// ## Memory Safety
    /// 
    /// Anon will consume the value on `Anon::init` or `Anon::new`, so the value
    /// will inherit the lifetime of Anon, allowing you to store `Vec<T>` as anon safely. 
    pub fn init<T>(&mut self, val: T) 
    where
        T: Any + 'static,
    {
        // extend the vector by the size of T, filling with 0s. 
        self.inner.extend((0..std::mem::size_of::<T>()).map(|_| 0));
        // cast the *mut u8 to *mut T (which we can do because T is 'static and *mut u8 is the same size)
        let ptr = self.inner.as_mut_ptr() as *mut T;
        // dereference ptr and assign it to the value
        unsafe { *(ptr) = val; }
        // assign the typeid correctly.
        self.typeid = TypeId::of::<T>();
    }

    // --- // Accessors // --- //

    pub fn inner(self) -> Vec<u8> {
        self.inner
    }

    /// The Size of this Anon, in bytes. 
    pub fn size(&self) -> usize {
        self.inner.len()
    }

    /// Returns the TypeId of the types this anon stores.
    /// Will return `TypeId::of::<i32>()` if uninit.
    pub fn typeid(&self) -> TypeId {
        self.typeid
    }

    /// Get a slice that points to the inner value.
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    /// Get a mutable slice that points to the inner value.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    /// Check whether or not the inner value is empty, or uninit. 
    pub fn is_uninit(&self) -> bool {
        self.inner.is_empty()
    }

    /// Cast the inner value to T.
    /// 
    /// Inernally, the inner value is Vec<u8>. To access, the
    /// *mut u8 inside the vec is cast to *const T and returned. 
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::Anon;
    /// 
    /// let x: i32 = 5;
    /// let anon = Anon::new(x);
    /// 
    /// let v: &i32 = anon.cast_ref::<i32>();
    /// ```
    pub fn cast_ref<T>(&self) -> &T
    where
        T: Any + 'static,
    {
        unsafe { &*(self.inner.as_ptr() as *const T) }
    }

    /// Cast the inner value to T.
    /// 
    /// Inernally, the inner value is Vec<u8>. To access, the
    /// *mut u8 inside the vec is cast to *mut T and returned. 
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::Anon;
    /// 
    /// let x: i32 = 5;
    /// let mut anon = Anon::new(x);
    /// 
    /// let v: &mut i32 = anon.cast_mut::<i32>();
    /// ```
    pub fn cast_mut<T>(&mut self) -> &mut T
    where
        T: Any + 'static,
    {
        unsafe { &mut *(self.inner.as_mut_ptr() as *mut T) }
    }

    /// Consume the Anon, returning the inner value as T.
    /// 
    /// ## Usage
    /// ```
    /// use anon_vec::Anon;
    /// let x: i32 = 5;
    /// let mut anon = Anon::new(x);
    /// 
    /// let v: i32 = anon.consume::<i32>();
    /// ```
    pub fn consume<T>(self) -> T
    where
        T: Any + Clone + 'static,
    {
        let out = unsafe { &*(self.inner.as_ptr() as *const T) };
        out.clone()
    }
}

#[cfg(test)]
mod tests {

    use crate::anon::Anon;

    #[repr(C)]
    #[derive(PartialEq, Debug, Clone)]
    struct Thing {
        pub a: i32,
        pub b: i32,
        pub c: i32,
    }

    impl Thing {
        fn new(a: i32, b: i32, c: i32) -> Self {
            Self { a, b, c }
        }

        fn sum(&self) -> i32 {
            self.a + self.b + self.c
        }
    }

    #[test]
    fn new() {
        let t = Thing::new(1, 2, 3);

        let anon = Anon::new(t);

        let thing = anon.cast_ref::<Thing>();

        assert_eq!(6, thing.sum());
    }

    #[test]
    fn uninit_init() {
        let mut anon = Anon::uninit();

        {
            let t = Thing::new(1, 2, 3);

            anon.init::<Thing>(t);
        }

        let thing = anon.cast_ref::<Thing>();

        assert_eq!(6, thing.sum());
    }

    #[test]
    fn vec() {
        let mut anon = Anon::uninit();

        {
            let t = 
                vec![
                    Thing::new(1, 2, 3),
                    Thing::new(1, 2, 3),
                    Thing::new(1, 2, 3),
                ];

            anon.init::<Vec<Thing>>(t);
        }

        let things = anon.cast_ref::<Vec<Thing>>();

        let v = things[0].sum() + things[1].sum() + things[2].sum();

        assert_eq!(v, 18);
    }

    #[test]
    fn consume() {
        let mut anon = Anon::uninit();

        {
            let t = 
                vec![
                    Thing::new(1, 2, 3),
                    Thing::new(1, 2, 3),
                    Thing::new(1, 2, 3),
                ];

            anon.init::<Vec<Thing>>(t);
        }

        let things = anon.consume::<Vec<Thing>>();

        let v = things[0].sum() + things[1].sum() + things[2].sum();

        assert_eq!(v, 18);
    }
}