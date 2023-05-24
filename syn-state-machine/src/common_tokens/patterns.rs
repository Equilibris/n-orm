use std::fmt::Debug;

use super::*;
use crate::*;

use Sum2::*;

pub mod range_patterns;
pub use range_patterns::*;

pub struct Pattern<T: Parsable>(pub Vec<PatternNoTopAlt<T>>);
impl<T: Parsable> Debug for Pattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pattern").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for Pattern<T> {
    type Source = (Option<Pipe>, Interlace<PatternNoTopAlt<T>, Pipe>);

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

pub enum PatternNoTopAlt<T: Parsable> {
    PatternWithoutRange(PatternWithoutRange<T>),
    RangePattern(RangePattern<T>),
}
impl<T: Parsable> Debug for PatternNoTopAlt<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PatternWithoutRange(arg0) => {
                f.debug_tuple("PatternWithoutRange").field(arg0).finish()
            }
            Self::RangePattern(arg0) => f.debug_tuple("RangePattern").field(arg0).finish(),
        }
    }
}
impl<T: Parsable> MappedParse for PatternNoTopAlt<T> {
    type Source = Sum2<PatternWithoutRange<T>, RangePattern<T>>;

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

#[derive(thiserror::Error)]
#[error("Failed to match pattern")]
pub struct PatternError<T: Parsable> {
    pub literal_pattern: SmErr<LiteralPattern>,
    pub identifier_pattern: Box<SmErr<IdentifierPattern<T>>>,
    pub wildcard_pattern: SmErr<WildcardPattern>,
    pub rest_pattern: SmErr<RestPattern>,
    pub reference_pattern: Box<SmErr<ReferencePattern<T>>>,
    pub struct_pattern: SmErr<StructPattern<T>>,
    pub tuple_struct_pattern: SmErr<TupleStructPattern<T>>,
    pub tuple_pattern: SmErr<TuplePattern<T>>,
    pub grouped_pattern: Box<SmErr<Paren<Pattern<T>>>>,
    pub slice_pattern: SmErr<SlicePattern<T>>,
    pub path_pattern: SmErr<PathPattern<T>>,
    pub macro_invocation: SmErr<MacroInvocation>,
}
impl<T: Parsable> Debug for PatternError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PatternError")
            .field("literal_pattern", &self.literal_pattern)
            .field("identifier_pattern", &self.identifier_pattern)
            .field("wildcard_pattern", &self.wildcard_pattern)
            .field("rest_pattern", &self.rest_pattern)
            .field("reference_pattern", &self.reference_pattern)
            .field("struct_pattern", &self.struct_pattern)
            .field("tuple_struct_pattern", &self.tuple_struct_pattern)
            .field("tuple_pattern", &self.tuple_pattern)
            .field("grouped_pattern", &self.grouped_pattern)
            .field("slice_pattern", &self.slice_pattern)
            .field("path_pattern", &self.path_pattern)
            .field("macro_invocation", &self.macro_invocation)
            .finish()
    }
}

