use super::*;
use crate::*;
use std::fmt::Debug;

pub struct ConstantItem<T: Parsable> {
    pub id: Ident,
    pub ty: Type<T>,
    pub expr: Option<Expression>,
}
impl<T: Parsable> Debug for ConstantItem<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstantItem")
            .field("id", &self.id)
            .field("ty", &self.ty)
            .field("expr", &self.expr)
            .finish()
    }
}
impl<T: Parsable> MappedParse for ConstantItem<T> {
    type Source = (
        KwConst,
        IdentifierOrUnder,
        Colon,
        Type<T>,
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
