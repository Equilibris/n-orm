use super::*;
use crate::*;
use std::fmt::Debug;

pub struct WhereClause<T: Parsable, Ty: Parsable>(pub Vec<WhereClauseItem<T, Ty>>);
impl<T: Parsable, Ty: Parsable> Debug for WhereClause<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WhereClause").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for WhereClause<T, Ty> {
    type Source = (KwWhere, Interlace<WhereClauseItem<T, Ty>, Comma>);

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

pub enum WhereClauseItem<T: Parsable, Ty: Parsable> {
    LifetimeWhereClauseItem(LifetimeWhereClauseItem),
    TypeBoundWhereClauseItem(TypeBoundWhereClauseItem<T, Ty>),
}
impl<T: Parsable, Ty: Parsable> Debug for WhereClauseItem<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LifetimeWhereClauseItem(arg0) => f
                .debug_tuple("LifetimeWhereClauseItem")
                .field(arg0)
                .finish(),
            Self::TypeBoundWhereClauseItem(arg0) => f
                .debug_tuple("TypeBoundWhereClauseItem")
                .field(arg0)
                .finish(),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for WhereClauseItem<T, Ty> {
    type Source = Sum2<LifetimeWhereClauseItem, TypeBoundWhereClauseItem<T, Ty>>;

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

pub struct TypeBoundWhereClauseItem<T: Parsable, Ty: Parsable> {
    pub r#for: Option<ForLifetimes<T, Ty>>,
    pub ty: SmOut<Ty>,
    pub bounds: Option<TypeParamBounds<T, Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for TypeBoundWhereClauseItem<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeBoundWhereClauseItem")
            .field("for", &self.r#for)
            .field("ty", &self.ty)
            .field("bounds", &self.bounds)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TypeBoundWhereClauseItem<T, Ty> {
    type Source = (
        Option<ForLifetimes<T, Ty>>,
        Ty,
        Colon,
        Option<TypeParamBounds<T, Ty>>,
    );

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
