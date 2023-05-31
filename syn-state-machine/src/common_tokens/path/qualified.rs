use super::super::*;
use crate::*;

pub struct QualifiedPathType<Ty: Parsable> {
    pub ty: SmOut<Ty>,
    pub r#as: Option<TypePath<Ty>>,
}
impl<Ty: Parsable> Debug for QualifiedPathType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QualifiedPathType")
            .field("ty", &self.ty)
            .field("as", &self.r#as)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for QualifiedPathType<Ty> {
    type Source = (Lt, Ty, Option<(KwAs, TypePath<Ty>)>, Gt);

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

pub struct QualifiedPathInType<Ty: Parsable>(
    pub QualifiedPathType<Ty>,
    pub Vec<TypePathSegment<Ty>>,
);
impl<Ty: Parsable> Debug for QualifiedPathInType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("QualifiedPathInType")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for QualifiedPathInType<Ty> {
    type Source = (
        QualifiedPathType<Ty>,
        Vec<(DoubleColon, TypePathSegment<Ty>)>,
    );

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

pub struct QualifiedPathInExpression<Ty: Parsable>(
    pub QualifiedPathType<Ty>,
    pub Vec<PathExprSegment<Ty>>,
);
impl<Ty: Parsable> Debug for QualifiedPathInExpression<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("QualifiedPathInExpression")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for QualifiedPathInExpression<Ty> {
    type Source = (
        QualifiedPathType<Ty>,
        Vec<(DoubleColon, PathExprSegment<Ty>)>,
    );

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
    use super::*;

    insta_match_test!(it_matches_simple_paths, QualifiedPathInType<Ident> : <hello as Default>::Default);
}
