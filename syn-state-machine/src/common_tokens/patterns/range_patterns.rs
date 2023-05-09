use super::super::*;
use crate::*;
use Either::*;

#[derive(Debug)]
pub enum RangePattern {
    RangeInclusivePattern(RangeInclusivePattern),
    RangeFromPattern(RangeFromPattern),
    RangeToInclusivePattern(RangeToInclusivePattern),
    ObsoleteRangePattern(ObsoleteRangePattern),
}
impl MappedParse for RangePattern {
    type Source = Either<
        Either<RangeInclusivePattern, RangeFromPattern>,
        Either<RangeToInclusivePattern, ObsoleteRangePattern>,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(Left(a)) => Self::RangeInclusivePattern(a),
            Left(Right(a)) => Self::RangeFromPattern(a),
            Right(Left(a)) => Self::RangeToInclusivePattern(a),
            Right(Right(a)) => Self::ObsoleteRangePattern(a),
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
    type Source = (RangePatternBound, DotDotEq);

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
    type Source = Either<
        Either<Either<CharLit, ByteLit>, Either<SignedIntegerLit, SignedFloatLit>>,
        PathExpression,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(Left(Left(a))) => Self::CharLit(a),
            Left(Left(Right(a))) => Self::ByteLit(a),
            Left(Right(Left(a))) => Self::SignedIntegerLit(a),
            Left(Right(Right(a))) => Self::SignedFloatLit(a),
            Right(a) => Self::PathExpression(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
