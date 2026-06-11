use std::cmp;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::mem;
use std::usize;

use crate::lattice::Lattice;
use crate::lattice::LatticeNode;
use crate::utils::get_two_mut_elems;

pub struct PMANode<'a> {
    edges: HashMap<&'a str, usize>,
    fail: usize,
    matched: Vec<usize>,
}

pub struct PaRattice<'a> {
    pma: Vec<PMANode<'a>>,
    phrases: Vec<(Vec<&'a str>, usize)>,
    dict: Vec<Vec<usize>>,
}

impl<'a> PaRattice<'a> {
    /// Returns PaRattice with the given paraphrase corpus.
    ///
    /// # Arguments
    ///
    /// * `dict` - A paraphrase corpus
    ///
    /// # Example
    ///
    /// ```
    /// use parattice::PaRattice;
    ///
    /// let paradict = vec![
    ///     vec![
    ///         vec!["blood", "stem", "cell"],
    ///         vec!["造血", "幹", "細胞"],
    ///         vec!["hematopoietic", "stem", "cell"],
    ///     ],
    ///     vec![
    ///         vec!["造血", "幹", "細胞", "移植"],
    ///         vec!["hematopoietic", "stem", "cell", "transplantation"],
    ///     ],
    ///     vec![vec!["stem", "cell"], vec!["幹", "細胞"]],
    ///     vec![
    ///         vec!["幹", "細胞", "移植"],
    ///         vec!["rescue", "transplant"],
    ///         vec!["stem", "cell", "rescue"],
    ///     ],
    ///     vec![vec!["rescue"], vec!["救命"]],
    ///     vec![vec!["blood"], vec!["血液"]],
    /// ];
    /// let parattice = PaRattice::new(paradict);
    /// ```
    pub fn new(dict: Vec<Vec<Vec<&'a str>>>) -> PaRattice<'a> {
        let mut pma = vec![PMANode {
            edges: HashMap::new(),
            fail: 0,
            matched: vec![],
        }];
        let mut phrases = vec![];
        let mut ids = vec![];
        for (i, group) in dict.iter().enumerate() {
            let mut id_group = vec![];
            for phrase in group {
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
                pma[node_id].matched.push(phrases.len());
                id_group.push(phrases.len());
                phrases.push((phrase.clone(), i));
            }
            ids.push(id_group);
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
        PaRattice {
            pma,
            phrases,
            dict: ids,
        }
    }

    fn backward_match(
        phrase: &[&str],
        lattice: &[LatticeNode],
        pos: usize,
        max_depth: usize,
    ) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut backward_queue = VecDeque::new();
        if lattice[pos].depth < max_depth {
            backward_queue.push_back((phrase.len(), pos, lattice[pos].depth));
        }
        while let Some((phrase_pos, lattice_node_id, depth)) = backward_queue.pop_front() {
            if phrase_pos == 0 {
                result.push((lattice_node_id, depth));
            } else {
                for &(edge_str, edge_target) in &lattice[lattice_node_id].backwards {
                    if edge_str == phrase[phrase_pos - 1] && lattice[edge_target].depth < max_depth
                    {
                        backward_queue.push_back((
                            phrase_pos - 1,
                            edge_target,
                            cmp::max(depth, lattice[edge_target].depth),
                        ));
                    }
                }
            }
        }
        result
    }

    fn next_pma_state_id(pma: &[PMANode], state_id: usize, edge_str: &str) -> usize {
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

    fn insert_branch(
        lattice: &mut Vec<LatticeNode<'a>>,
        state_id_cache: &mut Vec<BTreeSet<usize>>,
        phrase: &[&'a str],
        start_node_id: usize,
        end_node_id: usize,
        depth: usize,
    ) -> usize {
        let new_node_id = lattice.len();
        assert!(!lattice.is_empty());
        match phrase.len() {
            1 => {
                lattice[start_node_id].insert_forward(&phrase[0], end_node_id);
                lattice[end_node_id].insert_backward(&phrase[0], start_node_id);
                end_node_id
            }
            2 => {
                lattice[start_node_id].insert_forward(&phrase[0], new_node_id);
                lattice.push(LatticeNode::new(
                    (phrase[1], end_node_id),
                    (phrase[0], start_node_id),
                    depth,
                ));
                state_id_cache.push(BTreeSet::new());
                lattice[end_node_id].insert_backward(&phrase[1], new_node_id);
                new_node_id
            }
            3 => {
                lattice[start_node_id].insert_forward(&phrase[0], new_node_id);
                lattice.push(LatticeNode::new(
                    (phrase[1], new_node_id + 1),
                    (phrase[0], start_node_id),
                    depth,
                ));
                state_id_cache.push(BTreeSet::new());
                lattice.push(LatticeNode::new(
                    (phrase[2], end_node_id),
                    (phrase[1], new_node_id),
                    depth,
                ));
                state_id_cache.push(BTreeSet::new());
                lattice[end_node_id].insert_backward(&phrase[2], new_node_id + 1);
                new_node_id
            }
            _ => {
                lattice[start_node_id].insert_forward(&phrase[0], new_node_id);
                lattice.push(LatticeNode::new(
                    (phrase[1], new_node_id + 1),
                    (phrase[0], start_node_id),
                    depth,
                ));
                state_id_cache.push(BTreeSet::new());
                for i in 0..phrase.len() - 3 {
                    lattice.push(LatticeNode::new(
                        (phrase[i + 2], new_node_id + i + 2),
                        (phrase[i + 1], new_node_id + i),
                        depth,
                    ));
                    state_id_cache.push(BTreeSet::new());
                }
                lattice.push(LatticeNode::new(
                    (phrase[phrase.len() - 1], end_node_id),
                    (
                        phrase[phrase.len() - 2],
                        new_node_id + phrase.len() - 3,
                    ),
                    depth,
                ));
                state_id_cache.push(BTreeSet::new());
                lattice[end_node_id]
                    .insert_backward(&phrase[phrase.len() - 1], new_node_id + phrase.len() - 2);
                new_node_id
            }
        }
    }

    /// Returns a recursive paraphrase lattice of the given sentence.
    ///
    /// # Arguments
    ///
    /// * `words` - A sentence
    /// * `shrink` - If true, duplicated edges are shrinked
    /// * `max_depth` - A number of recursion
    ///
    /// # Example
    ///
    /// ```
    /// use parattice::PaRattice;
    ///
    /// let paradict = vec![
    ///     vec![
    ///         vec!["blood", "stem", "cell"],
    ///         vec!["造血", "幹", "細胞"],
    ///         vec!["hematopoietic", "stem", "cell"],
    ///     ],
    ///     vec![
    ///         vec!["造血", "幹", "細胞", "移植"],
    ///         vec!["hematopoietic", "stem", "cell", "transplantation"],
    ///     ],
    ///     vec![vec!["stem", "cell"], vec!["幹", "細胞"]],
    ///     vec![
    ///         vec!["幹", "細胞", "移植"],
    ///         vec!["rescue", "transplant"],
    ///         vec!["stem", "cell", "rescue"],
    ///     ],
    ///     vec![vec!["rescue"], vec!["救命"]],
    ///     vec![vec!["blood"], vec!["血液"]],
    /// ];
    /// let parattice = PaRattice::new(paradict);
    ///
    /// let words = vec!["造血", "幹", "細胞", "移植"];
    /// let lattice = parattice.get_lattice(&words, true, 2);
    /// ```
    pub fn get_lattice(&self, words: &[&'a str], shrink: bool, max_depth: usize) -> Lattice<'a> {
        let mut inserted_branches = HashSet::new();
        // generate initial lattice
        let mut lattice = vec![];
        let mut state_id_cache = vec![];
        if words.is_empty() {
            lattice.push(LatticeNode::new(None, None, 0));
            state_id_cache.push(BTreeSet::new());
        } else {
            lattice.push(LatticeNode::new((words[0], 1), None, 0));
            state_id_cache.push(BTreeSet::new());
            for node_id in 1..words.len() {
                lattice.push(LatticeNode::new(
                    (words[node_id], node_id + 1),
                    (words[node_id - 1], node_id - 1),
                    0,
                ));
                state_id_cache.push(BTreeSet::new());
            }
            lattice.push(LatticeNode::new(
                None,
                (words[words.len() - 1], words.len() - 1),
                0,
            ));
            state_id_cache.push(BTreeSet::new());
        }
        // search phrases
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        state_id_cache[0].insert(0);
        while let Some((lattice_node_id, pma_state_id)) = queue.pop_front() {
            for (lattice_edge_str, lattice_egde_target) in lattice[lattice_node_id].forwards.iter()
            {
                let pma_state_id_new =
                    Self::next_pma_state_id(&self.pma, pma_state_id, lattice_edge_str);
                if !state_id_cache[*lattice_egde_target].contains(&pma_state_id_new) {
                    // queue next node
                    queue.push_back((*lattice_egde_target, pma_state_id_new));
                    state_id_cache[*lattice_egde_target].insert(pma_state_id_new);
                }
            }
            for &phrase_id in &self.pma[pma_state_id].matched {
                let (phrase, group_id) = &self.phrases[phrase_id];
                let trunk_end = Self::main_branch_fw(&lattice, lattice_node_id, words.len());
                for (branch_start, depth) in
                    Self::backward_match(phrase, &lattice, lattice_node_id, max_depth)
                {
                    let trunk_start = Self::main_branch_bw(&lattice, branch_start, words.len());
                    if inserted_branches.contains(&(group_id, trunk_start, trunk_end)) {
                        continue;
                    }
                    inserted_branches.insert((group_id, trunk_start, trunk_end));
                    for &paraphrase_id in &self.dict[*group_id] {
                        if paraphrase_id == phrase_id {
                            continue;
                        }
                        let paraphrase = &self.phrases[paraphrase_id].0;
                        let inserted_first_node_id = Self::insert_branch(
                            &mut lattice,
                            &mut state_id_cache,
                            paraphrase,
                            branch_start,
                            lattice_node_id,
                            depth + 1,
                        );
                        let (state_id_cache_current, state_id_cache_next) = get_two_mut_elems(
                            &mut state_id_cache,
                            branch_start,
                            inserted_first_node_id,
                        );
                        for pma_state_id_cached in state_id_cache_current.iter() {
                            let pma_state_id_new = Self::next_pma_state_id(
                                &self.pma,
                                *pma_state_id_cached,
                                &paraphrase[0],
                            );
                            if !state_id_cache_next.contains(&pma_state_id_new) {
                                // queue added node
                                queue.push_back((inserted_first_node_id, pma_state_id_new));
                                state_id_cache_next.insert(pma_state_id_new);
                            }
                        }
                    }
                }
            }
        }
        if shrink {
            PaRattice::shrink_lattice(&mut lattice);
        }
        let new_lattice = PaRattice::index_left_to_right(&lattice);
        let mut trunk = HashMap::new();
        let mut node_id = 0;
        let mut orig_node_id = 0;
        trunk.insert(0, 0);
        while let Some((_, x)) = &new_lattice[node_id].forward_main {
            node_id = *x;
            orig_node_id += 1;
            trunk.insert(node_id, orig_node_id);
        }
        Lattice {
            trunk,
            capacity: new_lattice.iter().fold(0, |sum, x| sum + x.forwards.len()),
            lattice: new_lattice,
        }
    }

    fn main_branch_bw(g: &[LatticeNode], begin: usize, eos: usize) -> usize {
        let mut b = begin;
        while b > eos {
            b = g[b].backward_main.unwrap().1;
        }
        b
    }

    fn main_branch_fw(g: &[LatticeNode], end: usize, eos: usize) -> usize {
        let mut e = end;
        while e > eos {
            e = g[e].forward_main.unwrap().1;
        }
        e
    }

    fn shrink_lattice(lattice: &mut Vec<LatticeNode>) {
        let mut updated_node_bw: BTreeSet<usize> = (0..lattice.len()).collect();
        let mut updated_node_fw: BTreeSet<usize> = (0..lattice.len()).collect();
        while !updated_node_bw.is_empty() || !updated_node_fw.is_empty() {
            let mut backward_map = BTreeMap::new();
            for &i in &updated_node_bw {
                if !lattice[i].backwards.is_empty() {
                    backward_map
                        .entry(lattice[i].backwards.clone())
                        .or_insert(vec![])
                        .push(i);
                }
            }
            updated_node_bw.clear();
            for nodes in backward_map.values() {
                if nodes.len() >= 2 {
                    for i in 1..nodes.len() {
                        let backward_tmp =
                            mem::replace(&mut lattice[nodes[i]].backwards, BTreeSet::new());
                        for (edge_str, prev_node_id) in backward_tmp {
                            lattice[prev_node_id].forwards.remove(&(edge_str, nodes[i]));
                            if lattice[prev_node_id].forward_main == Some((edge_str, nodes[i])) {
                                lattice[prev_node_id].forward_main = Some((edge_str, nodes[0]));
                            }
                        }
                        let forward_tmp =
                            mem::replace(&mut lattice[nodes[i]].forwards, BTreeSet::new());
                        for (edge_str, next_node_id) in forward_tmp {
                            lattice[next_node_id]
                                .backwards
                                .remove(&(edge_str, nodes[i]));
                            lattice[next_node_id].backwards.insert((edge_str, nodes[0]));
                            if lattice[next_node_id].backward_main == Some((edge_str, nodes[i])) {
                                lattice[next_node_id].backward_main = Some((edge_str, nodes[0]));
                            }
                            lattice[nodes[0]].forwards.insert((edge_str, next_node_id));
                        }
                    }
                    updated_node_bw.insert(nodes[0]);
                    for &(_, i) in &lattice[nodes[0]].forwards {
                        updated_node_bw.insert(i);
                    }
                }
            }
            let mut forward_map = BTreeMap::new();
            for &i in &updated_node_fw {
                if !lattice[i].forwards.is_empty() {
                    forward_map
                        .entry(lattice[i].forwards.clone())
                        .or_insert(vec![])
                        .push(i);
                }
            }
            updated_node_fw.clear();
            for nodes in forward_map.values() {
                if nodes.len() >= 2 {
                    for i in 1..nodes.len() {
                        let forward_tmp =
                            mem::replace(&mut lattice[nodes[i]].forwards, BTreeSet::new());
                        for (edge_str, next_node_id) in forward_tmp {
                            lattice[next_node_id]
                                .backwards
                                .remove(&(edge_str, nodes[i]));
                            if lattice[next_node_id].backward_main == Some((edge_str, nodes[i])) {
                                lattice[next_node_id].backward_main = Some((edge_str, nodes[0]));
                            }
                        }
                        let backward_tmp =
                            mem::replace(&mut lattice[nodes[i]].backwards, BTreeSet::new());
                        for (edge_str, prev_node_id) in backward_tmp {
                            lattice[prev_node_id].forwards.remove(&(edge_str, nodes[i]));
                            lattice[prev_node_id].forwards.insert((edge_str, nodes[0]));
                            if lattice[prev_node_id].forward_main == Some((edge_str, nodes[i])) {
                                lattice[prev_node_id].forward_main = Some((edge_str, nodes[0]));
                            }
                            lattice[nodes[0]].backwards.insert((edge_str, prev_node_id));
                        }
                    }
                    updated_node_fw.insert(nodes[0]);
                    for &(_, i) in &lattice[nodes[0]].backwards {
                        updated_node_fw.insert(i);
                    }
                }
            }
        }
    }

    fn index_left_to_right(lattice: &[LatticeNode<'a>]) -> Vec<LatticeNode<'a>> {
        let mut node_id_map = vec![0; lattice.len()];
        let mut node_id_map_rev = Vec::with_capacity(lattice.len());
        let mut queue = VecDeque::new();
        let mut backward_counter = vec![0; lattice.len()];
        queue.push_back(0);
        while let Some(node_id) = queue.pop_front() {
            node_id_map[node_id] = node_id_map_rev.len();
            node_id_map_rev.push(node_id);
            for &(_, edge_target) in &lattice[node_id].forwards {
                backward_counter[edge_target] += 1;
                if backward_counter[edge_target] == lattice[edge_target].backwards.len() {
                    queue.push_back(edge_target);
                }
            }
        }
        let mut new_lattice = Vec::with_capacity(node_id_map_rev.len());
        for &node_id in &node_id_map_rev {
            let mut new_forwards = BTreeSet::new();
            let mut new_backwards = BTreeSet::new();
            for &(s, next_node_id) in &lattice[node_id].forwards {
                new_forwards.insert((s, node_id_map[next_node_id]));
            }
            for &(s, prev_node_id) in &lattice[node_id].backwards {
                new_backwards.insert((s, node_id_map[prev_node_id]));
            }
            let forward_main = lattice[node_id]
                .forward_main
                .map(|(x, i)| (x, node_id_map[i]));
            let backward_main = lattice[node_id]
                .backward_main
                .map(|(x, i)| (x, node_id_map[i]));
            new_lattice.push(LatticeNode {
                forwards: new_forwards,
                backwards: new_backwards,
                forward_main,
                backward_main,
                depth: 0,
            });
        }
        new_lattice
    }
}
