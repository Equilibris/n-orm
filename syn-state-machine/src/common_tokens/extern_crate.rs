use super::*;
use crate::*;

pub struct ExternCrate {
    pub id: Ident,
    pub r#as: Option<Ident>,
}

pub type CrateRef = FlatEither<Identifier, KwLowerSelf>;
pub type AsClause = Option<(KwAs, IdentifierOrUnder)>;

impl MappedParse for ExternCrate {
    type Source = (KwExtern, KwCrate, CrateRef, AsClause, Semi);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.2,
            r#as: src.3.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    use quote::quote;

    #[test]
    fn it_matches() {
        parse_terminal::<ExternCrate>(quote! { extern crate self as _; }).unwrap();
        parse_terminal::<ExternCrate>(quote! { extern crate hi; }).unwrap();
    }
}
