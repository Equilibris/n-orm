mod expressions;
mod qualified;
mod simple;
mod types;

pub use expressions::*;
pub use qualified::*;
pub use simple::*;
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

#[derive(Debug)]
pub struct GenericArgsBinding(pub Ident, pub Type);
impl MappedParse for GenericArgsBinding {
    type Source = (Identifier, Eq, Type);

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

#[derive(Debug)]
pub enum GenericArg {
    Lifetime(Lifetime),
    Type(Type),
    GenericArgConst(GenericArgsConst), // TODO:
    ArgsBinding(GenericArgsBinding),
}
impl MappedParse for GenericArg {
    type Source = Sum4<Lifetime, GenericArgsBinding, Type, GenericArgsConst>;

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

#[derive(Debug)]
pub struct GenericArgs(pub Vec<GenericArg>);
impl MappedParse for GenericArgs {
    type Source = (
        Lt,
        Option<(MinLength<Interlace<MBox<GenericArg>, Comma>>, Option<Comma>)>,
        Gt,
    );

    type Output = GenericArgs;
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

#[derive(Debug)]
pub enum PathExpression {
    PathInExpression(PathInExpression),
    QualifiedPathInExpression(QualifiedPathInExpression),
}
impl MappedParse for PathExpression {
    type Source = Sum2<PathInExpression, QualifiedPathInExpression>;

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

    insta_match_test!(it_matches_hello, TypePath: hello);
    insta_match_test!(it_matches_hello_world, TypePath: hello::world);
    insta_match_test!(it_matches_hello_world_hi, TypePath: hello::world::hi);

    insta_match_test!(it_matches_empty_generic_args, GenericArgs: <>);
    insta_match_test!(it_matches_lifetime_args, GenericArgs: <'a>);
    insta_match_test!(it_matches_typed_args, GenericArgs: <T>);
    insta_match_test!(it_matches_pathed_args, GenericArgs: <hello::world>);
    insta_match_test!(it_matches_multi_args, GenericArgs: <'a, T, hello::world>);

    insta_match_test!(
        it_matches_expr_path,
        PathInExpression: usize::hello::<Hello, World>
    );
}
