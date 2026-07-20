#[macro_export]
macro_rules! ipp {
    ($num:expr) => {{
        let bk = $num;
        $num += 1;
        bk
    }};
}

#[macro_export]
macro_rules! ppi {
    ($num:expr) => {{
        $num += 1;
        $num
    }};
}
#[macro_export]
macro_rules! idd {
    ($num:expr) => {{
        let bk = $num;
        $num -= 1;
        bk
    }};
}

#[macro_export]
macro_rules! ddi {
    ($num:expr) => {{
        $num -= 1;
        $num
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_case_ipp() {
        let mut i = 0;
        assert_eq!(ipp!(i), 0);
        assert_eq!(i, 1);
    }
    #[test]
    fn test_case_ppi() {
        let mut i = 0;
        assert_eq!(ppi!(i), 1);
        assert_eq!(i, 1);
    }
    #[test]
    fn test_case_idd() {
        let mut i: i32 = 0;
        assert_eq!(idd!(i), 0);
        assert_eq!(i, -1);
    }
    #[test]
    fn test_case_ddi() {
        let mut i = 0;
        assert_eq!(ddi!(i), -1);
        assert_eq!(i, -1);
    }
}
