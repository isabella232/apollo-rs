use anyhow::Result;
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};

use crate::{ast_src::KindsSrc, reformat};

pub(crate) fn generate_kinds(kinds: KindsSrc<'_>) -> Result<String> {
    let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = kinds
        .punct
        .iter()
        .filter(|(token, _name)| token.len() == 1)
        .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
        .unzip();

    let punctuation_values = kinds.punct.iter().map(|(token, _name)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation = kinds
        .punct
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let full_keywords_values = &kinds.keywords;
    let full_keywords = full_keywords_values
        .iter()
        .map(|kw| format_ident!("{}_KW", kw));

    let all_keywords_values = kinds.keywords.iter().collect::<Vec<_>>();
    let all_keywords_idents = all_keywords_values.iter().map(|kw| format_ident!("{}", kw));
    let all_keywords = all_keywords_values
        .iter()
        .map(|name| format_ident!("{}_KW", name))
        .collect::<Vec<_>>();

    let literals = kinds
        .literals
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let tokens = kinds
        .tokens
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let nodes = kinds
        .nodes
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let ast = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub, clippy::manual_non_exhaustive)]
        /// A token generated by the `Parser`.
        #[non_exhaustive]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum SyntaxKind {
            // Technical SyntaxKinds: they appear temporally during parsing,
            // but never end up in the final tree
            #[doc(hidden)]
            TOMBSTONE,
            #[doc(hidden)]
            EOF,
            #(#punctuation,)*
            #(#all_keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::SyntaxKind::*;

        impl SyntaxKind {
            pub fn is_keyword(self) -> bool {
                matches!(self, #(#all_keywords)|*)
            }

            pub fn is_punct(self) -> bool {
                matches!(self, #(#punctuation)|*)
            }

            pub fn is_literal(self) -> bool {
                matches!(self, #(#literals)|*)
            }

            pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#full_keywords_values => #full_keywords,)*
                    _ => return None,
                };
                Some(kw)
            }

            pub fn from_char(c: char) -> Option<SyntaxKind> {
                let tok = match c {
                    #(#single_byte_tokens_values => #single_byte_tokens,)*
                    _ => return None,
                };
                Some(tok)
            }
        }

        /// Create a new `SyntaxKind`
        #[macro_export]
        macro_rules! S {
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
            #([#all_keywords_idents] => { $crate::SyntaxKind::#all_keywords };)*
            [ident] => { $crate::SyntaxKind::IDENT };
            [float_value] => { $crate::SyntaxKind::FLOAT_VALUE };
            [string_value] => { $crate::SyntaxKind::STRING_VALUE };
            [int_value] => { $crate::SyntaxKind::INT_VALUE };
        }
    };

    reformat(&ast.to_string())
}
