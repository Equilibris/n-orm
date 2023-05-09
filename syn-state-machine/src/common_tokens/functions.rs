use either::Either::Left;
use either::Either::Right;

use super::*;
use crate::*;

#[derive(Debug)]
pub struct Function<T: Parsable = Tokens> {
    pub qualifiers: FunctionQualifiers,

    pub ident: Ident,

    pub generic_params: Option<GenericParams>,
    pub where_clause: Option<WhereClause>,

    pub self_param: Option<WithAttrs<T, SelfParam>>,
    pub args: Vec<WithAttrs<T, FunctionParam>>,

    pub returns: Option<Type>,
    pub body: Option<BlockExpression>,
}
impl<T: Parsable> MappedParse for Function<T> {
    type Source = (
        FunctionQualifiers,
        KwFn,
        Identifier,
        Option<GenericParams>,
        Paren<FunctionParameters<T>>,
        Option<FunctionReturnType>,
        Option<WhereClause>,
        Either<BlockExpression, Semi>,
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

            self_param: src.4.self_param,
            args: src.4.params,
            returns: src.5.map(|v| v.1),
            body: src.7.left(),
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

#[derive(Debug)]
pub struct FunctionParameters<T: Parsable = Tokens> {
    pub self_param: Option<WithAttrs<T, SelfParam>>,

    pub params: Vec<WithAttrs<T, FunctionParam>>,
}
impl<T: Parsable> MappedParse for FunctionParameters<T> {
    type Source = Either<
        (
            Option<(WithAttrs<T, SelfParam>, Comma)>,
            Interlace<WithAttrs<T, FunctionParam>, Comma>,
        ),
        (WithAttrs<T, SelfParam>, Option<Comma>),
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(a) => Self {
                self_param: a.0.map(|v| v.0),
                params: a.1 .0,
            },
            Right(a) => Self {
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

#[derive(Debug)]
pub struct TypedSelf {
    pub is_mut: bool,
    pub ty: Type,
}
impl MappedParse for TypedSelf {
    type Source = (Option<KwMut>, KwLowerSelf, Colon, Type);

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

#[derive(Debug)]
pub enum SelfParam {
    Shorthand(ShorthandSelf),
    Typed(TypedSelf),
}
impl MappedParse for SelfParam {
    type Source = Either<TypedSelf, ShorthandSelf>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(a) => Self::Typed(a),
            Right(a) => Self::Shorthand(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

type FunctionParamPattern = (PatternNoTopAlt, Colon, Either<Type, Elipsis>);

#[derive(Debug)]
pub enum FunctionParam {
    Patterned(PatternNoTopAlt, Either<Type, Elipsis>),
    Type(Type),
    Elipsis,
}
impl MappedParse for FunctionParam {
    type Source = Either<Either<FunctionParamPattern, Elipsis>, Type>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(Left(a)) => Self::Patterned(a.0, a.2),
            Left(Right(_)) => Self::Elipsis,
            Right(a) => Self::Type(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type FunctionReturnType = (Arrow, Type);

pub type Abi = StringLit;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_terminal;

    #[test]
    fn it_matches_self() {
        println!(
            "{:#?}",
            parse_terminal::<SelfParam>(quote::quote!(self)).unwrap()
        );

        println!(
            "{:#?}",
            parse_terminal::<TypedSelf>(quote::quote!(mut self: Box<Self>)).unwrap()
        );
    }
    #[test]
    fn it_matches_simple_funs() {
        println!(
            "{:#?}",
            parse_terminal::<(
                // FunctionQualifiers,
                // KwFn,
                // Identifier,
                // Option<GenericParams>,
                FunctionParameters,
                // Option<FunctionReturnType>,
                // Option<WhereClause>,
                // Either<BlockExpression, Semi>,
            )>(quote::quote! {self})
            .unwrap()
        );
    }
}
