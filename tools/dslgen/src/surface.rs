//! Read data declarations without parsing nightly function/impl bodies.
use proc_macro2::{Delimiter, TokenTree};
use quote::ToTokens;
use syn::parse::{discouraged::Speculative, ParseStream, Parser};
use syn::{Attribute, Item, Token, Visibility};

type Skipped = Vec<(usize, String)>;

pub fn parse(text: &str) -> syn::Result<(Vec<Item>, Skipped)> {
    (|input: ParseStream<'_>| scan(input)).parse_str(text)
}

fn skip(input: ParseStream<'_>) -> syn::Result<()> {
    let mut angles = 0usize;
    while !input.is_empty() {
        let token: TokenTree = input.parse()?;
        match token {
            TokenTree::Punct(p) if p.as_char() == '<' => angles += 1,
            TokenTree::Punct(p) if p.as_char() == '>' => angles = angles.saturating_sub(1),
            TokenTree::Punct(p) if p.as_char() == ';' && angles == 0 => break,
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace && angles == 0 => break,
            _ => (),
        }
    }
    Ok(())
}

fn scan(input: ParseStream<'_>) -> syn::Result<(Vec<Item>, Skipped)> {
    let _inner = input.call(Attribute::parse_inner)?;
    let mut items = vec![];
    let mut skipped = vec![];
    while !input.is_empty() {
        let ahead = input.fork();
        let attrs = ahead.call(Attribute::parse_outer)?;
        let vis: Visibility = ahead.parse()?;
        if ahead.peek(Token![mod]) {
            ahead.parse::<Token![mod]>()?;
            let name: syn::Ident = ahead.parse()?;
            let test_only = name == "tests"
                || attrs
                    .iter()
                    .any(|a| a.meta.to_token_stream().to_string().replace(' ', "") == "cfg(test)");
            if ahead.peek(Token![;]) {
                ahead.parse::<Token![;]>()?;
                input.advance_to(&ahead);
                continue;
            }
            if ahead.peek(syn::token::Brace) {
                let content;
                let braces = syn::braced!(content in ahead);
                if test_only {
                    let _: proc_macro2::TokenStream = content.parse()?;
                } else {
                    let (children, diagnostics) = scan(&content)?;
                    skipped.extend(diagnostics);
                    let mut module: syn::ItemMod = syn::parse_quote!(mod #name {});
                    module.attrs = attrs;
                    module.vis = vis;
                    module.content = Some((braces, children));
                    items.push(Item::Mod(module));
                }
                input.advance_to(&ahead);
                continue;
            }
        }
        let declaration = ahead.peek(Token![struct])
            || ahead.peek(Token![enum])
            || ahead.peek(Token![type])
            || ahead.peek(Token![use])
            || ahead.peek(Token![union]);
        let macro_item = {
            let fork = ahead.fork();
            fork.parse::<syn::Path>().is_ok() && fork.peek(Token![!])
        };
        if declaration || macro_item {
            let attempt = input.fork();
            match attempt.parse::<Item>() {
                Ok(item) => {
                    input.advance_to(&attempt);
                    items.push(item);
                    continue;
                }
                Err(e) => {
                    skipped.push((e.span().start().line, format!("unparsed declaration: {e}")))
                }
            }
        }
        skip(input)?;
    }
    Ok((items, skipped))
}
