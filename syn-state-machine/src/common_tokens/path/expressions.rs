use super::super::*;
use crate::*;

pub struct PathInExpression<T: Parsable> {
    pub leading: bool,
    pub segments: Vec<PathExprSegment<T>>,
}
impl<T: Parsable> Debug for PathInExpression<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathInExpression")
            .field("leading", &self.leading)
            .field("segments", &self.segments)
            .finish()
    }
}
impl<T: Parsable> MappedParse for PathInExpression<T> {
    type Source = (
        Option<DoubleColon>,
        MinLength<Interlace<PathExprSegment<T>, DoubleColon>>,
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

pub struct PathExprSegment<T: Parsable>(pub PathIdentSegment, pub Option<GenericArgs<T>>);
impl<T: Parsable> Debug for PathExprSegment<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PathExprSegment")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable> MappedParse for PathExprSegment<T> {
    type Source = (PathIdentSegment, Option<(DoubleColon, GenericArgs<T>)>);

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
