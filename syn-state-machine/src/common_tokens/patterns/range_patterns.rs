use super::super::*;
use crate::*;

pub enum RangePattern<Ty: Parsable> {
    RangeInclusivePattern(RangeInclusivePattern<Ty>),
    RangeFromPattern(RangeFromPattern<Ty>),
    RangeToInclusivePattern(RangeToInclusivePattern<Ty>),
    ObsoleteRangePattern(ObsoleteRangePattern<Ty>),
}
impl<Ty: Parsable> Debug for RangePattern<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RangeInclusivePattern(arg0) => {
                f.debug_tuple("RangeInclusivePattern").field(arg0).finish()
            }
            Self::RangeFromPattern(arg0) => f.debug_tuple("RangeFromPattern").field(arg0).finish(),
            Self::RangeToInclusivePattern(arg0) => f
                .debug_tuple("RangeToInclusivePattern")
                .field(arg0)
                .finish(),
            Self::ObsoleteRangePattern(arg0) => {
                f.debug_tuple("ObsoleteRangePattern").field(arg0).finish()
            }
        }
    }
}
impl<Ty: Parsable> MappedParse for RangePattern<Ty> {
    type Source = Sum4<
        RangeInclusivePattern<Ty>,
        RangeToInclusivePattern<Ty>,
        ObsoleteRangePattern<Ty>,
        RangeFromPattern<Ty>,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum4::Val0(a) => Self::RangeInclusivePattern(a),
            Sum4::Val1(a) => Self::RangeToInclusivePattern(a),
            Sum4::Val2(a) => Self::ObsoleteRangePattern(a),
            Sum4::Val3(a) => Self::RangeFromPattern(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct RangeInclusivePattern<Ty: Parsable>(
    pub RangePatternBound<Ty>,
    pub RangePatternBound<Ty>,
);
impl<Ty: Parsable> Debug for RangeInclusivePattern<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RangeInclusivePattern")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for RangeInclusivePattern<Ty> {
    type Source = (RangePatternBound<Ty>, DotDotEq, RangePatternBound<Ty>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.2))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct RangeFromPattern<Ty: Parsable>(pub RangePatternBound<Ty>);
impl<Ty: Parsable> Debug for RangeFromPattern<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RangeFromPattern").field(&self.0).finish()
    }
}
impl<Ty: Parsable> MappedParse for RangeFromPattern<Ty> {
    type Source = (RangePatternBound<Ty>, DotDot);

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

pub struct RangeToInclusivePattern<Ty: Parsable>(pub RangePatternBound<Ty>);
impl<Ty: Parsable> Debug for RangeToInclusivePattern<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RangeToInclusivePattern")
            .field(&self.0)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for RangeToInclusivePattern<Ty> {
    type Source = (DotDotEq, RangePatternBound<Ty>);

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

pub struct ObsoleteRangePattern<Ty: Parsable>(pub RangePatternBound<Ty>, pub RangePatternBound<Ty>);
impl<Ty: Parsable> Debug for ObsoleteRangePattern<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ObsoleteRangePattern")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<Ty: Parsable> MappedParse for ObsoleteRangePattern<Ty> {
    type Source = (RangePatternBound<Ty>, Elipsis, RangePatternBound<Ty>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.2))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum RangePatternBound<Ty: Parsable> {
    CharLit(CharLit),
    ByteLit(ByteLit),
    SignedIntegerLit(SignedIntegerLit),
    SignedFloatLit(SignedFloatLit),
    PathExpression(PathExpression<Ty>),
}
impl<Ty: Parsable> Debug for RangePatternBound<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharLit(arg0) => f.debug_tuple("CharLit").field(arg0).finish(),
            Self::ByteLit(arg0) => f.debug_tuple("ByteLit").field(arg0).finish(),
            Self::SignedIntegerLit(arg0) => f.debug_tuple("SignedIntegerLit").field(arg0).finish(),
            Self::SignedFloatLit(arg0) => f.debug_tuple("SignedFloatLit").field(arg0).finish(),
            Self::PathExpression(arg0) => f.debug_tuple("PathExpression").field(arg0).finish(),
        }
    }
}
impl<Ty: Parsable> MappedParse for RangePatternBound<Ty> {
    type Source = Sum5<CharLit, ByteLit, SignedIntegerLit, SignedFloatLit, PathExpression<Ty>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum5::Val0(a) => Self::CharLit(a),
            Sum5::Val1(a) => Self::ByteLit(a),
            Sum5::Val2(a) => Self::SignedIntegerLit(a),
            Sum5::Val3(a) => Self::SignedFloatLit(a),
            Sum5::Val4(a) => Self::PathExpression(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(it_matches_inclusive, RangePattern<Infallible>: 0..=0);
    insta_match_test!(it_matches_from,      RangePattern<Infallible>: 0..  );
    insta_match_test!(it_matches_to,        RangePattern<Infallible>:  ..=0);
    insta_match_test!(it_matches_obsolete,  RangePattern<Infallible>: 0...0);
}
