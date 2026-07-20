#[macro_use]
extern crate log;

pub mod defer;
pub mod error;

pub mod map_btree;
pub mod map_hash;
pub mod vec;
pub mod wg;

pub mod statis;

pub mod unsafe_cell_type;

pub mod stock_pool;
pub mod fast_thread_pool;

pub mod elapsed_time;

pub mod static_type;

#[cfg(test)]
mod unsat_ir_instantiations {
    use super::map_btree::SyncBtreeMap;
    use super::map_hash::SyncHashMap;
    use super::static_type::StaticType;
    use super::unsafe_cell_type::U;
    use super::vec::SyncVec;

    #[test]
    #[ignore = "compile-only LLVM IR instantiation"]
    fn instantiate_mirscan_callers() {
        let mut btree = SyncBtreeMap::<u8, u8>::new();
        let _ = btree.insert(1, 1);
        let _ = btree.insert_mut(2, 2);
        let _ = btree.remove(&1);
        let _ = btree.remove_mut(&2);
        let _ = btree.len();
        let _ = btree.is_empty();
        btree.clear();
        btree.clear_mut();
        let _ = btree.insert_mut(1, 1);
        let _ = btree.get(&1);
        drop(btree.get_mut(&1));
        let _ = btree.contains_key(&1);
        let _ = btree.iter();
        drop(btree.iter_mut());
        let _ = btree.dirty_ref();

        let mut hash = SyncHashMap::<u8, u8>::new();
        let _ = hash.insert(1, 1);
        let _ = hash.insert_mut(2, 2);
        let _ = hash.remove(&1);
        let _ = hash.remove_mut(&2);
        let _ = hash.len();
        let _ = hash.is_empty();
        hash.clear();
        hash.clear_mut();
        hash.shrink_to_fit();
        hash.shrink_to_fit_mut();
        let _ = hash.insert_mut(1, 1);
        let _ = hash.get(&1);
        drop(hash.get_mut(&1));
        let _ = hash.contains_key(&1);
        let _ = hash.iter();
        drop(hash.iter_mut());
        let _ = hash.dirty_ref();

        let mut vec = SyncVec::<u8>::new();
        let _ = vec.insert(0, 1);
        let _ = vec.insert_mut(0, 2);
        let _ = vec.push_vec(vec![3]);
        let _ = vec.push_mut(4);
        let _ = vec.pop();
        let _ = vec.pop_mut();
        let _ = vec.remove(0);
        let _ = vec.remove_mut(0);
        let _ = vec.is_empty();
        vec.clear();
        vec.shrink_to_fit();
        let _ = vec.get(0);
        drop(vec.get_mut(0));
        let _ = vec.contains(&0);
        let _ = vec.iter();
        drop(vec.iter_mut());
        let _ = vec.dirty_ref();

        let value = U::new(1_u8);
        let _ = value.as_mut();
        let static_value = StaticType::<u8>::new();
        static_value.init_call(|| 1_u8);
        let _ = static_value.get_unchecked();
        let _ = static_value.get_safe();

        super::fast_thread_pool::lite::_test_thread_lite(0);
    }
}
