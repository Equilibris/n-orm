#![feature(iter_intersperse)]
use std::collections::HashMap;

use proc_macro::TokenStream as Ts1;
use proc_macro2::{Ident, Span};
use proc_macro2::{TokenStream, TokenTree};
use proc_macro_error::{abort, proc_macro_error, OptionExt, ResultExt};
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{parse2, parse_macro_input, DeriveInput, Expr, ItemStruct};
use syn::{Attribute, Visibility};

#[derive(Debug)]
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

#[derive(Debug)]
struct Selection {
    pub fields: Vec<syn::Field>,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct SingleFieldIndex {
    pub field: syn::Field,
    pub index_info: IndexInfo,
    pub ty: IndexType,
}
#[derive(Debug)]
struct CompoundIndex {
    pub fields: Vec<(syn::Field, IndexType)>,
    pub index_info: IndexInfo,
}

#[derive(Debug)]
enum TargetAttr {
    Coll(TokenStream),
    CollOption(String, String, Span),
    CollIndex(TokenStream),

    SerdeRename(String),
    SerdeRenameAll(String),
}

#[derive(Default)]
struct Options {
    pub collection_sharing: bool,
}

fn match_equality(gr: TokenStream, allow_implicit: bool) -> Option<(String, String)> {
    let mut it = gr.into_iter();
    match (it.next(), it.next(), it.next(), it.next()) {
        (
            Some(TokenTree::Ident(left)),
            Some(TokenTree::Punct(p)),
            Some(TokenTree::Ident(right)),
            None,
        ) if p.as_char() == '=' => Some((left.to_string(), right.to_string())),

        (Some(TokenTree::Ident(left)), None, None, None) if allow_implicit => {
            Some((left.to_string(), "true".to_string()))
        }
        _ => None,
    }
}

impl TargetAttr {
    fn build_option(gr: TokenStream, loc: Span) -> TargetAttr {
        let (left, right) = match_equality(gr, true)
            .unwrap_or_else(|| abort!(loc, "Expected equality: #[coll(attr = value)]"));
        Self::CollOption(left, right, loc)
    }
    fn transform_into_expected_values(self) -> Self {
        use TargetAttr::*;

        match self {
            Coll(a) => {
                let loc = a.span();

                let mut it = a.clone().into_iter();

                let (Some(TokenTree::Ident(id)),Some(TokenTree::Group(gr)),None) = (it.next(), it.next(), it.next()) else {
                     return Coll(a)
                };
                match id.to_string().as_str() {
                    "option" => Self::build_option(gr.stream(), loc),
                    "index" => CollIndex(gr.stream()),
                    _ => abort!(loc, "Unexpected function call"),
                }
            }
            a => a,
        }
    }

    fn filter_map(attr: &Attribute) -> Option<Self> {
        attr.path
            .get_ident()
            .map(|v| v.to_string())
            .and_then(|v| match v.as_ref() {
                "coll" => Some(match get_inner_tokens(attr.clone().tokens) {
                    Some(v) => Self::Coll(v),
                    None => abort!(attr.span(), "coll macro requires arguments #[coll(...)]"),
                }),
                "coll" => Some(match get_inner_tokens(attr.clone().tokens) {
                    Some(v) => Self::Coll(v),
                    None => abort!(attr.span(), "coll macro requires arguments #[coll(...)]"),
                }),
                _ => None,
            })
    }
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
            _ => abort!(loc, "Expected assignment or value #[coll(index single name, type=Text, sparse)]"),
        }
    }

    (info, ty)
}

fn parse_index_attr(
    g: TokenStream,
    loc: Span,
    allow_compound: bool,
    allow_single: bool,
) -> (Ident, bool, IndexInfo, IndexType) {
    let mut inner = g.clone().into_iter();

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
            _ if !allow_single => abort!(loc, "Please provide index selection on form #[coll(index(compound name, ...))]"),
            _ if !allow_compound => abort!(loc, "Please provide index selection on form #[coll(index(single name, ...))]"),
            _ => abort!(loc, "Please provide index selection on form #[coll(index(single name, ...))] or #[coll(index(compound name, ...))]"),
    };

    let (index_info, ty) = parse_index_data(inner, g.span(), is_single, true);

    (index, is_single, index_info, ty)
}

fn get_db_info<'a>(attrs: &mut impl Iterator<Item = &'a syn::Attribute>) {}

