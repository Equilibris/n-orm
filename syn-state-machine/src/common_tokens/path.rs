mod expressions;
mod qualified;
mod simple;
mod types;

pub use expressions::*;
pub use qualified::*;
pub use simple::*;
pub use std::fmt::Debug;
pub use types::*;

use super::*;
use crate::*;

#[derive(Debug)]
pub enum PathIdentSegment {
    Id(Ident),
    DCrate,
}
impl MappedParse for PathIdentSegment {
    type Source =
        Sum2<FlatSum5<Identifier, KwSuper, KwLowerSelf, KwCrate, KwUpperSelf>, DollarCrate>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(v) => Self::Id(v),
            Sum2::Val1(_) => Self::DCrate,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

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

pub enum PathExpression<T: Parsable> {
    PathInExpression(PathInExpression<T>),
    QualifiedPathInExpression(QualifiedPathInExpression<T>),
}
impl<T: Parsable> Debug for PathExpression<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathInExpression(arg0) => f.debug_tuple("PathInExpression").field(arg0).finish(),
            Self::QualifiedPathInExpression(arg0) => f
                .debug_tuple("QualifiedPathInExpression")
                .field(arg0)
                .finish(),
        }
    }
}
impl<T: Parsable> MappedParse for PathExpression<T> {
    type Source = Sum2<PathInExpression<T>, QualifiedPathInExpression<T>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::PathInExpression(a),
            Sum2::Val1(a) => Self::QualifiedPathInExpression(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    insta_match_test!(it_matches_hello, TypePath<Infallible>: hello);
    insta_match_test!(it_matches_hello_world, TypePath<Infallible>: hello::world);
    insta_match_test!(it_matches_hello_world_hi, TypePath<Infallible>: hello::world::hi);

    insta_match_test!(it_matches_empty_generic_args, GenericArgs<Infallible>: <>);
    insta_match_test!(it_matches_lifetime_args, GenericArgs<Infallible>: <'a>);
    insta_match_test!(it_matches_typed_args, GenericArgs<Infallible>: <T>);
    insta_match_test!(it_matches_pathed_args, GenericArgs<Infallible>: <hello::world>);
    insta_match_test!(it_matches_multi_args, GenericArgs<Infallible>: <'a, T, hello::world>);

    insta_match_test!(
        it_matches_expr_path,
        PathInExpression<Infallible>: usize::hello::<Hello, World>
    );
}
