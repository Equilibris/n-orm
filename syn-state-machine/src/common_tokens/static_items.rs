use super::*;
use crate::*;
use std::fmt::Debug;

pub struct StaticItem<Ty: Parsable> {
    pub id: Ident,
    pub ty: SmOut<Ty>,

    pub r#mut: bool,

    pub expr: Option<Expression>,
}
impl<Ty: Parsable> Debug for StaticItem<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticItem")
            .field("id", &self.id)
            .field("ty", &self.ty)
            .field("mut", &self.r#mut)
            .field("expr", &self.expr)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for StaticItem<Ty> {
    type Source = (
        KwStruct,
        Option<KwMut>,
        IdentifierOrUnder,
        Colon,
        Ty,
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
