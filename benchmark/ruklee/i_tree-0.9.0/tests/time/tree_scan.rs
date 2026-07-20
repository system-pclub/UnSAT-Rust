use i_tree::node::{Color, EMPTY_REF};
use i_tree::tree::Tree;
use crate::time::time_interval_value::TimeIntervalValue;
use crate::time::time_value::TimeValue;

pub(super) struct TreeTimeScan {
    tree: Tree<TimeIntervalValue>,
}

impl TreeTimeScan {
    pub(super) fn new() -> Self {
        let tree = Tree::new(TimeIntervalValue {
            value: 0,
            start: 0,
            end: 0,
        }, 16);

        Self { tree }
    }

    pub(super) fn insert(&mut self, item: TimeIntervalValue, stop: i32) {
        let mut index = self.tree.root;
        let mut p_index = EMPTY_REF;
        let mut is_left = false;

        while index != EMPTY_REF {
            let node = self.tree.node(index);
            p_index = index;
            if node.value.end <= stop {
                index = self.tree.delete_index(index);
                p_index = EMPTY_REF;
            } else {
                is_left = item < node.value;
                if is_left {
                    index = node.left;
                } else {
                    index = node.right;
                }
            }
        }

        let new_index = self.tree.store.get_free_index();

        let new_node = self.tree.mut_node(new_index);
        new_node.left = EMPTY_REF;
        new_node.right = EMPTY_REF;
        new_node.color = Color::Red;
        new_node.value = item;
        new_node.parent = p_index;

        if p_index == EMPTY_REF {
            new_node.parent = EMPTY_REF;
            self.tree.root = new_index;
        } else {
            new_node.parent = p_index;

            let parent = self.tree.mut_node(p_index);
            if is_left {
                parent.left = new_index;
            } else {
                parent.right = new_index;
            }

            if parent.color == Color::Red {
                self.tree.fix_red_black_properties_after_insert(new_index, p_index);
            }
        }
    }

    pub(super) fn find_equal_or_lower(&mut self, t: &TimeValue) -> i32 {
        let index = self.find_equal_or_lower_index(t);
        if index != EMPTY_REF {
            self.tree.node(index).value.value
        } else {
            i32::MIN
        }
    }

    fn find_equal_or_lower_index(&mut self, t: &TimeValue) -> u32 {
        let mut index = self.tree.root;
        let mut result = EMPTY_REF;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.end <= t.time {
                index = self.tree.delete_index(index);
            } else {
                if node.value.value == t.value {
                    return index;
                } else if node.value.value < t.value {
                    result = index;
                    index = node.right;
                } else {
                    index = node.left;
                }
            }
        }

        return result;
    }
}