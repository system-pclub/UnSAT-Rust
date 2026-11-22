#[macro_export]
macro_rules! ref_cast {
    ($from:expr) => {{
        &*($from as *const _ as *const _)
    }};
}
#[cfg(test)]

mod tests {
    #[allow(dead_code)]
    struct S1 {
        age: i32,
    }
    struct S2 {
        age: i32,
    }

    fn proc_s2(s: &S2) -> i32 {
        s.age + 1
    }
    #[test]
    fn test_ref_cast() {
        let s1 = S1 { age: 10 };
        assert_eq!(proc_s2(unsafe { ref_cast!(&s1) }), 11);
    }
}
