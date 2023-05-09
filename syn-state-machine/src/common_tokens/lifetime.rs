use super::*;
use crate::*;

#[derive(Debug)]
pub struct LifetimeOrLable;

impl MappedParse for LifetimeOrLable {
    type Source = (FJointPunct<'\''>, Identifier);

    type Output = Ident;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(src.1)
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
