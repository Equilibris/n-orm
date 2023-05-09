use super::*;
use crate::*;

use Either::*;

pub mod range_patterns;
pub use range_patterns::*;

#[derive(Debug)]
pub struct Pattern(pub Vec<PatternNoTopAlt>);
impl MappedParse for Pattern {
    type Source = (Option<Pipe>, Interlace<PatternNoTopAlt, Pipe>);

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

#[derive(Debug)]
pub enum PatternNoTopAlt {
    PatternWithoutRange(PatternWithoutRange),
    RangePattern(RangePattern),
}
impl MappedParse for PatternNoTopAlt {
    type Source = Either<PatternWithoutRange, RangePattern>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(a) => Self::PatternWithoutRange(a),
            Right(a) => Self::RangePattern(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to match pattern")]
pub struct PatternError {
    pub literal_pattern: SmErr<LiteralPattern>,
    pub identifier_pattern: Box<SmErr<IdentifierPattern>>,
    pub wildcard_pattern: SmErr<WildcardPattern>,
    pub rest_pattern: SmErr<RestPattern>,
    pub reference_pattern: Box<SmErr<ReferencePattern>>,
    pub struct_pattern: SmErr<StructPattern>,
    pub tuple_struct_pattern: SmErr<TupleStructPattern>,
    pub tuple_pattern: SmErr<TuplePattern>,
    pub grouped_pattern: Box<SmErr<Paren<Pattern>>>,
    pub slice_pattern: SmErr<SlicePattern>,
    pub path_pattern: SmErr<PathPattern>,
    pub macro_invocation: SmErr<MacroInvocation>,
}

#[derive(Debug)]
pub enum PatternWithoutRange {
    LiteralPattern(LiteralPattern),
    IdentifierPattern(Box<IdentifierPattern>),
    WildcardPattern(WildcardPattern),
    RestPattern(RestPattern),
    ReferencePattern(Box<ReferencePattern>),
    StructPattern(StructPattern),
    TupleStructPattern(TupleStructPattern),
    TuplePattern(TuplePattern),
    GroupedPattern(Box<Pattern>),
    SlicePattern(SlicePattern),
    PathPattern(PathPattern),
    MacroInvocation(MacroInvocation),
}
impl MappedParse for PatternWithoutRange {
    type Source = PBox<
        Either<
            Either<
                Either<
                    Either<LiteralPattern, IdentifierPattern>,
                    Either<WildcardPattern, RestPattern>,
                >,
                Either<
                    Either<ReferencePattern, StructPattern>,
                    Either<TupleStructPattern, TuplePattern>,
                >,
            >,
            Either<Either<Paren<Pattern>, SlicePattern>, Either<PathExpression, MacroInvocation>>,
        >,
    >;

    type Output = Self;
    type Error = PatternError;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match *src {
            Left(Left(Left(Left(a)))) => Self::LiteralPattern(a),
            Left(Left(Left(Right(a)))) => Self::IdentifierPattern(Box::new(a)),
            Left(Left(Right(Left(a)))) => Self::WildcardPattern(a),
            Left(Left(Right(Right(a)))) => Self::RestPattern(a),
            Left(Right(Left(Left(a)))) => Self::ReferencePattern(Box::new(a)),
            Left(Right(Left(Right(a)))) => Self::StructPattern(a),
            Left(Right(Right(Left(a)))) => Self::TupleStructPattern(a),
            Left(Right(Right(Right(a)))) => Self::TuplePattern(a),

            Right(Left(Left(a))) => Self::GroupedPattern(Box::new(a)),
            Right(Left(Right(a))) => Self::SlicePattern(a),
            Right(Right(Left(a))) => Self::PathPattern(a),
            Right(Right(Right(a))) => Self::MacroInvocation(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        let src = *src;

        let EitherParsingError(
            EitherParsingError(
                EitherParsingError(
                    EitherParsingError(literal_pattern, identifier_pattern),
                    EitherParsingError(wildcard_pattern, rest_pattern),
                ),
                EitherParsingError(
                    EitherParsingError(reference_pattern, struct_pattern),
                    EitherParsingError(tuple_struct_pattern, tuple_pattern),
                ),
            ),
            EitherParsingError(
                EitherParsingError(grouped_pattern, slice_pattern),
                EitherParsingError(path_pattern, macro_invocation),
            ),
        ) = src;

        PatternError {
            literal_pattern,
            identifier_pattern: Box::new(identifier_pattern),
            wildcard_pattern,
            rest_pattern,
            reference_pattern: Box::new(reference_pattern),
            struct_pattern,
            tuple_struct_pattern,
            tuple_pattern,
            grouped_pattern: Box::new(grouped_pattern),
            slice_pattern,
            path_pattern,
            macro_invocation,
        }
    }
}

pub type PathPattern = PathExpression;

#[derive(Debug)]
pub struct SlicePattern(pub Vec<Pattern>);
impl MappedParse for SlicePattern {
    type Source = Bracket<Option<SlicePatternItems>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.map(|v| v.0).unwrap_or_default()))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct SlicePatternItems(pub Vec<Pattern>);
impl MappedParse for SlicePatternItems {
    type Source = (MinLength<Interlace<Pattern, Comma>>, Option<Comma>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct TupleStructPattern {
    pub path: PathInExpression,
    pub items: Vec<Pattern>,
}
impl MappedParse for TupleStructPattern {
    type Source = (PathInExpression, Paren<TupleStructItems>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            path: src.0,
            items: src.1 .0,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TupleStructItems(pub Vec<Pattern>);
impl MappedParse for TupleStructItems {
    type Source = (MinLength<Interlace<Pattern, Comma>>, Option<Comma>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct TuplePattern(pub Option<TuplePatternItems>);
impl MappedParse for TuplePattern {
    type Source = (PathInExpression, Paren<Option<TuplePatternItems>>);

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
pub enum TuplePatternItems {
    Fields(Vec<Pattern>),
    Rest,
}
impl MappedParse for TuplePatternItems {
    type Source = Either<
        Either<RestPattern, (Pattern, Comma)>,
        (MinLength<Interlace<Pattern, Comma>, 2>, Option<Comma>),
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(Left(_)) => Self::Rest,
            Left(Right(a)) => Self::Fields(vec![a.0]),
            Right(a) => Self::Fields(a.0 .0),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct StructPattern {
    pub path: PathInExpression,

    pub et_cetera: bool,

    pub fields: Vec<StructPatternField>,
}
impl MappedParse for StructPattern {
    type Source = (PathInExpression, Brace<Option<StructPatternElements>>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Some(a) => match a {
                StructPatternElements::StructPatternEtCetera(_) => Self {
                    path: src.0,
                    et_cetera: true,
                    fields: Vec::new(),
                },
                StructPatternElements::StructPatternFields(a, Some(_)) => Self {
                    path: src.0,
                    et_cetera: true,
                    fields: a.0,
                },
                StructPatternElements::StructPatternFields(a, None) => Self {
                    path: src.0,
                    et_cetera: false,
                    fields: a.0,
                },
            },
            None => Self {
                path: src.0,
                et_cetera: false,
                fields: Vec::new(),
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum StructPatternElements {
    StructPatternEtCetera(StructPatternEtCetera),
    StructPatternFields(StructPatternFields, Option<StructPatternEtCetera>),
}
impl MappedParse for StructPatternElements {
    type Source = Either<
        (
            StructPatternFields,
            Option<Either<Comma, (Comma, StructPatternEtCetera)>>,
        ),
        StructPatternEtCetera,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(a) => Self::StructPatternFields(a.0, a.1.and_then(|v| v.right()).map(|v| v.1)),
            Right(a) => Self::StructPatternEtCetera(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub enum StructPatternField<T: Parsable = Tokens> {
    Tuple {
        attrs: Attrs<T>,
        id: IntegerLit,
        pattern: Pattern,
    },
    Id {
        attrs: Attrs<T>,
        id: Ident,
        pattern: Pattern,
    },
    IdShorthand {
        attrs: Attrs<T>,
        r#ref: bool,
        r#mut: bool,
        id: Ident,
    },
}
impl<T: Parsable> MappedParse for StructPatternField<T> {
    type Source = WithAttrs<
        T,
        Either<
            Either<(TupleIndex, Colon, Pattern), (Identifier, Colon, Pattern)>,
            (Option<KwRef>, Option<KwMut>, Identifier),
        >,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Left(Left(a)) => Self::Tuple {
                attrs: src.0,
                id: a.0,
                pattern: a.2,
            },
            Left(Right(a)) => Self::Id {
                attrs: src.0,
                id: a.0,
                pattern: a.2,
            },
            Right(a) => Self::IdShorthand {
                attrs: src.0,
                r#ref: a.0.is_some(),
                r#mut: a.1.is_some(),
                id: a.2,
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct StructPatternEtCetera<T: Parsable = Tokens>(pub Attrs<T>);
impl<T: Parsable> MappedParse for StructPatternEtCetera<T> {
    type Source = WithAttrs<T, (FJointPunct<'.'>, Dot)>;

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

pub struct StructPatternFields<T: Parsable = Tokens>(pub Vec<StructPatternField<T>>);
impl<T: Parsable> MappedParse for StructPatternFields<T> {
    type Source = MinLength<Interlace<StructPatternField<T>, Comma>>;

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
pub struct ReferencePattern {
    pub ref_count: usize,
    pub r#mut: bool,
    pub pattern: PatternWithoutRange,
}
impl MappedParse for ReferencePattern {
    type Source = (Either<Amp, (Amp, Amp)>, Option<KwMut>, PatternWithoutRange);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            ref_count: usize::from(src.0.is_left()) + 1,
            r#mut: src.1.is_some(),
            pattern: src.2,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type RestPattern = (FJointPunct<'.'>, Dot);
pub type WildcardPattern = Underscore;

#[derive(Debug)]
pub enum LiteralPattern {
    Bool(bool),
    CharLit(CharLit),
    ByteLit(ByteLit),
    StringLit(StringLit),
    ByteStringLit(ByteStringLit),
    NegIntLit(NegativeIntegerLit),
    NegFloatLit(NegativeFloatLit),
    IntLit(IntegerLit),
    FloatLit(FloatLit),
}
impl MappedParse for LiteralPattern {
    type Source = Either<
        Either<
            Either<Either<bool, CharLit>, Either<ByteLit, StringLit>>,
            Either<Either<ByteStringLit, NegativeIntegerLit>, Either<NegativeFloatLit, IntegerLit>>,
        >,
        FloatLit,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(Left(Left(Left(a)))) => Self::Bool(a),
            Left(Left(Left(Right(a)))) => Self::CharLit(a),
            Left(Left(Right(Left(a)))) => Self::ByteLit(a),
            Left(Left(Right(Right(a)))) => Self::StringLit(a),
            Left(Right(Left(Left(a)))) => Self::ByteStringLit(a),
            Left(Right(Left(Right(a)))) => Self::NegIntLit(a),
            Left(Right(Right(Left(a)))) => Self::NegFloatLit(a),
            Left(Right(Right(Right(a)))) => Self::IntLit(a),
            Right(a) => Self::FloatLit(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct IdentifierPattern {
    pub r#ref: bool,
    pub r#mut: bool,

    pub id: Ident,

    pub at_pattern: Option<PatternNoTopAlt>,
}
impl MappedParse for IdentifierPattern {
    type Source = (
        Option<KwRef>,
        Option<KwMut>,
        Identifier,
        Option<(At, PatternNoTopAlt)>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#ref: src.0.is_some(),
            r#mut: src.1.is_some(),
            id: src.2,
            at_pattern: src.3.map(|v| v.1),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
