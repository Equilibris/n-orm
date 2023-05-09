use super::*;
use crate::*;

#[derive(Debug)]
pub struct BareFunctionType {
    pub r#for: Option<ForLifetimes>,
    pub qualifiers: FunctionTypeQualifiers,
    pub params: Option<FunctionParametersMaybeNamedVariadic>,
    pub ret: Option<BareFunctionReturnType>,
}
impl MappedParse for BareFunctionType {
    type Source = (
        Option<ForLifetimes>,
        FunctionTypeQualifiers,
        KwFn,
        Paren<Option<FunctionParametersMaybeNamedVariadic>>,
        Option<BareFunctionReturnType>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#for: src.0,
            qualifiers: src.1,
            params: src.3,
            ret: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub enum FunctionParametersMaybeNamedVariadic {
    MaybeNamedFunctionParameters(MaybeNamedFunctionParameters),
    MaybeNamedFunctionParametersVariadic(MaybeNamedFunctionParametersVariadic),
}
impl MappedParse for FunctionParametersMaybeNamedVariadic {
    type Source = Either<MaybeNamedFunctionParametersVariadic, MaybeNamedFunctionParameters>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Either::Left(a) => Self::MaybeNamedFunctionParametersVariadic(a),
            Either::Right(a) => Self::MaybeNamedFunctionParameters(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct MaybeNamedFunctionParametersVariadic<T: Parsable = Tokens>(
    pub Vec<MaybeNamedParam>,
    pub Attrs<T>,
);
impl<T: Parsable> MappedParse for MaybeNamedFunctionParametersVariadic<T> {
    type Source = (
        MinLength<Interlace<MaybeNamedParam, Comma>>,
        Comma,
        Attrs<T>,
        Elipsis,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0 .0, src.2))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct MaybeNamedFunctionParameters(pub Vec<MaybeNamedParam>);
impl MappedParse for MaybeNamedFunctionParameters {
    type Source = (Interlace<MaybeNamedParam, Comma>, Option<Comma>);

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
pub struct MaybeNamedParam {
    pub id: Option<Ident>,
    pub ty: Type,
}
impl MappedParse for MaybeNamedParam {
    type Source = (Option<(IdentifierOrUnder, Colon)>, Type);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.0.map(|v| v.0),
            ty: src.1,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct FunctionTypeQualifiers {
    pub r#unsafe: bool,
    pub r#extern: Option<Option<Abi>>,
}
impl MappedParse for FunctionTypeQualifiers {
    type Source = (Option<KwUnsafe>, Option<(KwExtern, Option<Abi>)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#unsafe: src.0.is_some(),
            r#extern: src.1.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct BareFunctionReturnType(pub TypeNoBounds);
impl MappedParse for BareFunctionReturnType {
    type Source = (Arrow, TypeNoBounds);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
