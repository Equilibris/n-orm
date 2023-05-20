use super::super::*;
use crate::*;

#[derive(Debug)]
pub enum RangePattern {
    RangeInclusivePattern(RangeInclusivePattern),
    RangeFromPattern(RangeFromPattern),
    RangeToInclusivePattern(RangeToInclusivePattern),
    ObsoleteRangePattern(ObsoleteRangePattern),
}
impl MappedParse for RangePattern {
    type Source = Sum4<
        RangeInclusivePattern,
        RangeToInclusivePattern,
        ObsoleteRangePattern,
        RangeFromPattern,
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

#[derive(Debug)]
pub struct RangeInclusivePattern(pub RangePatternBound, pub RangePatternBound);
impl MappedParse for RangeInclusivePattern {
    type Source = (RangePatternBound, DotDotEq, RangePatternBound);

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

#[derive(Debug)]
pub struct RangeFromPattern(pub RangePatternBound);
impl MappedParse for RangeFromPattern {
    type Source = (RangePatternBound, DotDot);

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
pub struct RangeToInclusivePattern(pub RangePatternBound);
impl MappedParse for RangeToInclusivePattern {
    type Source = (DotDotEq, RangePatternBound);

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

#[derive(Debug)]
pub struct ObsoleteRangePattern(pub RangePatternBound, pub RangePatternBound);
impl MappedParse for ObsoleteRangePattern {
    type Source = (RangePatternBound, Elipsis, RangePatternBound);

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

#[derive(Debug)]
pub enum RangePatternBound {
    CharLit(CharLit),
    ByteLit(ByteLit),
    SignedIntegerLit(SignedIntegerLit),
    SignedFloatLit(SignedFloatLit),
    PathExpression(PathExpression),
}
impl MappedParse for RangePatternBound {
    type Source = Sum5<CharLit, ByteLit, SignedIntegerLit, SignedFloatLit, PathExpression>;

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

    insta_match_test!(it_matches_inclusive, RangePattern : 0..=10);
    insta_match_test!(it_matches_from, RangePattern : 0..);
    insta_match_test!(it_matches_to, RangePattern : ..=0);
    insta_match_test!(it_matches_obsolete, RangePattern : 0...0);
}
