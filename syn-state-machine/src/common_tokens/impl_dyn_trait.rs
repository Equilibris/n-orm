use super::*;
use crate::*;

#[derive(Debug)]
pub struct ImplTraitType(pub SmOut<TypeParamBounds>);
impl MappedParse for ImplTraitType {
    type Source = (KwImpl, TypeParamBounds);

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

#[derive(Debug)]
pub struct ImplTraitTypeOneBound(pub TraitBound);
impl MappedParse for ImplTraitTypeOneBound {
    type Source = (KwImpl, TraitBound);

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

#[derive(Debug)]
pub struct TraitObjectType(pub SmOut<TypeParamBounds>);
impl MappedParse for TraitObjectType {
    type Source = (KwDyn, TypeParamBounds);

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

#[derive(Debug)]
pub struct TraitObjectTypeOneBound(pub TraitBound);
impl MappedParse for TraitObjectTypeOneBound {
    type Source = (KwDyn, TraitBound);

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

    insta_match_test!(it_matches_dyn_hello, TraitObjectTypeOneBound: dyn Hello);
    insta_match_test!(it_matches_impl_hello, ImplTraitTypeOneBound: impl Hello);

    insta_match_test!(it_matches_dyn_hello_type, TraitObjectType: dyn Hello);
    insta_match_test!(it_matches_impl_hello_type, ImplTraitType: impl Hello);
    insta_match_test!(
        it_matches_compound_dyn_type,
        TraitObjectType: dyn 'a + Hello
    );
    insta_match_test!(
        it_matches_compound_impl_type,
        ImplTraitType: impl 'a +Hello
    );
}
