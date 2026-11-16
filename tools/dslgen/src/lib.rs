use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use quote::ToTokens;
use serde::Serialize;
use syn::{
    spanned::Spanned, Fields, GenericArgument, GenericParam, Item, PathArguments, Type, UseTree,
};

pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, Serialize)]
pub struct Source {
    pub file: String,
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Ty {
    Named {
        name: String,
        args: Vec<Ty>,
    },
    Parameter {
        name: String,
    },
    Const {
        expression: String,
    },
    Pointer {
        element: Box<Ty>,
        mutable: bool,
    },
    Sequence {
        element: Box<Ty>,
        length: Option<String>,
    },
    Tuple {
        elements: Vec<Ty>,
    },
    Opaque {
        rust: String,
    },
}

impl Ty {
    pub fn display(&self) -> String {
        match self {
            Self::Named { name, args } if args.is_empty() => name.clone(),
            Self::Named { name, args } => format!(
                "{}<{}>",
                name,
                args.iter().map(Ty::display).collect::<Vec<_>>().join(", ")
            ),
            Self::Parameter { name } => format!("${name}"),
            Self::Const { expression } => format!("const({expression})"),
            Self::Pointer { element, mutable } => format!(
                "*{} {}",
                if *mutable { "mut" } else { "const" },
                element.display()
            ),
            Self::Sequence { element, length } => match length {
                Some(n) => format!("[{}; {n}]", element.display()),
                None => format!("[{}]", element.display()),
            },
            Self::Tuple { elements } => format!(
                "({})",
                elements
                    .iter()
                    .map(Ty::display)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Opaque { rust } => format!("opaque({rust})"),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Parameter {
    pub name: String,
    pub kind: String,
    pub const_type: Option<Ty>,
    pub default: Option<Ty>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub ty: Ty,
    pub source: Source,
}

#[derive(Clone, Debug, Serialize)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Field>,
    pub tuple: bool,
    pub source: Source,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Shape {
    Record { fields: Vec<Field>, tuple: bool },
    Enum { variants: Vec<Variant> },
    Alias { target: Ty },
}

#[derive(Clone, Debug, Serialize)]
pub struct Definition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub shape: Shape,
    pub source: Source,
    pub conditions: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
    pub source: Source,
    pub message: String,
}

#[derive(Default)]
pub struct Registry {
    raw: BTreeMap<String, (Item, String, PathBuf)>,
    imports: BTreeMap<String, BTreeMap<String, String>>,
    globs: BTreeMap<String, Vec<String>>,
    pub definitions: BTreeMap<String, Definition>,
    pub macros: Vec<Source>,
    pub diagnostics: Vec<Diagnostic>,
    pub files: Vec<String>,
    duplicates: BTreeMap<String, Vec<(Item, String, PathBuf)>>,
}

fn ident(i: &syn::Ident) -> String {
    i.to_string().trim_start_matches("r#").to_string()
}

fn qualified(module: &str, name: &str) -> String {
    if module.is_empty() {
        name.to_string()
    } else {
        format!("{module}::{name}")
    }
}

fn source(file: &Path, span: proc_macro2::Span) -> Source {
    Source {
        file: file.to_string_lossy().into_owned(),
        line: span.start().line,
    }
}

impl Registry {
    pub fn load(inputs: &[PathBuf]) -> Result<Self> {
        let mut files = BTreeMap::new();
        for input in inputs {
            let (base, prefix) = namespace(input);
            let mut local = BTreeMap::new();
            Self::files(&base, input, &mut local)?;
            for (file, module) in local {
                files.insert(file, qualified(&prefix, &module));
            }
        }
        let mut registry = Self::default();
        for (file, module) in files {
            let text = fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
            let (items, skipped) =
                surface::parse(&text).map_err(|e| format!("{}: {e}", file.display()))?;
            registry.files.push(file.to_string_lossy().into_owned());
            registry
                .diagnostics
                .extend(skipped.into_iter().map(|(line, message)| Diagnostic {
                    source: Source {
                        file: file.to_string_lossy().into_owned(),
                        line,
                    },
                    message,
                }));
            registry.collect(&items, &module, &file)?;
        }
        registry.finish()?;
        Ok(registry)
    }

    fn files(base: &Path, path: &Path, files: &mut BTreeMap<PathBuf, String>) -> Result<()> {
        if path.is_dir() {
            let mut children = fs::read_dir(path)
                .map_err(|e| e.to_string())?
                .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>>>()?;
            children.sort();
            for child in children {
                if child.file_name().is_some_and(|n| {
                    n == "target" || n == "generated" || n == "tests" || n == "benches"
                }) {
                    continue;
                }
                Self::files(base, &child, files)?;
            }
        } else if path.extension().is_some_and(|e| e == "rs") {
            if path
                .file_stem()
                .is_some_and(|n| n == "tests" || n == "benches")
            {
                return Ok(());
            }
            let relative = path
                .strip_prefix(base)
                .map_err(|e| e.to_string())?
                .with_extension("");
            let mut parts: Vec<_> = relative
                .iter()
                .map(|s| s.to_string_lossy().into_owned())
                .collect();
            if parts
                .last()
                .is_some_and(|p| matches!(p.as_str(), "lib" | "main" | "mod"))
            {
                parts.pop();
            }
            let module = parts.join("::");
            if let Some(previous) = files.insert(path.to_path_buf(), module.clone()) {
                if previous != module {
                    return Err(format!("conflicting module roots for {}", path.display()));
                }
            }
        } else if !path.exists() {
            return Err(format!("source does not exist: {}", path.display()));
        }
        Ok(())
    }

    fn collect(&mut self, items: &[Item], module: &str, file: &Path) -> Result<()> {
        for item in items {
            let name = match item {
                Item::Struct(i) => Some(ident(&i.ident)),
                Item::Enum(i) => Some(ident(&i.ident)),
                Item::Type(i) => Some(ident(&i.ident)),
                _ => None,
            };
            if let Some(name) = name {
                let name = qualified(module, &name);
                if let std::collections::btree_map::Entry::Vacant(entry) =
                    self.raw.entry(name.clone())
                {
                    entry.insert((item.clone(), module.to_string(), file.to_path_buf()));
                } else {
                    self.duplicates.entry(name).or_default().push((
                        item.clone(),
                        module.to_string(),
                        file.to_path_buf(),
                    ));
                }
            }
            match item {
                Item::Mod(i) => {
                    if let Some((_, items)) = &i.content {
                        self.collect(items, &qualified(module, &ident(&i.ident)), file)?;
                    }
                }
                Item::Use(i) => self.use_tree(module, "", &i.tree)?,
                Item::Macro(i) => self
                    .macros
                    .push(source(file, i.mac.path.segments[0].ident.span())),
                _ => (),
            }
        }
        Ok(())
    }

    fn use_tree(&mut self, module: &str, prefix: &str, tree: &UseTree) -> Result<()> {
        match tree {
            UseTree::Path(p) => {
                self.use_tree(module, &qualified(prefix, &ident(&p.ident)), &p.tree)?
            }
            UseTree::Group(g) => {
                for t in &g.items {
                    self.use_tree(module, prefix, t)?;
                }
            }
            UseTree::Glob(_) => self
                .globs
                .entry(module.into())
                .or_default()
                .push(prefix.into()),
            UseTree::Name(n) => {
                let name = ident(&n.ident);
                let (alias, path) = if name == "self" {
                    (
                        prefix.rsplit("::").next().unwrap_or(prefix).to_string(),
                        prefix.into(),
                    )
                } else {
                    (name.clone(), qualified(prefix, &name))
                };
                self.imports
                    .entry(module.into())
                    .or_default()
                    .insert(alias, path);
            }
            UseTree::Rename(n) => {
                let path = if n.ident == "self" {
                    prefix.into()
                } else {
                    qualified(prefix, &ident(&n.ident))
                };
                self.imports
                    .entry(module.into())
                    .or_default()
                    .insert(ident(&n.rename), path);
            }
        }
        Ok(())
    }

    fn resolve(&self, module: &str, path: &str, seen: &mut BTreeSet<String>) -> Result<String> {
        if seen.len() >= 64 {
            return Err(format!(
                "import expansion is cyclic or too deep: {module}::{path}"
            ));
        }
        let token = format!("{module}|{path}");
        if !seen.insert(token.clone()) {
            return Err(format!(
                "cyclic or ambiguous import resolving {path} in {module}"
            ));
        }
        let result = self.resolve_inner(module, path, seen);
        seen.remove(&token);
        result
    }

    fn resolve_inner(
        &self,
        module: &str,
        path: &str,
        seen: &mut BTreeSet<String>,
    ) -> Result<String> {
        if let Some(path) = path.strip_prefix("crate::") {
            return self.resolve(module.split("::").next().unwrap_or(""), path, seen);
        }
        if let Some(path) = path.strip_prefix("self::") {
            return self.resolve(module, path, seen);
        }
        if let Some(path) = path.strip_prefix("super::") {
            return self.resolve(module.rsplit_once("::").map_or("", |(p, _)| p), path, seen);
        }
        let local = qualified(module, path);
        if self.raw.contains_key(&local) {
            return Ok(local);
        }
        let (first, rest) = path.split_once("::").unwrap_or((path, ""));
        if let Some(target) = self.imports.get(module).and_then(|m| m.get(first)) {
            return self.resolve(
                module,
                &if rest.is_empty() {
                    target.clone()
                } else {
                    qualified(target, rest)
                },
                seen,
            );
        }
        if let Some((parent, leaf)) = path.rsplit_once("::") {
            let parent = match parent.strip_prefix("crate::") {
                Some(p) => p,
                None => parent,
            };
            for scope in [qualified(module, parent), parent.to_string()] {
                if self.imports.contains_key(&scope) || self.globs.contains_key(&scope) {
                    let resolved = self.resolve(&scope, leaf, seen)?;
                    if self.raw.contains_key(&resolved) {
                        return Ok(resolved);
                    }
                }
            }
        }
        let mut candidates = BTreeSet::new();
        if let Some(globs) = self.globs.get(module) {
            for glob in globs {
                let target = glob.strip_prefix("crate::").unwrap_or(glob);
                let direct = qualified(target, path);
                if self.raw.contains_key(&direct) {
                    candidates.insert(direct);
                }
            }
        }
        if candidates.len() > 1 {
            return Err(format!(
                "ambiguous glob import {path} in {module}: {candidates:?}"
            ));
        }
        if let Some(candidate) = candidates.into_iter().next() {
            return Ok(candidate);
        }
        if self.raw.contains_key(path) {
            return Ok(path.into());
        }
        Ok(path.into())
    }

    fn ty(&self, ty: &Type, module: &str, parameters: &BTreeSet<String>) -> Result<Ty> {
        Ok(match ty {
            Type::Reference(t) => self.ty(&t.elem, module, parameters)?,
            Type::Paren(t) => self.ty(&t.elem, module, parameters)?,
            Type::Group(t) => self.ty(&t.elem, module, parameters)?,
            Type::Ptr(t) => Ty::Pointer {
                element: Box::new(self.ty(&t.elem, module, parameters)?),
                mutable: t.mutability.is_some(),
            },
            Type::Slice(t) => Ty::Sequence {
                element: Box::new(self.ty(&t.elem, module, parameters)?),
                length: None,
            },
            Type::Array(t) => Ty::Sequence {
                element: Box::new(self.ty(&t.elem, module, parameters)?),
                length: Some(t.len.to_token_stream().to_string()),
            },
            Type::Tuple(t) => Ty::Tuple {
                elements: t
                    .elems
                    .iter()
                    .map(|t| self.ty(t, module, parameters))
                    .collect::<Result<_>>()?,
            },
            Type::Path(t) if t.qself.is_none() => {
                let name = t
                    .path
                    .segments
                    .iter()
                    .map(|s| ident(&s.ident))
                    .collect::<Vec<_>>()
                    .join("::");
                if parameters.contains(&name) {
                    Ty::Parameter { name }
                } else {
                    let mut args = vec![];
                    for segment in &t.path.segments {
                        if let PathArguments::AngleBracketed(a) = &segment.arguments {
                            for arg in &a.args {
                                match arg {
                                    GenericArgument::Lifetime(_) => (),
                                    GenericArgument::Type(t) => {
                                        args.push(self.ty(t, module, parameters)?)
                                    }
                                    GenericArgument::Const(c) => {
                                        let expression = c.to_token_stream().to_string();
                                        args.push(if parameters.contains(&expression) {
                                            Ty::Parameter { name: expression }
                                        } else {
                                            Ty::Const { expression }
                                        });
                                    }
                                    _ => {
                                        return Ok(Ty::Opaque {
                                            rust: ty.to_token_stream().to_string(),
                                        })
                                    }
                                }
                            }
                        }
                    }
                    match self.resolve(module, &name, &mut BTreeSet::new()) {
                        Ok(name) => Ty::Named { name, args },
                        Err(_) => Ty::Opaque {
                            rust: ty.to_token_stream().to_string(),
                        },
                    }
                }
            }
            _ => Ty::Opaque {
                rust: ty.to_token_stream().to_string(),
            },
        })
    }

    fn fields(
        &self,
        fields: &Fields,
        module: &str,
        file: &Path,
        parameters: &BTreeSet<String>,
    ) -> Result<Vec<Field>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                Ok(Field {
                    name: f.ident.as_ref().map_or_else(|| i.to_string(), ident),
                    ty: self.ty(&f.ty, module, parameters)?,
                    source: source(file, f.ty.span()),
                })
            })
            .collect()
    }

