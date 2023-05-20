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

#[derive(Debug)]
pub struct TypeParamBounds(pub Vec<TypeParamBound>);
impl MappedParse for TypeParamBounds {
    type Source = Interlace<TypeParamBound, Plus>;

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
pub enum TypeParamBound {
    Lifetime(Lifetime),
    TraitBound(TraitBound),
}
impl MappedParse for TypeParamBound {
    type Source = Sum2<Lifetime, TraitBound>;

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

#[derive(Debug)]
pub struct TraitBound {
    pub q: bool,

    pub r#for: Option<GenericParams>,
    pub ty: TypePath,
}
type TraitBoundInternal = (Option<FPunct<'?'>>, Option<ForLifetimes>, TypePath);
impl MappedParse for TraitBound {
    type Source = Sum2<MBox<TraitBoundInternal>, Paren<TraitBoundInternal>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val1(src) | Sum2::Val0(src) => Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(it_matches_lifetime, Lifetime : 'a);
    insta_match_test!(it_matches_lifetimes_bounds, LifetimeBounds : 'a + 'b);
    insta_match_test!(it_matches_bound_path, TraitBound: std::fmt::Debug);
    insta_match_test!(it_matches_for_paths, TraitBound: for<'a> std::fmt::Debug);
    insta_match_test!(
        it_matches_path_type_param_bound,
        TypeParamBound: std::fmt::Debug
    );
    insta_match_test!(
        it_matches_for_paths_type_param_bound,
        TypeParamBound: for<'a> std::fmt::Debug
    );
    insta_match_test!(
        it_matches_lifetime_type_param_bound,
        TypeParamBound: 'a
    );
    insta_match_test!(
        it_matches_for_lifetimes,
        ForLifetimes: for<'a, 'b>
    );
}
