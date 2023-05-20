use super::*;
use crate::*;
use std::fmt::Debug;

pub struct Union<T: Parsable = Tokens> {
    pub id: Ident,
    pub genetic_params: Option<GenericParams>,
    pub where_clause: Option<WhereClause>,
    pub fields: StructFields<T>,
}
impl<T: Parsable> MappedParse for Union<T> {
    type Source = (
        KwUnion,
        Ident,
        Option<GenericParams>,
        Option<WhereClause>,
        Brace<MinLength<StructFields<T>>>,
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
            fields: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable> Debug for Union<T>
where
    SmOut<T>: Debug,
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

    insta_match_test!(it_matches_union, Union : union MyUnion {
        f1: u32,
        f2: f32,
    });
}