pub enum PatternWithoutRange<T: Parsable> {
    LiteralPattern(LiteralPattern),
    IdentifierPattern(Box<IdentifierPattern<T>>),
    WildcardPattern(WildcardPattern),
    RestPattern(RestPattern),
    ReferencePattern(Box<ReferencePattern<T>>),
    StructPattern(StructPattern<T>),
    TupleStructPattern(TupleStructPattern<T>),
    TuplePattern(TuplePattern<T>),
    GroupedPattern(Box<Pattern<T>>),
    SlicePattern(SlicePattern<T>),
    PathPattern(PathPattern<T>),
    MacroInvocation(MacroInvocation),
}
impl<T: Parsable> Debug for PatternWithoutRange<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LiteralPattern(arg0) => f.debug_tuple("LiteralPattern").field(arg0).finish(),
            Self::IdentifierPattern(arg0) => {
                f.debug_tuple("IdentifierPattern").field(arg0).finish()
            }
            Self::WildcardPattern(arg0) => f.debug_tuple("WildcardPattern").field(arg0).finish(),
            Self::RestPattern(arg0) => f.debug_tuple("RestPattern").field(arg0).finish(),
            Self::ReferencePattern(arg0) => f.debug_tuple("ReferencePattern").field(arg0).finish(),
            Self::StructPattern(arg0) => f.debug_tuple("StructPattern").field(arg0).finish(),
            Self::TupleStructPattern(arg0) => {
                f.debug_tuple("TupleStructPattern").field(arg0).finish()
            }
            Self::TuplePattern(arg0) => f.debug_tuple("TuplePattern").field(arg0).finish(),
            Self::GroupedPattern(arg0) => f.debug_tuple("GroupedPattern").field(arg0).finish(),
            Self::SlicePattern(arg0) => f.debug_tuple("SlicePattern").field(arg0).finish(),
            Self::PathPattern(arg0) => f.debug_tuple("PathPattern").field(arg0).finish(),
            Self::MacroInvocation(arg0) => f.debug_tuple("MacroInvocation").field(arg0).finish(),
        }
    }
}
impl<T: Parsable> MappedParse for PatternWithoutRange<T> {
    type Source = PBox<
        Sum12<
            LiteralPattern,
            IdentifierPattern<T>,
            WildcardPattern,
            RestPattern,
            ReferencePattern<T>,
            StructPattern<T>,
            TupleStructPattern<T>,
            TuplePattern<T>,
            Paren<Pattern<T>>,
            SlicePattern<T>,
            PathExpression<T>,
            MacroInvocation,
        >,
    >;

    type Output = Self;
    type Error = PatternError<T>;

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
            Sum12::Val8(a) => Self::GroupedPattern(Box::new(a.0)),
            Sum12::Val9(a) => Self::SlicePattern(a),
            Sum12::Val10(a) => Self::PathPattern(a),
            Sum12::Val11(a) => Self::MacroInvocation(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        let src = *src;

        let Sum12Err {
            a: literal_pattern,
            b: identifier_pattern,
            c: wildcard_pattern,
            d: rest_pattern,
            e: reference_pattern,
            f: struct_pattern,
            g: tuple_struct_pattern,
            h: tuple_pattern,
            i: grouped_pattern,
            j: slice_pattern,
            k: path_pattern,
            l: macro_invocation,
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

pub type PathPattern<T> = PathExpression<T>;

pub struct SlicePattern<T: Parsable>(pub Vec<Pattern<T>>);
impl<T: Parsable> Debug for SlicePattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SlicePattern").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for SlicePattern<T> {
    type Source = Bracket<Option<SlicePatternItems<T>>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0.map(|v| v.0).unwrap_or_default()))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct SlicePatternItems<T: Parsable>(pub Vec<Pattern<T>>);
impl<T: Parsable> MappedParse for SlicePatternItems<T> {
    type Source = (MinLength<Interlace<Pattern<T>, Comma>>, Option<Comma>);

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

pub struct TupleStructPattern<T: Parsable> {
    pub path: PathInExpression<T>,
    pub items: Vec<Pattern<T>>,
}
impl<T: Parsable> Debug for TupleStructPattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TupleStructPattern")
            .field("path", &self.path)
            .field("items", &self.items)
            .finish()
    }
}
impl<T: Parsable> MappedParse for TupleStructPattern<T> {
    type Source = (PathInExpression<T>, Paren<TupleStructItems<T>>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            path: src.0,
            items: src.1 .0 .0,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TupleStructItems<T: Parsable>(pub Vec<Pattern<T>>);
impl<T: Parsable> Debug for TupleStructItems<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TupleStructItems").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for TupleStructItems<T> {
    type Source = (MinLength<Interlace<Pattern<T>, Comma>>, Option<Comma>);

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

pub struct TuplePattern<T: Parsable>(pub Option<TuplePatternItems<T>>);
impl<T: Parsable> Debug for TuplePattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TuplePattern").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for TuplePattern<T> {
    type Source = (PathInExpression<T>, Paren<Option<TuplePatternItems<T>>>);

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

pub enum TuplePatternItems<T: Parsable> {
    Fields(Vec<Pattern<T>>),
    Rest,
}
impl<T: Parsable> Debug for TuplePatternItems<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fields(arg0) => f.debug_tuple("Fields").field(arg0).finish(),
            Self::Rest => write!(f, "Rest"),
        }
    }
}
impl<T: Parsable> MappedParse for TuplePatternItems<T> {
    type Source = Sum2<
        Sum2<RestPattern, (Pattern<T>, Comma)>,
        (MinLength<Interlace<Pattern<T>, Comma>, 2>, Option<Comma>),
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

pub struct StructPattern<T: Parsable> {
    pub path: PathInExpression<T>,

