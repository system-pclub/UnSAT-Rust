use i_float::int::point::IntPoint;
use i_tree::node::{Color, EMPTY_REF};
use i_tree::tree::Tree;
use crate::edge::segment::IdSegment;

pub(super) struct TreeScan {
    pub(super) tree: Tree<IdSegment>,
}

impl TreeScan {
    pub(crate) fn insert(&mut self, item: IdSegment, stop: i32) {
        let mut index = self.tree.root;
        let mut p_index = EMPTY_REF;
        let mut is_left = false;

        while index != EMPTY_REF {
            let node = self.tree.node(index);
            p_index = index;
            if node.value.segment.b.x <= stop {
                let n_parent = node.parent;
                _ = self.tree.delete_index(index);
                if n_parent != EMPTY_REF {
                    index = n_parent;
                } else {
                    index = self.tree.root;
                    p_index = EMPTY_REF;
                }
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
            self.tree.root = new_index;
        } else {
            if is_left {
                self.tree.mut_node(p_index).left = new_index;
            } else {
                self.tree.mut_node(p_index).right = new_index;
            }

            if self.tree.node(p_index).color == Color::Red {
                self.tree.fix_red_black_properties_after_insert(new_index, p_index);
            }
        }
    }

    pub(crate) fn find_under(&mut self, p: &IntPoint, stop: i32) -> Option<IdSegment> {
        let mut index = self.tree.root;
        let mut result: u32 = EMPTY_REF;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.segment.b.x <= stop {
                let n_parent = node.parent;
                _ = self.tree.delete_index(index);
                if n_parent != EMPTY_REF {
                    index = n_parent;
                } else {
                    index = self.tree.root;
                }
            } else {
                if node.value.segment.is_under(p) {
                    result = index;
                    index = node.right;
                } else {
                    index = node.left;
                }
            }
        }

        if result == EMPTY_REF {
            None
        } else {
            Some(self.tree.node(result).value.clone())
        }
    }
}