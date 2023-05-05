use super::*;
use crate::*;

pub struct LifetimeOrLable;

impl MappedParse for LifetimeOrLable {
    type Source = (FJointPunct<'\''>, Identifier);

    type Output = Ident;
    type Error = SmError<Self::Source>;

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(src.1)
    }

    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
