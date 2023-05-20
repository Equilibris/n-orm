use super::*;
use crate::*;
use std::fmt::Debug;

pub type AssociateItems<T = Tokens> = Vec<AssociateItem<T>>;

pub enum AssociateItem<T: Parsable = Tokens> {
    MacroInvocation(Attrs<T>, MacroInvocationSemi),
    TypeAlias(Attrs<T>, Option<Visibility>, TypeAlias),
    ConstantItem(Attrs<T>, Option<Visibility>, ConstantItem),
    Function(Attrs<T>, Option<Visibility>, Function<T>),
}
impl<T: Parsable> MappedParse for AssociateItem<T> {
    type Source = WithAttrs<
        T,
        Sum2<
            MacroInvocationSemi,
            (
                Option<Visibility>,
                Sum3<MBox<TypeAlias>, MBox<ConstantItem>, MBox<Function<T>>>,
            ),
        >,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Sum2::Val0(a) => Self::MacroInvocation(src.0, a),
            Sum2::Val1((vis, Sum3::Val0(a))) => Self::TypeAlias(src.0, vis, a),
            Sum2::Val1((vis, Sum3::Val1(a))) => Self::ConstantItem(src.0, vis, a),
            Sum2::Val1((vis, Sum3::Val2(a))) => Self::Function(src.0, vis, a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable> Debug for AssociateItem<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacroInvocation(arg0, arg1) => f
                .debug_tuple("MacroInvocation")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::TypeAlias(arg0, arg1, arg2) => f
                .debug_tuple("TypeAlias")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::ConstantItem(arg0, arg1, arg2) => f
                .debug_tuple("ConstantItem")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    #[test]
    pub fn sm_size_prune() {
        dbg!(std::mem::size_of::<<AssociateItem as Parsable>::StateMachine>());
        dbg!(std::mem::size_of::<<Function as Parsable>::StateMachine>());
    }
}
