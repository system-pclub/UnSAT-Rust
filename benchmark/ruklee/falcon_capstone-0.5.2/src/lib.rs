pub mod capstone;
pub mod capstone_sys;

#[cfg(test)]
mod ruklee_monomorphize_tests {
    use super::capstone::{cs_arch, cs_opt_type, cs_opt_value, Capstone, InstrBuf, CS_MODE_32};
    use super::capstone_sys::cs_insn;

    #[test]
    fn instrbuf_get_is_present_in_default_test_harness_ir() {
        let ptr = std::ptr::NonNull::<cs_insn>::dangling().as_ptr();
        let buf = InstrBuf::new(ptr, std::hint::black_box(0), false, cs_arch::CS_ARCH_X86);

        let _ = buf.get(std::hint::black_box(0));

        // This test exists only to make the default `cargo test --lib --no-run`
        // harness monomorphize `InstrBuf::get` for Ruklee IR generation.  Avoid
        // running `Drop`, which would call into Capstone's `cs_free` on the
        // synthetic pointer above if someone executes the test normally.
        std::mem::forget(buf);
    }

    #[test]
    fn capstone_methods_are_present_in_default_test_harness_ir() {
        if let Ok(decoder) = Capstone::new(cs_arch::CS_ARCH_X86, CS_MODE_32) {
            let decoder = std::mem::ManuallyDrop::new(decoder);

            let _ = decoder.option(
                cs_opt_type::CS_OPT_DETAIL,
                std::hint::black_box(cs_opt_value::CS_OPT_OFF),
            );
            let _ = decoder.reg_name(std::hint::black_box(0));
            let _ = decoder.group_name(std::hint::black_box(0));
        }
    }
}
