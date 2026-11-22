use crate::xpath::path::path_to_string;
use anyhow::Result;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

static CARGO_SCRIPT_SKIP_RUSTC_LINK_LIB: &str = "CARGO_SCRIPT_SKIP_RUSTC_LINK_LIB";

pub enum LibKind {
    Shared(String),
    Static(String),
    Auto(String),
}

fn libname_strip(lib_name: &str) -> String {
    if cfg!(target_os = "linux") {
        // Note: here remove the suffix in linux, beacause windows contain '*.a' as static library
        let temp = lib_name.strip_prefix("lib").unwrap_or(lib_name);
        path_to_string(std::path::Path::new(&temp).file_stem().unwrap_or(temp.as_ref()))
    } else {
        lib_name.to_string()
    }
}

pub fn format_target_link_libraries(kind: LibKind) -> Option<String> {
    match std::env::var(CARGO_SCRIPT_SKIP_RUSTC_LINK_LIB) {
        Ok(value) if value == "1" => None,
        _ => {
            let line = format!(
                "cargo:rustc-link-lib={}",
                match kind {
                    LibKind::Shared(name) => "dylib=".to_string() + &libname_strip(&name),
                    LibKind::Static(name) => "static=".to_string() + &libname_strip(&name),
                    LibKind::Auto(name) => libname_strip(&name),
                }
            );
            Some(line)
        },
    }
}

pub fn target_link_libraries<Libs>(kinds: Libs)
where
    Libs: IntoIterator<Item = LibKind>,
{
    for kind in kinds {
        if let Some(line) = format_target_link_libraries(kind) {
            println!("{}", line);
        }
    }
}

pub fn target_link_directories<Paths>(p: Paths)
where
    Paths: IntoIterator,
    Paths::Item: AsRef<std::path::Path>,
{
    for path in p {
        let path_str = path.as_ref().display().to_string();
        if path_str.is_empty() {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path_str);
        println!("cargo:rustc-link-search={}", path_str);
    }
}

#[derive(Default)]
pub struct Target {
    pub name: String,
    pub files: String,
    pub include_dir: Option<String>,
    pub lib_dir: Option<String>,
    pub dep: Vec<String>,
    pub type_: String,
}
pub struct Module {
    module_name: String,
    targets: Vec<Target>,
    out_path: PathBuf,
    version: String,
}

impl Module {
    pub fn builder() -> ModuleBuilder {
        ModuleBuilder::default()
    }
    pub fn write(&self) -> Result<()> {
        create_dir_all(&self.out_path)?;
        self.write_version_file()?;
        self.write_target_file()?;
        self.write_config_file()?;
        Ok(())
    }

    fn get_target_file_name(&self) -> String {
        format!("{}Targets.cmake", self.module_name)
    }

    fn write_config_file(&self) -> Result<()> {
        let file_name = format!("{}Config.cmake", self.module_name);
        let mut file = File::create(self.out_path.join(file_name))?;
        let text = self.get_config_str()?;
        file.write_all(text.as_bytes())?;

        Ok(())
    }
    fn get_config_str(&self) -> Result<String> {
        let s = format!(
            r#"include(${{CMAKE_CURRENT_LIST_DIR}}/{})"#,
            self.get_target_file_name()
        );
        Ok(s)
    }

    fn write_version_file(&self) -> Result<()> {
        let file_name = format!("{}ConfigVersion.cmake", self.module_name);
        let mut file = File::create(self.out_path.join(file_name))?;
        let text = self.get_verion_str()?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }

