use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use syn::{Item, Attribute, UseTree, Visibility};
use syn::spanned::Spanned;
use quote::ToTokens;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use std::io::Write;
use clap::{command, Parser};
use anyhow::{bail, Result};




#[derive(Parser)]
#[command(name = "extract-comment")]
struct Cli {
    #[arg(long)]
    dirs: Vec<PathBuf>,

    #[arg(long)]
    out: PathBuf,
}



#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.out.extension().map_or(false, |ext| ext == "json") {
        bail!("Output file must have .json extension");
    }

    let mut units = extract_units( cli.dirs, vec![
        "intrinsics/"
    ]).await?;
    keep_only_unsafe_fn_and_unsafe_trait(&mut units);

    // Output to json
    let json_string = serde_json::to_string_pretty(&units)?;

    let mut file = fs::File::create(&cli.out)?;
    file.write_all(json_string.as_bytes())?;
            
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub line_number_start: usize,
    pub line_number_end: usize,

    /// Function name, struct name, or trait name 
    pub name: String,

    /// Function, struct, or trait with signature (for functions) or declaration (for structs and traits)
    pub declaration_text: String,

    /// For functions, the parent struct or trait they belong to (if any). For standalone functions, this will be None.
    pub parent_struct_or_trait: Option<String>,

    pub is_unsafe: bool,

    pub comment: String
}

impl Unit {
    pub fn new(line_number_start: usize, line_number_end: usize, name: String, declaration_text: String, is_unsafe: bool,  parent_struct_or_trait: Option<String>, comment: String) -> Self {
        Unit {
            line_number_start,
            line_number_end,
            name,
            declaration_text,
            is_unsafe,
            parent_struct_or_trait,
            comment
        }
    }
}


// Create a wrapper struct for better TOML structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Units {
    pub files: HashMap<String, Vec<Unit>>,
}

pub fn keep_only_unsafe_fn_and_unsafe_trait(units: &mut Units) {
    for (_file_path, units_list) in units.files.iter_mut() {
        units_list.retain(|unit| unit.is_unsafe && unit.comment.trim() != "" );
    }
    // remove files that have no unsafe functions or traits
    units.files.retain(|_file_path, units_list| !units_list.is_empty());
}

