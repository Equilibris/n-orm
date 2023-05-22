use super::super::*;
use crate::*;

pub enum RangePattern<T: Parsable> {
    RangeInclusivePattern(RangeInclusivePattern<T>),
    RangeFromPattern(RangeFromPattern<T>),
    RangeToInclusivePattern(RangeToInclusivePattern<T>),
    ObsoleteRangePattern(ObsoleteRangePattern<T>),
}
impl<T: Parsable> Debug for RangePattern<T>
where
    SmOut<T>: Debug,
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
impl<T: Parsable> MappedParse for RangePattern<T> {
    type Source = Sum4<
        RangeInclusivePattern<T>,
        RangeToInclusivePattern<T>,
        ObsoleteRangePattern<T>,
        RangeFromPattern<T>,
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

pub struct RangeInclusivePattern<T: Parsable>(pub RangePatternBound<T>, pub RangePatternBound<T>);
impl<T: Parsable> Debug for RangeInclusivePattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RangeInclusivePattern")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable> MappedParse for RangeInclusivePattern<T> {
    type Source = (RangePatternBound<T>, DotDotEq, RangePatternBound<T>);

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

pub struct RangeFromPattern<T: Parsable>(pub RangePatternBound<T>);
impl<T: Parsable> Debug for RangeFromPattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RangeFromPattern").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for RangeFromPattern<T> {
    type Source = (RangePatternBound<T>, DotDot);

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

pub struct RangeToInclusivePattern<T: Parsable>(pub RangePatternBound<T>);
impl<T: Parsable> Debug for RangeToInclusivePattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RangeToInclusivePattern")
            .field(&self.0)
            .finish()
    }
}
impl<T: Parsable> MappedParse for RangeToInclusivePattern<T> {
    type Source = (DotDotEq, RangePatternBound<T>);

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

pub struct ObsoleteRangePattern<T: Parsable>(pub RangePatternBound<T>, pub RangePatternBound<T>);
impl<T: Parsable> Debug for ObsoleteRangePattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ObsoleteRangePattern")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Parsable> MappedParse for ObsoleteRangePattern<T> {
    type Source = (RangePatternBound<T>, Elipsis, RangePatternBound<T>);

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

pub enum RangePatternBound<T: Parsable> {
    CharLit(CharLit),
    ByteLit(ByteLit),
    SignedIntegerLit(SignedIntegerLit),
    SignedFloatLit(SignedFloatLit),
    PathExpression(PathExpression<T>),
}
impl<T: Parsable> Debug for RangePatternBound<T>
where
    SmOut<T>: Debug,
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
impl<T: Parsable> MappedParse for RangePatternBound<T> {
    type Source = Sum5<CharLit, ByteLit, SignedIntegerLit, SignedFloatLit, PathExpression<T>>;

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

    insta_match_test!(it_matches_inclusive, RangePattern <Infallible>: 0..=10);
    insta_match_test!(it_matches_from, RangePattern <Infallible>: 0..);
    insta_match_test!(it_matches_to, RangePattern <Infallible>: ..=0);
    insta_match_test!(it_matches_obsolete, RangePattern <Infallible>: 0...0);
}
