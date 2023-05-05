use crate::*;

pub type WithAttrs<T, C> = (Attrs<T>, C);

pub type Attrs<T> = Vec<OuterAttr<T>>;
pub type InnerAttrs<T> = Vec<InnerAttr<T>>;

pub struct OuterAttr<T: Parsable = Vec<TokenTree>>(pub SmOutput<T>);

impl<T: Parsable> MappedParse for OuterAttr<T> {
    type Source = (FPunct<'#'>, Bracket<T>);

    type Output = Self;
    type Error = SmError<Self::Source>;

    fn map(src: SmOutput<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmError<<Self as MappedParse>::Source>) -> Self::Error {
        src
    }
}

pub struct InnerAttr<T: Parsable = Vec<TokenTree>>(pub SmOutput<T>);

impl<T: Parsable> MappedParse for InnerAttr<T> {
    type Source = (FPunct<'#'>, FPunct<'!'>, Bracket<T>);

    type Output = Self;
    type Error = SmError<Self::Source>;

    fn map(src: SmOutput<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(Self(src.2))
    }

    fn map_err(src: SmError<<Self as MappedParse>::Source>) -> Self::Error {
        src
    }
}
