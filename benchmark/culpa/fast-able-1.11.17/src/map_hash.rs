use parking_lot::ReentrantMutexGuard;
use serde::{Deserializer, Serialize, Serializer};
use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::collections::{
    hash_map::IntoIter as MapIntoIter, hash_map::Iter as MapIter, hash_map::IterMut as MapIterMut,
    HashMap as Map, HashMap,
};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// this sync map used to many reader,writer less.space-for-time strategy
pub struct SyncHashMap<K: Eq + Hash, V> {
    dirty: UnsafeCell<Map<K, V>>,

    #[cfg(feature = "reentrant_lock")]
    lock: parking_lot::ReentrantMutex<()>,

    #[cfg(feature = "std_lock")]
    lock: std::sync::Mutex<()>,
}

/// this is safety, dirty mutex ensure
unsafe impl<K: Eq + Hash, V> Send for SyncHashMap<K, V> {}

/// this is safety, dirty mutex ensure
unsafe impl<K: Eq + Hash, V> Sync for SyncHashMap<K, V> {}

impl<K, V> std::ops::Index<&K> for SyncHashMap<K, V>
where
    K: Eq + Hash,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        unsafe { &(&*self.dirty.get())[index] }
    }
}

impl<K, V> SyncHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    pub fn new() -> Self {
        Self {
            dirty: UnsafeCell::new(Map::new()),
            lock: Default::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dirty: UnsafeCell::new(Map::with_capacity(capacity)),
            lock: Default::default(),
        }
    }

    pub fn with_map(map: Map<K, V>) -> Self {
        Self {
            dirty: UnsafeCell::new(map),
            lock: Default::default(),
        }
    }

    pub fn insert(&self, k: K, v: V) -> Option<V> {
        let _lock = self.lock.lock();
        let m = unsafe { &mut *self.dirty.get() };
        let r = m.insert(k, v);
        r
    }

    pub fn insert_mut(&mut self, k: K, v: V) -> Option<V> {
        let m = unsafe { &mut *self.dirty.get() };
        m.insert(k, v)
    }

    pub fn remove(&self, k: &K) -> Option<V> {
        let g = self.lock.lock();
        let m = unsafe { &mut *self.dirty.get() };
        let r = m.remove(k);
        drop(g);
        r
    }

    pub fn remove_mut(&mut self, k: &K) -> Option<V> {
        let m = unsafe { &mut *self.dirty.get() };
        m.remove(k)
    }

    pub fn len(&self) -> usize {
        unsafe { (&*self.dirty.get()).len() }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (&*self.dirty.get()).is_empty() }
    }

    pub fn clear(&self) {
        let g = self.lock.lock();
        let m = unsafe { &mut *self.dirty.get() };
        m.clear();
        drop(g);
    }

    pub fn clear_mut(&mut self) {
        let m = unsafe { &mut *self.dirty.get() };
        m.clear();
    }

    pub fn shrink_to_fit(&self) {
        let g = self.lock.lock();
        let m = unsafe { &mut *self.dirty.get() };
        m.shrink_to_fit();
        drop(g);
    }

    pub fn shrink_to_fit_mut(&mut self) {
        let m = unsafe { &mut *self.dirty.get() };
        m.shrink_to_fit()
    }

    pub fn from(map: Map<K, V>) -> Self
    where
        K: Eq + Hash,
    {
        let s = Self::with_map(map);
        s
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// Since reading a map is unlocked, it is very fast
    ///
    /// test bench_sync_hash_map_read   ... bench:           8 ns/iter (+/- 0)
    /// # Examples
    ///
    /// ```
    /// use dark_std::sync::{SyncHashMap};
    ///
    /// let mut map = SyncHashMap::new();
    /// map.insert_mut(1, "a");
    /// assert_eq!(*map.get(&1).unwrap(), "a");
    /// assert_eq!(map.get(&2).is_none(), true);
    /// ```
    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        unsafe { (&*self.dirty.get()).get(k) }
    }

    #[inline]
    pub fn get_mut<Q: ?Sized>(&self, k: &Q) -> Option<SyncMapRefMut<'_, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let m = unsafe { &mut *self.dirty.get() };

        #[cfg(feature = "std_lock")]
        let _g = match self.lock.lock() {
            Ok(v) => v,
            Err(_) => return None,
        };

        #[cfg(feature = "reentrant_lock")]
        let _g = self.lock.lock();

        Some(SyncMapRefMut {
            _g,
            value: m.get_mut(k)?,
        })
    }

    #[inline]
    pub fn contains_key(&self, x: &K) -> bool
    where
        K: PartialEq,
    {
        let m = unsafe { &mut *self.dirty.get() };
        m.contains_key(x)
    }

    pub fn iter_mut(&self) -> IterMut<'_, K, V> {
        let m = unsafe { &mut *self.dirty.get() };

        #[cfg(feature = "std_lock")]
        let _g = self.lock.lock().unwrap();

        #[cfg(feature = "reentrant_lock")]
        let _g = self.lock.lock();

        return IterMut {
            _g,
            inner: m.iter_mut(),
        };
    }

    pub fn iter(&self) -> MapIter<'_, K, V> {
        let m = unsafe { &*self.dirty.get() };
        return m.iter();
    }

    pub fn dirty_ref(&self) -> &HashMap<K, V> {
        unsafe { &*self.dirty.get() }
    }

    pub fn into_inner(self) -> HashMap<K, V> {
        self.dirty.into_inner()
    }
}

