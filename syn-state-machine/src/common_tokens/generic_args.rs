use super::*;
use crate::*;
use std::fmt::Debug;

// Not used externally
pub struct GenericArgsBinding<Ty: Parsable>(pub Ident, pub SmOut<Ty>);
impl<Ty: Parsable> Debug for GenericArgsBinding<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericArgsBinding")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for GenericArgsBinding<Ty> {
    type Source = (Identifier, Eq, Ty);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.2))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// Not used externally
#[derive(Debug)]
pub enum GenericArgsConst {
    BlockExpression(BlockExpression),
    LiteralExpression(Literal),
    NegLiteralExpression(Literal),
    SimplePathSegment(SimplePathSegment),
}
impl MappedParse for GenericArgsConst {
    type Source = Sum4<BlockExpression, Literal, (Minus, Literal), SimplePathSegment>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum4::Val0(a) => Self::BlockExpression(a),
            Sum4::Val1(a) => Self::LiteralExpression(a),
            Sum4::Val2((_, a)) => Self::NegLiteralExpression(a),
            Sum4::Val3(a) => Self::SimplePathSegment(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// Not used externally
pub enum GenericArg<Ty: Parsable> {
    Lifetime(Lifetime),
    Type(SmOut<Ty>),
    GenericArgConst(GenericArgsConst), // TODO:
    ArgsBinding(GenericArgsBinding<Ty>),
}
impl<Ty: Parsable> Debug for GenericArg<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lifetime(arg0) => f.debug_tuple("Lifetime").field(arg0).finish(),
            Self::Type(arg0) => f.debug_tuple("Type").field(arg0).finish(),
            Self::GenericArgConst(arg0) => f.debug_tuple("GenericArgConst").field(arg0).finish(),
            Self::ArgsBinding(arg0) => f.debug_tuple("ArgsBinding").field(arg0).finish(),
        }
    }
}
impl<Ty: Parsable> MappedParse for GenericArg<Ty> {
    type Source = Sum4<Lifetime, GenericArgsBinding<Ty>, Ty, GenericArgsConst>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum4::Val0(a) => Self::Lifetime(a),
            Sum4::Val1(a) => Self::ArgsBinding(a),
            Sum4::Val2(a) => Self::Type(a),
            Sum4::Val3(a) => Self::GenericArgConst(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// Used deeply in path
pub struct GenericArgs<Ty: Parsable>(pub Vec<GenericArg<Ty>>);
impl<Ty: Parsable> Debug for GenericArgs<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericArgs").field(&self.0).finish()
    }
}
impl<Ty: Parsable> MappedParse for GenericArgs<Ty> {
    type Source = (
        Lt,
        Option<(
            MinLength<Interlace<MBox<GenericArg<Ty>>, Comma>>,
            Option<Comma>,
        )>,
        Gt,
    );

    type Output = GenericArgs<Ty>;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1.map(|v| v.0 .0).unwrap_or_default()))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    insta_match_test!(it_matches_empty_generic_args, GenericArgs<Infallible>: <>);
    insta_match_test!(it_matches_lifetime_args, GenericArgs<Infallible>: <'a>);
    insta_match_test!(it_matches_typed_args, GenericArgs<Ident>: <T>);
    insta_match_test!(it_matches_pathed_args, GenericArgs<TypePath<Infallible>>: <hello::world>);
    insta_match_test!(it_matches_multi_args, GenericArgs<TypePath<Infallible>>: <'a, T, hello::world>);
}
