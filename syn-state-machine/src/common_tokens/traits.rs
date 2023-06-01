use super::*;
use crate::*;
use std::fmt::Debug;

pub struct Trait<T: Parsable, Ty: Parsable> {
    pub r#unsafe: bool,

    pub id: Ident,
    pub genetic_params: Option<GenericParams<T, Ty>>,
    pub bound: Option<TypeParamBounds<T, Ty>>,
    pub where_clause: Option<WhereClause<T, Ty>>,

    pub attrs: InnerAttrs<T>,
    pub associate_items: AssociateItems<T, Ty>,
}
impl<T: Parsable, Ty: Parsable> MappedParse for Trait<T, Ty> {
    type Source = (
        Option<KwUnsafe>,
        KwTrait,
        Identifier,
        Option<GenericParams<T, Ty>>,
        Option<(Colon, Option<TypeParamBounds<T, Ty>>)>,
        Option<WhereClause<T, Ty>>,
        Brace<WithInnerAttrs<T, AssociateItems<T, Ty>>>,
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
impl<T: Parsable, Ty: Parsable> Debug for Trait<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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
        it_matches_trait, Trait <Infallible, MBox<Type<Infallible>>>:
        unsafe trait HelloWorld<T> : From<T> where T: Sized {
            type Hello: World;
        }
    );
}
