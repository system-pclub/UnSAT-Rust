use std::{
    cell::UnsafeCell,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

pub struct StaticType<T> {
    val: UnsafeCell<Option<T>>,
    init_lock: spin::Mutex<bool>,
}

impl<T> StaticType<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            val: UnsafeCell::new(None),
            init_lock: spin::Mutex::new(false),
        }
    }
    #[must_use]
    pub fn is_init(&self) -> bool {
        *self.init_lock.lock()
    }

    // plase use `init_call` or `get_or_init`
    // this is not thread safe
    // this fn not public
    #[inline(always)]
    fn init(&self, val: T) {
        *self.get_mut() = Some(val);
    }

    #[inline(always)]
    pub fn init_call<F: FnOnce() -> T>(&self, call: F) {
        let mut lock = self.init_lock.lock();
        if !*lock {
            *lock = true;
            self.init(call());
        }
    }

    #[inline(always)]
    pub fn get_or_init<F: FnOnce() -> T>(&self, call: F) -> &T {
        self.init_call(call);
        self.get_unchecked()
    }

    /// this fn not safed; please before use this fn must use `init_call` or `get_or_init`
    #[inline(always)]
    pub fn get(&self) -> Option<&T> {
        unsafe { &*self.val.get() }.as_ref()
    }

    /// this fn not safed; please before use this fn must use `init_call` or `get_or_init`
    #[inline(always)]
    pub fn get_unchecked(&self) -> &T {
        unsafe { &*self.val.get() }
            .as_ref()
            .unwrap_or_else(|| unreachable!("get_unchecked StaticType not set"))
    }

    /// this fn not safed; please before use this fn must use `init_call` or `get_or_init`
    #[inline(always)]
    pub fn get_mut(&self) -> &mut Option<T> {
        unsafe { &mut *self.val.get() }
    }

    #[inline(always)]
    pub fn get_safe(&self) -> Option<&T> {
        if !*self.init_lock.lock() {
            return None;
        }
        unsafe { &*self.val.get() }.as_ref()
    }

    // 强制drop内存, 请确保在此时没有使用此内存
    // force drop memory, please ensure that the memory is not used before this
    pub fn force_drop(&self) -> Option<T> {
        *self.init_lock.lock() = false;
        self.get_mut().take()
    }
}

impl<T> Deref for StaticType<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
            .unwrap_or_else(|| unreachable!("StaticType not set"))
    }
}

// impl derefmut
impl<T> DerefMut for StaticType<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
            .as_mut()
            .unwrap_or_else(|| unreachable!("StaticType not set"))
    }
}

impl<T> AsRef<T> for StaticType<T> {
    fn as_ref(&self) -> &T {
        self.get_unchecked()
    }
}

unsafe impl<T: Send> Send for StaticType<T> {}
unsafe impl<T: Sync> Sync for StaticType<T> {}

impl<T: Default> Default for StaticType<T> {
    fn default() -> Self {
        Self {
            val: Default::default(),
            init_lock: Default::default(),
        }
    }
}

impl<T: Debug> Debug for StaticType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.get();
        f.write_fmt(format_args!("{val:?}"))
    }
}

impl<T: Display> Display for StaticType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.get().map(|x| format!("{x}"));
        let val = val.unwrap_or_else(|| format!("None"));
        f.write_str(&val)
    }
}

#[test]
fn test_static_type() {
    //定义静态变量
    static STATIC_TYPE: StaticType<i32> = StaticType::new();

    // 没有初始化之前
    assert_eq!(format!("{}", STATIC_TYPE), "None");

    // 初始化数据
    STATIC_TYPE.init_call(|| 22);

    // 第二次初始化数据, 预期不执行
    STATIC_TYPE.init_call(|| 33);

    // 控制台输出相应的数据
    assert_eq!(format!("{}", STATIC_TYPE), "22");

    // 获取数据,数据相同
    assert_eq!(*STATIC_TYPE, 22);
}
