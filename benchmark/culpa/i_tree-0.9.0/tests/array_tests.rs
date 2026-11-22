#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use i_tree::node::EMPTY_REF;
    use i_tree::tree::Tree;

    #[test]
    fn test_ordered_list_0() {
        let mut list = vec![6, 3, 4, 8, 1];
        let mut tree = Tree::new(0, list.len());
        for a in list.iter() {
            tree.insert(a.clone());
        }

        let ordered = tree.ordered_list();
        list.sort_unstable();

        assert_eq!(ordered, list);
    }

    #[test]
    fn test_ordered_list_random() {
        let n = 100;
        let mut list: Vec<_> = (0..n).collect();
        let sorted = list.clone();
        let mut tree = Tree::new(0, list.len());
        for _ in 0..10000 {
            list.shuffle(&mut rand::thread_rng());
            tree.clear_all();
            for a in list.iter() {
                tree.insert(a.clone());
            }

            let ordered = tree.ordered_list();
            assert_eq!(ordered, sorted);
        }
    }

    #[test]
    fn test_init_0() {
        let list = vec![0];
        let tree = Tree::with_sorted_array(0, &list, 0);
        let ordered = tree.ordered_list();
        assert_eq!(ordered, list);
    }

    #[test]
    fn test_init_1() {
        let list = vec![0, 1];
        let tree = Tree::with_sorted_array(0, &list, 0);
        let ordered = tree.ordered_list();
        assert_eq!(ordered, list);
    }

    #[test]
    fn test_init_2() {
        let list: Vec<_> = (0..2).collect();
        let tree = Tree::with_sorted_array(0, &list, 0);
        let ordered = tree.ordered_list();
        assert_eq!(ordered, list);
    }

    #[test]
    fn test_init_sequence() {
        for i in 3..=3000 {
            let list: Vec<_> = (0..i).collect();
            let tree = Tree::with_sorted_array(0, &list, 0);
            let ordered = tree.ordered_list();
            assert_eq!(ordered, list);
        }
    }

    #[test]
    fn test_next_by_order_0() {
        test_next_by_order(&[0]);
    }

    #[test]
    fn test_next_by_order_1() {
        test_next_by_order(&[0, 1]);
    }

    #[test]
    fn test_next_by_order_2() {
        let vec: Vec<_> = (0..2).collect();
        test_next_by_order(&vec);
    }

    #[test]
    fn test_next_by_order_sequence() {
        for i in 3..=1000 {
            let vec: Vec<_> = (0..i).collect();
            test_next_by_order(&vec);
        }
    }

    fn test_next_by_order(list: &[i32]) {
        let tree = Tree::with_sorted_array(0, &list, 0);
        let mut n_index = tree.first_by_order();
        let mut ordered = Vec::with_capacity(list.len());
        while n_index != EMPTY_REF {
            ordered.push(tree.node(n_index).value.clone());
            n_index = tree.next_by_order(n_index);
        }

        assert_eq!(ordered, list);
    }
}