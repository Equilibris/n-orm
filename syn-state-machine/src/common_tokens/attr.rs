use crate::*;

pub type WithAttrs<T, C> = (Attrs<T>, C);
pub type WithInnerAttrs<T, C> = (InnerAttrs<T>, C);

pub type Attrs<T> = Vec<OuterAttr<T>>;
pub type InnerAttrs<T> = Vec<InnerAttr<T>>;

pub struct OuterAttr<T: Parsable = Vec<TokenTree>>(pub SmOut<T>);

impl<T: Parsable> std::fmt::Debug for OuterAttr<T>
where
    SmOut<T>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OuterAttr").field(&self.0).finish()
    }
}

impl<T: Parsable> MappedParse for OuterAttr<T> {
    type Source = (FPunct<'#'>, Bracket<T>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(src: SmOut<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<<Self as MappedParse>::Source>) -> Self::Error {
        src
    }
}

pub struct InnerAttr<T: Parsable = Vec<TokenTree>>(pub SmOut<T>);
impl<T: Parsable> std::fmt::Debug for InnerAttr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OuterAttr").finish()
    }
}

impl<T: Parsable> MappedParse for InnerAttr<T> {
    type Source = (FPunct<'#'>, FPunct<'!'>, Bracket<T>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(src: SmOut<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(Self(src.2))
    }

    fn map_err(src: SmErr<<Self as MappedParse>::Source>) -> Self::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(it_matches_simple_function, OuterAttr<(Ident, Paren<Ident>)>: #[hello(world)]);
}