/// Step 1: Collect all Rust source files from the target directories.
/// Step 2: For each source file, use syn to parse the Units:
/// A Unit can be either a function (can associate with a struct or a trait), a struct, a trait.
/// Step 3: For each Unit, extract the associated comments above them (doc comments).
pub async fn extract_units(dirs: Vec<PathBuf>, blacklist: Vec<&str>) -> Result<Units> {
    let mut files_map: HashMap<String, Vec<Unit>> = HashMap::new();
    // Step 1: Collect all Rust source files from the target directories
    let mut rust_files = Vec::new();
    let mut blacklisted_rust_files = Vec::new();
    for dir in dirs {
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let path_str = path.display().to_string();
                if blacklist.iter().any(|b| path_str.contains(b)) {
                    blacklisted_rust_files.push(path.to_path_buf());
                    continue;
                }
                rust_files.push(path.to_path_buf());
            }
        }
    }

    println!("Found {} Rust source files to process.", rust_files.len());
    let blacklisted_units = index_blacklisted_units(&blacklisted_rust_files)?;
    let mut files_contains_unsafe_fn_or_trait = 0;
    // Step 2 & 3: Parse each file and extract units with comments
    for file_path in rust_files {
        let mut content = fs::read_to_string(&file_path)?;
        content = content.replace("~const", ""); 
        
        // Parse the file using syn
        let syntax = match syn::parse_file(&content) {
            Ok(syntax) => syntax,
            Err(err) => {
                // Skip files that can't be parsed
                eprintln!("Warning: Could not parse file {}: {}", file_path.display(), err);
                let span = err.span();
                let start = span.start();
                let end = span.end();

                eprintln!(
                    "Could not parse file {}:{}:{} to {}:{}: {}",
                    file_path.display(),
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    err
                );
                continue;
            }
        };
        
        // Extract units from the parsed file
        let mut file_has_unsafe = false;
        for item in syntax.items {
            // Preserve public APIs re-exported from a blacklisted implementation module. For
            // example, `ptr::copy` is public here but its signature and docs live under
            // `intrinsics/`; the other compiler-only intrinsics must remain excluded.
            if let Item::Use(use_item) = &item {
                if matches!(use_item.vis, Visibility::Public(_)) {
                    for (target, public_name) in use_targets(&use_item.tree) {
                        let Some((origin_name, origin_module)) = target.split_last() else {
                            continue;
                        };
                        let Some(origin) = blacklisted_units.get(&(
                            origin_module.join("::"),
                            origin_name.to_string(),
                        )) else {
                            continue;
                        };

                        let mut unit = origin.clone();
                        unit.name = public_name;
                        unit.line_number_start = use_item.span().start().line;
                        unit.line_number_end = use_item.span().end().line;
                        unit.parent_struct_or_trait = None;
                        file_has_unsafe = true;
                        files_map.entry(file_path.display().to_string())
                            .or_insert_with(Vec::new)
                            .push(unit);
                    }
                }
            }

            if let Some(unit) = extract_unit_from_item(&item) {
                if unit.is_unsafe && !unit.comment.is_empty() {
                    file_has_unsafe = true;
                }
                files_map.entry(file_path.display().to_string()).or_insert_with(Vec::new).push(unit);
            }

            // Handle trait blocks to extract trait method signatures.
            if let Item::Trait(trait_item) = &item {
                for trait_item_inner in &trait_item.items {
                    if let syn::TraitItem::Fn(method) = trait_item_inner {
                        let comment = extract_doc_comment(&method.attrs);
                        let declaration_text = method.sig.to_token_stream().to_string();
                        let method_name = method.sig.ident.to_string();
                        let start_line = method.span().start().line;
                        let end_line = method.span().end().line;
                        let is_unsafe = method.sig.unsafety.is_some();
                        let parent_struct_or_trait = Some(trait_item.ident.to_string());

                        if is_unsafe && !comment.is_empty() {
                            file_has_unsafe = true;
                        }

                        files_map.entry(file_path.display().to_string())
                            .or_insert_with(Vec::new)
                            .push(Unit::new(
                                start_line,
                                end_line,
                                method_name,
                                declaration_text,
                                is_unsafe,
                                parent_struct_or_trait,
                                comment,
                            ));
                    }
                }
            }
            
            // Handle impl blocks to extract methods
            if let Item::Impl(impl_item) = &item {
                for impl_item_inner in &impl_item.items {
                    if let syn::ImplItem::Fn(method) = impl_item_inner {
                        let comment = extract_doc_comment(&method.attrs);
                        // Extract only the method signature, not the body
                        let declaration_text = method.sig.to_token_stream().to_string();
                        let method_name = method.sig.ident.to_string();
                        // Get line numbers from span
                        let start_line = method.span().start().line;
                        let end_line = method.span().end().line;
                        let is_unsafe = method.sig.unsafety.is_some();
                        let parent_struct_or_trait = impl_item.self_ty.to_token_stream().to_string().into();
                        
                        if is_unsafe && !comment.is_empty() {
                            file_has_unsafe = true;
                        }
                        
                        files_map.entry(file_path.display().to_string())
                        .or_insert_with(Vec::new)
                        .push(
                            Unit::new(start_line, 
                                end_line, 
                                method_name,
                                declaration_text, 
                                is_unsafe,
                                parent_struct_or_trait,
                                comment))
                            ;
                    }
                }
            }
        }
        
        if file_has_unsafe {
            files_contains_unsafe_fn_or_trait += 1;
        }
    }
    
    println!("Files containing unsafe functions/traits with comments: {}", files_contains_unsafe_fn_or_trait);
    
    Ok(Units { files: files_map })
}

