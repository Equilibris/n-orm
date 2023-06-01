use std::fmt::Debug;

use super::*;
use crate::*;

pub mod range_patterns;
pub use range_patterns::*;

pub struct Pattern<T: Parsable, Ty: Parsable>(pub Vec<PatternNoTopAlt<T, Ty>>);
impl<T: Parsable, Ty: Parsable> Debug for Pattern<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pattern").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for Pattern<T, Ty> {
    type Source = (Option<Pipe>, Interlace<PatternNoTopAlt<T, Ty>, Pipe>);

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

pub enum PatternNoTopAlt<T: Parsable, Ty: Parsable> {
    PatternWithoutRange(PatternWithoutRange<T, Ty>),
    RangePattern(RangePattern<Ty>),
}
impl<T: Parsable, Ty: Parsable> Debug for PatternNoTopAlt<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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
impl<T: Parsable, Ty: Parsable> MappedParse for PatternNoTopAlt<T, Ty> {
    type Source = Sum2<PatternWithoutRange<T, Ty>, RangePattern<Ty>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::PatternWithoutRange(a),
            Sum2::Val1(a) => Self::RangePattern(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(thiserror::Error)]
#[error("Failed to match pattern")]
pub struct PatternError<T: Parsable, Ty: Parsable> {
    pub literal_pattern: SmErr<LiteralPattern>,
    pub identifier_pattern: Box<SmErr<IdentifierPattern<T, Ty>>>,
    pub wildcard_pattern: SmErr<WildcardPattern>,
    pub rest_pattern: SmErr<RestPattern>,
    pub reference_pattern: Box<SmErr<ReferencePattern<T, Ty>>>,
    pub struct_pattern: SmErr<StructPattern<T, Ty>>,
    pub tuple_struct_pattern: SmErr<TupleStructPattern<T, Ty>>,
    pub tuple_pattern: SmErr<TuplePattern<T, Ty>>,
    pub grouped_pattern: Box<SmErr<Paren<Pattern<T, Ty>>>>,
    pub slice_pattern: SmErr<SlicePattern<T, Ty>>,
    pub path_pattern: SmErr<PathPattern<T>>,
    pub macro_invocation: SmErr<MacroInvocation>,
}
impl<T: Parsable, Ty: Parsable> Debug for PatternError<T, Ty> {
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

pub enum PatternWithoutRange<T: Parsable, Ty: Parsable> {
    Literal(LiteralPattern),
    Identifier(Box<IdentifierPattern<T, Ty>>),
    Wildcard(WildcardPattern),
    Rest(RestPattern),
    Reference(Box<ReferencePattern<T, Ty>>),
    Struct(StructPattern<T, Ty>),
    TupleStruct(TupleStructPattern<T, Ty>),
    Tuple(TuplePattern<T, Ty>),
    Grouped(Box<Pattern<T, Ty>>),
    Slice(SlicePattern<T, Ty>),
    Path(PathPattern<T>),
    Macro(MacroInvocation),
}
impl<T: Parsable, Ty: Parsable> Debug for PatternWithoutRange<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.debug_tuple("LiteralPattern").field(arg0).finish(),
            Self::Identifier(arg0) => f.debug_tuple("IdentifierPattern").field(arg0).finish(),
            Self::Wildcard(arg0) => f.debug_tuple("WildcardPattern").field(arg0).finish(),
            Self::Rest(arg0) => f.debug_tuple("RestPattern").field(arg0).finish(),
            Self::Reference(arg0) => f.debug_tuple("ReferencePattern").field(arg0).finish(),
            Self::Struct(arg0) => f.debug_tuple("StructPattern").field(arg0).finish(),
            Self::TupleStruct(arg0) => f.debug_tuple("TupleStructPattern").field(arg0).finish(),
            Self::Tuple(arg0) => f.debug_tuple("TuplePattern").field(arg0).finish(),
            Self::Grouped(arg0) => f.debug_tuple("GroupedPattern").field(arg0).finish(),
            Self::Slice(arg0) => f.debug_tuple("SlicePattern").field(arg0).finish(),
            Self::Path(arg0) => f.debug_tuple("PathPattern").field(arg0).finish(),
            Self::Macro(arg0) => f.debug_tuple("MacroInvocation").field(arg0).finish(),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for PatternWithoutRange<T, Ty> {
    type Source = PBox<
        Sum12<
            LiteralPattern,
            IdentifierPattern<T, Ty>,
            WildcardPattern,
            RestPattern,
            ReferencePattern<T, Ty>,
            StructPattern<T, Ty>,
            TupleStructPattern<T, Ty>,
            TuplePattern<T, Ty>,
            Paren<Pattern<T, Ty>>,
            SlicePattern<T, Ty>,
            PathExpression<T>,
            MacroInvocation,
        >,
    >;

    type Output = Self;
    type Error = PatternError<T, Ty>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match *src {
            Sum12::Val0(a) => Self::Literal(a),
            Sum12::Val1(a) => Self::Identifier(Box::new(a)),
            Sum12::Val2(a) => Self::Wildcard(a),
            Sum12::Val3(a) => Self::Rest(a),
            Sum12::Val4(a) => Self::Reference(Box::new(a)),
            Sum12::Val5(a) => Self::Struct(a),
            Sum12::Val6(a) => Self::TupleStruct(a),
            Sum12::Val7(a) => Self::Tuple(a),
            Sum12::Val8(a) => Self::Grouped(Box::new(a.0)),
            Sum12::Val9(a) => Self::Slice(a),
            Sum12::Val10(a) => Self::Path(a),
            Sum12::Val11(a) => Self::Macro(a),
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

pub struct SlicePattern<T: Parsable, Ty: Parsable>(pub Vec<Pattern<T, Ty>>);
impl<T: Parsable, Ty: Parsable> Debug for SlicePattern<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SlicePattern").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for SlicePattern<T, Ty> {
    type Source = Bracket<Option<SlicePatternItems<T, Ty>>>;

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

pub struct SlicePatternItems<T: Parsable, Ty: Parsable>(pub Vec<Pattern<T, Ty>>);
impl<T: Parsable, Ty: Parsable> MappedParse for SlicePatternItems<T, Ty> {
    type Source = (MinLength<Interlace<Pattern<T, Ty>, Comma>>, Option<Comma>);

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

pub struct TupleStructPattern<T: Parsable, Ty: Parsable> {
    pub path: PathInExpression<Ty>,
    pub items: Vec<Pattern<T, Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for TupleStructPattern<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TupleStructPattern")
            .field("path", &self.path)
            .field("items", &self.items)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TupleStructPattern<T, Ty> {
    type Source = (PathInExpression<Ty>, Paren<TupleStructItems<T, Ty>>);

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

pub struct TupleStructItems<T: Parsable, Ty: Parsable>(pub Vec<Pattern<T, Ty>>);
impl<T: Parsable, Ty: Parsable> Debug for TupleStructItems<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TupleStructItems").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TupleStructItems<T, Ty> {
    type Source = (MinLength<Interlace<Pattern<T, Ty>, Comma>>, Option<Comma>);

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

pub struct TuplePattern<T: Parsable, Ty: Parsable>(
    pub PathInExpression<Ty>,
    pub Option<TuplePatternItems<T, Ty>>,
);
impl<T: Parsable, Ty: Parsable> Debug for TuplePattern<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TuplePattern").field(&self.0).finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TuplePattern<T, Ty> {
    type Source = (
        PathInExpression<Ty>,
        Paren<Option<TuplePatternItems<T, Ty>>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.1 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum TuplePatternItems<T: Parsable, Ty: Parsable> {
    Fields(Vec<Pattern<T, Ty>>),
    Rest,
}
impl<T: Parsable, Ty: Parsable> Debug for TuplePatternItems<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fields(arg0) => f.debug_tuple("Fields").field(arg0).finish(),
            Self::Rest => write!(f, "Rest"),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TuplePatternItems<T, Ty> {
    type Source = Sum3<
        RestPattern,
        (Pattern<T, Ty>, Comma),
        (
            MinLength<Interlace<Pattern<T, Ty>, Comma>, 2>,
            Option<Comma>,
        ),
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum3::Val0(_) => Self::Rest,
            Sum3::Val1(a) => Self::Fields(vec![a.0]),
            Sum3::Val2(a) => Self::Fields(a.0 .0),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct StructPattern<T: Parsable, Ty: Parsable> {
    pub path: PathInExpression<Ty>,

    pub et_cetera: bool,

    pub fields: Vec<StructPatternField<T, Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for StructPattern<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructPattern")
            .field("path", &self.path)
            .field("et_cetera", &self.et_cetera)
            .field("fields", &self.fields)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for StructPattern<T, Ty> {
    type Source = (
        PathInExpression<Ty>,
        Brace<Option<StructPatternElements<T, Ty>>>,
    );

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

pub enum StructPatternElements<T: Parsable, Ty: Parsable> {
    StructPatternEtCetera(StructPatternEtCetera<T>),
    StructPatternFields(StructPatternFields<T, Ty>, Option<StructPatternEtCetera<T>>),
}
impl<T: Parsable, Ty: Parsable> MappedParse for StructPatternElements<T, Ty> {
    type Source = Sum2<
        (
            StructPatternFields<T, Ty>,
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
            Sum2::Val0(a) => Self::StructPatternFields(
                a.0,
                a.1.and_then(|v| if let Sum2::Val1(a) = v { Some(a) } else { None })
                    .map(|v| v.1),
            ),
            Sum2::Val1(a) => Self::StructPatternEtCetera(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum StructPatternField<T: Parsable, Ty: Parsable> {
    Tuple {
        attrs: Attrs<T>,
        id: IntegerLit,
        pattern: Pattern<T, Ty>,
    },
    Id {
        attrs: Attrs<T>,
        id: Ident,
        pattern: Pattern<T, Ty>,
    },
    IdShorthand {
        attrs: Attrs<T>,
        r#ref: bool,
        r#mut: bool,
        id: Ident,
    },
}

impl<T: Parsable, Ty: Parsable> Debug for StructPatternField<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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

impl<T: Parsable, Ty: Parsable> MappedParse for StructPatternField<T, Ty> {
    type Source = WithAttrs<
        T,
        Sum3<
            (TupleIndex, Colon, Pattern<T, Ty>),
            (Identifier, Colon, Pattern<T, Ty>),
            (Option<KwRef>, Option<KwMut>, Identifier),
        >,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Sum3::Val0(a) => Self::Tuple {
                attrs: src.0,
                id: a.0,
                pattern: a.2,
            },
            Sum3::Val1(a) => Self::Id {
                attrs: src.0,
                id: a.0,
                pattern: a.2,
            },
            Sum3::Val2(a) => Self::IdShorthand {
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

pub struct StructPatternFields<T: Parsable, Ty: Parsable>(pub Vec<StructPatternField<T, Ty>>);
impl<T: Parsable, Ty: Parsable> MappedParse for StructPatternFields<T, Ty> {
    type Source = MinLength<Interlace<StructPatternField<T, Ty>, Comma>>;

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

pub struct ReferencePattern<T: Parsable, Ty: Parsable> {
    pub ref_count: usize,
    pub r#mut: bool,
    pub pattern: PatternWithoutRange<T, Ty>,
}
impl<T: Parsable, Ty: Parsable> Debug for ReferencePattern<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReferencePattern")
            .field("ref_count", &self.ref_count)
            .field("mut", &self.r#mut)
            .field("pattern", &self.pattern)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for ReferencePattern<T, Ty> {
    type Source = (
        Sum2<Amp, (Amp, Amp)>,
        Option<KwMut>,
        PatternWithoutRange<T, Ty>,
    );

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

pub struct IdentifierPattern<T: Parsable, Ty: Parsable> {
    pub r#ref: bool,
    pub r#mut: bool,

    pub id: Ident,

    pub at_pattern: Option<PatternNoTopAlt<T, Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for IdentifierPattern<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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
impl<T: Parsable, Ty: Parsable> MappedParse for IdentifierPattern<T, Ty> {
    type Source = (
        Option<KwRef>,
        Option<KwMut>,
        Identifier,
        Option<(At, PatternNoTopAlt<T, Ty>)>,
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