    pub et_cetera: bool,

    pub fields: Vec<StructPatternField<T>>,
}
impl<T: Parsable> Debug for StructPattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructPattern")
            .field("path", &self.path)
            .field("et_cetera", &self.et_cetera)
            .field("fields", &self.fields)
            .finish()
    }
}
impl<T: Parsable> MappedParse for StructPattern<T> {
    type Source = (PathInExpression<T>, Brace<Option<StructPatternElements<T>>>);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 .0 {
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

pub enum StructPatternElements<T: Parsable> {
    StructPatternEtCetera(StructPatternEtCetera<T>),
    StructPatternFields(StructPatternFields<T>, Option<StructPatternEtCetera<T>>),
}
impl<T: Parsable> MappedParse for StructPatternElements<T> {
    type Source = Sum2<
        (
            StructPatternFields<T>,
            Option<Sum2<Comma, (Comma, StructPatternEtCetera<T>)>>,
        ),
        StructPatternEtCetera<T>,
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

pub enum StructPatternField<T: Parsable> {
    Tuple {
        attrs: Attrs<T>,
        id: IntegerLit,
        pattern: Pattern<T>,
    },
    Id {
        attrs: Attrs<T>,
        id: Ident,
        pattern: Pattern<T>,
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
                .field("ref", r#ref)
                .field("mut", r#mut)
                .field("id", id)
                .finish(),
        }
    }
}

impl<T: Parsable> MappedParse for StructPatternField<T> {
    type Source = WithAttrs<
        T,
        Sum2<
            Sum2<(TupleIndex, Colon, Pattern<T>), (Identifier, Colon, Pattern<T>)>,
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

pub struct StructPatternEtCetera<T: Parsable>(pub Attrs<T>);
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

pub struct StructPatternFields<T: Parsable>(pub Vec<StructPatternField<T>>);
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

pub struct ReferencePattern<T: Parsable> {
    pub ref_count: usize,
    pub r#mut: bool,
    pub pattern: PatternWithoutRange<T>,
}
impl<T: Parsable> Debug for ReferencePattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReferencePattern")
            .field("ref_count", &self.ref_count)
            .field("mut", &self.r#mut)
            .field("pattern", &self.pattern)
            .finish()
    }
}
impl<T: Parsable> MappedParse for ReferencePattern<T> {
    type Source = (Sum2<Amp, (Amp, Amp)>, Option<KwMut>, PatternWithoutRange<T>);

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

pub struct IdentifierPattern<T: Parsable> {
    pub r#ref: bool,
    pub r#mut: bool,

    pub id: Ident,

    pub at_pattern: Option<PatternNoTopAlt<T>>,
}
impl<T: Parsable> Debug for IdentifierPattern<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdentifierPattern")
            .field("ref", &self.r#ref)
            .field("mut", &self.r#mut)
            .field("id", &self.id)
            .field("at_pattern", &self.at_pattern)
            .finish()
    }
}
impl<T: Parsable> MappedParse for IdentifierPattern<T> {
    type Source = (
        Option<KwRef>,
        Option<KwMut>,
        Identifier,
        Option<(At, PatternNoTopAlt<T>)>,
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
