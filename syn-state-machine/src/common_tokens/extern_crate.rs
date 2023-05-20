use super::*;
use crate::*;

#[derive(Debug)]
pub struct ExternCrate {
    pub id: Ident,
    pub r#as: Option<Ident>,
}

pub type CrateRef = FlatSum2<Identifier, KwLowerSelf>;
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

    insta_match_test!(it_matches_extern, ExternCrate : extern crate hi;);
    insta_match_test!(it_matches_extern_as, ExternCrate : extern crate hi as _;);
}
