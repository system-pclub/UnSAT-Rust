use super::*;

#[derive(Serialize)]
pub struct Selector {
    pub syntax: String,
    pub result: Ty,
    pub guard: Option<String>,
    pub source: Source,
}

#[derive(Serialize)]
pub struct Catalog {
    pub version: u32,
    pub definitions: BTreeMap<String, Definition>,
    pub selectors: BTreeMap<String, Vec<Selector>>,
    pub aliases: BTreeMap<String, String>,
    pub alternatives: BTreeMap<String, Vec<Definition>>,
    /// Declarations whose cfg alternatives were resolved for the analysis
    /// target (exactly one alternative true, all others false).
    pub cfg_selected: BTreeMap<String, Vec<String>>,
    pub files: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
    pub unexpanded_macros: Vec<Source>,
}

impl Catalog {
    pub fn generate(registry: &Registry) -> Result<Self> {
        let mut definitions = registry.definitions.clone();
        let mut alternatives = BTreeMap::new();
        let mut cfg_selected = BTreeMap::new();
        for (name, items) in &registry.duplicates {
            let mut options = vec![registry.definitions[name].clone()];
            for (item, module, file) in items {
                let mut local = Registry {
                    raw: registry.raw.clone(),
                    imports: registry.imports.clone(),
                    globs: registry.globs.clone(),
                    ..Registry::default()
                };
                local
                    .raw
                    .insert(name.clone(), (item.clone(), module.clone(), file.clone()));
                local.finish()?;
                options.push(local.definitions.remove(name).unwrap());
            }
            definitions.remove(name);
            let verdicts: Vec<Option<bool>> =
                options.iter().map(|o| cfg::conditions_hold(&o.conditions)).collect();
            let chosen: Vec<usize> = (0..options.len()).filter(|&i| verdicts[i] == Some(true)).collect();
            let rest_false = verdicts.iter().filter(|v| **v == Some(false)).count() == options.len() - 1;
            if chosen.len() == 1 && rest_false {
                let definition = options[chosen[0]].clone();
                cfg_selected.insert(name.clone(), definition.conditions.clone());
                definitions.insert(name.clone(), definition);
            } else {
                alternatives.insert(name.clone(), options);
            }
        }
        let mut selectors = BTreeMap::new();
        for definition in definitions.values().cloned().collect::<Vec<_>>() {
            let mut members = vec![];
            match &definition.shape {
                Shape::Record { fields, tuple } => {
                    for f in fields {
                        members.push(Selector {
                            syntax: if *tuple {
                                format!("[{}]", f.name)
                            } else {
                                format!(".{}", f.name)
                            },
                            result: f.ty.clone(),
                            guard: None,
                            source: f.source.clone(),
                        });
                    }
                }
                Shape::Enum { variants } => {
                    for variant in variants {
                        // A unit variant is a discriminator value, not a payload type.
                        if variant.fields.is_empty() {
                            continue;
                        }
                        let name = format!("{}::{}", definition.name, variant.name);
                        definitions.insert(
                            name.clone(),
                            Definition {
                                name: name.clone(),
                                parameters: definition.parameters.clone(),
                                shape: Shape::Record {
                                    fields: variant.fields.clone(),
                                    tuple: variant.tuple,
                                },
                                source: variant.source.clone(),
                                conditions: definition.conditions.clone(),
                            },
                        );
                        selectors.insert(
                            name.clone(),
                            variant
                                .fields
                                .iter()
                                .map(|f| Selector {
                                    syntax: if variant.tuple {
                                        format!("[{}]", f.name)
                                    } else {
                                        format!(".{}", f.name)
                                    },
                                    result: f.ty.clone(),
                                    guard: None,
                                    source: f.source.clone(),
                                })
                                .collect(),
                        );
                        members.push(Selector {
                            syntax: format!(".{}", variant.name),
                            result: Ty::Named {
                                name,
                                args: definition
                                    .parameters
                                    .iter()
                                    .map(|p| Ty::Parameter {
                                        name: p.name.clone(),
                                    })
                                    .collect(),
                            },
                            guard: Some(variant.name.clone()),
                            source: variant.source.clone(),
                        });
                    }
                }
                Shape::Alias { target } => members.push(Selector {
                    syntax: String::new(),
                    result: target.clone(),
                    guard: None,
                    source: definition.source.clone(),
                }),
            }
            selectors.insert(definition.name, members);
        }
        let mut aliases = BTreeMap::new();
        for (module, imports) in &registry.imports {
            for name in imports.keys() {
                if let Ok(target) = registry.resolve(module, name, &mut BTreeSet::new()) {
                    if definitions.contains_key(&target)
                        && qualified(module, name) != target
                        && !definitions.contains_key(&qualified(module, name))
                    {
                        aliases.insert(qualified(module, name), target);
                    }
                }
            }
        }
        for (module, globs) in &registry.globs {
            for glob in globs {
                for name in registry.definitions.keys() {
                    let short = name.rsplit("::").next().unwrap();
                    if let Ok(target) =
                        registry.resolve(module, &qualified(glob, short), &mut BTreeSet::new())
                    {
                        if definitions.contains_key(&target)
                            && !definitions.contains_key(&qualified(module, short))
                        {
                            aliases.entry(qualified(module, short)).or_insert(target);
                        }
                    }
                }
            }
        }
        Ok(Self {
            version: 2,
            definitions,
            selectors,
            aliases,
            alternatives,
            cfg_selected,
            files: registry.files.clone(),
            diagnostics: registry.diagnostics.clone(),
            unexpanded_macros: registry.macros.clone(),
        })
    }

