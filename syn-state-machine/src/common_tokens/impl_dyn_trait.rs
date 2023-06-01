use super::*;
use crate::*;

pub struct ImplTraitType<T: Parsable, Ty: Parsable>(pub SmOut<TypeParamBounds<T, Ty>>);
impl<T: Parsable, Ty: Parsable> Debug for ImplTraitType<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImplTraitType").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for ImplTraitType<T, Ty> {
    type Source = (KwImpl, TypeParamBounds<T, Ty>);

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

pub struct ImplTraitTypeOneBound<T: Parsable, Ty: Parsable>(pub TraitBound<T, Ty>);
impl<T: Parsable, Ty: Parsable> Debug for ImplTraitTypeOneBound<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImplTraitTypeOneBound")
            .field(&self.0)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for ImplTraitTypeOneBound<T, Ty> {
    type Source = (KwImpl, TraitBound<T, Ty>);

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

pub struct TraitObjectType<T: Parsable, Ty: Parsable>(pub SmOut<TypeParamBounds<T, Ty>>);
impl<T: Parsable, Ty: Parsable> Debug for TraitObjectType<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TraitObjectType").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TraitObjectType<T, Ty> {
    type Source = (KwDyn, TypeParamBounds<T, Ty>);

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

pub struct TraitObjectTypeOneBound<T: Parsable, Ty: Parsable>(pub TraitBound<T, Ty>);
impl<T: Parsable, Ty: Parsable> Debug for TraitObjectTypeOneBound<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TraitObjectTypeOneBound")
            .field(&self.0)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TraitObjectTypeOneBound<T, Ty> {
    type Source = (KwDyn, TraitBound<T, Ty>);

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
    use super::*;

    insta_match_test!(it_matches_dyn_hello, TraitObjectTypeOneBound<Infallible, Infallible>: dyn Hello);
    insta_match_test!(it_matches_impl_hello, ImplTraitTypeOneBound<Infallible, Infallible>: impl Hello);

    insta_match_test!(it_matches_dyn_hello_type, TraitObjectType<Infallible, Infallible>: dyn Hello);
    insta_match_test!(it_matches_impl_hello_type, ImplTraitType<Infallible, Infallible>: impl Hello);
    insta_match_test!(
        it_matches_compound_dyn_type,
        TraitObjectType<Infallible, Infallible>: dyn 'a + Hello
    );
    insta_match_test!(
        it_matches_compound_impl_type,
        ImplTraitType<Infallible, Infallible>: impl 'a +Hello
    );
}
