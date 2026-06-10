pub(crate) fn cmp_utf16_ascii_caseinsensitive(left: &[u16], right: &str) -> bool {
    let left = core::char::decode_utf16(left.iter().copied());
    let right = right.chars();
    for (lc, rc) in core::iter::zip(left, right) {
        let lc = lc.unwrap();
        if lc.to_ascii_lowercase() != rc.to_ascii_lowercase() {
            return false;
        }
    }
    true
}
/// Returns the length of the c-string from the raw pointer `ptr` by searching
/// for the terminating null-byte. With support for no_std
pub fn c_strlen(ptr: *const u8) -> usize {
    let mut i = 0;
    unsafe {
        klee_ext_bind::bind!(& i, "i");
        klee_ext_bind::callsite!("src-win-x64-pebwalk-utils-rs-18-25");
        let mut curr = *(ptr.add(i));
        while curr != 0 {
            i += 1;
            klee_ext_bind::bind!(& i, "i");
            klee_ext_bind::callsite!("src-win-x64-pebwalk-utils-rs-21-21");
            curr = *(ptr.add(i));
        }
    }
    i
}
/// Compares two slices, element by element, for equality. With support for no_std
pub fn slicecmp<T: core::cmp::PartialEq>(left: &[T], right: &[T]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for (l, r) in core::iter::zip(left, right) {
        if l != r {
            return false;
        }
    }
    true
}
