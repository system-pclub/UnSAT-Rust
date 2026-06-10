## Aragonite

Aragonite is a form of calcium-carbonate (CaCO3) commonly found in the materials for shell formation. In that spirit, aragonite is a set of tools necessary that makes generating position-independent shellcode in rust easy to do.

## Supported targets

- Windows, x64
- Linux, x64

## Usage

1. Currently, development is only supported on linux-x64 hosts with gnu-build chain (aka `x86_64-unknown-linux-gnu`). Ensure you have the required tooling/environment to compile code for this target
1. Install the `cargo-aragonite` build tool via `cargo install cargo-aragonite`. This is a thin-wrapper that sets up the correct environment, release profile, and build scripts to generate proper position-independent shellcode.
1. Annotate your main function with the `aragonite_main` attribute. This handles setting up the correct attributes for the linker script, and automatic clean exits if a target family is supplied. See the `examples/src/bin` folder for examples.
1. Build your shellcode with the build tool: `cargo aragonite build`. Any extra arguments are transparently passed to cargo.
1. The produced shellcode will be in `targets/x86_64-unknown-linux-gnu/aragonite/[binaryname]`

## Detailed Information

### `#[aragonite_main]` attributes

#### `family`

Can be set to the following values:

| value | description |
|----|----|
| `win` | support for windows targets, performs automatic cleanup by calling the `ExitProcess(0)` function in `kernel32.dll` |
| `linux` | support for linux targets, performs automatic cleanup by calling the `sys_exit(0)` syscall for the target arch |

##### Example

```rust
#[aragonite_main(family = "win")]
fn main() {
    // my code here, will automatically call ExitProcess(0) at the end of the function
}
```

#### `arch`

Can be set to the following values:

| value | description |
|----|----|
| `x64` | support for x64 targets, used to select correct ABI based on `family` attribute |

##### Example

```rust
#[aragonite_main(family = "win", arch = "x64")]
fn main() {
    // my code here, the code for ExitProcess(0) will assume it's running in a 64-bit process
}
```

#### `no_cleanup`

This flag disables automatic cleanup code generation if a `family` is specified.

##### Example

```rust
#[aragonite_main(no_cleanup, family = "win")]
fn main() {
    // my code here, no extra code will be added
}
```
