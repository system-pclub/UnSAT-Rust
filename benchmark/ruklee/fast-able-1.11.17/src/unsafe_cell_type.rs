use std::{
    cell::UnsafeCell,
    ops::{Add, Deref, Div, Mul, Sub},
};

/// unsafe 无锁类型, 请手动保证访问安全, api 与 AtomicCell 一致
pub struct U<T> {
    _inner: UnsafeCell<T>,
}

impl<T> U<T> {
    pub const fn new(v: T) -> U<T> {
        U {
            _inner: UnsafeCell::new(v),
        }
    }
    #[inline(always)]
    pub fn store(&self, v: T) {
        let s = unsafe { &mut *self._inner.get() };
        *s = v;
    }
    #[inline(always)]
    pub fn as_mut(&self) -> &mut T {
        unsafe { &mut *self._inner.get() }
    }
}

impl<T: Clone> Clone for U<T> {
    fn clone(&self) -> Self {
        Self {
            _inner: UnsafeCell::new(self.deref().clone()),
        }
    }
}

impl<T: Eq> Eq for U<T> {}

impl<T: PartialEq> PartialEq for U<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T: Default> Default for U<T> {
    fn default() -> Self {
        Self {
            _inner: Default::default(),
        }
    }
}

impl<T: Clone> U<T> {
    pub fn load(&self) -> T {
        self.deref().clone()
    }
    pub fn fetch_end(&self, v: T) -> T {
        let r = self.deref().clone();
        self.store(v);
        r
    }
}

impl<T: Add<Output = T> + Clone> U<T> {
    pub fn fetch_add(&self, v: T) -> T {
        let r = self.deref().clone();
        let s = unsafe { &mut *self._inner.get() };
        *s = self.deref().clone() + v;
        r
    }
}

impl<T: Sub<Output = T> + Clone> U<T> {
    pub fn fetch_sub(&self, v: T) -> T {
        let r = self.deref().clone();
        let s = unsafe { &mut *self._inner.get() };
        *s = self.deref().clone() - v;
        r
    }
}

use core::fmt::Debug;
impl<T: Debug> Debug for U<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.deref()))
    }
}

use core::fmt::Display;
impl<T: Display> Display for U<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.deref()))
    }
}

impl<T: Add<Output = T> + Clone> Add for U<T> {
    fn add(self, rhs: Self) -> Self::Output {
        let v1 = unsafe { &*self._inner.get() };
        let v2 = unsafe { &*rhs._inner.get() };
        Self::Output::new(v1.clone() + v2.clone())
    }

    type Output = U<T>;
}

impl<T: Sub<Output = T> + Clone> Sub for U<T> {
    fn sub(self, rhs: Self) -> Self::Output {
        let v1 = unsafe { &*self._inner.get() };
        let v2 = unsafe { &*rhs._inner.get() };
        Self::Output::new(v1.clone() - v2.clone())
    }

    type Output = U<T>;
}

impl<T: Div<Output = T> + Clone> Div for U<T> {
    fn div(self, rhs: Self) -> Self::Output {
        let v1 = unsafe { &*self._inner.get() };
        let v2 = unsafe { &*rhs._inner.get() };
        Self::Output::new(v1.clone() / v2.clone())
    }

    type Output = U<T>;
}

impl<T: Mul<Output = T> + Clone> Mul for U<T> {
    fn mul(self, rhs: Self) -> Self::Output {
        let v1 = unsafe { &*self._inner.get() };
        let v2 = unsafe { &*rhs._inner.get() };
        Self::Output::new(v1.clone() * v2.clone())
    }

    type Output = U<T>;
}

unsafe impl<T: Send> Send for U<T> {}
unsafe impl<T: Sync> Sync for U<T> {}

impl<T> Deref for U<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self._inner.get() }
    }
}

impl<T> AsRef<T> for U<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> From<T> for U<T> {
    fn from(value: T) -> Self {
        U {
            _inner: UnsafeCell::new(value),
        }
    }
}

#[test]
fn test() {
    let v1 = U::new(1);
    let v2 = 1.into();
    let v3 = v1.clone() + v2;
    println!("r: {}", v3);
    assert_eq!(v3.load(), 2);

    let v2 = 1.into();
    let v3 = v1.clone() - v2;
    println!("r: {}", v3);
    assert_eq!(v3.load(), 0);

    let v3 = v1.fetch_add(3);
    println!("r: {}", v3);
    assert_eq!(v3, 1);
    assert_eq!(v1.load(), 4);

    let v3 = v1.fetch_end(5);
    println!("r: {}", v3);
    assert_eq!(v3, 4);

    let v3 = v1.load();
    println!("r: {}", v3);
    assert_eq!(v3, 5);

    v1.store(6);
    println!("r: {}", v1.load());
    assert_eq!(v1.load(), 6);

    v1.fetch_sub(5);
    println!("r: {}", v1.load());
    assert_eq!(v1.load(), 1);
}

#[test]
fn test_mut_thread() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    static V: U<usize> = U::new(0);
    std::thread::spawn(move || loop {
        V.load();
    });
    std::thread::spawn(move || loop {
        V.load();
    });
    std::thread::spawn(move || loop {
        V.load();
    });
    std::thread::spawn(move || loop {
        V.load();
    });
    std::thread::spawn(move || loop {
        V.load();
    });
    std::thread::spawn(move || loop {
        V.load();
    });

    for i in 0..1000000 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let r = V.fetch_add(i);
        debug!("loop {}: {r}", i);
    }
}