    fn finish(&mut self) -> Result<()> {
        for (name, (item, module, file)) in &self.raw {
            let (generics, span) = match item {
                Item::Struct(i) => (&i.generics, i.ident.span()),
                Item::Enum(i) => (&i.generics, i.ident.span()),
                Item::Type(i) => (&i.generics, i.ident.span()),
                _ => unreachable!(),
            };
            let parameter_names = generics
                .params
                .iter()
                .filter_map(|p| match p {
                    GenericParam::Type(t) => Some(ident(&t.ident)),
                    GenericParam::Const(c) => Some(ident(&c.ident)),
                    _ => None,
                })
                .collect::<BTreeSet<_>>();
            let parameters = generics
                .params
                .iter()
                .filter_map(|p| match p {
                    GenericParam::Type(t) => Some(self.ty_default(t, module, &parameter_names)),
                    GenericParam::Const(c) => Some(self.ty(&c.ty, module, &parameter_names).map(
                        |const_type| Parameter {
                            name: ident(&c.ident),
                            kind: "const".into(),
                            const_type: Some(const_type),
                            default: c.default.as_ref().map(|e| Ty::Const {
                                expression: e.to_token_stream().to_string(),
                            }),
                        },
                    )),
                    _ => None,
                })
                .collect::<Result<Vec<_>>>()?;
            let shape = match item {
                Item::Struct(i) => Shape::Record {
                    fields: self.fields(&i.fields, module, file, &parameter_names)?,
                    tuple: matches!(i.fields, Fields::Unnamed(_)),
                },
                Item::Enum(i) => Shape::Enum {
                    variants: i
                        .variants
                        .iter()
                        .map(|v| {
                            Ok(Variant {
                                name: ident(&v.ident),
                                fields: self.fields(&v.fields, module, file, &parameter_names)?,
                                tuple: matches!(v.fields, Fields::Unnamed(_)),
                                source: source(file, v.ident.span()),
                            })
                        })
                        .collect::<Result<_>>()?,
                },
                Item::Type(i) => Shape::Alias {
                    target: self.ty(&i.ty, module, &parameter_names)?,
                },
                _ => unreachable!(),
            };
            self.definitions.insert(
                name.clone(),
                Definition {
                    name: name.clone(),
                    parameters,
                    shape,
                    source: source(file, span),
                    conditions: attributes(item)
                        .iter()
                        .filter(|a| a.path().is_ident("cfg") || a.path().is_ident("cfg_attr"))
                        .map(|a| a.meta.to_token_stream().to_string())
                        .collect(),
                },
            );
        }
        Ok(())
    }

