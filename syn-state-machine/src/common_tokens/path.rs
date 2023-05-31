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

pub enum PathExpression<Ty: Parsable> {
    PathInExpression(PathInExpression<Ty>),
    QualifiedPathInExpression(QualifiedPathInExpression<Ty>),
}
impl<Ty: Parsable> Debug for PathExpression<Ty>
where
    SmOut<Ty>: Debug,
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
impl<Ty: Parsable> MappedParse for PathExpression<Ty> {
    type Source = Sum2<PathInExpression<Ty>, QualifiedPathInExpression<Ty>>;

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
}
