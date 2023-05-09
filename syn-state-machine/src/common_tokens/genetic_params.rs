use super::*;
use crate::*;

#[derive(Debug)]
pub struct LifetimeParam {
    pub id: Ident,
    pub bounds: Option<LifetimeBounds>,
}
impl MappedParse for LifetimeParam {
    type Source = (LifetimeOrLable, Option<(Colon, LifetimeBounds)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.0,
            bounds: src.1.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct TypeParam {
    pub id: Ident,
    pub bounds: Option<TypeParamBounds>,
    pub ty: Option<Type>,
}
impl MappedParse for TypeParam {
    type Source = (
        Identifier,
        Option<(Colon, TypeParamBounds)>,
        Option<(Eq, Type)>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.0,
            bounds: src.1.map(|v| v.1),
            ty: src.2.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct ConstParam {
    pub id: Ident,

    pub ty: Type,
    pub content: Option<TokenTree>,
}
impl MappedParse for ConstParam {
    type Source = (KwConst, Identifier, Colon, Type, Option<TokenTree>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            ty: src.3,
            content: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub enum GenericParam<T: Parsable = Tokens> {
    LifetimeParam(Attrs<T>, LifetimeParam),
    TypeParam(Attrs<T>, TypeParam),
    ConstParam(Attrs<T>, ConstParam),
}
impl<T: Parsable> MappedParse for GenericParam<T> {
    type Source = WithAttrs<T, Either<Either<LifetimeParam, TypeParam>, ConstParam>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Either::Left(Either::Right(a)) => Self::TypeParam(src.0, a),
            Either::Left(Either::Left(a)) => Self::LifetimeParam(src.0, a),
            Either::Right(a) => Self::ConstParam(src.0, a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct GenericParams<T: Parsable = Tokens>(pub Vec<GenericParam<T>>);
impl<T: Parsable> MappedParse for GenericParams<T> {
    type Source = (Lt, Interlace<GenericParam<T>, Comma>, Gt);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_matches_lifetime() {
        let v = parse_terminal::<Lifetime>(quote::quote!('a)).unwrap();

        assert_eq!(v.0.to_string().as_str(), "a");
    }
}
