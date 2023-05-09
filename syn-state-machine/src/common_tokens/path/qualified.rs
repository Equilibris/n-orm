use super::super::*;
use crate::*;

#[derive(Debug)]
pub struct QualifiedPathType {
    pub ty: Type,
    pub r#as: Option<TypePath>,
}
impl MappedParse for QualifiedPathType {
    type Source = (Lt, Type, Option<(KwAs, TypePath)>, Gt);

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

#[derive(Debug)]
pub struct QualifiedPathInType(pub QualifiedPathType, pub Vec<TypePathSegment>);
impl MappedParse for QualifiedPathInType {
    type Source = (QualifiedPathType, Vec<(DoubleColon, TypePathSegment)>);

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

#[derive(Debug)]
pub struct QualifiedPathInExpression(pub QualifiedPathType, pub Vec<PathExprSegment>);
impl MappedParse for QualifiedPathInExpression {
    type Source = (QualifiedPathType, Vec<(DoubleColon, PathExprSegment)>);

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
