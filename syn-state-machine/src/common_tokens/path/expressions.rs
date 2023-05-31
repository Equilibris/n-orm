use super::super::*;
use crate::*;

pub struct PathInExpression<Ty: Parsable> {
    pub leading: bool,
    pub segments: Vec<PathExprSegment<Ty>>,
}
impl<Ty: Parsable> Debug for PathInExpression<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathInExpression")
            .field("leading", &self.leading)
            .field("segments", &self.segments)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for PathInExpression<Ty> {
    type Source = (
        Option<DoubleColon>,
        MinLength<Interlace<PathExprSegment<Ty>, DoubleColon>>,
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

// Used in qualified
pub struct PathExprSegment<Ty: Parsable>(pub PathIdentSegment, pub Option<GenericArgs<Ty>>);
impl<Ty: Parsable> Debug for PathExprSegment<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PathExprSegment")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for PathExprSegment<Ty> {
    type Source = (PathIdentSegment, Option<(DoubleColon, GenericArgs<Ty>)>);

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

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(
        it_matches_expr_path,
        PathInExpression<Ident>: usize::hello::<Hello, World>
    );
}
