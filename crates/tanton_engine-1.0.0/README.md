# Tanton Engine

Tanton Engine is a Rust re-write of the [Stockfish](https://stockfishchess.org/) chess engine.

It is a fork of a now unmaintained project [Pleco](https://github.com/sfleischman105/Pleco).

[![Tanton crate](https://img.shields.io/crates/v/tanton_engine.svg)](https://crates.io/crates/tanton_engine)

This project is split into two crates, `tanton_engine` (the current folder), which contains the
UCI (Universal Chess Interface) compatible Engine & AI, and `tanton`, which contains the library functionality.

The overall goal of tanton is to recreate the Stockfish engine in rust, for comparison and
educational purposes. As such, the majority of the algorithms used here are a direct port of stockfish's, and the
credit for all of the advanced algorithms used for searching, evaluation, and many others, go directly to the
maintainers and authors of Stockfish.

- [Documentation](https://docs.rs/tanton_engine)
- [crates.io](https://crates.io/crates/tanton_engine)

## Standalone Installation and Use

Currently, Tanton's use as a standalone program is limited in functionality. A UCI client is needed to properly interact with the program.
As a recommendation, check out [Arena](http://www.playwitharena.com/).

The easiest way to use the engine would be to check out the "releases" tab,
[here](https://github.com/chase-manning/tanton/releases).

If you would rather build it yourself (for a specific architecture, or otherwise), clone the repo
and navigate into the created folder with the following commands:

```
$ git clone https://github.com/chase-manning/tanton
$ cd tanton/
```

Once inside the tanton directory, build the binaries using `cargo`:

```
$ cargo build --release
```

The compiled program will appear in `./target/release/`.

Tanton can now be run with a `./Tanton` on Linux or a `./Tanton.exe` on Windows.
