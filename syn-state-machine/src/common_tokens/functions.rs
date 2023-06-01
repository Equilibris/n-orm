use std::fmt::Debug;

use super::*;
use crate::*;

pub struct Function<T: Parsable, Ty: Parsable> {
    pub qualifiers: FunctionQualifiers,

    pub ident: Ident,

    pub generic_params: Option<GenericParams<T>>,
    pub where_clause: Option<WhereClause<T, Ty>>,

    pub self_param: Option<WithAttrs<T, SelfParam<Ty>>>,
    pub args: Vec<WithAttrs<T, FunctionParam<T, Ty>>>,

    pub returns: Option<SmOut<Ty>>,
    pub body: Option<BlockExpression>,
}
impl<T: Parsable, Ty: Parsable> Debug for Function<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("qualifiers", &self.qualifiers)
            .field("ident", &self.ident)
            .field("generic_params", &self.generic_params)
            .field("where_clause", &self.where_clause)
            .field("self_param", &self.self_param)
            .field("args", &self.args)
            .field("returns", &self.returns)
            .field("body", &self.body)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for Function<T, Ty> {
    type Source = (
        FunctionQualifiers,
        KwFn,
        Identifier,
        Option<GenericParams<T>>,
        Paren<FunctionParameters<T, Ty>>,
        Option<FunctionReturnType<Ty>>,
        Option<WhereClause<T, Ty>>,
        Sum2<BlockExpression, Semi>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            qualifiers: src.0,
            ident: src.2,

            generic_params: src.3,
            where_clause: src.6,

            self_param: src.4 .0.self_param,
            args: src.4 .0.params,
            returns: src.5.map(|v| v.0),
            body: if let Sum2::Val0(a) = src.7 {
                Some(a)
            } else {
                None
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct FunctionQualifiers {
    pub r#const: bool,
    pub r#async: bool,
    pub r#unsafe: bool,
    pub r#extern: Option<Option<StringLit>>,
}
impl MappedParse for FunctionQualifiers {
    type Source = (
        Option<KwConst>,
        Option<KwAsync>,
        Option<KwUnsafe>,
        Option<(KwExtern, Option<Abi>)>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#const: src.0.is_some(),
            r#async: src.1.is_some(),
            r#unsafe: src.2.is_some(),
            r#extern: src.3.map(|(_, a)| a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct FunctionParameters<T: Parsable, Ty: Parsable> {
    pub self_param: Option<WithAttrs<T, SelfParam<Ty>>>,

    pub params: Vec<WithAttrs<T, FunctionParam<T, Ty>>>,
}
impl<T: Parsable, Ty: Parsable> Debug for FunctionParameters<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionParameters")
            .field("self_param", &self.self_param)
            .field("params", &self.params)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for FunctionParameters<T, Ty> {
    type Source = Sum2<
        (
            Option<(WithAttrs<T, SelfParam<Ty>>, Comma)>,
            Interlace<WithAttrs<T, FunctionParam<T, Ty>>, Comma>,
        ),
        (WithAttrs<T, SelfParam<Ty>>, Option<Comma>),
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self {
                self_param: a.0.map(|v| v.0),
                params: a.1 .0,
            },
            Sum2::Val1(a) => Self {
                self_param: Some(a.0),
                params: Vec::new(),
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct ShorthandSelf {
    pub r#ref: Option<Option<Lifetime>>,
    pub r#mut: bool,
}
impl MappedParse for ShorthandSelf {
    type Source = (Option<(Amp, Option<Lifetime>)>, Option<KwMut>, KwLowerSelf);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#ref: src.0.map(|v| v.1),
            r#mut: src.1.is_some(),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TypedSelf<Ty: Parsable> {
    pub is_mut: bool,
    pub ty: SmOut<Ty>,
}
impl<Ty: Parsable> Debug for TypedSelf<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedSelf")
            .field("is_mut", &self.is_mut)
            .field("ty", &self.ty)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for TypedSelf<Ty> {
    type Source = (Option<KwMut>, KwLowerSelf, Colon, Ty);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            is_mut: src.0.is_some(),
            ty: src.3,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum SelfParam<Ty: Parsable> {
    Shorthand(ShorthandSelf),
    Typed(TypedSelf<Ty>),
}
impl<Ty: Parsable> Debug for SelfParam<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shorthand(arg0) => f.debug_tuple("Shorthand").field(arg0).finish(),
            Self::Typed(arg0) => f.debug_tuple("Typed").field(arg0).finish(),
        }
    }
}
impl<Ty: Parsable> MappedParse for SelfParam<Ty> {
    type Source = Sum2<TypedSelf<Ty>, ShorthandSelf>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::Typed(a),
            Sum2::Val1(a) => Self::Shorthand(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

type FunctionParamPattern<T, Ty> = (PatternNoTopAlt<T, Ty>, Colon, Sum2<Ty, Elipsis>);

pub enum FunctionParam<T: Parsable, Ty: Parsable> {
    Patterned(PatternNoTopAlt<T, Ty>, Sum2<SmOut<Ty>, Elipsis>),
    Type(SmOut<Ty>),
    Elipsis,
}
impl<T: Parsable, Ty: Parsable> Debug for FunctionParam<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Patterned(arg0, arg1) => {
                f.debug_tuple("Patterned").field(arg0).field(arg1).finish()
            }
            Self::Type(arg0) => f.debug_tuple("Type").field(arg0).finish(),
            Self::Elipsis => write!(f, "Elipsis"),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for FunctionParam<T, Ty> {
    type Source = Sum3<MBox<FunctionParamPattern<T, Ty>>, Elipsis, MBox<Ty>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum3::Val0(a) => Self::Patterned(a.0, a.2),
            Sum3::Val1(_) => Self::Elipsis,
            Sum3::Val2(a) => Self::Type(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct FunctionReturnType<Ty: Parsable>(pub SmOut<Ty>);
impl<Ty: Parsable> Debug for FunctionReturnType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FunctionReturnType").field(&self.0).finish()
    }
}
impl<Ty: Parsable> MappedParse for FunctionReturnType<Ty> {
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

pub type Abi = StringLit;

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(it_matches_shorthand_self, SelfParam<Infallible>: self);
    insta_match_test!(it_matches_typed_self, SelfParam<TypePath<Ident>>: mut self: Box<Self>);

    insta_match_test!(it_matches_complex_function, Function<Infallible, Ident>: const async unsafe extern "C" fn hello<T>(self, a: T) -> T;);
}
