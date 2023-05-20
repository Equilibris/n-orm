use std::marker::PhantomData;

use crate::{Interlace, InterlaceTrail, MappedParse, Parsable, SmErr, SmOut};

pub struct MinLength<T, const COUNT: usize = 1>(PhantomData<T>);

#[derive(Debug, thiserror::Error)]
pub enum LengthError<T: std::error::Error> {
    #[error("{}", .0)]
    Inner(T),
    #[error("Expected minimum length {} but got len {}",.0,.1)]
    InvalidLength(usize, usize),
}

impl<T: Parsable, const COUNT: usize> MappedParse for MinLength<Vec<T>, COUNT> {
    type Source = Vec<T>;

    type Output = SmOut<Self::Source>;
    type Error = LengthError<SmErr<Self::Source>>;

    fn map(
        src: crate::SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        if src.len() >= COUNT {
            Ok(src)
        } else {
            Err(LengthError::InvalidLength(COUNT, src.len()))
        }
    }

    fn map_err(src: crate::SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        LengthError::Inner(src)
    }
}
impl<A: Parsable, B: Parsable, const COUNT: usize> MappedParse
    for MinLength<Interlace<A, B>, COUNT>
{
    type Source = Interlace<A, B>;

    type Output = SmOut<Self::Source>;
    type Error = LengthError<SmErr<Self::Source>>;

    fn map(
        src: crate::SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        if src.0.len() >= COUNT {
            Ok(src)
        } else {
            Err(LengthError::InvalidLength(COUNT, src.0.len()))
        }
    }

    fn map_err(src: crate::SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        LengthError::Inner(src)
    }
}

impl<A: Parsable, B: Parsable, const COUNT: usize> MappedParse
    for MinLength<InterlaceTrail<A, B>, COUNT>
{
    type Source = InterlaceTrail<A, B>;

    type Output = SmOut<Self::Source>;
    type Error = LengthError<SmErr<Self::Source>>;

    fn map(
        src: crate::SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        if src.0.len() >= COUNT {
            Ok(src)
        } else {
            Err(LengthError::InvalidLength(COUNT, src.0.len()))
        }
    }

    fn map_err(src: crate::SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        LengthError::Inner(src)
    }
}