    fn ty_default(
        &self,
        p: &syn::TypeParam,
        module: &str,
        parameters: &BTreeSet<String>,
    ) -> Result<Parameter> {
        Ok(Parameter {
            name: ident(&p.ident),
            kind: "type".into(),
            const_type: None,
            default: p
                .default
                .as_ref()
                .map(|t| self.ty(t, module, parameters))
                .transpose()?,
        })
    }
}

fn attributes(item: &Item) -> &[syn::Attribute] {
    match item {
        Item::Struct(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        _ => &[],
    }
}

fn namespace(input: &Path) -> (PathBuf, String) {
    let start = if input.is_dir() {
        input
    } else {
        input.parent().unwrap_or(Path::new("."))
    };
    for directory in start.ancestors() {
        if let Ok(text) = fs::read_to_string(directory.join("Cargo.toml")) {
            let mut package = false;
            for line in text.lines().map(str::trim) {
                if line.starts_with('[') {
                    package = line == "[package]";
                }
                if package {
                    if let Some((name, value)) = line.split_once('=') {
                        if name.trim() == "name" {
                            let prefix = value.trim().trim_matches('"').replace('-', "_");
                            let base = directory.join("src");
                            if input.starts_with(&base) {
                                return (base, prefix);
                            }
                        }
                    }
                }
            }
        }
    }
    (start.to_path_buf(), String::new())
}

mod catalog;
mod surface;
pub use catalog::Catalog;
