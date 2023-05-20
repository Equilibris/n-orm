use super::*;
use crate::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct StaticItem {
    pub id: Ident,
    pub ty: Type,

    pub r#mut: bool,

    pub expr: Option<Expression>,
}
impl MappedParse for StaticItem {
    type Source = (
        KwStruct,
        Option<KwMut>,
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
            r#mut: src.1.is_some(),

            id: src.2,
            ty: src.4,
            expr: src.5.map(|v| v.1),
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
