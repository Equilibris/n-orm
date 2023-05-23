use super::*;
use crate::*;
use std::fmt::Debug;

pub struct ConstantItem<Ty: Parsable> {
    pub id: Ident,
    pub ty: SmOut<Ty>,
    pub expr: Option<Expression>,
}
impl<Ty: Parsable> Debug for ConstantItem<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstantItem")
            .field("id", &self.id)
            .field("ty", &self.ty)
            .field("expr", &self.expr)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for ConstantItem<Ty> {
    type Source = (
        KwConst,
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
