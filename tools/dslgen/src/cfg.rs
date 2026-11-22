//! Evaluate `#[cfg(...)]` predicates for the analysed target.
//!
//! RuKLEE analyses crates built for x86_64-unknown-linux-gnu. Only predicates
//! whose value is fixed by that target are decided; anything else (features,
//! debug_assertions, unknown keys) is `None`, so a declaration whose selection
//! depends on it stays ambiguous instead of being guessed.

use syn::punctuated::Punctuated;
use syn::{Expr, Lit, Meta, Token};

fn key_value(key: &str, value: &str) -> Option<bool> {
    let known: &[&str] = match key {
        "target_pointer_width" => &["64"],
        "target_arch" => &["x86_64"],
        "target_os" => &["linux"],
        "target_family" => &["unix"],
        "target_env" => &["gnu"],
        "target_endian" => &["little"],
        "target_vendor" => &["unknown"],
        "target_has_atomic" => &["8", "16", "32", "64", "ptr"],
        "panic" => &["unwind"],
        _ => return None,
    };
    Some(known.contains(&value))
}

fn word(name: &str) -> Option<bool> {
    match name {
        "unix" => Some(true),
        "windows" | "test" | "doc" | "doctest" | "miri" => Some(false),
        _ => None,
    }
}

/// Three-valued evaluation of a cfg predicate.
pub fn eval(meta: &Meta) -> Option<bool> {
    match meta {
        Meta::Path(path) => word(&path.get_ident()?.to_string()),
        Meta::NameValue(nv) => {
            let Expr::Lit(lit) = &nv.value else { return None };
            let Lit::Str(value) = &lit.lit else { return None };
            key_value(&nv.path.get_ident()?.to_string(), &value.value())
        }
        Meta::List(list) => {
            let name = list.path.get_ident()?.to_string();
            let args = list
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .ok()?;
            let values: Vec<Option<bool>> = args.iter().map(eval).collect();
            match name.as_str() {
                "all" => {
                    if values.contains(&Some(false)) { Some(false) }
                    else if values.iter().all(|v| *v == Some(true)) { Some(true) }
                    else { None }
                }
                "any" => {
                    if values.contains(&Some(true)) { Some(true) }
                    else if values.iter().all(|v| *v == Some(false)) { Some(false) }
                    else { None }
                }
                "not" if values.len() == 1 => values[0].map(|v| !v),
                _ => None,
            }
        }
    }
}

/// Whether every `cfg(...)` condition of a declaration holds for the target.
/// `conditions` are the attribute metas as recorded by the registry
/// (`cfg (...)` or `cfg_attr (...)`); `cfg_attr` does not gate the item.
pub fn conditions_hold(conditions: &[String]) -> Option<bool> {
    let mut result = Some(true);
    for text in conditions {
        let Ok(Meta::List(list)) = syn::parse_str::<Meta>(text) else { return None };
        if !list.path.is_ident("cfg") {
            continue;
        }
        let Ok(inner) = list.parse_args::<Meta>() else { return None };
        match eval(&inner) {
            Some(false) => return Some(false),
            Some(true) => {}
            None => result = None,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::conditions_hold;

    fn holds(text: &str) -> Option<bool> {
        conditions_hold(&[text.to_string()])
    }

    #[test]
    fn target_predicates() {
        assert_eq!(holds("cfg (target_pointer_width = \"64\")"), Some(true));
        assert_eq!(holds("cfg (target_pointer_width = \"16\")"), Some(false));
        assert_eq!(holds("cfg (all (unix , target_arch = \"x86_64\"))"), Some(true));
        assert_eq!(holds("cfg (any (windows , target_os = \"macos\"))"), Some(false));
        assert_eq!(holds("cfg (not (target_os = \"linux\"))"), Some(false));
        assert_eq!(holds("cfg (feature = \"x\")"), None);
        assert_eq!(holds("cfg (any (feature = \"x\" , unix))"), Some(true));
        assert_eq!(holds("cfg_attr (test , derive (Debug))"), Some(true));
        assert_eq!(conditions_hold(&[]), Some(true));
    }
}
