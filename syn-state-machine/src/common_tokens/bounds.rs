use super::*;
use crate::*;

pub struct Lifetime;

impl MappedParse for Lifetime {
    type Source = Either<Either<StaticLifetime, UnderLifetime>, LifetimeOrLable>;

    type Output = Ident;
    type Error = SmError<Self::Source>;

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Either::Right(a) => a,
            Either::Left(Either::Left(a)) => a.1.into(),
            Either::Left(Either::Right(a)) => a.1.into(),
        })
    }

    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type LifetimeBounds = Interlace<Lifetime, FPunct<'+'>>;

pub struct TypeParamBounds {
    r#for: SmOutput<Option<GenericParams>>,
    ty: TypePath,
}

impl MappedParse for TypeParamBounds {
    type Source = (Option<(KwFor, GenericParams)>, TypePath);

    type Output = Self;
    type Error = SmError<Self::Source>;

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
        todo!()
    }
}
