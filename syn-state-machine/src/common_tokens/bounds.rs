use super::*;
use crate::*;

#[derive(Debug)]
pub struct Lifetime(pub Ident);
impl MappedParse for Lifetime {
    type Source = Sum3<StaticLifetime, UnderLifetime, LifetimeOrLable>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(match src {
            Sum3::Val0(a) => a.1.into(),
            Sum3::Val1(a) => a.1.into(),
            Sum3::Val2(a) => a,
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

pub struct TypeParamBounds<T: Parsable, Ty: Parsable>(pub Vec<TypeParamBound<T, Ty>>);
impl<T: Parsable, Ty: Parsable> Debug for TypeParamBounds<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypeParamBounds").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TypeParamBounds<T, Ty> {
    type Source = Interlace<TypeParamBound<T, Ty>, Plus>;

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

pub enum TypeParamBound<T: Parsable, Ty: Parsable> {
    Lifetime(Lifetime),
    TraitBound(TraitBound<T, Ty>),
}
impl<T: Parsable, Ty: Parsable> Debug for TypeParamBound<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lifetime(arg0) => f.debug_tuple("Lifetime").field(arg0).finish(),
            Self::TraitBound(arg0) => f.debug_tuple("TraitBound").field(arg0).finish(),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TypeParamBound<T, Ty> {
    type Source = Sum2<Lifetime, TraitBound<T, Ty>>;

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

pub struct TraitBound<T: Parsable, Ty: Parsable> {
    pub q: bool,

    pub r#for: Option<GenericParams<T, Ty>>,
    pub ty: TypePath<Ty>,
}
impl<T: Parsable, Ty: Parsable> Debug for TraitBound<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TraitBound")
            .field("q", &self.q)
            .field("for", &self.r#for)
            .field("ty", &self.ty)
            .finish()
    }
}
type TraitBoundInternal<T, Ty> = (
    Option<FPunct<'?'>>,
    Option<ForLifetimes<T, Ty>>,
    TypePath<Ty>,
);
impl<T: Parsable, Ty: Parsable> MappedParse for TraitBound<T, Ty> {
    type Source = Sum2<MBox<TraitBoundInternal<T, Ty>>, Paren<TraitBoundInternal<T, Ty>>>;

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

pub struct ForLifetimes<T: Parsable, Ty: Parsable>(pub GenericParams<T, Ty>);
impl<T: Parsable, Ty: Parsable> Debug for ForLifetimes<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ForLifetimes").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for ForLifetimes<T, Ty> {
    type Source = (KwFor, GenericParams<T, Ty>);

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
    insta_match_test!(it_matches_bound_path, TraitBound<Infallible, SimplePath>: std::fmt::Debug);
    insta_match_test!(it_matches_for_paths,  TraitBound<Infallible, SimplePath>: for<'a> std::fmt::Debug);
    insta_match_test!(
        it_matches_path_type_param_bound,
        TypeParamBound<Infallible, SimplePath>: std::fmt::Debug
    );
    insta_match_test!(
        it_matches_for_paths_type_param_bound,
        TypeParamBound<Infallible, SimplePath>: for<'a> std::fmt::Debug
    );
    insta_match_test!(
        it_matches_lifetime_type_param_bound,
        TypeParamBound<Infallible, SimplePath>: 'a
    );
    insta_match_test!(
        it_matches_for_lifetimes,
        ForLifetimes<Infallible, Infallible>: for<'a, 'b>
    );
}
