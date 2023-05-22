use super::*;
use crate::*;
use std::fmt::Debug;

pub struct Trait<T: Parsable> {
    pub r#unsafe: bool,

    pub id: Ident,
    pub genetic_params: Option<GenericParams<T>>,
    pub bound: Option<TypeParamBounds<T>>,
    pub where_clause: Option<WhereClause<T>>,

    pub attrs: InnerAttrs<T>,
    pub associate_items: AssociateItems<T>,
}
impl<T: Parsable> MappedParse for Trait<T> {
    type Source = (
        Option<KwUnsafe>,
        KwTrait,
        Identifier,
        Option<GenericParams<T>>,
        Option<(Colon, Option<TypeParamBounds<T>>)>,
        Option<WhereClause<T>>,
        Brace<WithInnerAttrs<T, AssociateItems<T>>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#unsafe: src.0.is_some(),
            id: src.2,
            genetic_params: src.3,
            where_clause: src.5,
            bound: src.4.and_then(|v| v.1),
            attrs: src.6 .0 .0,
            associate_items: src.6 .0 .1,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable> Debug for Trait<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trait")
            .field("unsafe", &self.r#unsafe)
            .field("id", &self.id)
            .field("genetic_params", &self.genetic_params)
            .field("bound", &self.bound)
            .field("where_clause", &self.where_clause)
            .field("attrs", &self.attrs)
            .field("associate_items", &self.associate_items)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(
        it_matches_trait, Trait <Infallible>:
        unsafe trait HelloWorld<T> : From<T> where T: Sized {
            type Hello: World;
        }
    );
}
