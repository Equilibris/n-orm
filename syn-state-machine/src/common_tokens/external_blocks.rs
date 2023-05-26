use super::*;
use crate::*;
use std::fmt::Debug;

pub struct ExternBlock<T: Parsable, Ty: Parsable> {
    r#unsafe: bool,
    abi: Option<Abi>,
    attrs: InnerAttrs<T>,
    items: Vec<ExternalItem<T, Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for ExternBlock<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternBlock")
            .field("unsafe", &self.r#unsafe)
            .field("abi", &self.abi)
            .field("attrs", &self.attrs)
            .field("items", &self.items)
            .finish()
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for ExternBlock<T, Ty> {
    type Source = (
        Option<KwUnsafe>,
        KwExtern,
        Option<Abi>,
        Brace<WithInnerAttrs<T, Vec<ExternalItem<T, Ty>>>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            r#unsafe: src.0.is_some(),
            abi: src.2,
            attrs: src.3 .0 .0,
            items: src.3 .0 .1,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum ExternalItem<T: Parsable, Ty: Parsable> {
    Macro(Attrs<T>, MacroInvocationSemi),
    Static(Attrs<T>, Option<Visibility>, StaticItem<Ty>),
    Function(Attrs<T>, Option<Visibility>, Function<T, Ty>),
}

impl<T: Parsable, Ty: Parsable> Debug for ExternalItem<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Macro(arg0, arg1) => f.debug_tuple("Macro").field(arg0).field(arg1).finish(),
            Self::Static(arg0, arg1, arg2) => f
                .debug_tuple("Static")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Function(arg0, arg1, arg2) => f
                .debug_tuple("Function")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for ExternalItem<T, Ty> {
    type Source = WithAttrs<
        T,
        Sum2<
            MacroInvocationSemi,
            (Option<Visibility>, Sum2<StaticItem<Ty>, Function<T, Ty>>),
        >,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Sum2::Val0(v) => Self::Macro(src.0, v),
            Sum2::Val1((a, Sum2::Val0(v))) => Self::Static(src.0, a, v),
            Sum2::Val1((a, Sum2::Val1(v))) => Self::Function(src.0, a, v),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;
    use std::convert::Infallible;

    insta_match_test!(it_matches_item, ExternalItem<Infallible,MBox<Type<Infallible>>>:
        fn with_name(format: *const u8);
    );

    insta_match_test!(it_matches_simple_extern_block, ExternBlock<Infallible,MBox<Type<Infallible>>>: 
    extern "C" {
        fn foo(x: i32);
        fn with_name(format: *const u8);
    } );
}

