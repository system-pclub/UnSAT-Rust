extern crate parattice;

use parattice::Lattice;
use parattice::PaRattice;
use parattice::SearchIndexNode;

#[test]
fn dump_for_search_index_test() {
    let paradict = vec![
        vec![
            vec!["blood", "stem", "cell"],
            vec!["造血", "幹", "細胞"],
            vec!["hematopoietic", "stem", "cell"],
        ],
        vec![
            vec!["造血", "幹", "細胞", "移植"],
            vec!["hematopoietic", "stem", "cell", "transplantation"],
        ],
        vec![vec!["stem", "cell"], vec!["幹", "細胞"]],
        vec![
            vec!["幹", "細胞", "移植"],
            vec!["rescue", "transplant"],
            vec!["stem", "cell", "rescue"],
        ],
        vec![vec!["rescue"], vec!["救命"]],
        vec![vec!["blood"], vec!["血液"]],
    ];
    let parattice = PaRattice::new(paradict);
    let words = vec!["造血", "幹", "細胞", "移植"];
    let lattice = parattice.get_lattice(&words, true, 10);
    let index_data = lattice.dump_for_search_index();
    let mut expected = vec![
        ("造血", 0, 3, 0, 1),
        ("blood", 0, 2, 0, 3),
        ("血液", 0, 2, 0, 3),
        ("hematopoietic", 0, 1, 0, 3),
        ("stem", 1, 4, 0, 3),
        ("stem", 1, 6, 0, 4),
        ("幹", 1, 10, 0, 4),
        ("rescue", 2, 11, 0, 4),
        ("幹", 2, 7, 0, 3),
        ("stem", 2, 5, 0, 3),
        ("stem", 2, 6, 0, 4),
        ("stem", 3, 8, 1, 3),
        ("幹", 3, 9, 1, 2),
        ("幹", 3, 10, 1, 4),
        ("救命", 3, 11, 1, 4),
        ("rescue", 3, 11, 1, 4),
        ("cell", 4, 13, 0, 3),
        ("cell", 5, 12, 0, 4),
        ("cell", 5, 13, 0, 3),
        ("cell", 6, 15, 0, 4),
        ("細胞", 7, 13, 0, 3),
        ("cell", 8, 13, 1, 3),
        ("cell", 8, 14, 1, 4),
        ("細胞", 9, 13, 2, 3),
        ("細胞", 9, 14, 2, 4),
        ("細胞", 10, 15, 1, 4),
        ("transplant", 11, 16, 1, 4),
        ("rescue", 12, 16, 0, 4),
        ("救命", 12, 16, 0, 4),
        ("移植", 13, 16, 3, 4),
        ("rescue", 14, 16, 1, 4),
        ("transplantation", 15, 16, 0, 4),
    ];
    expected.sort();
    assert_eq!(expected, search_index_relative_to_absolute(&index_data));
}

#[test]
fn serialize_test() {
    let paradict = vec![
        vec![
            vec!["blood", "stem", "cell"],
            vec!["造血", "幹", "細胞"],
            vec!["hematopoietic", "stem", "cell"],
        ],
        vec![
            vec!["造血", "幹", "細胞", "移植"],
            vec!["hematopoietic", "stem", "cell", "transplantation"],
        ],
        vec![vec!["stem", "cell"], vec!["幹", "細胞"]],
        vec![
            vec!["幹", "細胞", "移植"],
            vec!["rescue", "transplant"],
            vec!["stem", "cell", "rescue"],
        ],
        vec![vec!["rescue"], vec!["救命"]],
        vec![vec!["blood"], vec!["血液"]],
    ];
    let parattice = PaRattice::new(paradict);
    let words = vec!["造血", "幹", "細胞", "移植"];
    let lattice = parattice.get_lattice(&words, true, 10);
    let bytes = lattice.to_bytes();
    let lattice_from_bytes = Lattice::new_from_bytes(&bytes);
    assert_eq!(lattice, lattice_from_bytes);
}