/// Index documented unsafe functions in blacklisted modules. They are not emitted directly, but
/// their signature and docs can be attached to a public re-export in a non-blacklisted module.
fn index_blacklisted_units(files: &[PathBuf]) -> Result<HashMap<(String, String), Unit>> {
    let mut units = HashMap::new();

    for file_path in files {
        let Some(module_path) = module_path(file_path) else {
            continue;
        };
        let mut content = fs::read_to_string(file_path)?;
        content = content.replace("~const", "");
        let syntax = match syn::parse_file(&content) {
            Ok(syntax) => syntax,
            Err(err) => {
                eprintln!(
                    "Warning: Could not parse blacklisted file {} while resolving public re-exports: {}",
                    file_path.display(), err
                );
                continue;
            }
        };

        for item in syntax.items {
            let Some(unit) = extract_unit_from_item(&item) else {
                continue;
            };
            if unit.is_unsafe && !unit.comment.trim().is_empty() {
                units.insert((module_path.clone(), unit.name.clone()), unit);
            }
        }
    }

    Ok(units)
}

/// Convert `.../src/foo/mod.rs` and `.../src/foo.rs` into their `crate::foo` module paths.
fn module_path(file_path: &std::path::Path) -> Option<String> {
    let components: Vec<_> = file_path.components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect();
    let src_index = components.iter().rposition(|component| component == "src")?;
    let mut module_components = components.get(src_index + 1..)?.to_vec();
    let file_name = module_components.pop()?;

    if file_name != "mod.rs" {
        module_components.push(file_name.strip_suffix(".rs")?.to_string());
    }

    let mut result = vec!["crate".to_string()];
    result.extend(module_components);
    Some(result.join("::"))
}

/// Flatten a `use` tree into `(origin path, public name)` pairs.
fn use_targets(tree: &UseTree) -> Vec<(Vec<String>, String)> {
    fn visit(tree: &UseTree, prefix: &mut Vec<String>, targets: &mut Vec<(Vec<String>, String)>) {
        match tree {
            UseTree::Path(path) => {
                prefix.push(path.ident.to_string());
                visit(&path.tree, prefix, targets);
                prefix.pop();
            }
            UseTree::Name(name) => {
                let name = name.ident.to_string();
                let mut target = prefix.clone();
                target.push(name.clone());
                targets.push((target, name));
            }
            UseTree::Rename(rename) => {
                let mut target = prefix.clone();
                target.push(rename.ident.to_string());
                targets.push((target, rename.rename.to_string()));
            }
            UseTree::Group(group) => {
                for item in &group.items {
                    visit(item, prefix, targets);
                }
            }
            UseTree::Glob(_) => {}
        }
    }

    let mut targets = Vec::new();
    visit(tree, &mut Vec::new(), &mut targets);
    targets
}