pub struct SyncMapRefMut<'a, V> {
    #[cfg(feature = "reentrant_lock")]
    _g: ReentrantMutexGuard<'a, ()>,

    #[cfg(feature = "std_lock")]
    _g: std::sync::MutexGuard<'a, ()>,

    value: &'a mut V,
}

impl<'a, V> Deref for SyncMapRefMut<'_, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, V> DerefMut for SyncMapRefMut<'_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<'a, V> Debug for SyncMapRefMut<'_, V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<'a, V> PartialEq<Self> for SyncMapRefMut<'_, V>
where
    V: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<'a, V> Eq for SyncMapRefMut<'_, V> where V: Eq {}

pub struct IterMy<'a, K, V> {
    inner: MapIter<'a, K, V>,
}

impl<'a, K, V> Deref for IterMy<'a, K, V> {
    type Target = MapIter<'a, K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, K, V> Iterator for IterMy<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}


pub struct IterMut<'a, K, V> {
    #[cfg(feature = "reentrant_lock")]
    _g: ReentrantMutexGuard<'a, ()>,

    #[cfg(feature = "std_lock")]
    _g: std::sync::MutexGuard<'a, ()>,

    inner: MapIterMut<'a, K, V>,
}

impl<'a, K, V> Deref for IterMut<'a, K, V> {
    type Target = MapIterMut<'a, K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, K, V> DerefMut for IterMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> IntoIterator for &'a SyncHashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = MapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe { (&*self.dirty.get()).iter() }
    }
}

impl<K, V> IntoIterator for SyncHashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    type IntoIter = MapIntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.dirty.into_inner().into_iter()
    }
}

impl<K: Eq + Hash, V> From<Map<K, V>> for SyncHashMap<K, V> {
    fn from(arg: Map<K, V>) -> Self {
        Self::from(arg)
    }
}

impl<K, V> serde::Serialize for SyncHashMap<K, V>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.dirty_ref().serialize(serializer)
    }
}

impl<'de, K, V> serde::Deserialize<'de> for SyncHashMap<K, V>
where
    K: Eq + Hash + serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let m = Map::deserialize(deserializer)?;
        Ok(Self::from(m))
    }
}

impl<K, V> Debug for SyncHashMap<K, V>
where
    K: Eq + Hash + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.dirty_ref().fmt(f)
    }
}

impl<K: Clone + Eq + Hash, V: Clone> Clone for SyncHashMap<K, V> {
    fn clone(&self) -> Self {
        let c = (*self.dirty_ref()).clone();
        SyncHashMap::from(c)
    }
}

