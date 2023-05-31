use super::super::*;
use crate::*;
use std::fmt::Debug;

pub struct TypePathFn<Ty: Parsable> {
    pub args: Vec<SmOut<Ty>>,
    pub out: Option<SmOut<Ty>>,
}
impl<Ty: Parsable> Debug for TypePathFn<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypePathFn")
            .field("args", &self.args)
            .field("out", &self.out)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for TypePathFn<Ty> {
    type Source = (Paren<TypePathFnInputs<Ty>>, Option<(Arrow, Ty)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            args: src.0 .0 .0,
            out: src.1.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
pub struct TypePathFnInputs<Ty: Parsable>(pub Vec<SmOut<Ty>>);
impl<Ty: Parsable> Debug for TypePathFnInputs<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypePathFnInputs").field(&self.0).finish()
    }
}
impl<Ty: Parsable> MappedParse for TypePathFnInputs<Ty> {
    type Source = (Interlace<Ty, Comma>, Option<Comma>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// Used in path/qualified
pub enum TypePathSegment<Ty: Parsable> {
    Simple {
        id: PathIdentSegment,
    },
    Generic {
        id: PathIdentSegment,
        generic_args: GenericArgs<Ty>,
    },
    TypePathFn {
        id: PathIdentSegment,
        path_fn: TypePathFn<Ty>,
    },
}
impl<Ty: Parsable> Debug for TypePathSegment<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple { id } => f.debug_struct("Simple").field("id", id).finish(),
            Self::Generic { id, generic_args } => f
                .debug_struct("Generic")
                .field("id", id)
                .field("generic_args", generic_args)
                .finish(),
            Self::TypePathFn { id, path_fn } => f
                .debug_struct("TypePathFn")
                .field("id", id)
                .field("path_fn", path_fn)
                .finish(),
        }
    }
}
impl<Ty: Parsable> MappedParse for TypePathSegment<Ty> {
    type Source = (
        PathIdentSegment,
        Option<(
            Option<DoubleColon>,
            MBox<Sum2<GenericArgs<Ty>, TypePathFn<Ty>>>,
        )>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Some((_, Sum2::Val0(a))) => Self::Generic {
                id: src.0,
                generic_args: a,
            },
            Some((_, Sum2::Val1(a))) => Self::TypePathFn {
                id: src.0,
                path_fn: a,
            },
            None => Self::Simple { id: src.0 },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// Used in path/qualified, types, implementations, bounds
pub struct TypePath<Ty: Parsable> {
    pub leading: bool,
    pub segments: Vec<TypePathSegment<Ty>>,
}
impl<Ty: Parsable> Debug for TypePath<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypePath")
            .field("leading", &self.leading)
            .field("segments", &self.segments)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for TypePath<Ty> {
    type Source = (
        Option<DoubleColon>,
        MinLength<Interlace<TypePathSegment<Ty>, DoubleColon>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            leading: src.0.is_some(),
            segments: src.1 .0,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    insta_match_test!(it_matches_hello, TypePath<Ident>: hello);
    insta_match_test!(it_matches_tri_path, TypePath<Ident>: hello::world::hi);
    insta_match_test!(it_matches_bi_path, TypePath<Ident>: hello::world);
    insta_match_test!(it_matches_long_generic, TypePath<Ident>: hello::<Hi>);
    insta_match_test!(it_matches_short_generic, TypePath<Ident>: hello<Hi>);
    insta_match_test!(it_matches_fun, TypePath<Infallible>: Fn());
    insta_match_test!(it_matches_fun_ret, TypePath<Paren<()>>: Fn() -> ());

    #[test]
    fn it_matches_multigeneric_type_path() {
        println!(
            "{:#?}",
            parse::<TypePath<TypePath<Ident>>>(quote::quote!(hello<hello::Hi, 10, 'a>)).unwrap()
        );
    }
}
