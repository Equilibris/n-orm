use std::collections::HashMap;

use proc_macro::{Span, TokenStream as Ts1};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::quote;
use quote::ToTokens;
use syn::{parse_macro_input, Token};

macro_rules! path_eq {
    ($id:expr,$e:expr) => {
        $id.pop().unwrap().value().ident.to_string() == $e.to_string()
    };
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn profile(attr: Ts1, item: Ts1) -> Ts1 {
    let (e, profiles) = attr
        .into_iter()
        .fold((None, Vec::new()), |mut acc, x| match x {
            proc_macro::TokenTree::Ident(id) => {
                acc.1.push(id);
                acc
            }
            _ => (
                Some(acc.0.map_or_else(
                    || {
                        Diagnostic::spanned(
                            x.span().into(),
                            Level::Error,
                            "Unexpected token, profile name must be ident".into(),
                        )
                    },
                    |z: Diagnostic| {
                        z.span_error(
                            x.span().into(),
                            "Unexpected token, profile name must be ident".into(),
                        )
                    },
                )),
                acc.1,
            ),
        });

    if let Some(e) = e {
        abort!(e)
    }

    let item = parse_macro_input!(item as syn::Item);

    let mut iso_default = false;

    match item {
        syn::Item::Struct(item) => {
            let ty_name = item.ident.to_string();

            let mut m = HashMap::new();
            for profile in profiles {
                m.insert(profile.to_string(), TokenStream::default());
            }

            for mut attr in item.attrs {
                if attr.path.segments.len() == 1 && path_eq!(attr.path.segments, "iso_default") {
                    iso_default = true;
                }
                if path_eq!(attr.path.segments, "iso") || iso_default {
                    for v in m.values_mut() {
                        match attr.tokens.clone().into_iter().next() {
                            Some(TokenTree::Group(s))
                                if s.delimiter() == Delimiter::Parenthesis =>
                            {
                                let ts = s.into_token_stream();
                                v.extend(quote! {#[#ts]})
                            }
                            _ => todo!(),
                        }
                    }
                }
            }
            todo!()
        }
        _ => todo!("Profiles are only implemented on enums and structs"),
    }
}
