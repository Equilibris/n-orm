#![feature(iter_intersperse)]
use std::collections::HashMap;

use proc_macro::{Span, TokenStream as Ts1};
use proc_macro2::Ident;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::{Attribute, Visibility};

type Morphisms = HashMap<(String, String), HashMap<String, TokenStream>>;

#[derive(Default, Clone)]
struct FieldMetadata {
    pub inner_stream: TokenStream,
    pub destructor_stream: TokenStream,
}

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
    fields_data: &mut HashMap<String, FieldMetadata>,
    iso_default: &mut bool,
    default_profile: &Option<String>,
    src_name: &String,
    morphisms: &mut Morphisms,

    on_field: Option<&Ident>,
) -> Option<String> {
    let id = attr
        .path
        .segments
        .clone()
        .into_iter()
        .map(|s| s.ident.to_string())
        .collect::<String>();

    let loc = attr.span();

    use TokenTree::*;

    if id.as_str() == "iso_toggle" {
        *iso_default ^= true;
    } else if id.as_str() == "clear_morphisms" {
        let _ = std::mem::take(morphisms);
    } else if id.as_str() == "into" {
        let mut stream = get_inner_tokens(attr.tokens)
            .ok_or_else(|| abort!(loc, "Expected grouping but this was not provided"))
            .unwrap()
            .into_iter();

        match (stream.next(), stream.next(), stream.next()) {
            (Some(Ident(to)), Some(Ident(from)), None) => {
                let k = (to.to_string(), from.to_string());
                morphisms.insert(k, HashMap::default());
            }
            _ => abort!(loc, "Expects two indents #[into(From To)]"),
        }
    } else if let (Some(field), "transform") = (on_field, id.as_str()) {
        let mut stream = get_inner_tokens(attr.tokens)
            .ok_or_else(|| abort!(loc, "Expected grouping but this was not provided"))
            .unwrap()
            .into_iter();

        match (stream.next(), stream.next(), stream.next()) {
            (Some(Ident(to)), Some(Ident(from)), Some(Punct(p))) if p.as_char() == ',' => {
                morphisms
                    .get_mut(&(to.to_string(), from.to_string()))
                    .unwrap_or_else(|| {
                        abort!(
                            loc,
                            "Please declare morphism with #[into({} {})]",
                            to.to_string(),
                            from.to_string()
                        )
                    })
                    .insert(field.to_string(), stream.collect::<TokenStream>());
            }
            _ => abort!(loc, "Expected the form #[transform(To From, ...)]"),
        }
    } else if id.as_str() == "on" {
        let stream = get_inner_tokens(attr.tokens)
            .ok_or_else(|| abort!(loc, "Expected grouping but this was not provided"))
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        match (stream.get(0), stream.get(1)) {
            (Some(Ident(i)), Some(Punct(p))) if stream.len() >= 3 && p.as_char() == ',' => {
                let profile = i.to_string();

                if let Some(profile) = fields_data.get_mut(&profile) {
                    profile.inner_stream.extend(stream.into_iter().skip(2));
                } else {
                    abort!(loc, "Profile {} not defined", profile);
                }
            }
            (Some(Ident(i)), None) if on_field.is_some() => return Some(i.to_string()),
            (None, None) if on_field.is_some() => {
                if let Some(profile) = default_profile {
                    return Some(profile.clone());
                } else {
                    // TODO: help
                    abort!(loc, "Ambiguous profile. Please specify");
                }
            }
            _ => {
                if let Some(profile) = default_profile.as_ref() {
                    fields_data
                        .get_mut(profile)
                        .unwrap()
                        .inner_stream
                        .extend(stream);
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

        for v in fields_data.values_mut() {
            v.inner_stream.extend(stream.clone());
        }
    } else if *iso_default {
        let stream = attr.to_token_stream().into_iter().collect::<Vec<_>>();

        for v in fields_data.values_mut() {
            v.inner_stream.extend(stream.clone());
        }
    } else {
        fields_data
            .get_mut(src_name)
            .unwrap()
            .inner_stream
            .extend(attr.to_token_stream());
    }
    None
}

fn inject_struct_interior(
    delim: Delimiter,

    grouping_interior: HashMap<String, FieldMetadata>,
    (common_destructure, common_struct): (TokenStream, TokenStream),
    token_entires: &mut HashMap<String, TokenStream>,
    impl_generics: syn::ImplGenerics,
    where_clause: &syn::WhereClause,
    post_name_generics: syn::TypeGenerics,
    output: &mut TokenStream,
    semi: bool,
    morphisms: Morphisms,
) {
    for (profile_name, profile) in grouping_interior.iter() {
        let ungrouped_profile = token_entires
            .get_mut(profile_name)
            .expect("Profile should exist");

        ungrouped_profile.extend(
            Group::new(delim, {
                let mut v = common_struct.clone();
                v.extend(profile.inner_stream.clone());
                v
            })
            .to_token_stream(),
        );
        ungrouped_profile.extend(where_clause.to_token_stream());
        if semi {
            ungrouped_profile.extend(quote! {;});
        }

        output.extend(ungrouped_profile.clone());
    }

    for ((from, to), key_injective) in morphisms {
        let mut transform_inject_stream = TokenStream::new();

        for (key, transform) in key_injective {
            let key = Ident::new(key.as_str(), Span::call_site().into());
            transform_inject_stream.extend(quote! { #key : { #transform }, })
        }

        transform_inject_stream.extend(common_destructure.clone());

        let from_destructure = Group::new(delim, {
            let mut v = common_destructure.clone();
            v.extend(
                grouping_interior
                    .get(&from)
                    .expect("From value")
                    .destructor_stream
                    .clone(),
            );
            v
        });
        let to_destructure = Group::new(delim, transform_inject_stream);

        let (from, to) = (
            Ident::new(from.as_str(), Span::call_site().into()),
            Ident::new(to.as_str(), Span::call_site().into()),
        );

        output.extend(quote! {
            impl #impl_generics Into<#to #post_name_generics> for #from #post_name_generics #where_clause {
                fn into(self) -> #to #post_name_generics {
                    let Self #from_destructure = self;

                    #to #to_destructure
                }
            }
        })
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn profile(attr: Ts1, item: Ts1) -> Ts1 {
    let (e, profiles) = attr
        .into_iter()
        .fold((None, Vec::new()), |mut acc, x| match x {
            proc_macro::TokenTree::Ident(id) => {
                acc.1.push(id.to_string());
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

            let mut fields_data = HashMap::new();
            let mut common_destructure = TokenStream::new();
            let mut common_struct = TokenStream::new();

            let mut morphisms: Morphisms = HashMap::new();

            for profile in profiles.iter() {
                morphisms.insert((profile.clone(), src_name.clone()), HashMap::new());
                morphisms.insert((src_name.clone(), profile.to_string()), HashMap::new());
                fields_data.insert(profile.to_string(), FieldMetadata::default());
            }

            fields_data.insert(src_name.clone(), FieldMetadata::default());

            let default_profile = if profiles.len() == 1 {
                Some(profiles.first().unwrap().clone())
            } else {
                None
            };

            let mut visibility_entires = HashMap::new();
            let mut token_entires = HashMap::new();
            for profile in profiles {
                token_entires.insert(profile.clone(), TokenStream::new());
                visibility_entires.insert(profile, Visibility::Inherited);
            }
            token_entires.insert(src_name.clone(), TokenStream::new());

            for attr in item.attrs {
                handle_attr(
                    attr,
                    &mut fields_data,
                    &mut iso_default,
                    &default_profile,
                    &src_name,
                    &mut morphisms,
                    None,
                );
            }

            for (key, field) in fields_data.iter_mut() {
                std::mem::swap(
                    &mut field.inner_stream,
                    token_entires.get_mut(key).expect("memswap"),
                )
            }

            for (k, v) in token_entires.iter_mut() {
                v.extend(item.vis.clone().to_token_stream());
                v.extend(item.struct_token.to_token_stream());
                v.extend(Ident::new(k.as_str(), Span::call_site().into()).to_token_stream());
                v.extend(impl_generics.clone().to_token_stream());
            }

            let mut output = TokenStream::new();
            match item.fields {
                syn::Fields::Named(fields) => {
                    for field in fields.named.iter() {
                        let mut limited_fields = Vec::new();

                        for attr in &field.attrs {
                            if let Some(x) = handle_attr(
                                attr.clone(),
                                &mut fields_data,
                                &mut iso_default,
                                &default_profile,
                                &src_name,
                                &mut morphisms,
                                field.ident.as_ref(),
                            ) {
                                limited_fields.push(x)
                            }
                        }

                        let (ident, vis, ty) =
                            (field.ident.as_ref().unwrap(), &field.vis, &field.ty);

                        if limited_fields.len() == 0 {
                            common_destructure.extend(quote! {#ident,});
                            common_struct.extend(quote! {#vis #ident : #ty,});
                        }

                        for profile_key in limited_fields.iter() {
                            let m = fields_data
                                .get_mut(profile_key.to_string().as_str())
                                .expect("Prof key");

                            m.destructor_stream.extend(quote! {#ident,});

                            m.inner_stream.extend(quote! {#vis #ident : #ty,});
                        }
                    }

                    inject_struct_interior(
                        Delimiter::Brace,
                        fields_data,
                        (common_destructure, common_struct),
                        &mut token_entires,
                        impl_generics,
                        where_clause,
                        post_name_generics,
                        &mut output,
                        false,
                        morphisms,
                    );
                }
                syn::Fields::Unnamed(fields) => {
                    for (index, field) in fields.unnamed.iter().enumerate() {
                        for attr in &field.attrs {
                            handle_attr(
                                attr.clone(),
                                &mut fields_data,
                                &mut iso_default,
                                &default_profile,
                                &src_name,
                                &mut morphisms,
                                None,
                            );
                        }

                        let ident =
                            Ident::new(format!("e{}", index).as_str(), Span::call_site().into());

                        common_destructure.extend(quote! {#ident,});

                        for profile in fields_data.values_mut() {
                            let (vis, ty) = (&field.vis, &field.ty);
                            profile.inner_stream.extend(quote! {#vis #ty,})
                        }
                    }

                    inject_struct_interior(
                        Delimiter::Parenthesis,
                        fields_data,
                        (common_destructure, common_struct),
                        &mut token_entires,
                        impl_generics,
                        where_clause,
                        post_name_generics,
                        &mut output,
                        true,
                        morphisms,
                    );
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
