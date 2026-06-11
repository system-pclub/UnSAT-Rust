# mod3d_base

A base library for 3d model generation.

This provides a simple abstract definition of 3D model objects that
can be created, converted into base objects for a GPU, and then
instantiated arbitrarily at reasonably high performance.

## Usage


```
cargo add mod3d_base
```

## Features

An optional 'serde' feature is provided that permmits serialization /
deserialization of (some) of the structures; in the longer term this
will support all the structures, but it might incur a performance
penalty to support this. That penalty will not apply if serde is not used.

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
