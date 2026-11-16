# rtvm

**Rust Tos Virtual Machine**

Rtvm is an TVM written in Rust that is focused on **speed** and **simplicity**.
It has a fast and flexible implementation with a simple interface and embedded Host.
It passes all `ethereum/tests` test suites.

Here is a list of guiding principles that Rtvm follows.

* **EVM compatibility and stability** - this goes without saying but it is nice to put it here. In the blockchain industry, stability is the most desired attribute of any system.
* **Speed** - is one of the most important things and most decisions are made to complement this.
* **Simplicity** - simplification of internals so that it can be easily understood and extended, and interface that can be easily used or integrated into other projects.
* **interfacing** - `[no_std]` so that it can be used as wasm lib and integrate with JavaScript and cpp binding if needed.

# Project

Structure:

* crates
  * rtvm -> main TVM library.
  * rtvm-primitives -> Primitive data types.
  * rtvm-interpreter -> Execution loop with instructions
  * rtvm-precompile -> TVM precompiles
* bins:
  * rtvme: cli binary, used for running state test jsons

# Building from source

```shell
git clone https://github.com/tos-network/rtvm.git
cd rtvm
cargo build --release
```

