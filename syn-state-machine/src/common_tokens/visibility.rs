use crate::*;

use super::*;

#[derive(Debug)]
pub enum Visibility {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    PubIn(SimplePath),
}

impl MappedParse for Visibility {
    type Source = Sum5<
        (KwPub, Paren<KwCrate>),
        (KwPub, Paren<KwLowerSelf>),
        (KwPub, Paren<KwSuper>),
        (KwPub, Paren<(KwIn, SimplePath)>),
        KwPub,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum5::Val0(_) => Self::PubCrate,
            Sum5::Val1(_) => Self::PubSelf,
            Sum5::Val2(_) => Self::PubSuper,
            Sum5::Val3((_, (_, a))) => Self::PubIn(a),
            Sum5::Val4(_) => Self::Pub,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(it_matches_pub, Visibility : pub);
    insta_match_test!(it_matches_pub_crate, Visibility : pub(crate));
    insta_match_test!(it_matches_pub_self, Visibility : pub(self));
    insta_match_test!(it_matches_pub_super, Visibility : pub(super));
    insta_match_test!(it_matches_pub_in, Visibility : pub(in super::super));
}
