use std::fmt::Debug;

use super::*;
use crate::*;

pub struct BareFunctionType<T: Parsable, Ty: Parsable> {
    pub r#for: Option<ForLifetimes<T, Ty>>,
    pub qualifiers: FunctionTypeQualifiers,
    pub params: Option<FunctionParametersMaybeNamedVariadic<T, Ty>>,
    pub ret: Option<BareFunctionReturnType<Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for BareFunctionType<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BareFunctionType")
            .field("for", &self.r#for)
            .field("qualifiers", &self.qualifiers)
            .field("params", &self.params)
            .field("ret", &self.ret)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for BareFunctionType<T, Ty> {
    type Source = (
        Option<ForLifetimes<T, Ty>>,
        FunctionTypeQualifiers,
        KwFn,
        Paren<Option<FunctionParametersMaybeNamedVariadic<T, Ty>>>,
        Option<BareFunctionReturnType<Ty>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#for: src.0,
            qualifiers: src.1,
            params: src.3 .0,
            ret: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum FunctionParametersMaybeNamedVariadic<T: Parsable, Ty: Parsable> {
    MaybeNamedFunctionParameters(MaybeNamedFunctionParameters<Ty>),
    MaybeNamedFunctionParametersVariadic(MaybeNamedFunctionParametersVariadic<T, Ty>),
}
impl<T: Parsable, Ty: Parsable> Debug for FunctionParametersMaybeNamedVariadic<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaybeNamedFunctionParameters(arg0) => f
                .debug_tuple("MaybeNamedFunctionParameters")
                .field(arg0)
                .finish(),
            Self::MaybeNamedFunctionParametersVariadic(arg0) => f
                .debug_tuple("MaybeNamedFunctionParametersVariadic")
                .field(arg0)
                .finish(),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for FunctionParametersMaybeNamedVariadic<T, Ty> {
    type Source =
        Sum2<MaybeNamedFunctionParametersVariadic<T, Ty>, MaybeNamedFunctionParameters<Ty>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::MaybeNamedFunctionParametersVariadic(a),
            Sum2::Val1(a) => Self::MaybeNamedFunctionParameters(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct MaybeNamedFunctionParametersVariadic<T: Parsable, Ty: Parsable>(
    pub Vec<MaybeNamedParam<Ty>>,
    pub Attrs<T>,
);

impl<T: Parsable, Ty: Parsable> Debug for MaybeNamedFunctionParametersVariadic<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MaybeNamedFunctionParametersVariadic")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for MaybeNamedFunctionParametersVariadic<T, Ty> {
    type Source = (
        MinLength<Interlace<MaybeNamedParam<Ty>, Comma>>,
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

pub struct MaybeNamedFunctionParameters<Ty: Parsable>(pub Vec<MaybeNamedParam<Ty>>);
impl<Ty: Parsable> Debug for MaybeNamedFunctionParameters<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MaybeNamedFunctionParameters")
            .field(&self.0)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for MaybeNamedFunctionParameters<Ty> {
    type Source = (Interlace<MaybeNamedParam<Ty>, Comma>, Option<Comma>);

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

pub struct MaybeNamedParam<Ty: Parsable> {
    pub id: Option<Ident>,
    pub ty: SmOut<Ty>,
}
impl<Ty: Parsable> Debug for MaybeNamedParam<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaybeNamedParam")
            .field("id", &self.id)
            .field("ty", &self.ty)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for MaybeNamedParam<Ty> {
    type Source = (Option<(IdentifierOrUnder, Colon)>, Ty);

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

pub struct BareFunctionReturnType<Ty: Parsable>(pub SmOut<Ty>);
impl<Ty: Parsable> Debug for BareFunctionReturnType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BareFunctionReturnType")
            .field(&self.0)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for BareFunctionReturnType<Ty> {
    type Source = (Arrow, Ty);

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

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(
        it_matches_complex_fun,
        BareFunctionType<Infallible, PBox<TypeNoBounds<Infallible>>>: for<'a> unsafe extern "C" fn(Hello, World, ...) -> i64
    );
    insta_match_test!(it_matches_return, BareFunctionType<Infallible, PBox<TypeNoBounds<Infallible>>>: fn(Hello, World) -> i64);
    insta_match_test!(it_matches_simple, BareFunctionType<Infallible, PBox<TypeNoBounds<Infallible>>>: fn());
    insta_match_test!(
        it_matches_qualified,
        BareFunctionType<Infallible,PBox<TypeNoBounds<Infallible>>>: for<'a> unsafe extern "C" fn()
    );
}
