#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use i_tree::tree::Tree;

    #[test]
    fn test_00() {
        let mut vec = vec![6, 3, 4, 8, 1];
        let mut tree = Tree::new(0, vec.len());
        for a in vec.iter() {
            tree.insert(a.clone());
        }

        let ordered = tree.ordered_list();
        vec.sort();

        assert_eq!(vec, ordered);
    }

    #[test]
    fn test_01() {
        let n = 100;
        let mut vec: Vec<i32> = (0..n as i32).collect();
        let mut tree = Tree::new(0, n);
        let mut rng = thread_rng();
        for _ in 0..10000 {
            tree.clear_all();
            vec.shuffle(&mut rng);
            for a in vec.iter() {
                tree.insert(a.clone());
            }

            let ordered = tree.ordered_list();
            vec.sort();

            assert_eq!(vec, ordered);
        }
    }
}