    fn get_verion_str(&self) -> Result<String> {
        let s = format!(r#"set(PACKAGE_VERSION "{}")"#, self.version);

        Ok(s)
    }

    fn get_target_str(&self) -> Result<String> {
        let mut s = String::new();
        s += r#"
get_filename_component(var_import_prefix "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(var_import_prefix "${var_import_prefix}" PATH)"#
            .trim();
        s += "\n";
        for target in &self.targets {
            let sub_target_name = format!("{}::{}", self.module_name, target.name);
            s += &format!(r#"add_library({} {} IMPORTED)"#, sub_target_name, target.type_);
            s += "\n";

            s += &format!(r#"set_target_properties({} PROPERTIES"#, sub_target_name);
            s += "\n";

            {
                let mut prop_line = "".to_string();
                if let Some(include_dir) = &target.include_dir {
                    prop_line = format!(
                        r#"INTERFACE_INCLUDE_DIRECTORIES  "${{var_import_prefix}}/{}""#,
                        include_dir
                    );
                }
                s += &prop_line;
                s += "\n";
            }
            {
                let mut prop_line = "".to_string();
                if let Some(lib_dir) = &target.lib_dir {
                    prop_line = format!(
                        r#"IMPORTED_LOCATION  "${{var_import_prefix}}/{}/{}""#,
                        lib_dir, target.files
                    );
                }
                s += &prop_line;

                s += "\n";
            }
            {
                s += "IMPORTED_NO_SONAME TRUE";
                s += "\n";
            }
            {
                if !target.dep.is_empty() {
                    let dep_part = target.dep.join(";");
                    s += &format!(r#"INTERFACE_LINK_LIBRARIES "{}""#, dep_part);
                    s += "\n";
                }
            }
            s += ")";

            s += "\n";
            s += r#"set(${CMAKE_FIND_PACKAGE_NAME}_FOUND TRUE)"#;

            s += "\n";
            s += &format!(
                r#"message(STATUS "Using {} ${{{}_VERSION}}")"#,
                self.module_name, self.module_name
            );
        }
        Ok(s)
    }

    fn write_target_file(&self) -> Result<()> {
        let file_name = self.get_target_file_name();
        let mut file = File::create(self.out_path.join(file_name))?;

        let text = self.get_target_str()?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }
}

#[derive(Default)]
pub struct ModuleBuilder {
    target_name: Vec<Target>,
    out_path: Option<PathBuf>,
    module_name: Option<String>,
    version: Option<String>,
}
impl ModuleBuilder {
    pub fn add_target(mut self, target: Target) -> Self {
        self.target_name.push(target);
        self
    }
    pub fn module_name(mut self, module_name: impl Into<String>) -> Self {
        self.module_name = Some(module_name.into());
        self
    }
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    pub fn out_path(mut self, out_path: impl AsRef<Path>) -> Self {
        self.out_path = Some(out_path.as_ref().to_path_buf());
        self
    }

    pub fn build(self) -> Module {
        Module {
            version: self.version.expect("Version must be set"),
            module_name: self.module_name.expect("Module name must be set"),
            targets: self.target_name,
            out_path: self.out_path.expect("Output path must be set"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_target_link_libraries() {
        assert_eq!(
            format_target_link_libraries(LibKind::Shared("libm".into())),
            Some("cargo:rustc-link-lib=dylib=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Static("libm".into())),
            Some("cargo:rustc-link-lib=static=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm".into())),
            Some("cargo:rustc-link-lib=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("m.a".into())),
            Some("cargo:rustc-link-lib=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm.a".into())),
            Some("cargo:rustc-link-lib=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm.lib".into())),
            Some("cargo:rustc-link-lib=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm.so".into())),
            Some("cargo:rustc-link-lib=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Shared("m".into())),
            Some("cargo:rustc-link-lib=dylib=m".to_string())
        );

        assert_eq!(
            format_target_link_libraries(LibKind::Static("m".into())),
            Some("cargo:rustc-link-lib=static=m".to_string())
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("m".into())),
            Some("cargo:rustc-link-lib=m".to_string())
        );
    }
    #[test]
    fn test_format_target_link_libraries_skip() {
        std::env::set_var("CARGO_SCRIPT_SKIP_RUSTC_LINK_LIB", "1");

        assert_eq!(format_target_link_libraries(LibKind::Auto("m".into())), None);
    }

    #[test]
    fn test_target_link_libraries() {
        target_link_libraries([LibKind::Shared("z".to_string())]);
        target_link_libraries(vec![LibKind::Shared("z".to_string())]);
    }

    #[test]
    fn test_target_link_directories() {
        target_link_directories([""]);
        target_link_directories(["path1"]);
        target_link_directories(vec!["path1"]);
    }

    #[test]
    fn test_module_writer() {
        // Mod
        let m = Module::builder()
            .module_name("FUCK")
            .version("2.0")
            .out_path("/workspace/cyberex/target")
            .add_target(Target {
                name: "YOU".to_string(),
                files: "fuck.so".to_string(),
                include_dir: Some("include".to_string()),
                lib_dir: Some("lib".to_string()),
                dep: vec!["dl".to_string(), "ssl".to_string()],
                type_: "SHARED".to_string(),
            })
            .build();
        assert_eq!(
            m.get_target_str().unwrap().trim(),
            r#"
get_filename_component(var_import_prefix "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(var_import_prefix "${var_import_prefix}" PATH)
add_library(FUCK::YOU SHARED IMPORTED)
set_target_properties(FUCK::YOU PROPERTIES
INTERFACE_INCLUDE_DIRECTORIES  "${var_import_prefix}/include"
IMPORTED_LOCATION  "${var_import_prefix}/lib/fuck.so"
IMPORTED_NO_SONAME TRUE
INTERFACE_LINK_LIBRARIES "dl;ssl"
)
set(${CMAKE_FIND_PACKAGE_NAME}_FOUND TRUE)
message(STATUS "Using FUCK ${FUCK_VERSION}")
        "#
            .trim()
        );
        assert_eq!(
            m.get_verion_str().unwrap().trim(),
            r#"set(PACKAGE_VERSION "2.0")"#.trim()
        );
        assert_eq!(
            m.get_config_str().unwrap().trim(),
            r#"include(${CMAKE_CURRENT_LIST_DIR}/FUCKTargets.cmake)"#.trim()
        );
        m.write().unwrap();
    }
}
