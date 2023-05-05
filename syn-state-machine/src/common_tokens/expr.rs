use crate::*;

pub enum Expression {}

impl MappedParse for Expression {
    type Source = TokenTree;

    type Output = ();
    type Error = ();

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
        todo!()
    }
}
