use super::*;
use crate::*;

pub struct ImplTraitType<T: Parsable>(pub SmOut<TypeParamBounds<T>>);
impl<T: Parsable> Debug for ImplTraitType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImplTraitType").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for ImplTraitType<T> {
    type Source = (KwImpl, TypeParamBounds<T>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct ImplTraitTypeOneBound<T: Parsable>(pub TraitBound<T>);
impl<T: Parsable> Debug for ImplTraitTypeOneBound<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImplTraitTypeOneBound")
            .field(&self.0)
            .finish()
    }
}
impl<T: Parsable> MappedParse for ImplTraitTypeOneBound<T> {
    type Source = (KwImpl, TraitBound<T>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TraitObjectType<T: Parsable>(pub SmOut<TypeParamBounds<T>>);
impl<T: Parsable> Debug for TraitObjectType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TraitObjectType").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for TraitObjectType<T> {
    type Source = (KwDyn, TypeParamBounds<T>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TraitObjectTypeOneBound<T: Parsable>(pub TraitBound<T>);
impl<T: Parsable> Debug for TraitObjectTypeOneBound<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TraitObjectTypeOneBound")
            .field(&self.0)
            .finish()
    }
}
impl<T: Parsable> MappedParse for TraitObjectTypeOneBound<T> {
    type Source = (KwDyn, TraitBound<T>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;
    use crate::*;

    insta_match_test!(it_matches_dyn_hello, TraitObjectTypeOneBound<Infallible>: dyn Hello);
    insta_match_test!(it_matches_impl_hello, ImplTraitTypeOneBound<Infallible>: impl Hello);

    insta_match_test!(it_matches_dyn_hello_type, TraitObjectType<Infallible>: dyn Hello);
    insta_match_test!(it_matches_impl_hello_type, ImplTraitType<Infallible>: impl Hello);
    insta_match_test!(
        it_matches_compound_dyn_type,
        TraitObjectType<Infallible>: dyn 'a + Hello
    );
    insta_match_test!(
        it_matches_compound_impl_type,
        ImplTraitType<Infallible>: impl 'a +Hello
    );
}
