use std::fmt::Debug;

use super::*;
use crate::*;

use Sum2::*;

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
    type Source = Sum2<PatternWithoutRange, RangePattern>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Val0(a) => Self::PatternWithoutRange(a),
            Val1(a) => Self::RangePattern(a),
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
        Sum12<
            LiteralPattern,
            IdentifierPattern,
            WildcardPattern,
            RestPattern,
            ReferencePattern,
            StructPattern,
            TupleStructPattern,
            TuplePattern,
            Paren<Pattern>,
            SlicePattern,
            PathExpression,
            MacroInvocation,
        >,
    >;

    type Output = Self;
    type Error = PatternError;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match *src {
            Sum12::Val0(a) => Self::LiteralPattern(a),
            Sum12::Val1(a) => Self::IdentifierPattern(Box::new(a)),
            Sum12::Val2(a) => Self::WildcardPattern(a),
            Sum12::Val3(a) => Self::RestPattern(a),
            Sum12::Val4(a) => Self::ReferencePattern(Box::new(a)),
            Sum12::Val5(a) => Self::StructPattern(a),
            Sum12::Val6(a) => Self::TupleStructPattern(a),
            Sum12::Val7(a) => Self::TuplePattern(a),
            Sum12::Val8(a) => Self::GroupedPattern(Box::new(a)),
            Sum12::Val9(a) => Self::SlicePattern(a),
            Sum12::Val10(a) => Self::PathPattern(a),
            Sum12::Val11(a) => Self::MacroInvocation(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        let src = *src;

        let Sum12Err {
            v0: literal_pattern,
            v1: identifier_pattern,
            v2: wildcard_pattern,
            v3: rest_pattern,
            v4: reference_pattern,
            v5: struct_pattern,
            v6: tuple_struct_pattern,
            v7: tuple_pattern,
            v8: grouped_pattern,
            v9: slice_pattern,
            v10: path_pattern,
            v11: macro_invocation,
        } = src;

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
    type Source = Sum2<
        Sum2<RestPattern, (Pattern, Comma)>,
        (MinLength<Interlace<Pattern, Comma>, 2>, Option<Comma>),
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Val0(Val0(_)) => Self::Rest,
            Val0(Val1(a)) => Self::Fields(vec![a.0]),
            Val1(a) => Self::Fields(a.0 .0),
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
    type Source = Sum2<
        (
            StructPatternFields,
            Option<Sum2<Comma, (Comma, StructPatternEtCetera)>>,
        ),
        StructPatternEtCetera,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Val0(a) => Self::StructPatternFields(
                a.0,
                a.1.and_then(|v| if let Sum2::Val1(a) = v { Some(a) } else { None })
                    .map(|v| v.1),
            ),
            Val1(a) => Self::StructPatternEtCetera(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

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

impl<T: Parsable> Debug for StructPatternField<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tuple { attrs, id, pattern } => f
                .debug_struct("Tuple")
                .field("attrs", attrs)
                .field("id", id)
                .field("pattern", pattern)
                .finish(),
            Self::Id { attrs, id, pattern } => f
                .debug_struct("Id")
                .field("attrs", attrs)
                .field("id", id)
                .field("pattern", pattern)
                .finish(),
            Self::IdShorthand {
                attrs,
                r#ref,
                r#mut,
                id,
            } => f
                .debug_struct("IdShorthand")
                .field("attrs", attrs)
                .field("r#ref", r#ref)
                .field("r#mut", r#mut)
                .field("id", id)
                .finish(),
        }
    }
}

impl<T: Parsable> MappedParse for StructPatternField<T> {
    type Source = WithAttrs<
        T,
        Sum2<
            Sum2<(TupleIndex, Colon, Pattern), (Identifier, Colon, Pattern)>,
            (Option<KwRef>, Option<KwMut>, Identifier),
        >,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Val0(Val0(a)) => Self::Tuple {
                attrs: src.0,
                id: a.0,
                pattern: a.2,
            },
            Val0(Val1(a)) => Self::Id {
                attrs: src.0,
                id: a.0,
                pattern: a.2,
            },
            Val1(a) => Self::IdShorthand {
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
    type Source = (Sum2<Amp, (Amp, Amp)>, Option<KwMut>, PatternWithoutRange);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            ref_count: usize::from(matches!(src.0, Sum2::Val0(_))) + 1,
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
    type Source = Sum9<
        bool,
        CharLit,
        ByteLit,
        StringLit,
        ByteStringLit,
        NegativeIntegerLit,
        NegativeFloatLit,
        IntegerLit,
        FloatLit,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum9::Val0(a) => Self::Bool(a),
            Sum9::Val1(a) => Self::CharLit(a),
            Sum9::Val2(a) => Self::ByteLit(a),
            Sum9::Val3(a) => Self::StringLit(a),
            Sum9::Val4(a) => Self::ByteStringLit(a),
            Sum9::Val5(a) => Self::NegIntLit(a),
            Sum9::Val6(a) => Self::NegFloatLit(a),
            Sum9::Val7(a) => Self::IntLit(a),
            Sum9::Val8(a) => Self::FloatLit(a),
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