///
pub mod buckets {
    use super::{SyncHashMap, SyncMapRefMut};
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Clone)]
    pub struct SyncHashMapB<K: Eq + Hash, V> {
        inner: Vec<SyncHashMap<K, V>>,
        len: usize,
    }
    /*
    pub trait Bk: Eq + Hash {
        fn k(&self) -> u64;
    }

    impl Bk for u32 {
        fn k(&self) -> u64 {
            *self as u64
        }
    }

    impl Bk for u64 {
        fn k(&self) -> u64 {
            *self
        }
    }

    impl Bk for i64 {
        fn k(&self) -> u64 {
            self.abs() as u64
        }
    }

    impl Bk for i32 {
        fn k(&self) -> u64 {
            self.abs() as u64
        }
    } */

    impl<K: Eq + Hash, V> SyncHashMapB<K, V> {
        pub fn new(bucket_count: Option<usize>) -> Self {
            let count = bucket_count.unwrap_or_else(|| 10);
            let mut arr = vec![];
            for _ in 0..count {
                arr.push(SyncHashMap::new());
            }
            Self {
                inner: arr,
                len: count,
            }
        }

        fn key_conv_to_index(&self, k: &K) -> usize {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            k.hash(&mut hasher);
            let hash = hasher.finish();

            // let hash = k.k();

            // println!("hash: {hash}");
            let index = (hash % self.len as u64) as usize;
            // println!("index: {index}");
            index
        }

        /// ```
        /// use fast_able::{SyncHashMapB};
        ///
        /// let mut map = SyncHashMapB::new(None);
        /// map.insert(1, "a");
        /// map.insert(2, "b");
        /// assert_eq!(*map.get(&1).unwrap(), "a");
        /// assert_eq!(map.get(&2).is_none(), false);
        /// assert_eq!(map.get(&3).is_none(), true);
        /// ```
        pub fn insert(&self, k: K, v: V) -> Option<V> {
            let index = self.key_conv_to_index(&k);
            self.inner[index].insert(k, v)
        }

        pub fn insert_mut(&mut self, k: K, v: V) -> Option<V> {
            let index = self.key_conv_to_index(&k);
            self.inner[index].insert_mut(k, v)
        }

        pub fn remove(&self, k: &K) -> Option<V> {
            let index = self.key_conv_to_index(&k);
            self.inner[index].remove(k)
        }

        pub fn is_empty(&self) -> bool {
            for ele in &self.inner {
                if !ele.is_empty() {
                    return false;
                }
            }
            true
        }

        pub fn len(&self) -> usize {
            let mut len = 0;
            for ele in &self.inner {
                len += ele.len();
            }
            len
        }

        pub fn clear(&self) {
            for ele in &self.inner {
                ele.clear();
            }
        }

        /// Returns a reference to the value corresponding to the key.
        ///
        /// The key may be any borrowed form of the map's key type, but
        /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
        /// the key type.
        ///
        /// Since reading a map is unlocked, it is very fast
        ///
        /// test bench_sync_hash_map_read   ... bench:           8 ns/iter (+/- 0)
        /// # Examples
        ///
        /// ```
        /// use fast_able::{SyncHashMapB};
        ///
        /// let mut map = SyncHashMapB::new(None);
        /// map.insert_mut(1, "a");
        /// assert_eq!(*map.get(&1).unwrap(), "a");
        /// assert_eq!(map.get(&2).is_none(), true);
        /// ```
        #[inline]
        pub fn get(&self, k: &K) -> Option<&V> {
            let index = self.key_conv_to_index(k);
            self.inner[index].get(k)
        }

        pub fn get_mut(&self, k: &K) -> Option<SyncMapRefMut<'_, V>> {
            let index = self.key_conv_to_index(k);
            self.inner[index].get_mut(k)
        }
    }

    /* impl<'a, K: Eq + Hash, V> Iterator for &'a SyncHashMapB<K, V> {
        type Item = &'a Self;

        fn next(&mut self) -> Option<Self::Item> {
            for ele in &mut self.inner {
                ele.into_iter()
            }
            None
        }
    } */

    /* impl<'a, K, V> IntoIterator for &'a SyncHashMapB<K, V>
    where
        K: Eq + Hash,
    {
        type Item = (&'a K, &'a V);
        type IntoIter = Vec<(K, V)>;

        fn into_iter(self) -> Self::IntoIter {

            let mut iter_arr = vec![];
            for ele in self.inner {
                let iter = unsafe { (&*ele.dirty.get()).iter() };
                iter_arr.push(iter);
            }
            iter_arr

        }
    } */
}
