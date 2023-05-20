use super::*;
use crate::*;

#[derive(Debug)]
pub enum UseTree {
    Star(SimplePathOrNone),
    Recursion {
        name: SimplePathOrNone,
        deep: Vec<UseTree>,
    },
    Standard(SimplePath, Option<Ident>),
}

impl MappedParse for UseTree {
    type Source = Sum2<
        (
            Option<(SimplePathOrNone, DoubleColon)>,
            Brace<(Interlace<UseTree, Comma>, Option<Comma>)>,
        ),
        Sum2<(Option<(SimplePathOrNone, DoubleColon)>, Star), (SimplePath, AsClause)>,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        use Sum2::*;

        Ok(match src {
            Val1(Val1(a)) => Self::Standard(a.0, a.1.map(|(_, a)| a)),
            Val0(a) => Self::Recursion {
                name: match a.0 {
                    Some(a) => a.0,
                    None => SimplePathOrNone::default(),
                },
                deep: a.1 .0 .0,
            },
            Val1(Val0((a, _))) => Self::Star({
                match a {
                    Some(a) => a.0,
                    None => SimplePathOrNone::default(),
                }
            }),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct Use;

impl MappedParse for Use {
    type Source = (KwUse, UseTree, Semi);

    type Output = UseTree;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(src.1)
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use quote::quote;

    fn use_tree_to_idents(source: &mut Vec<Ident>, use_tree: UseTree) {
        match use_tree {
            UseTree::Star(a) => {
                for p in a.segments {
                    match p {
                        SimplePathSegment::Id(a) => source.push(a),
                        SimplePathSegment::DCrate => unimplemented!(),
                    }
                }
            }
            UseTree::Recursion { name, deep } => {
                for p in name.segments {
                    match p {
                        SimplePathSegment::Id(a) => source.push(a),
                        SimplePathSegment::DCrate => unimplemented!(),
                    }
                }

                for v in deep {
                    use_tree_to_idents(source, v);
                }
            }
            UseTree::Standard(s, t) => {
                for p in s.segments {
                    match p {
                        SimplePathSegment::Id(a) => source.push(a),
                        SimplePathSegment::DCrate => unimplemented!(),
                    }
                }

                if let Some(v) = t {
                    source.push(v)
                }
            }
        }
    }

    #[test]
    fn it_matches_simple_path() {
        for i in [
            quote! { use hello::world; },
            quote! { use hei; },
            quote! { use hei as h; },
        ] {
            let p = parse_terminal::<Use>(i);

            let p = match p {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            };

            if let UseTree::Standard(..) = p {
            } else {
                unreachable!()
            }
        }
    }
    #[test]
    fn it_matches_star_path() {
        for i in [quote! { use *; }, quote! { use hei::*; }] {
            let p = parse_terminal::<Use>(i);

            let p = match p {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            };

            if let UseTree::Star(_) = p {
            } else {
                unreachable!()
            }
        }
    }
    #[test]
    fn it_matches_complex_path() {
        let mut v = Vec::new();

        use_tree_to_idents(
            &mut v,
            parse_terminal::<Use>(
                quote::quote! { use { hello::*, world::hi as Hi, nested::{ hello::world, hi }, }; },
            )
            .unwrap(),
        );

        for (lhs, rhs) in v.into_iter().zip([
            "hello", "world", "hi", "Hi", "nested", "hello", "world", "hi",
        ]) {
            assert_eq!(lhs.to_string().as_str(), rhs)
        }
    }
}
