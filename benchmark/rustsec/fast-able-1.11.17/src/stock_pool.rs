use std::{
    cell::UnsafeCell,
    ops::{Index, IndexMut},
};

/// 设计一个高性能股票池, 数据全部以数组的形式存储, 通过数组的下标来获取数据; 此下标是股票代码转换而来;
/// 4个数组, 一个存储上海主板股票, 一个存储深圳主板股票, 一个存储创业板股票; 还有一个数组存储科创板股票;
/// 每个数组的长度是固定的, 都根据A股市场的股票数量来确定; 例如上海主板股票数量是1500, 则数组的长度是1500;
pub struct StockPool<T> {
    stock_list: [Vec<UnsafeCell<Option<T>>>; 10],
}

impl<T: Default> StockPool<T> {
    // 会生成默认值的股票池;
    #[deprecated(note = "plase use new_default")]
    #[must_use]
    pub fn new() -> StockPool<T> {
        Self::new_default()
    }

    #[must_use]
    pub fn new_default() -> StockPool<T> {
        let get = || {
            let mut add_v = Vec::with_capacity(9999);
            for _ in 0..10_0000 {
                add_v.push(UnsafeCell::new(Some(T::default())));
            }
            add_v
        };
        StockPool {
            stock_list: [
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
            ],
        }
    }
}

impl<T> StockPool<T> {
    // 初始化股票池; 调用此方法则必须调用insert方法来插入数据;
    #[must_use]
    pub fn new_empty() -> StockPool<T> {
        let get = || {
            let mut add_v = Vec::with_capacity(9999);
            for _ in 0..10_0000 {
                add_v.push(UnsafeCell::new(None));
            }
            add_v
        };
        StockPool {
            stock_list: [
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
                get(),
            ],
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.all_items().len()
    }

    #[inline(always)]
    fn _get<'a>(&'a self, i7: i32) -> &'a mut Option<T> {
        // 高性能写法
        let i7 = i7 as usize;
        let i6 = i7 % 1_000_000;
        let i = i6 / 1000_00;
        let i5 = i6 % 1000_00;
        let r = &self.stock_list[i]
            .get(i5)
            .unwrap_or_else(|| unreachable!("股票代码在股票池中不存在: {i7}({i}-{i5})"));
        unsafe { &mut *r.get() }
    }

    pub fn all_items(&self) -> Vec<&T> {
        let mut items = Vec::new();
        for ele in self.stock_list.iter() {
            for ele in ele {
                if let Some(item) = unsafe { &*ele.get() } {
                    items.push(item);
                }
            }
        }

        items
    }

    #[inline(always)]
    pub fn is_empty(&self, i7: i32) -> bool {
        self._get(i7).is_none()
    }

    /// 获取股票数据; 股票代码是7位数字, 1开头说明是上海票, 2开头说明是深圳票; 其它6位数与常规股票代码一样;
    #[inline(always)]
    pub fn get(&self, i7: i32) -> Option<&T> {
        self._get(i7).as_ref()
    }

    #[inline(always)]
    pub fn get_unchecked(&self, i7: i32) -> &T {
        self._get(i7)
            .as_ref()
            .unwrap_or_else(|| unreachable!("股票数据为空: {i7}"))
    }

    #[inline(always)]
    pub fn get_mut(&self, i7: i32) -> Option<&mut T> {
        self._get(i7).as_mut()
    }

    #[inline(always)]
    pub fn insert(&self, i7: i32, val: T) -> Option<T> {
        self._get(i7).replace(val)
    }

    #[inline(always)]
    pub fn remove(&self, i7: i32) -> Option<T> {
        self._get(i7).take()
    }
}

impl<T: Default> Index<i32> for StockPool<T> {
    type Output = T;
    fn index(&self, i7: i32) -> &Self::Output {
        self.get(i7)
            .unwrap_or_else(|| unreachable!("股票代码在股票池中不存在: {}", i7))
    }
}

impl<T: Default> IndexMut<i32> for StockPool<T> {
    fn index_mut(&mut self, i7: i32) -> &mut Self::Output {
        self.get_mut(i7)
            .unwrap_or_else(|| unreachable!("股票代码在股票池中不存在: {}", i7))
    }
}

unsafe impl<T: Sync> Sync for StockPool<T> {}
unsafe impl<T: Send> Send for StockPool<T> {}

// impl iter
// impl <T> Iterator for StockPool<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// impl<T: Default + Debug> IndexMut<i32> for StockPool<T> {
//     type Output = T;
//     fn index_mut(&mut self, index: i32) -> &mut Self::Output {
//         self.get(i7)
//             .expect(format!("股票代码在股票池中不存在: {}", i7).as_str())
//     }
// }

#[test]
fn test_stock_pool() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = StockPool::<i32>::new_empty();
    pool.insert(1000001, 100);
    println!("pool.all_items().len {}", pool.all_items().len());
    assert_eq!(pool[1000001], 100);

    let pool = StockPool::<i32>::new_default();
    assert_eq!(pool[1000001], 0);

    let pool = StockPool::<String>::new_empty();
    assert_eq!(pool.get(1000001), None);
    // assert_eq!(pool[1000001], ""); // panic, because the stock code is not exist. please use fn insert to insert data.

    let pool = StockPool::<String>::new_default();
    assert_eq!(pool[1000001], "".to_string());

    let pool = StockPool::<i32>::new_default();
    println!("pool.all_items().len {}", pool.all_items().len());

    let pool = StockPool::<String>::new_default();
    assert_eq!(pool[1000001], "".to_string());
}

#[test]
fn test_stock_pool_2() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = StockPool::<i32>::new_empty();
    for i in 1..=99999 {
        pool.insert(1_600_000 + i, i + 100000);
    }
    for i in 1..=99999 {
        pool.insert(2_000_000 + i, i + 300000);
    }
    for i in 1..=99999 {
        pool.insert(2_300_000 + i, i + 300000);
    }
    for i in 1..=99999 {
        pool.insert(400_000 + i, i + 400000);
    }

    for i in 1..=99999 {
        pool.insert(1_200_000 + i, i + 220000);
    }

    for i in 1..=99999 {
        pool.insert(1_500_000 + i, i + 550000);
    }

    for i in 1..=99999 {
        let ii = 1_600_000 + i;
        assert_eq!((ii, pool[ii]), (ii, i + 100000));
    }
    for i in 1..=99999 {
        assert_eq!(pool[2_000_000 + i], i + 300000);
    }
    for i in 1..=99999 {
        let ii = 2_300_000 + i;
        assert_eq!((ii, pool[ii]), (ii, i + 300000));
    }
    for i in 1..=99999 {
        let ii = 400_000 + i;
        assert_eq!((ii, pool[ii]), (ii, i + 400000));
    }

    for i in 1..=99999 {
        assert_eq!(pool[1_500_000 + i], i + 550000);
    }
    for i in 1..=99999 {
        assert_eq!(pool[1_200_000 + i], i + 220000);
    }
}
