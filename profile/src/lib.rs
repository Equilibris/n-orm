#![feature(iter_intersperse)]
use std::collections::HashMap;

use proc_macro::{Span, TokenStream as Ts1};
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use proc_macro2::{Ident, Punct, Spacing};
use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Token};
use syn::{Attribute, Visibility};

fn get_inner_tokens(tokens: TokenStream) -> Option<TokenStream> {
    let mut iter = tokens.into_iter();

    if let TokenTree::Group(inner) = iter.next()? {
        Some(inner.stream())
    } else {
        None
    }
}

fn handle_attr(
    attr: Attribute,
    token_entires: &mut HashMap<String, TokenStream>,
    iso_default: &mut bool,
    default_profile: &Option<String>,
    src_name: &String,
) {
    let id = attr
        .path
        .segments
        .clone()
        .into_iter()
        .map(|s| s.ident.to_string())
        .collect::<String>();

    if id.as_str() == "iso_default" {
        *iso_default ^= true;
    } else if id.as_str() == "on" {
        let loc = attr.span();
        let stream = get_inner_tokens(attr.tokens)
            .ok_or_else(|| abort!(loc, "Expected grouping but this was not provided"))
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        match stream.get(0) {
            Some(TokenTree::Ident(i))
                if stream.len() >= 3
                    && matches!(stream.get(1).unwrap(), TokenTree::Punct(p) if p.as_char() == ',') =>
            {
                let profile = i.to_string();

                if let Some(profile) = token_entires.get_mut(&profile) {
                    profile.extend(stream.into_iter().skip(2));
                } else {
                    abort!(loc, "Profile {} not defined", profile);
                }
            }
            _ => {
                if let Some(profile) = default_profile.as_ref() {
                    token_entires.get_mut(profile).unwrap().extend(stream);
                } else {
                    // TODO: help
                    abort!(loc, "Ambiguous profile. Please specify");
                }
            }
        };
    } else if id.as_str() == "iso" {
        let loc = attr.span();
        let stream = get_inner_tokens(attr.tokens)
            .ok_or_else(|| abort!(loc, "Expected grouping but this was not provided"))
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        for v in token_entires.values_mut() {
            v.extend(stream.clone());
        }
    } else if *iso_default {
        let stream = attr.to_token_stream().into_iter().collect::<Vec<_>>();

        for v in token_entires.values_mut() {
            v.extend(stream.clone());
        }
    } else {
        token_entires
            .get_mut(src_name)
            .unwrap()
            .extend(attr.to_token_stream());
    }
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
        syn::Item::Struct(mut item) => {
            item.generics.make_where_clause();
            let (impl_generics, post_name_generics, where_clause) = item.generics.split_for_impl();
            let where_clause = where_clause.unwrap();

            let src_ident = item.ident.clone();
            let src_name = item.ident.to_string();

            let default_profile = if profiles.len() == 1 {
                Some(profiles.iter().next().unwrap().clone().to_string())
            } else {
                None
            };

            let mut visibility_entires = HashMap::new();
            let mut token_entires = HashMap::new();
            for profile in profiles {
                token_entires.insert(profile.to_string(), TokenStream::new());
                visibility_entires.insert(profile.to_string(), Visibility::Inherited);
            }
            token_entires.insert(src_name.clone(), TokenStream::new());

            for attr in item.attrs {
                handle_attr(
                    attr,
                    &mut token_entires,
                    &mut iso_default,
                    &default_profile,
                    &src_name,
                )
            }

            for (k, v) in token_entires.iter_mut() {
                v.extend(item.vis.clone().to_token_stream());
                v.extend(item.struct_token.to_token_stream());
                v.extend(Ident::new(k.as_str(), Span::call_site().into()).to_token_stream());
                v.extend(post_name_generics.clone().to_token_stream());
            }

            let mut output = TokenStream::new();
            match item.fields {
                syn::Fields::Named(fields) => todo!(),
                syn::Fields::Unnamed(fields) => {
                    let mut grouping_interior = HashMap::with_capacity(token_entires.len());
                    let mut field_name_destructure = TokenStream::new();
                    let mut field_type_destructure = TokenStream::new();
                    for profile in token_entires.keys() {
                        grouping_interior.insert(profile.clone(), TokenStream::new());
                    }

                    for (index, field) in fields.unnamed.iter().enumerate() {
                        for attr in &field.attrs {
                            handle_attr(
                                attr.clone(),
                                &mut grouping_interior,
                                &mut iso_default,
                                &default_profile,
                                &src_name,
                            )
                        }

                        let ident =
                            Ident::new(format!("e{}", index).as_str(), Span::call_site().into());

                        let ty = &field.ty;
                        field_name_destructure.extend(quote! {#ident,});
                        field_type_destructure.extend(quote! {#ty,});

                        for profile in grouping_interior.values_mut() {
                            profile.extend(field.vis.to_token_stream());
                            profile.extend(field.ty.to_token_stream());
                            profile.extend(Punct::new(',', Spacing::Alone).to_token_stream());
                        }
                    }

                    let field_names = Group::new(Delimiter::Parenthesis, field_name_destructure);
                    let field_types = Group::new(Delimiter::Parenthesis, field_type_destructure);

                    for (profile_name, profile) in grouping_interior {
                        let ungrouped_profile = token_entires.get_mut(&profile_name).unwrap();

                        ungrouped_profile
                            .extend(Group::new(Delimiter::Parenthesis, profile).to_token_stream());
                        ungrouped_profile.extend(where_clause.to_token_stream());
                        ungrouped_profile.extend(
                            item.semi_token
                                .expect("Tuple struct requires semi")
                                .to_token_stream(),
                        );

                        let profile_name_ident =
                            Ident::new(profile_name.as_str(), Span::call_site().into());

                        output.extend(ungrouped_profile.clone());

                        if profile_name != src_name {
                            output.extend(quote! {
                                impl #impl_generics Into<#profile_name_ident #post_name_generics> for #src_ident #post_name_generics #where_clause {
                                    fn into(self) -> #profile_name_ident #post_name_generics {
                                        let Self #field_names = self;

                                        #profile_name_ident #field_names
                                    }
                                }
                                impl #impl_generics Into<#src_ident #post_name_generics> for #profile_name_ident #post_name_generics #where_clause {
                                    fn into(self) -> #src_ident #post_name_generics {
                                        let Self #field_names = self;

                                        #src_ident #field_names
                                    }
                                }
                            });
                        }
                    }
                }
                syn::Fields::Unit => {
                    for profile in token_entires.values_mut() {
                        // Unit profiles cannot be generic
                        // profile.extend(where_clause.to_token_stream());
                        profile.extend(
                            item.semi_token
                                .expect("Semi is required for unit struct")
                                .to_token_stream(),
                        );
                    }

                    for (profile_name, profile) in token_entires {
                        output.extend(profile);

                        if profile_name != src_name {
                            let profile_name =
                                Ident::new(profile_name.as_str(), Span::call_site().into());
                            output.extend(quote! {
                                impl Into<#profile_name> for #src_ident {
                                    fn into(self) -> #profile_name {
                                        #profile_name
                                    }
                                }
                                impl Into<#src_ident> for #profile_name {
                                    fn into(self) -> #src_ident {
                                        #src_ident
                                    }
                                }
                            });
                        }
                    }
                }
            }
            output.into()
        }
        _ => todo!("Profiles are only implemented on enums and structs"),
    }
}
