use i_tree::node::{Color, EMPTY_REF};
use i_tree::tree::Tree;

#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;
    use rand::thread_rng;
    use i_tree::tree::Tree;
    use crate::TreeValidation;

    #[test]
    fn test_00() {
        let tree = Tree::new(0, 8);
        let a = tree.find(1).unwrap_or(i32::MAX);
        assert_eq!(a, i32::MAX);
    }

    #[test]
    fn test_01() {
        let mut tree = Tree::new(0, 8);
        tree.insert(5);
        let a0 = tree.find(1).unwrap_or(i32::MAX);
        let a1 = tree.find(5).unwrap_or(i32::MAX);
        let a2 = tree.find(7).unwrap_or(i32::MAX);
        assert_eq!(a0, i32::MAX);
        assert_eq!(a1, 5);
        assert_eq!(a2, i32::MAX);
    }

    #[test]
    fn test_02() {
        let mut tree = Tree::new(0, 8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(1);
        let a0 = tree.find(5).unwrap_or(i32::MAX);
        let a1 = tree.find(3).unwrap_or(i32::MAX);
        let a2 = tree.find(1).unwrap_or(i32::MAX);

        assert_eq!(a0, 5);
        assert_eq!(a1, 3);
        assert_eq!(a2, 1);
    }

    #[test]
    fn test_03() {
        let mut tree = Tree::new(0, 8);
        tree.insert(10);
        tree.insert(15);
        tree.insert(5);
        tree.insert(4);

        assert_eq!(tree.verify_red_property(tree.root), true);
    }

    #[test]
    fn test_04() {
        let mut tree = Tree::new(0, 8);
        tree.insert(10);
        tree.insert(15);
        tree.insert(5);
        tree.insert(4);
        tree.insert(6);

        tree.verify_height(tree.root);
    }

    #[test]
    fn test_05() {
        let mut tree = Tree::new(0, 8);
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);

        let value = tree.node(tree.root).value;

        assert_eq!(true, value == 20 || value == 10, "Rotation not performed correctly.");
        assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");

        tree.verify_height(tree.root);
    }

    #[test]
    fn test_06() {
        let mut tree = Tree::new(0, 8);
        tree.insert(10);
        tree.insert(15);
        tree.insert(5);
        tree.insert(4);
        tree.insert(6);
        tree.insert(20);

        tree.delete(&15);

        assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
        assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");

        tree.verify_height(tree.root);
    }

    #[test]
    fn test_7() {
        let mut tree = Tree::new(0, 8);
        tree.insert(40);
        tree.insert(20);
        tree.insert(60);
        tree.insert(10);
        tree.insert(30);
        tree.insert(50);
        tree.insert(70);

        assert_eq!(tree.value(tree.root), 40, "Root data is incorrect.");
        assert_eq!(tree.left_value(tree.root), 20, "Left child of root is incorrect.");
        assert_eq!(tree.right_value(tree.root), 60, "Right child of root is incorrect.");
    }

    #[test]
    fn test_8() {
        let mut tree = Tree::new(0, 8);
        tree.insert(2);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        tree.delete(&2);
        tree.delete(&1);

        assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
        assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
    }

    #[test]
    fn test_9a() {
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (1..=100).collect();
        for _ in 0..100 {
            values.shuffle(&mut rng);
            let mut tree = Tree::new(0, 128);
            for i in 0..values.len() {
                tree.insert(values[i])
            }

            for i in 0..20 {
                tree.delete(&values[i])
            }

            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }
    }

    #[test]
    fn test_9b() {
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (1..=100).collect();
        for _ in 0..100 {
            values.shuffle(&mut rng);
            let mut tree = Tree::new(0, 128);
            for i in 0..values.len() {
                tree.insert(values[i])
            }

            for i in 0..values.len() {
                tree.delete(&values[i])
            }

            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }
    }

    #[test]
    fn test_9c() {
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (1..=100).collect();
        for _ in 0..100 {
            values.shuffle(&mut rng);
            let mut tree = Tree::new(0, 128);
            let mut j = 0;
            while j < values.len() - 2 {
                tree.insert(values[j]);
                tree.insert(values[j + 1]);
                tree.insert(values[j + 2]);
                tree.delete(&values[j]);
                tree.insert(values[j]);
                j += 3
            }

            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }
    }

    #[test]
    fn test_10() {
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (1..=100).collect();
        values.shuffle(&mut rng);
        let mut tree = Tree::new(0, 128);
        for i in 0..values.len() {
            tree.insert(values[i])
        }

        for i in 0..values.len() {
            let res = tree.find(values[i]).unwrap_or(i32::MAX);
            assert_eq!(values[i], res, "Value not found after random insertions.");
        }
    }

    #[test]
    fn test_11() {
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (1..=100).collect();
        values.shuffle(&mut rng);
        let mut tree = Tree::new(0, 128);
        for i in 0..values.len() {
            tree.insert(values[i])
        }

        let depth = tree.max_depth(tree.root);
        let expected_max_depth = (2.0 * 101f64.log2()).round() as usize;
        assert_eq!(true, depth <= expected_max_depth);
    }

    #[test]
    fn test_12() {
        for _ in 0..100 {
            let mut tree = Tree::new(0, 16);
            let mut rng = thread_rng();
            let mut values: Vec<i32> = (1..=7).collect();
            values.shuffle(&mut rng);
            // print!("{:?}", &values);
            for value in values.iter() {
                tree.insert(value.clone());
            }

            for i in 0..3 {
                tree.delete(&values[i]);
            }

            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }
    }

    #[test]
    fn test_13() {
        let mut tree = Tree::new(0, 16);
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (1..=7).collect();
        values.shuffle(&mut rng);
        // print!("{:?}", &values);
        let mut j = 0;
        while j < values.len() - 2 {
            tree.insert(values[j]);
            tree.insert(values[j + 1]);
            tree.insert(values[j + 2]);
            tree.delete(&values[j]);
            j += 3
        }

        assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
        assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
    }

    #[test]
    fn test_14() {
        let mut tree = Tree::new(0, 16);
        let values = vec![1, 6, 2, 5, 4];

        for value in values.iter() {
            tree.insert(value.clone());
            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }

        for i in 0..2 {
            tree.delete(&values[i]);
            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }
    }

    #[test]
    fn test_15() {
        let mut tree = Tree::new(0, 16);
        let values = vec![5, 6, 1, 3, 4, 2];

        for value in values.iter() {
            tree.insert(value.clone());
            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }

        for i in 0..2 {
            tree.delete(&values[i]);
            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }
    }

    #[test]
    fn test_16() {
        let mut tree = Tree::new(0, 16);
        let values = vec![4, 1, 6, 3, 2, 5];

        for value in values.iter() {
            tree.insert(value.clone());
            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }

        for i in 0..2 {
            tree.delete(&values[i]);
            assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
            assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
        }
    }

    #[test]
    fn test_17() {
        let mut tree = Tree::new(0, 16);

        tree.insert(6);
        tree.insert(7);
        tree.insert(2);
        tree.delete(&6);

        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.delete(&1);

        tree.insert(5);

        assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
        assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
    }

    #[test]
    fn test_18() {
        let mut tree = Tree::new(0, 16);

        tree.insert(10);
        tree.insert(20);

        tree.delete(&10);
        tree.insert(0);
        tree.insert(3);
        tree.insert(6);

        tree.delete(&3);
        tree.insert(2);

        tree.delete(&2);
        tree.insert(4);

        tree.delete(&6);
        tree.delete(&0);
        tree.delete(&4);

        tree.insert(8);

        tree.delete(&20);
        tree.delete(&8);

        assert_eq!(true, tree.verify_red_property(tree.root), "Red node property violated after rotations.");
        assert_eq!(true, tree.verify_black_height_consistency(tree.root), "Black height inconsistent after deletion.");
    }
}

