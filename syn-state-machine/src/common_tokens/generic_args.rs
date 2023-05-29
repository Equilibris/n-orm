use super::*;
use crate::*;
use std::fmt::Debug;

pub struct GenericArgsBinding<T: Parsable>(pub Ident, pub Type<T>);
impl<T: Parsable> Debug for GenericArgsBinding<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericArgsBinding")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable> MappedParse for GenericArgsBinding<T> {
    type Source = (Identifier, Eq, Type<T>);

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

pub enum GenericArg<T: Parsable> {
    Lifetime(Lifetime),
    Type(Type<T>),
    GenericArgConst(GenericArgsConst), // TODO:
    ArgsBinding(GenericArgsBinding<T>),
}
impl<T: Parsable> Debug for GenericArg<T>
where
    SmOut<T>: Debug,
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
impl<T: Parsable> MappedParse for GenericArg<T> {
    type Source = Sum4<Lifetime, GenericArgsBinding<T>, Type<T>, GenericArgsConst>;

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

pub struct GenericArgs<T: Parsable>(pub Vec<GenericArg<T>>);
impl<T: Parsable> Debug for GenericArgs<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericArgs").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for GenericArgs<T> {
    type Source = (
        Lt,
        Option<(
            MinLength<Interlace<MBox<GenericArg<T>>, Comma>>,
            Option<Comma>,
        )>,
        Gt,
    );

    type Output = GenericArgs<T>;
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
