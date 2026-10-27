#![feature(test)]
extern crate test;
extern crate lzf;
use test::Bencher;

#[bench]
fn speed_on_big_example_native(b: &mut Bencher) {
    use std::fs::File;
    use std::io::Read;

    let file = "/home/badboy/projects/rust/lzf-rs/cantrbry/total.bin";

    let mut f = File::open(file).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut contents).unwrap();

    let compressed = lzf::compress(&contents[..]).unwrap();
    let compressed_ar = &compressed[..];
    let len = contents.len();

    b.iter(|| {
        let _ = lzf::native::decompress(compressed_ar, len).unwrap();
    });
}
#[bench]
fn speed_on_big_example(b: &mut Bencher) {
    use std::fs::File;
    use std::io::Read;

    let file = "/home/badboy/projects/rust/lzf-rs/cantrbry/total.bin";

    let mut f = File::open(file).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut contents).unwrap();

    let compressed = lzf::compress(&contents[..]).unwrap();
    let compressed_ar = &compressed[..];
    let len = contents.len();

    b.iter(|| {
        let _ = lzf::decompress(compressed_ar, len).unwrap();
    });
}
