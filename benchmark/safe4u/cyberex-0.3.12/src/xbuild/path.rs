use crate::plat::dist::{plat_dist, PlatDist};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct DevPath {
    pub include: PathBuf,
    pub lib: PathBuf,
}

impl DevPath {
    pub fn get_shared(&self) -> Vec<PathBuf> {
        let mut netca_libs = Vec::new();
        if let Ok(paths) = std::fs::read_dir(&self.lib) {
            for path in paths.flatten() {
                let path = path.path();
                if path.is_file() && path.extension() == Some(get_dist_lib_suffix().as_ref()) {
                    netca_libs.push(path);
                }
            }
        }

        netca_libs
    }

    pub fn include(&self) -> &PathBuf {
        &self.include
    }
    pub fn lib(&self) -> &PathBuf {
        &self.lib
    }
}

pub fn watch<Paths>(path: Paths) -> PathBuf
where
    Paths: AsRef<std::path::Path>,
{
    if path.as_ref().exists() {
        // use :: when rust > 1.77
        println!("cargo:rerun-if-changed={}", path.as_ref().join("*").display());
    }
    path.as_ref().to_path_buf()
}

fn get_dist_lib_name() -> String {
    match plat_dist() {
        PlatDist::Rh => "lib64",
        PlatDist::Debian => "lib",
        PlatDist::Other => "lib",
    }
    .to_string()
}

fn get_dist_lib_suffix() -> String {
    if cfg!(target_os = "linux") {
        "so".to_string()
    } else if cfg!(target_os = "windows") {
        "dll".to_string()
    } else {
        panic!("unkown os")
    }
}

pub fn lib_path_of_root<Paths>(lib_root: Paths) -> PathBuf
where
    Paths: AsRef<std::path::Path>,
{
    let p = Path::new(lib_root.as_ref());
    let lib_must = p.join(get_dist_lib_name());
    let lib_alt = if lib_must.ends_with("64") { "lib" } else { "lib64" };
    let path = if lib_must.exists() { lib_must } else { p.join(lib_alt) };
    watch(path)
}
pub fn include_path_of_root<Paths>(lib_root: Paths) -> PathBuf
where
    Paths: AsRef<std::path::Path>,
{
    let path = lib_root.as_ref().join("include");

    watch(path)
}

pub fn dev_path_of_root<Paths>(lib_root: Paths) -> DevPath
where
    Paths: AsRef<std::path::Path>,
{
    DevPath {
        include: include_path_of_root(&lib_root),
        lib: lib_path_of_root(lib_root),
    }
}
pub fn dev_path_of_root_env_or<K>(lib_root_key: K, default: DevPath) -> DevPath
where
    K: AsRef<OsStr>,
{
    match env::var(lib_root_key) {
        Err(_) => default,
        Ok(lib_root) => dev_path_of_root(lib_root),
    }
}

pub fn dev_path_of_root_env<K>(lib_root_key: K) -> DevPath
where
    K: AsRef<OsStr>,
{
    dev_path_of_root_env_or(lib_root_key, DevPath::default())
}

pub fn cargo_profile_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}
pub fn cargo_root_profile_dir() -> PathBuf {
    cargo_target_dir().parent().unwrap().to_path_buf()
}
pub fn cargo_target_dir() -> PathBuf {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with("target") {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.unwrap();
    target_dir.to_path_buf()
}

pub fn cargo_target_bin_dir() -> PathBuf {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let profile = std::env::var("PROFILE").unwrap();
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.unwrap();
    target_dir.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_lib_path_of_root() {
        match plat_dist() {
            PlatDist::Rh => assert_eq!(
                lib_path_of_root(PathBuf::from("/noexist")),
                PathBuf::from("/noexist/lib")
            ),
            PlatDist::Debian => assert_eq!(
                lib_path_of_root(PathBuf::from("/noexist")),
                PathBuf::from("/noexist/lib64")
            ),
            PlatDist::Other => {},
        };

        if cfg!(target_os = "windows") {
            assert_eq!(
                lib_path_of_root(
                    r#"D:\code\thirdlib\EaxLibrary\EaxComponent\gb_media\build\gb-media\thirdlib\ffmpeg\"#
                ),
                PathBuf::from(
                    r#"D:\code\thirdlib\EaxLibrary\EaxComponent\gb_media\build\gb-media\thirdlib\ffmpeg\lib"#
                )
            );
        }
        if cfg!(target_os = "linux") {
            println!("fuck you ");
            let catch_root = "/workspace/cyberex/thirdlib/ffmpeg";
            let catch_lib = "/workspace/cyberex/thirdlib/ffmpeg/lib";
            let catch_include = "/workspace/cyberex/thirdlib/ffmpeg/include";

            assert_eq!(lib_path_of_root(catch_root), PathBuf::from(catch_lib));
            assert_eq!(include_path_of_root(catch_root), PathBuf::from(catch_include));
            let dev_path = dev_path_of_root(catch_root);
            assert_eq!(dev_path.lib.display().to_string(), catch_lib);
            assert_eq!(dev_path.include.display().to_string(), catch_include);
            assert_eq!(
                {
                    let mut v = dev_path.get_shared();
                    v.sort();
                    v
                },
                {
                    let mut v = Vec::from([
                        PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavformat.so"),
                        PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavutil.so"),
                        PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavfilter.so"),
                        PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavcodec.so"),
                        PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libswscale.so"),
                        PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libswresample.so"),
                        PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavdevice.so"),
                    ]);
                    v.sort();
                    v
                }
            );

            let env_var = "FUCKYOU_ROOT";
            unsafe { std::env::set_var(env_var, "/workspace/cyberex/thirdlib/catch2") };
            let dev_path = dev_path_of_root_env(env_var);
            assert_eq!(
                dev_path.lib.display().to_string(),
                "/workspace/cyberex/thirdlib/catch2/lib"
            );
            assert_eq!(
                dev_path.include.display().to_string(),
                "/workspace/cyberex/thirdlib/catch2/include"
            );

            let dev_path = dev_path_of_root_env("noexist");
            assert_eq!(dev_path, DevPath::default());
        }
    }
    #[test]
    fn test_dev_path_of_root_env() {
        let env_var = "NO_SET_ENV";
        let default_value = DevPath {
            include: "fuck".into(),
            lib: "you".into(),
        };
        let dev_path = dev_path_of_root_env_or(env_var, default_value.clone());
        assert!(dev_path == default_value);
    }

    #[test]
    fn test_cargo_target_dir() {
        unsafe { std::env::set_var("PROFILE", "debug") };
        assert_eq!(cargo_target_bin_dir(), PathBuf::from("/workspace/cyberex/target/debug"));
        assert_eq!(cargo_target_dir(), PathBuf::from("/workspace/cyberex/target"));
        assert_eq!(cargo_profile_dir(), PathBuf::from("/workspace/cyberex"));
        assert_eq!(cargo_root_profile_dir(), PathBuf::from("/workspace/cyberex"));
    }
}
