use crate::*;

#[derive(Debug)]
pub enum Expression {}

impl MappedParse for Expression {
    type Source = TokenTree;

    type Output = Self;
    type Error = std::convert::Infallible;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        todo!()
    }
}

#[derive(Debug)]
pub struct BlockExpression {}
impl MappedParse for BlockExpression {
    type Source = std::convert::Infallible;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
