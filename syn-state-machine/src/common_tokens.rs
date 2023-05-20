mod attr;
mod bounds;
mod constant_items;
mod enumerations;
mod static_items;
mod structs;
mod traits;
mod unions;
mod implementatins {
    use super::*;
    use crate::*;
    use std::fmt::Debug;

    pub enum Implementation<T: Parsable = Tokens> {
        Inherent(InherentImpl<T>),
        Trait(TraitImpl<T>),
    }
    impl<T: Parsable> Debug for Implementation<T>
    where
        SmOut<T>: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Inherent(arg0) => f.debug_tuple("Inherent").field(arg0).finish(),
                Self::Trait(arg0) => f.debug_tuple("Trait").field(arg0).finish(),
            }
        }
    }
    impl<T: Parsable> MappedParse for Implementation<T> {
        type Source = Sum2<InherentImpl<T>, TraitImpl<T>>;

        type Output = Self;
        type Error = SmErr<Self::Source>;

        fn map(
            src: SmOut<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(match src {
                Sum2::Val0(a) => Self::Inherent(a),
                Sum2::Val1(a) => Self::Trait(a),
            })
        }

        fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
            src
        }
    }

    pub struct TraitImpl<T: Parsable = Tokens> {
        pub r#unsafe: bool,
        pub genetic_params: Option<GenericParams>,
        pub where_clause: Option<WhereClause>,
        pub neg: bool,
        pub r#trait: TypePath,
        pub ty: Type,

        pub attrs: InnerAttrs<T>,
        pub items: AssociateItems<T>,
    }

    impl<T: Parsable> Debug for TraitImpl<T>
    where
        SmOut<T>: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TraitImpl")
                .field("r#unsafe", &self.r#unsafe)
                .field("genetic_params", &self.genetic_params)
                .field("neg", &self.neg)
                .field("r#trait", &self.r#trait)
                .field("ty", &self.ty)
                .field("attrs", &self.attrs)
                .field("items", &self.items)
                .finish()
        }
    }
    impl<T: Parsable> MappedParse for TraitImpl<T> {
        type Source = (
            Option<KwUnsafe>,
            KwImpl,
            Option<MBox<GenericParams>>,
            Option<Exclamation>,
            MBox<TypePath>,
            KwFor,
            MBox<Type>,
            Option<WhereClause>,
            Brace<WithInnerAttrs<T, AssociateItems<T>>>,
        );

        type Output = Self;
        type Error = SmErr<Self::Source>;

        fn map(
            src: SmOut<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(Self {
                r#unsafe: src.0.is_some(),
                genetic_params: src.2,
                where_clause: src.7,
                neg: src.3.is_some(),
                r#trait: src.4,
                ty: src.6,
                attrs: src.8 .0,
                items: src.8 .1,
            })
        }

        fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
            src
        }
    }

    pub struct InherentImpl<T: Parsable = Tokens> {
        genetic_params: Option<GenericParams>,
        ty: Type,
        where_clause: Option<WhereClause>,

        attrs: InnerAttrs<T>,
        items: AssociateItems<T>,
    }
    impl<T: Parsable> MappedParse for InherentImpl<T> {
        type Source = (
            KwImpl,
            Option<GenericParams>,
            Type,
            Option<WhereClause>,
            Brace<WithInnerAttrs<T, AssociateItems<T>>>,
        );

        type Output = Self;
        type Error = SmErr<Self::Source>;

        fn map(
            src: SmOut<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(Self {
                genetic_params: src.1,
                ty: src.2,
                where_clause: src.3,
                attrs: src.4 .0,
                items: src.4 .1,
            })
        }

        fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
            src
        }
    }
    impl<T: Parsable> Debug for InherentImpl<T>
    where
        SmOut<T>: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("InherentImpl")
                .field("genetic_params", &self.genetic_params)
                .field("ty", &self.ty)
                .field("where_clause", &self.where_clause)
                .field("attrs", &self.attrs)
                .field("items", &self.items)
                .finish()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::insta_match_test;

        #[test]
        pub fn sm_size_prune() {
            dbg!(std::mem::size_of::<
                <Implementation as Parsable>::StateMachine,
            >());
        }
        insta_match_test!(
            it_matches_simple_inherent, Implementation :

            impl<T> Option<T> {
                pub fn is_some(&self) -> bool;
            }
        );
        insta_match_test!(
            it_matches_simple_trait, Implementation :

            unsafe impl<T: Copy> Copy for Option<T> {}
        );
    }
}
mod external_blocks {
    use super::*;
    use crate::*;
    use std::fmt::Debug;

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::insta_match_test;
    }
}
mod associate_items;
mod expr;
mod extern_crate;
mod function_pointer_types;
mod functions;
mod genetic_params;
mod identifier;
mod impl_dyn_trait;
mod keyword;
mod lifetime;
mod macro_invocation;
mod modules;
mod path;
mod patterns;
mod type_alias;
mod types;
mod use_declarations;
mod visibility;
mod where_clause;
mod punctual {
    use crate::*;

    pub type Eq = FPunct<'='>;
    pub type Minus = FPunct<'-'>;
    pub type Pipe = FPunct<'|'>;
    pub type At = FPunct<'@'>;
    pub type Amp = FPunct<'&'>;
    pub type Lt = FPunct<'<'>;
    pub type Gt = FPunct<'>'>;
    pub type Semi = FPunct<';'>;
    pub type Comma = FPunct<','>;
    pub type Star = FPunct<'*'>;
    pub type Colon = FPunct<':'>;
    pub type JColon = FJointPunct<':'>;
    pub type Exclamation = FPunct<'!'>;
    pub type Plus = FPunct<'+'>;
    pub type DoubleColon = (JColon, Colon);
    pub type Dot = FPunct<'.'>;
    pub type JDot = FJointPunct<'.'>;
    pub type Elipsis = (JDot, JDot, Dot);
    pub type DotDot = (JDot, Dot);
    pub type DotDotEq = (JDot, JDot, Eq);
    pub type Arrow = (FJointPunct<'-'>, FPunct<'>'>);

    pub type Underscore = FIdent<"_">;
}

pub type TupleIndex = crate::IntegerLit;

pub use associate_items::*;
pub use attr::*;
pub use bounds::*;
pub use constant_items::*;
pub use enumerations::*;
pub use expr::*;
pub use extern_crate::*;
pub use external_blocks::*;
pub use function_pointer_types::*;
pub use functions::*;
pub use genetic_params::*;
pub use identifier::*;
pub use impl_dyn_trait::*;
pub use implementatins::*;
pub use keyword::*;
pub use lifetime::*;
pub use macro_invocation::*;
pub use path::*;
pub use patterns::*;
pub use punctual::*;
pub use static_items::*;
pub use structs::*;
pub use structs::*;
pub use traits::*;
pub use type_alias::*;
pub use types::*;
pub use unions::*;
pub use unions::*;
pub use visibility::*;
pub use where_clause::*;
//
/*
impl MappedParse for CopyPase {
    type Source = ();

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

impl<T: Parsable> Debug for CopyPaste<T> where SmOut<T>: Debug {}
impl<T: Parsable> MappedParse for CopyPase<T> {
    type Source = ();

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
  */
