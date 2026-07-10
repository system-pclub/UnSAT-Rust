use crate::node::{Color, EMPTY_REF};
use crate::tree::Tree;

pub struct StackNode {
    index: u32,
    left: u32,
    right: u32,
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> Tree<T> {
    pub fn with_sorted_array(empty: T, slice: &[T], extra_capacity: usize) -> Self {
        let n = slice.len();
        let mut tree = Tree::new(empty, n + extra_capacity);

        if n == 0 {
            return tree;
        }

        let mut visited: Vec<bool> = vec![false; n];

        let si = 0;
        let mid = n >> 1;
        visited[mid] = true;
        tree.insert_root(slice[si + mid].clone());

        let log = (n + 1).ilog2();
        let s0 = n >> 1;

        let mut j = 1;
        for i in 1..log {
            let color = if i & 1 == 0 { Color::Black } else { Color::Red };
            let mut s = s0;

            let ni = 1 << (i - 1);
            for _ in 0..ni {
                let p = ((j - 1) >> 1) + 1;

                let parent = tree.mut_node(p);
                parent.left = j + 1;
                parent.right = j + 2;

                let lt = s >> i;
                let left = tree.store.get_free_index();
                let left_node = tree.mut_node(left);
                left_node.parent = p;
                left_node.color = color;
                left_node.value = slice[si + lt].clone();

                s += n;

                let rt = s >> i;
                let right = tree.store.get_free_index();
                let right_node = tree.mut_node(right);
                right_node.parent = p;
                right_node.color = color;
                right_node.value = slice[si + rt].clone();

                s += n;

                j += 2;
                visited[lt] = true;
                visited[rt] = true;
            }
        }

        for i in 0..n {
            if !visited[i] {
                tree.insert(slice[si + i].clone());
            }
        }

        tree
    }

    pub fn ordered_list(&self) -> Vec<T> {
        if self.root == EMPTY_REF {
            return Vec::new();
        }

        let height = self.height();
        let mut list = Vec::with_capacity(1 << height);
        let mut stack = Vec::with_capacity(height);

        self.fill_ordered_list_with_stack(&mut list, &mut stack);

        list
    }

    #[inline]
    pub fn fill_ordered_list(&self, list: &mut Vec<T>) {
        let height = self.height();
        let mut stack = Vec::with_capacity(height);
        self.fill_ordered_list_with_stack(list, &mut stack);
    }

    pub fn fill_ordered_list_with_stack(&self, list: &mut Vec<T>, stack: &mut Vec<StackNode>) {
        list.clear();
        stack.clear();

        let root_node = self.node(self.root);
        stack.push(StackNode { index: self.root, left: root_node.left, right: root_node.right });

        while !stack.is_empty() {
            let last_stack_index = stack.len() - 1;
            let s = &mut stack[last_stack_index];

            if s.left != EMPTY_REF {
                let index = s.left;
                s.left = EMPTY_REF; // to skip next time

                // go down left
                let node = self.node(index);
                let left = node.left;
                let right = node.right;
                stack.push(StackNode { index, left, right });
            } else {
                if s.index != EMPTY_REF {
                    let index = s.index;
                    s.index = EMPTY_REF; // to skip next time

                    // add value
                    list.push(self.node(index).value.clone());
                }

                if s.right != EMPTY_REF {
                    let index = s.right;
                    s.right = EMPTY_REF; // to skip next time

                    // go down right
                    let node = self.node(index);
                    let left = node.left;
                    let right = node.right;
                    stack.push(StackNode { index, left, right });
                } else {
                    // go up
                    stack.pop();
                }
            }
        }
    }

    #[inline(always)]
    pub fn first_by_order(&self) -> u32 {
        if self.root == EMPTY_REF {
            EMPTY_REF
        } else {
            self.find_left_minimum(self.root)
        }
    }

    pub fn next_by_order(&self, index: u32) -> u32 {
        let this = self.node(index);
        if this.right != EMPTY_REF {
            self.find_left_minimum(this.right)
        } else {
            // first parent bigger
            let mut i = this.parent;
            while i != EMPTY_REF {
                let node = self.node(i);
                if node.value > this.value {
                    return i;
                } else {
                    i = node.parent;
                }
            }

            // last element
            EMPTY_REF
        }
    }
}