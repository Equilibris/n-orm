use std::fmt::Debug;

use super::*;
use crate::*;

#[derive(Debug)]
pub struct LifetimeParam {
    pub id: Ident,
    pub bounds: Option<LifetimeBounds>,
}
impl MappedParse for LifetimeParam {
    type Source = (LifetimeOrLable, Option<(Colon, LifetimeBounds)>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.0,
            bounds: src.1.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// Internal
pub struct TypeParam<T: Parsable> {
    pub id: Ident,
    pub bounds: Option<TypeParamBounds<T>>,
    pub ty: Option<Type<T>>,
}
impl<T: Parsable> Debug for TypeParam<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeParam")
            .field("id", &self.id)
            .field("bounds", &self.bounds)
            .field("ty", &self.ty)
            .finish()
    }
}
impl<T: Parsable> MappedParse for TypeParam<T> {
    type Source = (
        Identifier,
        Option<(Colon, TypeParamBounds<T>)>,
        Option<(Eq, Type<T>)>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.0,
            bounds: src.1.map(|v| v.1),
            ty: src.2.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// Internal
pub struct ConstParam<Ty: Parsable> {
    pub id: Ident,

    pub ty: SmOut<Ty>,
    pub content: Option<TokenTree>,
}
impl<Ty: Parsable> Debug for ConstParam<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstParam")
            .field("id", &self.id)
            .field("ty", &self.ty)
            .field("content", &self.content)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for ConstParam<Ty> {
    type Source = (KwConst, Identifier, Colon, Ty, Option<TokenTree>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            ty: src.3,
            content: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum GenericParam<T: Parsable> {
    LifetimeParam(Attrs<T>, LifetimeParam),
    TypeParam(Attrs<T>, TypeParam<T>),
    ConstParam(Attrs<T>, ConstParam<Type<T>>),
}
impl<T: Parsable> Debug for GenericParam<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LifetimeParam(arg0, arg1) => f
                .debug_tuple("LifetimeParam")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::TypeParam(arg0, arg1) => {
                f.debug_tuple("TypeParam").field(arg0).field(arg1).finish()
            }
            Self::ConstParam(arg0, arg1) => {
                f.debug_tuple("ConstParam").field(arg0).field(arg1).finish()
            }
        }
    }
}
impl<T: Parsable> MappedParse for GenericParam<T> {
    type Source = WithAttrs<T, Sum3<LifetimeParam, TypeParam<T>, ConstParam<Type<T>>>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Sum3::Val0(a) => Self::LifetimeParam(src.0, a),
            Sum3::Val1(a) => Self::TypeParam(src.0, a),
            Sum3::Val2(a) => Self::ConstParam(src.0, a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct GenericParams<T: Parsable>(pub Vec<GenericParam<T>>);
impl<T: Parsable> Debug for GenericParams<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericParams").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for GenericParams<T> {
    type Source = (Lt, Interlace<MBox<GenericParam<T>>, Comma>, Gt);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(it_matches_const_param,         ConstParam<Ident>: const HELLO: i8);
    insta_match_test!(it_matches_const_param_bounded, ConstParam<Ident>: const HELLO: i8 = 10);

    insta_match_test!(it_matches_type_param,         TypeParam<Infallible>: Hello);
    insta_match_test!(it_matches_type_param_bounded, TypeParam<Infallible>: Hello: std::fmt::Debug);
}
