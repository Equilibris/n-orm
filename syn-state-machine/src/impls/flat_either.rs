use std::marker::PhantomData;

use crate::*;

pub struct FlatEither<A: Parsable, B: Parsable, C = SmOutput<A>>(PhantomData<(A, B, C)>)
where
    SmOutput<A>: Into<C>,
    SmOutput<B>: Into<C>;

impl<A: Parsable, B: Parsable, C> MappedParse for FlatEither<A, B, C>
where
    SmOutput<A>: Into<C>,
    SmOutput<B>: Into<C>,
{
    type Source = Either<A, B>;

    type Output = C;
    type Error = SmError<Self::Source>;

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Either::Left(a) => a.into(),
            Either::Right(b) => b.into(),
        })
    }

    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