fn search_index_relative_to_absolute<'a>(
    data: &'a Vec<SearchIndexNode>,
) -> Vec<(&'a str, usize, usize, usize, usize)> {
    let mut new_data = vec![];
    let mut node_id = 0;
    for node in data {
        node_id += node.increment;
        new_data.push((
            node.text,
            node_id - 1,
            node_id + node.length - 1,
            node.offset.0,
            node.offset.1,
        ));
    }
    new_data.sort();
    new_data
}

#[test]
fn get_trunk_span_test() {
    let paradict = vec![
        vec![
            vec!["blood", "stem", "cell"],
            vec!["造血", "幹", "細胞"],
            vec!["hematopoietic", "stem", "cell"],
        ],
        vec![
            vec!["造血", "幹", "細胞", "移植"],
            vec!["hematopoietic", "stem", "cell", "transplantation"],
        ],
        vec![vec!["stem", "cell"], vec!["幹", "細胞"]],
        vec![
            vec!["幹", "細胞", "移植"],
            vec!["rescue", "transplant"],
            vec!["stem", "cell", "rescue"],
        ],
        vec![vec!["rescue"], vec!["救命"]],
        vec![vec!["blood"], vec!["血液"]],
    ];
    let parattice = PaRattice::new(paradict);
    let words = vec!["造血", "幹", "細胞", "移植"];
    let lattice = parattice.get_lattice(&words, true, 10);
    assert_eq!(
        vec![("", 0), ("hematopoietic", 1), ("stem", 4), ("cell", 13)],
        lattice.get_trunk_span(vec![("", 1), ("stem", 4)])
    );
    assert_eq!(
        vec![("", 3), ("rescue", 11), ("transplant", 16)],
        lattice.get_trunk_span(vec![("", 3), ("rescue", 11)])
    );
    assert_eq!(
        vec![
            ("", 0),
            ("hematopoietic", 1),
            ("stem", 6),
            ("cell", 15),
            ("transplantation", 16)
        ],
        lattice.get_trunk_span(vec![("", 15), ("transplantation", 16)])
    );
}

#[test]
fn max_depth_test() {
    let paradict = vec![
        vec![
            vec!["blood", "stem", "cell"],
            vec!["造血", "幹", "細胞"],
            vec!["hematopoietic", "stem", "cell"],
        ],
        vec![
            vec!["造血", "幹", "細胞", "移植"],
            vec!["hematopoietic", "stem", "cell", "transplantation"],
        ],
        vec![vec!["stem", "cell"], vec!["幹", "細胞"]],
        vec![
            vec!["幹", "細胞", "移植"],
            vec!["rescue", "transplant"],
            vec!["stem", "cell", "rescue"],
        ],
        vec![vec!["rescue"], vec!["救命"]],
        vec![vec!["blood"], vec!["血液"]],
    ];
    let parattice = PaRattice::new(paradict);
    let words = vec!["造血", "幹", "細胞", "移植"];
    let lattice = parattice.get_lattice(&words, true, 1);
    let index_data = lattice.dump_for_search_index();
    let mut expected = vec![
        ("造血", 0, 3, 0, 1),
        ("blood", 0, 1, 0, 3),
        ("hematopoietic", 0, 2, 0, 3),
        ("rescue", 3, 6, 1, 4),
        ("stem", 3, 7, 1, 3),
        ("幹", 3, 8, 1, 2),
        ("stem", 1, 4, 0, 3),
        ("stem", 2, 4, 0, 3),
        ("stem", 2, 5, 0, 4),
        ("cell", 7, 10, 1, 4),
        ("cell", 7, 11, 1, 3),
        ("細胞", 8, 11, 2, 3),
        ("cell", 4, 11, 0, 3),
        ("cell", 5, 9, 0, 4),
        ("transplant", 6, 12, 1, 4),
        ("rescue", 10, 12, 1, 4),
        ("移植", 11, 12, 3, 4),
        ("transplantation", 9, 12, 0, 4),
    ];
    expected.sort();
    assert_eq!(expected, search_index_relative_to_absolute(&index_data));
}