trait TreeValidation {
    fn value(&self, index: u32) -> i32;
    fn left_value(&self, index: u32) -> i32;
    fn right_value(&self, index: u32) -> i32;
    fn verify_red_property(&self, index: u32) -> bool;
    fn verify_height(&self, index: u32) -> usize;
    fn verify_black_height_consistency(&self, index: u32) -> bool;
    fn black_height(&self, index: u32) -> (bool, usize);
    fn max_depth(&self, index: u32) -> usize;
}

impl TreeValidation for Tree<i32> {
    fn value(&self, index: u32) -> i32 {
        if index == EMPTY_REF { return i32::MAX; }
        self.node(index).value
    }

    fn left_value(&self, index: u32) -> i32 {
        if index == EMPTY_REF { return i32::MAX; }
        let node = self.node(index);
        if node.left == EMPTY_REF { return i32::MAX; }

        self.node(node.left).value
    }

    fn right_value(&self, index: u32) -> i32 {
        if index == EMPTY_REF { return i32::MAX; }
        let node = self.node(index);
        if node.right == EMPTY_REF { return i32::MAX; }

        self.node(node.right).value
    }

    fn verify_red_property(&self, index: u32) -> bool {
        if index == EMPTY_REF { return true; }

        let node = self.node(index);

        if node.color == Color::Red {
            let is_left_black = self.is_black(node.left);
            let is_right_black = self.is_black(node.right);

            if !(is_left_black && is_right_black) {
                return false;
            }
        }

        self.verify_red_property(node.left) && self.verify_red_property(node.right)
    }

    fn verify_height(&self, index: u32) -> usize {
        if index == EMPTY_REF { return 1; }

        let node = self.node(index);

        let left_height = self.verify_height(node.left);
        let right_height = self.verify_height(node.right);
        assert_eq!(left_height, right_height, "Black height inconsistent.");

        if node.color == Color::Black {
            left_height + 1
        } else {
            left_height
        }
    }

    fn verify_black_height_consistency(&self, index: u32) -> bool {
        self.black_height(index).0
    }

    fn black_height(&self, index: u32) -> (bool, usize) {
        if index == EMPTY_REF { return (true, 1); }
        let node = self.node(index);
        let (left_consistent, left_height) = self.black_height(node.left);
        let (right_consistent, right_height) = self.black_height(node.right);

        let consistent = left_consistent && right_consistent && left_height == right_height;

        if node.color == Color::Black {
            (consistent, left_height + 1)
        } else {
            (consistent, left_height)
        }
    }

    fn max_depth(&self, index: u32) -> usize {
        if index == EMPTY_REF { return 0; }
        let node = self.node(index);

        let left_depth = self.max_depth(node.left);
        let right_depth = self.max_depth(node.right);
        1 + left_depth.max(right_depth)
    }
}