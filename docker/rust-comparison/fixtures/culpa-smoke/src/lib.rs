pub fn unchecked_first(data: &[u8]) -> u8 {
    unsafe { *data.get_unchecked(0) }
}

pub fn checked_first(data: &[u8]) -> Option<u8> {
    if data.is_empty() {
        None
    } else {
        Some(unsafe { *data.get_unchecked(0) })
    }
}
