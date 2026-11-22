# Shaman

[![Build Status](https://travis-ci.org/zcdziura/shaman.svg)](https://travis-ci.org/zcdziura/shaman)

A pure-Rust, cross-platform implementation of the SHA family of hashing algorithms.

## Synopsis

Shaman is a kind-of fork of the popular, well-written [rust-crypto](https://github.com/DaGenix/rust-crypto) library.
Shaman seeks to be a cross-platform implementation of the SHA family of hashing algorithms. All of the extra functionality
found within rust-crypto have been removed. Unless otherwise shown, all credit for implementing these algorithms goes
to the original rust-crypto developers; I have done nothing but remove unnecessary code.

## Usage

To use Shaman, add the following to your Cargo.toml:

```toml
[dependencies]
shaman = "*"
```

and the following to your crate root:

```rust
extern crate shaman;
```

## Why Should I Use This?

That's the obvious question. Why should you use this library, when there's already a stable, widely used library available
that provides the same (and more) functionality? Because you only want to generate SHA hashes. Rust-crypto, as wonderful as
it is, provides a LOT of extra functionality that you may not want. Even if you're only using it to generate SHA hashes,
you are still expected to download, compile, and link all of rust-crypto's dependencies. Seems a little unnecessary to me!

Personally, I believe that a library should Do One Thing And Do It Well. As useful as it is to have one large library available
that suits all of your needs, if you only use it for a small subset of its features, you're wasting space. Even if your
project has quite a bit of functionality, you won't get anything more than what you need.

## License

Shaman, like it's upstream parent, is dual licensed under the MIT and Apache 2.0 licenses, the same licenses
as the Rust compiler.