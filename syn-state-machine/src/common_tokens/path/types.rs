use super::super::*;
use crate::*;

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
        Option<DoubleColon>,
        Option<Either<GenericArgs, TypePathFn>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.2 {
            Some(Either::Left(a)) => Self::Generic {
                id: src.0,
                generic_args: a,
            },
            Some(Either::Right(a)) => Self::TypePathFn {
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
    use quote::quote;

    use super::*;
    use crate::parse_terminal;

    #[test]
    fn it_matches_simple_paths() {
        println!("{:#?}", parse_terminal::<TypePath>(quote!(hello::world)));
        println!("{:#?}", parse_terminal::<TypePath>(quote!(hello)));
    }
}
