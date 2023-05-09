use super::super::*;
use crate::*;

#[derive(Debug)]
pub struct PathInExpression {
    pub leading: bool,
    pub segments: Vec<PathExprSegment>,
}
impl MappedParse for PathInExpression {
    type Source = (
        Option<DoubleColon>,
        MinLength<Interlace<PathExprSegment, DoubleColon>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            leading: src.0.is_some(),
            segments: src.1 .0,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct PathExprSegment(pub PathIdentSegment, pub Option<GenericArgs>);
impl MappedParse for PathExprSegment {
    type Source = (PathIdentSegment, Option<(DoubleColon, GenericArgs)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.1.map(|v| v.1)))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
