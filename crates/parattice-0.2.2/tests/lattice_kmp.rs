extern crate parattice;

use parattice::LatticeKMP;
use parattice::PaRattice;

#[test]
fn lattice_kmp_test() {
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
    let pattern = vec!["幹", "細胞"];
    let kmp = LatticeKMP::new(pattern);
    let mut results = kmp.search(&lattice);
    results.sort();
    let expected = vec![
        vec![("", 1), ("幹", 10), ("細胞", 15)],
        vec![("", 2), ("幹", 7), ("細胞", 13)],
        vec![("", 3), ("幹", 9), ("細胞", 13)],
        vec![("", 3), ("幹", 9), ("細胞", 14)],
        vec![("", 3), ("幹", 10), ("細胞", 15)],
    ];
    assert_eq!(expected, results);
}
