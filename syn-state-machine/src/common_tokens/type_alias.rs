use super::*;
use crate::*;
use std::fmt::Debug;

pub struct TypeAlias<T: Parsable> {
    pub id: Ident,
    pub params: Option<GenericParams<T>>,
    pub bounds: Option<TypeParamBounds<T>>,
    pub pre_where_clause: Option<WhereClause<T>>,

    pub ty: Option<Type<T>>,
    pub post_where_clause: Option<WhereClause<T>>,
}
impl<T: Parsable> Debug for TypeAlias<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeAlias")
            .field("id", &self.id)
            .field("params", &self.params)
            .field("bounds", &self.bounds)
            .field("pre_where_clause", &self.pre_where_clause)
            .field("ty", &self.ty)
            .field("post_where_clause", &self.post_where_clause)
            .finish()
    }
}
impl<T: Parsable> MappedParse for TypeAlias<T> {
    type Source = (
        KwType,
        Identifier,
        Option<GenericParams<T>>,
        Option<(Colon, TypeParamBounds<T>)>,
        Option<WhereClause<T>>,
        Option<(Eq, Type<T>, Option<WhereClause<T>>)>,
        Semi,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        let (ty, post_where_clause) = match src.5 {
            Some((_, a, b)) => (Some(a), b),
            None => (None, None),
        };

        Ok(Self {
            id: src.1,
            params: src.2,
            bounds: src.3.map(|v| v.1),
            pre_where_clause: src.4,
            ty,
            post_where_clause,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::TypeAlias;
    use crate::insta_match_test;
    use std::convert::Infallible;

    insta_match_test!(it_matches_simple, TypeAlias<Infallible>: type Point = (u8, u8););
    insta_match_test!(it_matches_complex, TypeAlias<Infallible>: type Point<T> where T: std::ops::Add = (T, T););
}
