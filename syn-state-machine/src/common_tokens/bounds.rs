use super::*;
use crate::*;

#[derive(Debug)]
pub struct Lifetime(pub Ident);
impl MappedParse for Lifetime {
    type Source = Sum2<Sum2<StaticLifetime, UnderLifetime>, LifetimeOrLable>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(match src {
            Sum2::Val1(a) => a,
            Sum2::Val0(Sum2::Val0(a)) => a.1.into(),
            Sum2::Val0(Sum2::Val1(a)) => a.1.into(),
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

pub struct TypeParamBounds<T: Parsable>(pub Vec<TypeParamBound<T>>);
impl<T: Parsable> Debug for TypeParamBounds<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypeParamBounds").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for TypeParamBounds<T> {
    type Source = Interlace<TypeParamBound<T>, Plus>;

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

pub enum TypeParamBound<T: Parsable> {
    Lifetime(Lifetime),
    TraitBound(TraitBound<T>),
}
impl<T: Parsable> Debug for TypeParamBound<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lifetime(arg0) => f.debug_tuple("Lifetime").field(arg0).finish(),
            Self::TraitBound(arg0) => f.debug_tuple("TraitBound").field(arg0).finish(),
        }
    }
}
impl<T: Parsable> MappedParse for TypeParamBound<T> {
    type Source = Sum2<Lifetime, TraitBound<T>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::Lifetime(a),
            Sum2::Val1(a) => Self::TraitBound(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TraitBound<T: Parsable> {
    pub q: bool,

    pub r#for: Option<GenericParams<T, Type<T>>>,
    pub ty: TypePath<Type<T>>,
}
impl<T: Parsable> Debug for TraitBound<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TraitBound")
            .field("q", &self.q)
            .field("for", &self.r#for)
            .field("ty", &self.ty)
            .finish()
    }
}
type TraitBoundInternal<T> = (
    Option<FPunct<'?'>>,
    Option<ForLifetimes<T>>,
    TypePath<Type<T>>,
);
impl<T: Parsable> MappedParse for TraitBound<T> {
    type Source = Sum2<MBox<TraitBoundInternal<T>>, Paren<TraitBoundInternal<T>>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(src) | Sum2::Val1(Paren(src)) => Self {
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

pub struct ForLifetimes<T: Parsable>(pub GenericParams<T, Type<T>>);
impl<T: Parsable> Debug for ForLifetimes<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ForLifetimes").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for ForLifetimes<T> {
    type Source = (KwFor, GenericParams<T, Type<T>>);

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

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(it_matches_lifetime, Lifetime : 'a);
    insta_match_test!(it_matches_lifetimes_bounds, LifetimeBounds : 'a + 'b);
    insta_match_test!(it_matches_bound_path, TraitBound<Infallible>: std::fmt::Debug);
    insta_match_test!(it_matches_for_paths, TraitBound<Infallible>: for<'a> std::fmt::Debug);
    insta_match_test!(
        it_matches_path_type_param_bound,
        TypeParamBound<Infallible>: std::fmt::Debug
    );
    insta_match_test!(
        it_matches_for_paths_type_param_bound,
        TypeParamBound<Infallible>: for<'a> std::fmt::Debug
    );
    insta_match_test!(
        it_matches_lifetime_type_param_bound,
        TypeParamBound<Infallible>: 'a
    );
    insta_match_test!(
        it_matches_for_lifetimes,
        ForLifetimes<Infallible>: for<'a, 'b>
    );
}
