#[macro_export]
macro_rules! debug_unreachable {
    ($($t:tt)*) => {
        if cfg!(debug_assertions) {
            unreachable!($($t)*);
        } else {
            unsafe { core::hint::unreachable_unchecked() };
        }
    };
}

macro_rules! assume {
    ($condition:expr) => {
        if !$condition {
            $crate::debug_unreachable!("assumption");
        }
    };
}

pub fn direct(flag: bool) -> u32 {
    match flag {
        true => 7,
        false => unsafe { core::hint::unreachable_unchecked() },
    }
}

pub fn macro_guard(flag: bool) -> u32 {
    assume!(flag);
    7
}

pub fn evaluate_once() -> u32 {
    let mut count = 0;
    assume!({ count += 1; true });
    count
}

#[no_mangle]
pub extern "C" fn direct_good() -> u32 { direct(true) }
#[no_mangle]
pub extern "C" fn direct_bad() -> u32 { direct(false) }
#[no_mangle]
pub extern "C" fn macro_good() -> u32 { macro_guard(true) }
#[no_mangle]
pub extern "C" fn macro_bad() -> u32 { macro_guard(false) }
#[no_mangle]
pub extern "C" fn prior_unreachable() -> u32 {
    let _ = direct(false);
    macro_guard(false)
}

#[cfg(test)]
mod tests {
    #[test]
    fn valid_branches_and_single_evaluation_are_preserved() {
        assert_eq!(super::direct(true), 7);
        assert_eq!(super::macro_guard(true), 7);
        assert_eq!(super::evaluate_once(), 1);
    }
}
