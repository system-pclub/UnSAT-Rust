/// Stable way to use branch likely

#[inline]
#[cold]
fn cold() {}

#[inline]
/// Condition is likely to happen.
pub fn likely(b: bool) -> bool {
    if !b {
        cold()
    }
    b
}

#[inline]
/// Condition is unlikely to happen.
pub fn unlikely(b: bool) -> bool {
    if b {
        cold()
    }
    b
}
