use super::*;
use crate::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ConstantItem {
    pub id: Ident,
    pub ty: Type,
    pub expr: Option<Expression>,
}
impl MappedParse for ConstantItem {
    type Source = (
        KwConst,
        IdentifierOrUnder,
        Colon,
        Type,
        Option<(Eq, Expression)>,
        Semi,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            ty: src.3,
            expr: src.4.map(|v| v.1),
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
}
