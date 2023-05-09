use super::*;
use crate::*;

#[derive(Debug)]
pub struct Lifetime(pub Ident);

impl MappedParse for Lifetime {
    type Source = Either<Either<StaticLifetime, UnderLifetime>, LifetimeOrLable>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(match src {
            Either::Right(a) => a,
            Either::Left(Either::Left(a)) => a.1.into(),
            Either::Left(Either::Right(a)) => a.1.into(),
        }))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct LifetimeBounds(pub Vec<Lifetime>);
impl MappedParse for LifetimeBounds {
    type Source = Interlace<Lifetime, Plus>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct TypeParamBounds(pub Vec<Either<Lifetime, TraitBound>>);

impl MappedParse for TypeParamBounds {
    type Source = Interlace<Either<Lifetime, TraitBound>, Plus>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct TraitBound {
    pub q: bool,

    pub r#for: Option<GenericParams>,
    pub ty: TypePath,
}
type TraitBoundInternal = (Option<FPunct<'?'>>, Option<ForLifetimes>, TypePath);
impl MappedParse for TraitBound {
    type Source = Either<MBox<TraitBoundInternal>, Paren<TraitBoundInternal>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Either::Right(src) | Either::Left(src) => Self {
                q: src.0.is_some(),
                r#for: src.1.map(|v| v.0),
                ty: src.2,
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct ForLifetimes(pub GenericParams);
impl MappedParse for ForLifetimes {
    type Source = (KwFor, GenericParams);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
