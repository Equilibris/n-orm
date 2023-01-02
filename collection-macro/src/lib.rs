#![feature(iter_intersperse)]
use std::collections::HashMap;

use proc_macro::TokenStream as Ts1;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use proc_macro2::{Ident, Span};
use proc_macro_error::{
    abort, abort_call_site, emit_warning, proc_macro_error, Diagnostic, Level, OptionExt,
};
use quote::quote;
use quote::ToTokens;
use syn::parse2;
use syn::spanned::Spanned;
use syn::{Attribute, Visibility};

enum Db {
    Mongo,
}

#[derive(Debug)]
enum IndexType {
    Up,
    Down,
    Text,
    Geo2D,
    Geo2DSphere,
    GeoHaystack,
    Hash,
}

#[derive(Default, Debug)]
struct LastField {
    pub single_field: u64,
    pub compound: u64,
    pub selections: u64,
}

struct Selection {
    pub fields: Vec<syn::Field>,
}

struct IndexField {
    pub fields: syn::Field,
    pub ty: IndexType,
}

#[derive(Default, Debug)]
struct IndexInfo {
    pub expire_after_seconds: Option<u64>,
    pub unique: bool,
    pub sparse: bool,
    pub hidden: bool,
}

struct SingleFieldIndex {
    pub field: syn::Field,
    pub index_info: IndexInfo,
    pub ty: IndexType,
}
struct CompoundIndex {
    pub fields: Vec<(syn::Field, IndexType)>,
    pub index_info: IndexInfo,
}

fn get_inner_tokens(tokens: TokenStream) -> Option<TokenStream> {
    let mut iter = tokens.into_iter();

    if let TokenTree::Group(inner) = iter.next()? {
        Some(inner.stream())
    } else {
        None
    }
}

fn parse_index_data(
    mut inner: impl Iterator<Item = TokenTree>,
    loc: Span,
    incl_type_parsing: bool,
    incl_info_parsing: bool,
) -> (IndexInfo, IndexType) {
    let mut info = IndexInfo::default();
    let mut ty = IndexType::Up;
    loop {
        match (inner.next(), inner.next()) {
            (Some(TokenTree::Ident(v)), Some(TokenTree::Punct(p)))
                if v.to_string().as_str() == "sparse" && p.as_char() == ',' && incl_info_parsing=>
            {
                info.sparse = true;
            }
            (Some(TokenTree::Ident(v)), Some(TokenTree::Punct(p)))
                if v.to_string().as_str() == "unique" && p.as_char() == ',' && incl_info_parsing=>
            {
                info.unique = true;
            }
            (Some(TokenTree::Ident(v)), None)
                if v.to_string().as_str() == "sparse" && incl_info_parsing=>
            {
                info.sparse = true;
                break;
            }
            (Some(TokenTree::Ident(v)), None)
                if v.to_string().as_str() == "unique" && incl_info_parsing=>
            {
                info.unique = true;
                break;
            }
            (Some(TokenTree::Ident(v)), Some(TokenTree::Punct(p)))
                if v.to_string().as_str() == "type" && p.as_char() == '=' && incl_type_parsing && incl_info_parsing=>
            {
                match match (inner.next(), inner.next()) {
                    (Some(TokenTree::Ident(id)), Some(TokenTree::Punct(p))) if p.as_char() == ','=> id,
                    (Some(TokenTree::Ident(id)), None) => id,
                    _ => abort!(loc, "Please follow type assignment by id and then termination or continuation (, ...)")
                } .to_string().as_str(){
                    "Up" => ty = IndexType::Up,
                    "Down" => ty = IndexType::Down,
                    "Text"=> ty = IndexType::Text,
                    "Geo2D"=> ty = IndexType::Geo2D,
                    "Geo2DSphere"=> ty = IndexType::Geo2DSphere,
                    "GeoHaystack"=> ty = IndexType::GeoHaystack,
                    "Hash"=> ty = IndexType::Hash,
                    _ => abort!(loc, "Please follow type assignment by one of Up, Down, Text, Geo2D, Geo2DSphere, GeoHaystack or Hash"),
                }
            }
            (None, _) => break,
            _ => abort!(loc, "Expected assignment or value"),
        }
    }

    (info, ty)
}

