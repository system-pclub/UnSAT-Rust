use crate::lattice::Lattice;
use crate::lattice::LatticeNode;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct PMANode<'a> {
    edges: HashMap<&'a str, usize>,
    fail: usize,
    matched: Vec<usize>,
}

pub struct LatticeSearcher<'a> {
    pma: Vec<PMANode<'a>>,
    patterns: Vec<Vec<&'a str>>,
}

impl<'a> LatticeSearcher<'a> {
    pub fn new(patterns: Vec<Vec<&'a str>>) -> LatticeSearcher<'a> {
        let mut pma = vec![PMANode {
            edges: HashMap::new(),
            fail: 0,
            matched: vec![],
        }];
        for (i, phrase) in patterns.iter().enumerate() {
            let mut node_id = 0;
            for word in phrase {
                node_id = if let Some(&node_id_next) = pma[node_id].edges.get(word) {
                    node_id_next
                } else {
                    let node_id_next = pma.len();
                    pma[node_id].edges.insert(word, node_id_next);
                    pma.push(PMANode {
                        edges: HashMap::new(),
                        fail: 0,
                        matched: vec![],
                    });
                    node_id_next
                }
            }
            pma[node_id].matched.push(i);
        }
        let mut queue = VecDeque::new();
        for &node_id in pma[0].edges.values() {
            queue.push_back(node_id);
        }
        while let Some(node_id) = queue.pop_front() {
            for (c, node_id_next) in pma[node_id].edges.clone().iter() {
                queue.push_back(*node_id_next);
                let mut node_id_fail = node_id;
                while node_id_fail != 0 {
                    node_id_fail = pma[node_id_fail].fail;
                    if let Some(&edge_target) = pma[node_id_fail].edges.get(c) {
                        node_id_fail = edge_target;
                        break;
                    }
                }
                let matched = pma[node_id_fail].matched.clone();
                let mut node = &mut pma[*node_id_next];
                node.fail = node_id_fail;
                node.matched.extend(matched);
            }
        }
        LatticeSearcher {
            pma: pma,
            patterns: patterns,
        }
    }

    fn next_pma_state_id(pma: &Vec<PMANode>, state_id: usize, edge_str: &str) -> usize {
        let mut next_state_id = state_id;
        loop {
            if let Some(&x) = pma[next_state_id].edges.get(edge_str) {
                return x;
            }
            if next_state_id == 0 {
                return 0;
            }
            next_state_id = pma[next_state_id].fail;
        }
    }

    fn backward_match(phrase: &Vec<&'a str>, lattice: &'a Vec<LatticeNode>, pos: usize) -> Vec<VecDeque<(&'a str, usize)>> {
        let mut result = vec![];
        let mut backward_queue = VecDeque::new();
        let mut init_path = VecDeque::new();
        init_path.push_front(("", pos));
        backward_queue.push_back((phrase.len(), pos, init_path));
        while let Some((phrase_pos, lattice_node_id, path)) = backward_queue.pop_front() {
            if phrase_pos == 0 {
                result.push(path);
            } else {
                for &(edge_str, edge_target) in lattice[lattice_node_id].backwards.keys() {
                    if edge_str == phrase[phrase_pos - 1] {
                        let mut new_path = path.clone();
                        new_path.front_mut().unwrap().0 = edge_str;
                        new_path.push_front(("", edge_target));
                        backward_queue.push_back((phrase_pos - 1, edge_target, new_path));
                    }
                }
            }
        }
        result
    }

    pub fn search(&self, lattice: &'a Lattice) -> Vec<Vec<(&'a str, usize)>> {
        let lattice = &lattice.lattice;
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        let mut state_id_cache = HashSet::new();
        let mut result = vec![];
        while let Some((lattice_node_id, pma_state_id)) = queue.pop_front() {
            for (lattice_edge_str, lattice_egde_target) in lattice[lattice_node_id].forwards.keys() {
                let pma_state_id_new = Self::next_pma_state_id(&self.pma, pma_state_id, lattice_edge_str);
                if !state_id_cache.contains(&(*lattice_egde_target, pma_state_id_new)) {
                    queue.push_back((*lattice_egde_target, pma_state_id_new));
                    state_id_cache.insert((*lattice_egde_target, pma_state_id_new));
                }
            }
            for &phrase_id in &self.pma[pma_state_id].matched {
                let phrase = &self.patterns[phrase_id];
                for m in Self::backward_match(phrase, &lattice, lattice_node_id) {
                    result.push(m.into_iter().collect());
                }
            }
        }
        result
    }
}
