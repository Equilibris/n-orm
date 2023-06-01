use std::fmt::Debug;

use super::*;
use crate::*;

pub struct BareFunctionType<T: Parsable, TyNB: Parsable> {
    pub r#for: Option<ForLifetimes<T, TyNB>>,
    pub qualifiers: FunctionTypeQualifiers,
    pub params: Option<FunctionParametersMaybeNamedVariadic<T, TyNB>>,
    pub ret: Option<BareFunctionReturnType<TyNB>>,
}
impl<T: Parsable, TyNB: Parsable> Debug for BareFunctionType<T, TyNB>
where
    SmOut<T>: Debug,
    SmOut<TyNB>: Debug,
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
impl<T: Parsable, TyNB: Parsable> MappedParse for BareFunctionType<T, TyNB> {
    type Source = (
        Option<ForLifetimes<T, TyNB>>,
        FunctionTypeQualifiers,
        KwFn,
        Paren<Option<FunctionParametersMaybeNamedVariadic<T, TyNB>>>,
        Option<BareFunctionReturnType<TyNB>>,
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

pub enum FunctionParametersMaybeNamedVariadic<T: Parsable, TyNB: Parsable> {
    Parameters(MaybeNamedFunctionParameters<TyNB>),
    ParametersVariadic(MaybeNamedFunctionParametersVariadic<T, TyNB>),
}
impl<T: Parsable, TyNB: Parsable> Debug for FunctionParametersMaybeNamedVariadic<T, TyNB>
where
    SmOut<T>: Debug,
    SmOut<TyNB>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parameters(arg0) => f
                .debug_tuple("MaybeNamedFunctionParameters")
                .field(arg0)
                .finish(),
            Self::ParametersVariadic(arg0) => f
                .debug_tuple("MaybeNamedFunctionParametersVariadic")
                .field(arg0)
                .finish(),
        }
    }
}
impl<T: Parsable, TyNB: Parsable> MappedParse for FunctionParametersMaybeNamedVariadic<T, TyNB> {
    type Source =
        Sum2<MaybeNamedFunctionParametersVariadic<T, TyNB>, MaybeNamedFunctionParameters<TyNB>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::ParametersVariadic(a),
            Sum2::Val1(a) => Self::Parameters(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct MaybeNamedFunctionParametersVariadic<T: Parsable, TyNB: Parsable>(
    pub Vec<MaybeNamedParam<TyNB>>,
    pub Attrs<T>,
);

impl<T: Parsable, TyNB: Parsable> Debug for MaybeNamedFunctionParametersVariadic<T, TyNB>
where
    SmOut<T>: Debug,
    SmOut<TyNB>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MaybeNamedFunctionParametersVariadic")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable, TyNB: Parsable> MappedParse for MaybeNamedFunctionParametersVariadic<T, TyNB> {
    type Source = (
        MinLength<Interlace<MaybeNamedParam<TyNB>, Comma>>,
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

pub struct MaybeNamedFunctionParameters<TyNB: Parsable>(pub Vec<MaybeNamedParam<TyNB>>);
impl<TyNB: Parsable> Debug for MaybeNamedFunctionParameters<TyNB>
where
    SmOut<TyNB>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MaybeNamedFunctionParameters")
            .field(&self.0)
            .finish()
    }
}
impl<TyNB: Parsable> MappedParse for MaybeNamedFunctionParameters<TyNB> {
    type Source = (Interlace<MaybeNamedParam<TyNB>, Comma>, Option<Comma>);

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

pub struct MaybeNamedParam<TyNB: Parsable> {
    pub id: Option<Ident>,
    pub ty: SmOut<TyNB>,
}
impl<TyNB: Parsable> Debug for MaybeNamedParam<TyNB>
where
    SmOut<TyNB>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaybeNamedParam")
            .field("id", &self.id)
            .field("ty", &self.ty)
            .finish()
    }
}
impl<TyNB: Parsable> MappedParse for MaybeNamedParam<TyNB> {
    type Source = (Option<(IdentifierOrUnder, Colon)>, TyNB);

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

pub struct BareFunctionReturnType<TyNB: Parsable>(pub SmOut<TyNB>);
impl<TyNB: Parsable> Debug for BareFunctionReturnType<TyNB>
where
    SmOut<TyNB>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BareFunctionReturnType")
            .field(&self.0)
            .finish()
    }
}
impl<TyNB: Parsable> MappedParse for BareFunctionReturnType<TyNB> {
    type Source = (Arrow, TyNB);

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

    type PTnbII = PBox<TypeNoBounds<Infallible, Infallible>>;

    insta_match_test!(
        it_matches_complex_fun,
        BareFunctionType<Infallible, PTnbII>:
        for<'a> unsafe extern "C" fn(Hello, World, ...) -> i64
    );
    insta_match_test!(
        it_matches_return,
        BareFunctionType<Infallible, PTnbII>:
        fn(Hello, World) -> i64
    );
    insta_match_test!(
        it_matches_simple,
        BareFunctionType<Infallible, PTnbII>:
        fn()
    );
    insta_match_test!(
        it_matches_qualified,
        BareFunctionType<Infallible, PTnbII>:
        for<'a> unsafe extern "C" fn()
    );
}