type CompoundIndexes = HashMap<String, CompoundIndex>;

fn parse_primary_attrs(
    mut attrs: impl Iterator<Item = TargetAttr>,
    options: &mut Options,
) -> ((Visibility, Ident, Ident), CompoundIndexes) {
    let mut compounds = HashMap::new();

    let Some(TargetAttr::Coll(t)) = attrs.next() else {
        abort!(Span::call_site(),"Database specifier must come before any. Example #[coll(pub UserCollection : \"users\")]");
    };

    let loc = t.span();
    let mut t = t.into_iter();

    let v = match (
        t.next(),
        // (t.next().and_then(|v| parse2::<syn::Expr>(v.into()).ok())),
        t.next(),
        t.next(),
        t.next(),
    ) {
        (Some(TokenTree::Ident(id)), Some(TokenTree::Ident(v)), None, None) => {
            (syn::Visibility::Inherited, id, v)
        }
        (Some(vis), Some(TokenTree::Ident(id)), Some(TokenTree::Ident(v)), None) => {
            let Ok(vis) = parse2::<syn::Visibility> (vis.to_token_stream()) else {abort!(loc,"Expected visability #[coll(pub UserCollection users)]");};

            (vis, id, v)
        }

        _ => abort!(
            loc,
            "Expected the first coll attr to be #[coll(pub UserCollection users)]"
        ),
    };

    for attr in attrs {
        match attr {
            TargetAttr::CollIndex(v) => {
                let loc = v.span();
                let (index, _, index_info, _) = parse_index_attr(v, loc, true, false);

                compounds.insert(
                    index.to_string(),
                    CompoundIndex {
                        fields: Vec::new(),
                        index_info,
                    },
                );
            }

            TargetAttr::Coll(a) => abort!(
                a.span(),
                "Unexpected tokens {}, expected index or option specification",
                a
            ),

            TargetAttr::CollOption(left, right, loc) => match left.as_str() {
                "collection_sharing" => {
                    options.collection_sharing = right.parse().unwrap_or_else(move |_| {
                        abort!(loc, "Option collection_sharing must be a bool")
                    })
                }
                a => abort!(loc, "No option {}", a),
            },
            a => todo!("lol {:#?}", a),
        }
    }

    (v, compounds)
}

fn handle_struct_body(
    item: syn::ItemStruct,
    compounds: &mut CompoundIndexes,
) -> HashMap<String, SingleFieldIndex> {
    let mut single_fields = HashMap::new();

    for field in &item.fields {
        let cfield = field.clone();

        for attr in field.attrs.iter().filter_map(TargetAttr::filter_map) {
            match attr {
                TargetAttr::CollIndex(index) => {
                    let loc = index.span();
                    let (index, is_single, index_info, ty) =
                        parse_index_attr(index, loc, true, true);

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
                _ => todo!(),
            }
        }
    }

    single_fields
}

#[proc_macro_error]
#[proc_macro_derive(Document, attributes(coll))]
pub fn document(input: Ts1) -> Ts1 {
    let mut options = Options::default();
    let item = parse_macro_input!(input as DeriveInput);

    if !matches!(&item.data, syn::Data::Struct(_)) {
        abort!(
            item.span(),
            "Collection macro not yet implemented for enums"
        )
    }

    let ((vis, coll_struct_id, db_coll), mut compound_indexes) = parse_primary_attrs(
        item.attrs
            .iter()
            .filter_map(TargetAttr::filter_map)
            .map(TargetAttr::transform_into_expected_values),
        &mut options,
    );
    let source_id = item.ident.clone();

    let item = parse2::<syn::ItemStruct>(item.to_token_stream()).unwrap_or_abort();
    let single_indexes = handle_struct_body(item, &mut compound_indexes);

    quote! {
        impl ::collection::Document for #source_id {
            type Collection = #coll_struct_id;
        }

        #[derive(Clone)]
        #vis struct #coll_struct_id (pub ::mongodb::Collection<#source_id>);
        impl ::collection::Collection for #coll_struct_id {
            type Internal = ::mongodb::Collection<#source_id>;
            type Document = #source_id;

            async fn fetch<F: ::collection::Fetcher<Self::Document, Self::Internal>>(
                &self,
                f: F,
            ) -> Result<F::Output, F::Error> {
                f.fetch(&self.0).await
            }
        }
    }
    .into()
}
