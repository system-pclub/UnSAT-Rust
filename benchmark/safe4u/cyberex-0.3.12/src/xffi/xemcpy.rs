#[macro_export]
macro_rules! xemcpy {
    ($dst:expr,$src:expr,$len:expr) => {{
        std::ptr::copy_nonoverlapping($src as _, $dst as *mut _ as *mut _, $len as _);
    }};
}