fn parse_index_attr(
    g: Group,
    loc: Span,
    allow_compound: bool,
    allow_single: bool,
) -> (Ident, bool, IndexInfo, IndexType) {
    let mut inner = g.to_token_stream().into_iter();
    let mut info = IndexInfo::default();
    let mut ty = IndexType::Up;

    let (index, is_single) = match (inner.next(), inner.next(), inner.next()) {
            (
                Some(TokenTree::Ident(v)),
                Some(TokenTree::Ident(id)),
                Some(TokenTree::Punct(p)),
                ) if v.to_string().as_str() == "single" && p.as_char() == ',' && allow_single=> (id, true),
                (
                    Some(TokenTree::Ident(v)),
                    Some(TokenTree::Ident(id)),
                    Some(TokenTree::Punct(p)),
                    ) if v.to_string().as_str() == "compound" && p.as_char() == ',' && allow_compound=> {
                    (id, false)
                }
            (
                Some(TokenTree::Ident(v)),
                Some(TokenTree::Ident(id)),
                None,
                ) if v.to_string().as_str() == "single" && allow_single => (id, true),
            (
                Some(TokenTree::Ident(v)),
                Some(TokenTree::Ident(id)),
                None,
                ) if v.to_string().as_str() == "compound" && allow_compound => {
                (id, false)
            }
            _ => abort!(loc, "Please provide index selection on form #[coll(index(single name, ...))] or #[coll(index(compound name, ...))]"),
    };

    let (index_info, ty) = parse_index_data(inner, g.span(), true, is_single);

    (index, is_single, index_info, ty)
}

fn mongo_struct(item: syn::ItemStruct, coll_name: Ident) -> TokenStream {
    let mut pre_selection_vals = TokenStream::new();
    // let mut selections = Vec::new();
    let mut single_fields = HashMap::new();
    let mut compounds = HashMap::new();

    let mut last_mod = LastField::default();

    for attr in item.attrs {
        let p = match attr.path.get_ident() {
            Some(v) => v,
            None => continue,
        };
        if p.to_string().as_str() == "coll" {
            let loc = attr.span();
            let mut t = get_inner_tokens(attr.tokens)
                .expect_or_abort("Expected one of #[coll(index(...))], #[coll(group(...))]")
                .into_iter();

            match (t.next(), t.next(), t.next()) {
                (Some(TokenTree::Ident(i)), Some(TokenTree::Group(g)), None)
                    if i.to_string().as_str() == "index" =>
                {
                    let loc = g.span();
                    let (index, _, index_info, _) = parse_index_attr(g, loc, true, false);

                    compounds.insert(
                        index.to_string(),
                        CompoundIndex {
                            fields: Vec::new(),
                            index_info,
                        },
                    );
                }
                _ => abort!(
                    loc,
                    "Expected one of #[coll(index(...))], #[coll(group(...))]"
                ),
            }
        }
    }

    for field in item.fields {
        let cfield = field.clone();

        for attr in field.attrs {
            let p = match attr.path.get_ident() {
                Some(v) => v,
                None => continue,
            };
            if p.to_string().as_str() == "serde" {
                emit_warning!(
                    attr.span(),
                    "Some serde attrs such as #[serde(rename = \"alt\")] may interfer with functions of the generation trait. We are working on fixing this"
                );
            }
            if p.to_string().as_str() == "coll" {
                let loc = attr.span();
                let mut t = attr.to_token_stream().into_iter();

                match (t.next(), t.next(), t.next()) {
                    (Some(TokenTree::Ident(i)), Some(TokenTree::Group(g)), None)
                        if i.to_string().as_str() == "index" =>
                    {
                        let loc = g.span();
                        let (index, is_single, index_info, ty) =
                            parse_index_attr(g, loc, true, true);

                        let index = index.to_string();
                        if is_single {
                            single_fields.insert(
                                index,
                                SingleFieldIndex {
                                    field: cfield.clone(),
                                    index_info,
                                    ty,
                                },
                            );
                        } else {
                            compounds
                                .get_mut(&index)
                                .expect_or_abort(format!("No index with name {}", index).as_str())
                                .fields
                                .push((cfield.clone(), ty));
                        }
                    }
                    _ => abort!(
                        attr.span(),
                        "Expected one of #[coll(index(...))], #[coll(group(...))]"
                    ),
                }
            }
        }
    }

    panic!("{:#?} {:#?}", compounds.keys(), single_fields.keys());
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn coll(attr: Ts1, item: Ts1) -> Ts1 {
    let (attr, item): (TokenStream, TokenStream) = (attr.into(), item.into());

    let mut attr = attr.into_iter();

    let mut output = item.clone();

    let (class, coll_name) = match (attr.next(), attr.next(), attr.next(), attr.next()) {
        (
            Some(TokenTree::Ident(db_name)),
            Some(TokenTree::Punct(p)),
            Some(TokenTree::Ident(id)),
            None,
        ) if db_name.to_string().as_str() == "mongo" && p.as_char() == ':' => (Db::Mongo, id),
        _ => abort_call_site!("Please provide Mongo : DB collection name and no more tokens"),
    };

    match class {
        Db::Mongo => match parse2::<syn::Item>(item) {
            Ok(item) => match item {
                syn::Item::Struct(s) => output.extend(mongo_struct(s, coll_name)),
                _ => abort_call_site!("Collection only impld for structs"),
            },
            Err(e) => abort!("{}", e),
        },
    }

    output.into()
}
