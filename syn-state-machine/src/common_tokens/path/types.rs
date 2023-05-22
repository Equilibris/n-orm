use super::super::*;
use crate::*;
use std::fmt::Debug;

pub struct TypePathFn<T: Parsable> {
    pub args: Vec<Type<T>>,
    pub out: Option<Type<T>>,
}
impl<T: Parsable> Debug for TypePathFn<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypePathFn")
            .field("args", &self.args)
            .field("out", &self.out)
            .finish()
    }
}
impl<T: Parsable> MappedParse for TypePathFn<T> {
    type Source = (Paren<TypePathFnInputs<T>>, Option<(Arrow, Type<T>)>);

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
pub struct TypePathFnInputs<T: Parsable>(pub Vec<Type<T>>);
impl<T: Parsable> Debug for TypePathFnInputs<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypePathFnInputs").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for TypePathFnInputs<T> {
    type Source = (Interlace<Type<T>, Comma>, Option<Comma>);

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

pub enum TypePathSegment<T: Parsable> {
    Simple {
        id: PathIdentSegment,
    },
    Generic {
        id: PathIdentSegment,
        generic_args: GenericArgs<T>,
    },
    TypePathFn {
        id: PathIdentSegment,
        path_fn: TypePathFn<T>,
    },
}
impl<T: Parsable> Debug for TypePathSegment<T>
where
    SmOut<T>: Debug,
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
impl<T: Parsable> MappedParse for TypePathSegment<T> {
    type Source = (
        PathIdentSegment,
        Option<(
            Option<DoubleColon>,
            MBox<Sum2<GenericArgs<T>, TypePathFn<T>>>,
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

pub struct TypePath<T: Parsable> {
    pub leading: bool,
    pub segments: Vec<TypePathSegment<T>>,
}
impl<T: Parsable> Debug for TypePath<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypePath")
            .field("leading", &self.leading)
            .field("segments", &self.segments)
            .finish()
    }
}
impl<T: Parsable> MappedParse for TypePath<T> {
    type Source = (
        Option<DoubleColon>,
        MinLength<Interlace<TypePathSegment<T>, DoubleColon>>,
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

    insta_match_test!(it_matches_hello, TypePath<Infallible>: hello);
    insta_match_test!(it_matches_tri_path, TypePath<Infallible>: hello::world::hi);
    insta_match_test!(it_matches_bi_path, TypePath<Infallible>: hello::world);
    insta_match_test!(it_matches_long_generic, TypePath<Infallible>: hello::<Hi>);
    insta_match_test!(it_matches_short_generic, TypePath<Infallible>: hello<Hi>);

    #[test]
    fn it_matches_multigeneric_type_path() {
        println!(
            "{:#?}",
            parse::<TypePath<Infallible>>(quote::quote!(hello<hello::Hi, 10, 'a>)).unwrap()
        );
    }
}
