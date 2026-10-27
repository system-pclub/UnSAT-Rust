fn main() {
    use jh::Digest;
    use jh::Jh256;

    // This is a safe, public API call.
    // It hashes an empty message, which is enough to exercise the internal
    // compressor path that uses `ptr::read_unaligned(data.offset(1..3))`.
    let mut hasher = Jh256::new();
    let _ = hasher.finalize();

    println!("hashed empty message");
}
