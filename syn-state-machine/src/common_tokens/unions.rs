use super::*;
use crate::*;
use std::fmt::Debug;

pub struct Union<T: Parsable, Ty: Parsable> {
    pub id: Ident,
    pub genetic_params: Option<GenericParams<T>>,
    pub where_clause: Option<WhereClause<T>>,
    pub fields: StructFields<T, Ty>,
}
impl<T: Parsable, Ty: Parsable> MappedParse for Union<T, Ty> {
    type Source = (
        KwUnion,
        Ident,
        Option<GenericParams<T>>,
        Option<WhereClause<T>>,
        Brace<MinLength<StructFields<T, Ty>>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            genetic_params: src.2,
            where_clause: src.3,
            fields: src.4 .0,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable, Ty: Parsable> Debug for Union<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Union")
            .field("id", &self.id)
            .field("genetic_params", &self.genetic_params)
            .field("where_clause", &self.where_clause)
            .field("fields", &self.fields)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(it_matches_union, Union <Infallible,Ident>: union MyUnion {
        f1: u32,
        f2: f32,
    });
}
