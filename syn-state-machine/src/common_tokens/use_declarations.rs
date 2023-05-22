use super::*;
use crate::*;

#[derive(Debug)]
pub enum UseTree {
    Star(SimplePathOrNone),
    Recursion {
        name: SimplePathOrNone,
        deep: Vec<UseTree>,
    },
    Standard(SimplePath, Option<Ident>),
}

impl MappedParse for UseTree {
    type Source = Sum2<
        (
            Option<(SimplePathOrNone, DoubleColon)>,
            Brace<(Interlace<UseTree, Comma>, Option<Comma>)>,
        ),
        Sum2<(Option<(SimplePathOrNone, DoubleColon)>, Star), (SimplePath, AsClause)>,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        use Sum2::*;

        Ok(match src {
            Val1(Val1(a)) => Self::Standard(a.0, a.1.map(|(_, a)| a)),
            Val0(a) => Self::Recursion {
                name: match a.0 {
                    Some(a) => a.0,
                    None => SimplePathOrNone::default(),
                },
                deep: a.1 .0 .0 .0,
            },
            Val1(Val0((a, _))) => Self::Star({
                match a {
                    Some(a) => a.0,
                    None => SimplePathOrNone::default(),
                }
            }),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct Use(pub UseTree);

impl MappedParse for Use {
    type Source = (KwUse, UseTree, Semi);

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
    use crate::*;
    use quote::quote;

    insta_match_test!(it_matches_simple_path, Use : use hello::world; );
    insta_match_test!(it_matches_simple_path_as, Use : use hello::world as h; );
    insta_match_test!(it_matches_star_path, Use : use hello::*; );
    insta_match_test!(it_matches_complex_path, Use :  use { hello::*, world::hi as Hi, nested::{ hello::world, hi }, }; );
}
