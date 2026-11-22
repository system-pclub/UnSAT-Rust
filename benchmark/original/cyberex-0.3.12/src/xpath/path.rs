pub fn path_to_string<Paths>(p: Paths) -> String
where
    Paths: AsRef<std::path::Path>,
{
    p.as_ref().display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path_to_string() {
        use std::path::{Path, PathBuf};
        assert_eq!(path_to_string("/hello/world"), "/hello/world");
        assert_eq!(path_to_string(Path::new("/hello/world")), "/hello/world");
        assert_eq!(path_to_string(PathBuf::from("/hello/world")), "/hello/world");
    }
}
