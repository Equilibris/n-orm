use super::*;
use crate::*;
use std::fmt::Debug;

pub enum Implementation<T: Parsable, Ty: Parsable> {
    Inherent(InherentImpl<T, Ty>),
    Trait(TraitImpl<T, Ty>),
}
impl<T: Parsable, Ty: Parsable> Debug for Implementation<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inherent(arg0) => f.debug_tuple("Inherent").field(arg0).finish(),
            Self::Trait(arg0) => f.debug_tuple("Trait").field(arg0).finish(),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for Implementation<T, Ty> {
    type Source = Sum2<InherentImpl<T, Ty>, TraitImpl<T, Ty>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::Inherent(a),
            Sum2::Val1(a) => Self::Trait(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TraitImpl<T: Parsable, Ty: Parsable> {
    pub r#unsafe: bool,
    pub genetic_params: Option<GenericParams<T>>,
    pub where_clause: Option<WhereClause<T, Ty>>,
    pub neg: bool,
    pub r#trait: TypePath<T>,
    pub ty: SmOut<Ty>,

    pub attrs: InnerAttrs<T>,
    pub items: AssociateItems<T, Ty>,
}

impl<T: Parsable, Ty: Parsable> Debug for TraitImpl<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TraitImpl")
            .field("unsafe", &self.r#unsafe)
            .field("genetic_params", &self.genetic_params)
            .field("neg", &self.neg)
            .field("trait", &self.r#trait)
            .field("ty", &self.ty)
            .field("attrs", &self.attrs)
            .field("items", &self.items)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TraitImpl<T, Ty> {
    type Source = (
        Option<KwUnsafe>,
        KwImpl,
        Option<MBox<GenericParams<T>>>,
        Option<Exclamation>,
        MBox<TypePath<T>>,
        KwFor,
        Ty,
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
            genetic_params: src.2,
            where_clause: src.7,
            neg: src.3.is_some(),
            r#trait: src.4,
            ty: src.6,
            attrs: src.8 .0 .0,
            items: src.8 .0 .1,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct InherentImpl<T: Parsable, Ty: Parsable> {
    genetic_params: Option<GenericParams<T>>,
    ty: SmOut<Ty>,
    where_clause: Option<WhereClause<T, Ty>>,

    attrs: InnerAttrs<T>,
    items: AssociateItems<T, Ty>,
}
impl<T: Parsable, Ty: Parsable> MappedParse for InherentImpl<T, Ty> {
    type Source = (
        KwImpl,
        Option<GenericParams<T>>,
        Ty,
        Option<WhereClause<T, Ty>>,
        Brace<WithInnerAttrs<T, AssociateItems<T, Ty>>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            genetic_params: src.1,
            ty: src.2,
            where_clause: src.3,
            attrs: src.4 .0 .0,
            items: src.4 .0 .1,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable, Ty: Parsable> Debug for InherentImpl<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InherentImpl")
            .field("genetic_params", &self.genetic_params)
            .field("ty", &self.ty)
            .field("where_clause", &self.where_clause)
            .field("attrs", &self.attrs)
            .field("items", &self.items)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(
        it_matches_simple_inherent, Implementation <Infallible, Type<Infallible>>:

        impl<T> Option<T> {
            pub fn is_some(&self) -> bool;
        }
    );
    insta_match_test!(
        it_matches_simple_trait, Implementation <Infallible, Type<Infallible>>:

        unsafe impl<T: Copy> Copy for Option<T> {}
    );
}