    pub fn markdown(&self) -> String {
        let mut text = String::from("# Source-derived Rust member catalog\n\nOne catalog for every API and rule. Generic definitions are preserved.\n\n| Receiver | Selector | Result | Source |\n| --- | --- | --- | --- |\n");
        for (name, members) in &self.selectors {
            for m in members {
                text.push_str(&format!(
                    "| `{}` | `{}` | `{}` | {}:{} |\n",
                    name,
                    if m.syntax.is_empty() {
                        "(alias)"
                    } else {
                        &m.syntax
                    },
                    m.result.display().replace('|', "&#124;"),
                    m.source.file,
                    m.source.line
                ));
            }
        }
        text.push_str(&format!("\n{} files; {} declarations/payload sorts; {} ambiguous declarations; {} declaration diagnostics; {} unexpanded item macros.\n", self.files.len(), self.definitions.len(), self.alternatives.len(), self.diagnostics.len(), self.unexpanded_macros.len()));
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn registry(text: &str) -> Registry {
        let (items, skipped) = surface::parse(text).unwrap();
        assert!(skipped.is_empty(), "{skipped:?}");
        let mut r = Registry::default();
        r.collect(&items, "sample", Path::new("sample.rs")).unwrap();
        r.finish().unwrap();
        r
    }
    #[test]
    fn catalog_contains_unreferenced_definitions() {
        let c = Catalog::generate(&registry(
            "struct A { b: B } struct B { n: usize } struct Unused { flag: bool }",
        ))
        .unwrap();
        assert!(c.definitions.contains_key("sample::Unused"));
        assert_eq!(c.selectors["sample::A"][0].syntax, ".b");
        assert!(!serde_json::to_string(&c).unwrap().contains("\"roots\""));
    }
    #[test]
    fn generics_and_enum_payload_order_are_preserved() {
        let c = Catalog::generate(&registry("struct Tree<T> { next: Option<Box<Tree<T>>> } enum Kind<T> { Array(T, usize), Named { child: T } }")).unwrap();
        assert_eq!(c.definitions["sample::Tree"].parameters[0].name, "T");
        assert_eq!(c.selectors["sample::Kind"][0].syntax, ".Array");
        assert_eq!(c.selectors["sample::Kind::Array"][0].syntax, "[0]");
        assert_eq!(c.selectors["sample::Kind::Array"][1].syntax, "[1]");
        assert_eq!(c.selectors["sample::Kind::Named"][0].syntax, ".child");
    }
    #[test]
    fn raw_pointer_fields_preserve_target_and_mutability() {
        let c = Catalog::generate(&registry(
            "struct Links<T> { src: *const T, dst: *mut [T] }",
        ))
        .unwrap();
        let fields = &c.selectors["sample::Links"];
        assert!(
            matches!(&fields[0].result, Ty::Pointer { element, mutable: false } if matches!(&**element, Ty::Parameter { name } if name == "T"))
        );
        assert!(
            matches!(&fields[1].result, Ty::Pointer { element, mutable: true } if matches!(&**element, Ty::Sequence { length: None, .. }))
        );
    }

    #[test]
    fn cfg_alternatives_are_not_silently_selected() {
        let c = Catalog::generate(&registry(
            "#[cfg(a)] struct A { x: u8 } #[cfg(b)] struct A { y: bool }",
        ))
        .unwrap();
        assert!(!c.definitions.contains_key("sample::A"));
        assert_eq!(c.alternatives["sample::A"].len(), 2);
    }
    #[test]
    fn cfg_alternatives_are_selected_for_the_analysis_target() {
        let c = Catalog::generate(&registry(
            "#[cfg(target_pointer_width = \"32\")] struct A { x: u32 } #[cfg(target_pointer_width = \"64\")] struct A { y: u64 }",
        ))
        .unwrap();
        assert!(c.definitions.contains_key("sample::A"));
        assert!(!c.alternatives.contains_key("sample::A"));
        assert!(c.cfg_selected.contains_key("sample::A"));
    }
    #[test]
    fn function_bodies_and_test_modules_do_not_affect_data_parsing() {
        let c = Catalog::generate(&registry("struct Real { n: usize } fn f() { strange nightly tokens } impl Real { invalid body tokens } #[cfg(test)] mod tests { struct Fake; }")).unwrap();
        assert!(c.definitions.contains_key("sample::Real"));
        assert!(!c.definitions.keys().any(|n| n.contains("Fake")));
    }
    #[test]
    fn actual_library_layout_has_source_derived_members() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = Catalog::generate(
            &Registry::load(&[repo.join("tools/rust/library/core/src/alloc/layout.rs")]).unwrap(),
        )
        .unwrap();
        assert!(c.selectors["core::alloc::layout::Layout"]
            .iter()
            .any(|e| e.syntax == ".size" && e.result.display() == "usize"));
        assert!(c.selectors["core::alloc::layout::Layout"]
            .iter()
            .any(|e| e.syntax == ".align" && e.result.display().contains("Alignment")));
    }

    #[test]
    fn unit_variants_have_no_payload_types() {
        let c = Catalog::generate(&registry(
            "struct Buffer { len: usize } enum Tag { Unit, Data(usize) }",
        ))
        .unwrap();
        assert!(!c.definitions.contains_key("sample::Tag::Unit"));
        assert!(c.selectors["sample::Tag"]
            .iter()
            .all(|s| s.syntax != ".Unit"));
    }
}
