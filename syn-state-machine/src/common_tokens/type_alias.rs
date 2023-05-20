use super::*;
use crate::*;

#[derive(Debug)]
pub struct TypeAlias {
    pub id: Ident,
    pub params: Option<GenericParams>,
    pub bounds: Option<TypeParamBounds>,
    pub pre_where_clause: Option<WhereClause>,

    pub ty: Option<Type>,
    pub post_where_clause: Option<WhereClause>,
}
impl MappedParse for TypeAlias {
    type Source = (
        KwType,
        Identifier,
        Option<GenericParams>,
        Option<(Colon, TypeParamBounds)>,
        Option<WhereClause>,
        Option<(Eq, Type, Option<WhereClause>)>,
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

    insta_match_test!(it_matches_simple, TypeAlias: type Point = (u8, u8););
    insta_match_test!(it_matches_complex, TypeAlias: type Point<T> where T: std::ops::Add = (T, T););
}
