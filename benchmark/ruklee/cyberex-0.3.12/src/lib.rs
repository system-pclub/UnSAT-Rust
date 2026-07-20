pub mod void;
pub mod golike;
pub mod xself;
pub mod buf_pro;
pub mod xfs;
pub mod xbuild;
pub mod xffi;
pub mod xnum;
pub mod env;
pub mod xpath;
pub mod plat;

#[cfg(test)]
mod compile_only_tests {
    use std::os::raw::c_char;

    #[test]
    #[ignore = "compile-only LLVM IR instantiation"]
    fn instantiate_void_and_string_pointer_helpers() {
        let mut value = 1u8;
        let _ = crate::void::opacue_to_mut(&mut value);
        let _ = crate::void::opacue_to_ref(&value);

        let allocated = crate::void::new(1u8);
        crate::void::delete::<u8>(allocated);
        let _ = crate::void::new_and_then(1u8, |value| {
            *value += 1;
            Ok(())
        });

        let c_string = b"x\0";
        let _ = crate::xffi::sto::cchar_to_string(c_string.as_ptr() as *const c_char);

        let mut fixed = [0u8; 4];
        crate::xffi::sto::string_to_buffer("x", fixed.as_mut_ptr(), fixed.len());
        crate::xffi::xtr::string_to_buffer("x", fixed.as_mut_ptr(), fixed.len());

        let mut dynamic = std::ptr::null_mut();
        let mut dynamic_len = 0usize;
        crate::xffi::sto::string_to_dbuffer("x", &mut dynamic, &mut dynamic_len);
        crate::xffi::xtr::string_to_dbuffer("x", &mut dynamic, &mut dynamic_len);
    }
}

#[cfg(feature = "enable-async")]
pub mod xasync;
