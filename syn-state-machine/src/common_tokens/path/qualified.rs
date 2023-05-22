use super::super::*;
use crate::*;

pub struct QualifiedPathType<T: Parsable> {
    pub ty: Type<T>,
    pub r#as: Option<TypePath<T>>,
}
impl<T: Parsable> Debug for QualifiedPathType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QualifiedPathType")
            .field("ty", &self.ty)
            .field("as", &self.r#as)
            .finish()
    }
}
impl<T: Parsable> MappedParse for QualifiedPathType<T> {
    type Source = (Lt, Type<T>, Option<(KwAs, TypePath<T>)>, Gt);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            ty: src.1,
            r#as: src.2.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct QualifiedPathInType<T: Parsable>(pub QualifiedPathType<T>, pub Vec<TypePathSegment<T>>);
impl<T: Parsable> Debug for QualifiedPathInType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("QualifiedPathInType")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable> MappedParse for QualifiedPathInType<T> {
    type Source = (QualifiedPathType<T>, Vec<(DoubleColon, TypePathSegment<T>)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(
            src.0,
            src.1.into_iter().map(|v| v.1).collect::<Vec<_>>(),
        ))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct QualifiedPathInExpression<T: Parsable>(
    pub QualifiedPathType<T>,
    pub Vec<PathExprSegment<T>>,
);
impl<T: Parsable> Debug for QualifiedPathInExpression<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("QualifiedPathInExpression")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable> MappedParse for QualifiedPathInExpression<T> {
    type Source = (QualifiedPathType<T>, Vec<(DoubleColon, PathExprSegment<T>)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.1.into_iter().map(|v| v.1).collect()))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;
    use crate::parse_terminal;

    insta_match_test!(it_matches_simple_paths, QualifiedPathInType<Infallible> : <hello as Default>::Default);
}
