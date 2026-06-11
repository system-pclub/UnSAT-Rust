use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str;
use std::usize;

#[derive(Debug, Eq, PartialEq)]
pub struct LatticeNode<'a> {
    pub forwards: BTreeSet<(&'a str, usize)>,
    pub backwards: BTreeSet<(&'a str, usize)>,
    pub forward_main: Option<(&'a str, usize)>,
    pub backward_main: Option<(&'a str, usize)>,
    pub depth: usize,
}

impl<'a> LatticeNode<'a> {
    pub fn new<T1: Into<Option<(&'a str, usize)>>, T2: Into<Option<(&'a str, usize)>>>(
        forward_main: T1,
        backward_main: T2,
        depth: usize,
    ) -> LatticeNode<'a> {
        let mut forwards = BTreeSet::new();
        let mut backwards = BTreeSet::new();
        let forward_main = forward_main.into();
        let backward_main = backward_main.into();
        if let Some(x) = forward_main {
            forwards.insert(x);
        }
        if let Some(x) = backward_main {
            backwards.insert(x);
        }
        LatticeNode {
            forwards,
            backwards,
            forward_main,
            backward_main,
            depth,
        }
    }

    pub fn insert_forward(&mut self, edge_str: &'a str, edge_target: usize) {
        self.forwards.insert((edge_str, edge_target));
    }

    pub fn insert_backward(&mut self, edge_str: &'a str, edge_target: usize) {
        self.backwards.insert((edge_str, edge_target));
    }
}

pub struct SearchIndexNode<'a> {
    pub text: &'a str,
    pub offset: (usize, usize),
    pub increment: usize,
    pub length: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Lattice<'a> {
    pub lattice: Vec<LatticeNode<'a>>,
    pub trunk: HashMap<usize, usize>,
    pub capacity: usize,
}

fn usize_to_vec(x: usize) -> Vec<u8> {
    vec![
        x as u8,
        (x >> 8) as u8,
        (x >> 16) as u8,
        (x >> 24) as u8,
        (x >> 32) as u8,
        (x >> 40) as u8,
        (x >> 48) as u8,
        (x >> 56) as u8,
    ]
}

fn vec_to_usize(x: &[u8]) -> usize {
    x[0] as usize
        | (x[1] as usize) << 8
        | (x[2] as usize) << 16
        | (x[3] as usize) << 24
        | (x[4] as usize) << 32
        | (x[5] as usize) << 40
        | (x[6] as usize) << 48
        | (x[7] as usize) << 56
}

impl<'a> Lattice<'a> {
    /// Returns a lattice from the given binary data.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte array
    ///
    /// # Example
    ///
    /// ```
    /// use parattice::PaRattice;
    /// use parattice::Lattice;
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
    /// let words = vec!["造血", "幹", "細胞", "移植"];
    /// let lattice = parattice.get_lattice(&words, true, 2);
    ///
    /// let bytes = lattice.to_bytes();
    /// let new_lattice = Lattice::new_from_bytes(&bytes);
    /// ```
    pub fn new_from_bytes(data: &'a [u8]) -> Lattice<'a> {
        let mut lattice = Vec::with_capacity(vec_to_usize(&data[0..8]));
        let mut offset = 8;
        while offset < data.len() {
            let num_forwards = vec_to_usize(&data[offset..offset + 8]);
            let num_backwards = vec_to_usize(&data[offset + 8..offset + 16]);
            offset += 16;
            let mut forwards = BTreeSet::new();
            let forward_main = if num_forwards != 0 {
                let forward_main_num_chars = vec_to_usize(&data[offset..offset + 8]);
                let forward_main_edge_string =
                    str::from_utf8(&data[offset + 8..offset + 8 + forward_main_num_chars]).unwrap();
                let forward_main_edge_target = vec_to_usize(
                    &data
                        [offset + 8 + forward_main_num_chars..offset + 16 + forward_main_num_chars],
                );
                offset += 16 + forward_main_num_chars;
                forwards.insert((forward_main_edge_string, forward_main_edge_target));
                for _ in 1..num_forwards {
                    let forward_num_chars = vec_to_usize(&data[offset..offset + 8]);
                    let forward_edge_string =
                        str::from_utf8(&data[offset + 8..offset + 8 + forward_num_chars]).unwrap();
                    let forward_edge_target = vec_to_usize(
                        &data[offset + 8 + forward_num_chars..offset + 16 + forward_num_chars],
                    );
                    offset += 16 + forward_num_chars;
                    forwards.insert((forward_edge_string, forward_edge_target));
                }
                Some((forward_main_edge_string, forward_main_edge_target))
            } else {
                None
            };
            let mut backwards = BTreeSet::new();
            let backward_main = if num_backwards != 0 {
                let backward_main_num_chars = vec_to_usize(&data[offset..offset + 8]);
                let backward_main_edge_string =
                    str::from_utf8(&data[offset + 8..offset + 8 + backward_main_num_chars])
                        .unwrap();
                let backward_main_edge_target = vec_to_usize(
                    &data[offset + 8 + backward_main_num_chars
                        ..offset + 16 + backward_main_num_chars],
                );
                offset += 16 + backward_main_num_chars;
                backwards.insert((backward_main_edge_string, backward_main_edge_target));
                for _ in 1..num_backwards {
                    let backward_num_chars = vec_to_usize(&data[offset..offset + 8]);
                    let backward_edge_string =
                        str::from_utf8(&data[offset + 8..offset + 8 + backward_num_chars]).unwrap();
                    let backward_edge_target = vec_to_usize(
                        &data[offset + 8 + backward_num_chars..offset + 16 + backward_num_chars],
                    );
                    offset += 16 + backward_num_chars;
                    backwards.insert((backward_edge_string, backward_edge_target));
                }
                Some((backward_main_edge_string, backward_main_edge_target))
            } else {
                None
            };
            lattice.push(LatticeNode {
                forwards,
                backwards,
                forward_main,
                backward_main,
                depth: 0,
            });
        }
        let mut trunk = HashMap::new();
        let mut node_id = 0;
        let mut orig_node_id = 0;
        trunk.insert(0, 0);
        while let Some(x) = lattice[node_id].forward_main {
            node_id = x.1;
            orig_node_id += 1;
            trunk.insert(node_id, orig_node_id);
        }
        Lattice {
            trunk,
            capacity: lattice.iter().fold(0, |sum, x| sum + x.forwards.len()),
            lattice,
        }
    }

    /// Returns binary data of the lattice.
    ///
    /// # Example
    ///
    /// ```
    /// use parattice::PaRattice;
    /// use parattice::Lattice;
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
    /// let words = vec!["造血", "幹", "細胞", "移植"];
    /// let lattice = parattice.get_lattice(&words, true, 2);
    ///
    /// let bytes = lattice.to_bytes();
    /// let new_lattice = Lattice::new_from_bytes(&bytes);
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        result.append(&mut usize_to_vec(self.lattice.len()));
        for node in &self.lattice {
            result.append(&mut usize_to_vec(node.forwards.len()));
            result.append(&mut usize_to_vec(node.backwards.len()));
            if let Some(x) = node.forward_main {
                result.append(&mut usize_to_vec(x.0.len()));
                result.append(&mut x.0.as_bytes().to_vec());
                result.append(&mut usize_to_vec(x.1));
                for &edge in &node.forwards {
                    if edge != x {
                        result.append(&mut usize_to_vec(edge.0.len()));
                        result.append(&mut edge.0.as_bytes().to_vec());
                        result.append(&mut usize_to_vec(edge.1));
                    }
                }
            }
            if let Some(x) = node.backward_main {
                result.append(&mut usize_to_vec(x.0.len()));
                result.append(&mut x.0.as_bytes().to_vec());
                result.append(&mut usize_to_vec(x.1));
                for &edge in &node.backwards {
                    if edge != x {
                        result.append(&mut usize_to_vec(edge.0.len()));
                        result.append(&mut edge.0.as_bytes().to_vec());
                        result.append(&mut usize_to_vec(edge.1));
                    }
                }
            }
        }
        result
    }

    /// Returns graphviz of the lattice.
    ///
    /// # Arguments
    ///
    /// * `is_numbered` - If true, node numbers are printed.
    ///
    /// # Example
    ///
    /// ```
    /// use parattice::PaRattice;
    /// use parattice::Lattice;
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
    /// let words = vec!["造血", "幹", "細胞", "移植"];
    /// let lattice = parattice.get_lattice(&words, true, 2);
    ///
    /// let dot = lattice.dump_dot(true);
    /// println!("{}", dot);
    /// ```
    pub fn dump_dot(&self, is_numbered: bool) -> String {
        let mut result = "digraph { graph [rankdir=LR];\n".to_string();
        for (i, node) in self.lattice.iter().enumerate() {
            if is_numbered {
                result = result
                    + &format!(
                        "\"{}\" [label=\"{}\",shape=plaintext,width=\"0.1\"];\n",
                        i, i
                    );
            } else {
                result = result + &format!("\"{}\" [label=\"\",shape=circle,width=\"0.1\"];\n", i);
            }
            for (j, &edge) in node.forwards.iter().enumerate() {
                result = result
                    + &format!(
                        "\"{}-{}-{}\" [label=\"{}\",shape=box];\n",
                        i, j, edge.1, edge.0
                    );
                if edge == node.forward_main.unwrap() {
                    result = result
                        + &format!(
                            "\"{}\" -> \"{}-{}-{}\" [arrowhead=none,color=\"#ff0000\"];\n",
                            i, i, j, edge.1
                        );
                } else {
                    result = result
                        + &format!(
                            "\"{}\" -> \"{}-{}-{}\" [arrowhead=none];\n",
                            i, i, j, edge.1
                        );
                }
                if self.lattice[edge.1].backward_main == Some((edge.0, i)) {
                    result = result
                        + &format!(
                            "\"{}-{}-{}\" -> \"{}\" [color=\"#0000ff\"];\n",
                            i, j, edge.1, edge.1
                        );
                } else {
                    result = result + &format!("\"{}-{}-{}\" -> \"{}\";\n", i, j, edge.1, edge.1);
                }
            }
        }
        result += "}";
        result
    }

    /// Returns a trunk path of the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path of the lattice. (e.g. a result of LatticeKMP)
    pub fn get_trunk_span(&self, path: Vec<(&'a str, usize)>) -> Vec<(&'a str, usize)> {
        let mut new_path: VecDeque<(&str, usize)> = path.into_iter().collect();
        let mut edge_bw = new_path.pop_front().unwrap();
        while !self.trunk.contains_key(&edge_bw.1) {
            let next_edge = self.lattice[edge_bw.1].backward_main.unwrap();
            new_path.push_front((next_edge.0, edge_bw.1));
            edge_bw = next_edge;
        }
        new_path.push_front(("", edge_bw.1));
        let mut edge_fw = *new_path.back().unwrap();
        while !self.trunk.contains_key(&edge_fw.1) {
            edge_fw = self.lattice[edge_fw.1].forward_main.unwrap();
            new_path.push_back(edge_fw);
        }
        new_path.into_iter().collect()
    }

    /// Returns trunk node IDs for each node ID.
    pub fn get_trunk_spans(&self) -> Vec<(usize, usize)> {
        let mut left_trunks = vec![0; self.lattice.len()];
        let mut right_trunks = vec![self.lattice.len() - 1; self.lattice.len()];
        for &node_id in self.trunk.keys() {
            left_trunks[node_id] = node_id;
            right_trunks[node_id] = node_id;
        }
        for node_id in 1..self.lattice.len() - 1 {
            for edge in &self.lattice[node_id].forwards {
                if left_trunks[edge.1] == 0
                    && self.lattice[edge.1].backward_main.unwrap().1 == node_id
                {
                    left_trunks[edge.1] = left_trunks[node_id];
                }
            }
        }
        for node_id in (1..self.lattice.len() - 1).rev() {
            for edge in &self.lattice[node_id].backwards {
                if right_trunks[edge.1] == self.lattice.len() - 1
                    && self.lattice[edge.1].forward_main.unwrap().1 == node_id
                {
                    right_trunks[edge.1] = right_trunks[node_id];
                }
            }
        }
        let mut result = Vec::with_capacity(self.lattice.len());
        for item in left_trunks.into_iter().zip(right_trunks.into_iter()) {
            result.push((self.trunk[&item.0], self.trunk[&item.1]));
        }
        result
    }

    /// Returns a vector of SearchIndexNode for the search index such as Elasticsearch.
    pub fn dump_for_search_index(&self) -> Vec<SearchIndexNode> {
        let trunk_spans = self.get_trunk_spans();
        let mut result = Vec::with_capacity(self.capacity);
        for i in 0..self.lattice.len() - 1 {
            for (j, edge) in self.lattice[i].forwards.iter().enumerate() {
                result.push(SearchIndexNode {
                    text: edge.0,
                    offset: (trunk_spans[i].0, trunk_spans[edge.1].1),
                    increment: if j == 0 { 1 } else { 0 },
                    length: edge.1 - i,
                });
            }
        }
        result
    }
}
