use super::super::*;
use crate::*;
use insta::assert_debug_snapshot;

#[derive(Debug)]
pub struct TypePathFn {
    pub args: Vec<Type>,
    pub out: Option<Type>,
}
impl MappedParse for TypePathFn {
    type Source = (Paren<TypePathFnInputs>, Option<(Arrow, Type)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            args: src.0 .0,
            out: src.1.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TypePathFnInputs(pub Vec<Type>);
impl MappedParse for TypePathFnInputs {
    type Source = (Interlace<Type, Comma>, Option<Comma>);

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

#[derive(Debug)]
pub enum TypePathSegment {
    Simple {
        id: PathIdentSegment,
    },
    Generic {
        id: PathIdentSegment,
        generic_args: GenericArgs,
    },
    TypePathFn {
        id: PathIdentSegment,
        path_fn: TypePathFn,
    },
}
impl MappedParse for TypePathSegment {
    type Source = (
        PathIdentSegment,
        Option<(Option<DoubleColon>, MBox<Sum2<GenericArgs, TypePathFn>>)>,
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

#[derive(Debug)]
pub struct TypePath {
    pub leading: bool,
    pub segments: Vec<TypePathSegment>,
}
impl MappedParse for TypePath {
    type Source = (
        Option<DoubleColon>,
        MinLength<Interlace<TypePathSegment, DoubleColon>>,
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

    insta_match_test!(it_matches_hello, TypePath: hello);
    insta_match_test!(it_matches_tri_path, TypePath: hello::world::hi);
    insta_match_test!(it_matches_bi_path, TypePath: hello::world);
    insta_match_test!(it_matches_long_generic, TypePath: hello::<Hi>);
    insta_match_test!(it_matches_short_generic, TypePath: hello<Hi>);

    #[test]
    fn it_matches_multigeneric_type_path() {
        println!(
            "{:#?}",
            parse::<TypePath>(quote::quote!(hello<hello::Hi, 10, 'a>)).unwrap()
        );
    }
}
