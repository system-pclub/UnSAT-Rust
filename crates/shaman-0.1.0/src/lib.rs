// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rand;
extern crate rustc_serialize as serialize;

pub mod buffer;
pub mod cryptoutil;
pub mod digest;
pub mod sha1;
pub mod sha2;
mod simd;
mod step_by;
mod symmetriccipher;