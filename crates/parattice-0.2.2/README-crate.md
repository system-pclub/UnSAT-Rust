# parattice: Recursive paraphrase lattice generator 🔄

This library takes a sentence and a paraphrase corpus, recursively finds
paraphrases based on the corpus, expands the given sentence, and generates a
paraphrase lattice.

This library also provides a method to search a phrase in the paraphrase
lattice.

## Example

This example generates a paraphrase lattice and searches a phrase in the generated lattice.

```rust
use parattice::Lattice;
use parattice::LatticeKMP;
use parattice::PaRattice;

// initialization
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

// lattice generation
let words = vec!["造血", "幹", "細胞", "移植"];
let lattice = parattice.get_lattice(&words, true, 2);

// dump a generated lattice
println!(lattice.dump_dot(true));

// serialization & deserialization
let bytes = lattice.to_bytes();
let new_lattice = Lattice::new_from_bytes(&bytes);

// search
let kmp = LatticeKMP::new(vec!["幹", "細胞"]);
let results = kmp.search(&new_lattice);
for result in &results {
    for edge in result {
        print!("({}, {}) ", edge.0, edge.1);
    }
    println!();
    let s = lattice.get_trunk_span(result.clone());
    for edge in &s {
        print!("({}, {}) ", edge.0, edge.1);
    }
    println!("\n===========");
}
```

## Patents

* [JP2019153267](https://patentscope2.wipo.int/search/en/detail.jsf?docId=JP274788235)
* [特許第6435467号](https://www.j-platpat.inpit.go.jp/c1800/PU/JP-2019-153267/E7C117D77F8BF276A28A31DC60BF7E4CC5B53B3F230980164BD96541AA9DAA0F/11/ja)

## License

Copyright 2020 [Xcoo, Inc.](https://xcoo.jp/)

Licensed under the [Apache License, Version 2.0](/LICENSE).