/// Extract a Unit from a syn::Item
fn extract_unit_from_item(item: &Item) -> Option<Unit> {
    match item {
        // Handle bodyless fn declarations (e.g. #[rustc_intrinsic] pub unsafe fn foo() -> T;)
        // syn cannot parse these as Item::Fn (no body), so they become Item::Verbatim.
        Item::Verbatim(tokens) => {
            let token_str = tokens.to_string();
            let trimmed = token_str.trim_end();
            if !trimmed.ends_with(';') {
                return None;
            }
            let without_semi = &trimmed[..trimmed.len() - 1];
            let with_body = format!("{} {{}}", without_semi);
            let fn_item = syn::parse_str::<syn::ItemFn>(&with_body).ok()?;
            let comment = extract_doc_comment(&fn_item.attrs);
            let declaration_text = fn_item.sig.to_token_stream().to_string();
            let name = fn_item.sig.ident.to_string();
            let is_unsafe = fn_item.sig.unsafety.is_some();
            // Recover line numbers from the original token stream
            let mut token_iter = tokens.clone().into_iter();
            let start_line = token_iter.next().map(|t| t.span().start().line).unwrap_or(0);
            let end_line = tokens.clone().into_iter().last().map(|t| t.span().end().line).unwrap_or(0);
            Some(Unit::new(start_line, end_line, name, declaration_text, is_unsafe, None, comment))
        }
        // Extract standalone functions - only signature
        Item::Fn(func) => {
            let comment = extract_doc_comment(&func.attrs);
            // Extract only the function signature, not the body
            let declaration_text = func.sig.to_token_stream().to_string();
            let start_line = func.span().start().line;
            let end_line = func.span().end().line;
            let name = func.sig.ident.to_string();
            let is_unsafe = func.sig.unsafety.is_some();
            Some(Unit::new(start_line, end_line, name, declaration_text, is_unsafe, None, comment))
        }
        // Extract structs - declaration with fields but without field implementations
        Item::Struct(struct_item) => {
            let comment = extract_doc_comment(&struct_item.attrs);
            // Build struct declaration: struct name + generics + where clause + fields
            let mut declaration_parts = vec![
                "struct".to_string(),
                struct_item.ident.to_string(),
            ];
            
            if !struct_item.generics.params.is_empty() {
                declaration_parts.push(struct_item.generics.params.to_token_stream().to_string());
            }
            
            if struct_item.generics.where_clause.is_some() {
                declaration_parts.push(struct_item.generics.where_clause.to_token_stream().to_string());
            }
            
            declaration_parts.push(struct_item.fields.to_token_stream().to_string());
            
            let declaration_text = declaration_parts.join(" ");
            let start_line = struct_item.span().start().line;
            let end_line = struct_item.span().end().line;
            
            Some(Unit::new(start_line, end_line, struct_item.ident.to_string(), declaration_text, false, None, comment))
        }
        // Extract traits - declaration with method signatures only
        Item::Trait(trait_item) => {
            let comment = extract_doc_comment(&trait_item.attrs);
            // Build trait declaration: trait name + generics + bounds
            let mut declaration_parts = vec![
                "trait".to_string(),
                trait_item.ident.to_string(),
            ];
            
            if !trait_item.generics.params.is_empty() {
                declaration_parts.push(format!("<{}>", trait_item.generics.params.to_token_stream().to_string()));
            }
            
            if !trait_item.supertraits.is_empty() {
                declaration_parts.push(format!(": {}", trait_item.supertraits.to_token_stream().to_string()));
            }
            
            if trait_item.generics.where_clause.is_some() {
                declaration_parts.push(trait_item.generics.where_clause.to_token_stream().to_string());
            }
            
            // // Add trait method signatures
            // let mut method_sigs = Vec::new();
            // for item in &trait_item.items {
            //     if let syn::TraitItem::Fn(method) = item {
            //         method_sigs.push(method.sig.to_token_stream().to_string());
            //     }
            // }
            
            let declaration_text = declaration_parts.join(" ");
            
            let start_line = trait_item.span().start().line;
            let end_line = trait_item.span().end().line;
            let is_unsafe = trait_item.unsafety.is_some();
            
            Some(Unit::new(start_line, end_line, trait_item.ident.to_string(), declaration_text, is_unsafe, None, comment))
            
        }
        _ => None,
    }
}

/// Extract doc comments from attributes
fn extract_doc_comment(attrs: &[Attribute]) -> String {
    let mut comments = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        comments.push(lit_str.value());
                    }
                }
            }
        }
    }
    
    comments.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{module_path, use_targets};
    use std::path::Path;

    #[test]
    fn module_path_from_rust_source_path() {
        assert_eq!(
            module_path(Path::new("tools/rust/library/core/src/intrinsics/mod.rs")),
            Some("crate::intrinsics".to_string())
        );
    }

    #[test]
    fn grouped_and_renamed_use_targets() {
        let item: syn::ItemUse = syn::parse_str(
            "pub use crate::intrinsics::{copy, copy_nonoverlapping as copy_no_overlap};"
        ).unwrap();
        assert_eq!(
            use_targets(&item.tree),
            vec![
                (vec!["crate".into(), "intrinsics".into(), "copy".into()], "copy".into()),
                (
                    vec!["crate".into(), "intrinsics".into(), "copy_nonoverlapping".into()],
                    "copy_no_overlap".into()
                ),
            ]
        );
    }
}
