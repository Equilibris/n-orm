use super::*;
use crate::*;

pub struct ExternCrate {
    pub id: Ident,
    pub r#as: Option<Ident>,
}

impl MappedParse for ExternCrate {
    type Source = (
        KwExtern,
        KwCrate,
        FlatEither<Identifier, KwLowerSelf>,
        Option<(KwAs, IdentifierUnder)>,
        Semi,
    );

    type Output = Self;
    type Error = SmError<Self::Source>;

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.2,
            r#as: src.3.map(|v| v.1),
        })
    }

    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
