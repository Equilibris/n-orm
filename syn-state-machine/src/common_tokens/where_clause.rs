use super::*;
use crate::*;

#[derive(Debug)]
pub struct WhereClause(pub Vec<WhereClauseItem>);
impl MappedParse for WhereClause {
    type Source = (KwWhere, Interlace<WhereClauseItem, Comma>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub enum WhereClauseItem {
    LifetimeWhereClauseItem(LifetimeWhereClauseItem),
    TypeBoundWhereClauseItem(TypeBoundWhereClauseItem),
}
impl MappedParse for WhereClauseItem {
    type Source = Sum2<LifetimeWhereClauseItem, TypeBoundWhereClauseItem>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::LifetimeWhereClauseItem(a),
            Sum2::Val1(a) => Self::TypeBoundWhereClauseItem(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct LifetimeWhereClauseItem {
    pub lifetime: Ident,
    pub bounds: LifetimeBounds,
}
impl MappedParse for LifetimeWhereClauseItem {
    type Source = (Lifetime, Colon, LifetimeBounds);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            lifetime: src.0 .0,
            bounds: src.2,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct TypeBoundWhereClauseItem {
    pub r#for: Option<ForLifetimes>,
    pub ty: Type,
    pub bounds: Option<TypeParamBounds>,
}
impl MappedParse for TypeBoundWhereClauseItem {
    type Source = (Option<ForLifetimes>, Type, Colon, Option<TypeParamBounds>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#for: src.0,
            ty: src.1,
            bounds: src.3,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
