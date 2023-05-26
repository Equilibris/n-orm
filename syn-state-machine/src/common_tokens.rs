mod associate_items;
mod attr;
mod bounds;
mod constant_items;
mod enumerations;
mod expr;
mod extern_crate;
mod external_blocks;
mod function_pointer_types;
mod functions;
mod genetic_params;
mod identifier;
mod impl_dyn_trait;
mod implementatins;
mod items;
mod keyword;
mod lifetime;
mod macro_invocation;
mod macro_rules;
mod modules;
mod path;
mod patterns;
mod static_items;
mod structs;
mod traits;
mod type_alias;
mod types;
mod unions;
mod use_declarations;
mod visibility;
mod where_clause;
mod ad_hoc_type {
    use crate::*;

    pub struct AdHocType(pub Vec<TokenTree>);

    #[derive(Debug, thiserror::Error)]
    pub enum AdHocTypeErr {
        #[error("Expected {} but got termination", ">".repeat(*.0))]
        UnexpectedTermination(usize),

        #[error("Type cannot be zero length")]
        ZeroLengthType,
    }

    #[derive(Default)]
    pub struct AdHotTypeM {
        content: Vec<TokenTree>,
        depth: usize,
    }
    impl StateMachine for AdHotTypeM {
        type Output = AdHocType;
        type Error = AdHocTypeErr;

        fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
            let Self { mut content, depth } = self;

            match val {
                TokenTree::Punct(a) if depth == 0 => match a.as_char() {
                    '<' => ControlFlow::Continue(Self {
                        content: {
                            content.push(val.clone());
                            content
                        },
                        depth: depth + 1,
                    }),

                    ',' => ControlFlow::Break(Ok((AdHocType(content), 1))),
                    '>' => ControlFlow::Break(Ok((AdHocType(content), 1))),

                    _ => ControlFlow::Continue(Self {
                        content: {
                            content.push(val.clone());
                            content
                        },
                        depth,
                    }),
                },
                TokenTree::Punct(a) if a.as_char() == '>' => ControlFlow::Continue(Self {
                    content: {
                        content.push(val.clone());
                        content
                    },
                    depth: depth - 1,
                }),
                TokenTree::Punct(a) if a.as_char() == '<' => ControlFlow::Continue(Self {
                    content: {
                        content.push(val.clone());
                        content
                    },
                    depth: depth + 1,
                }),
                _ => ControlFlow::Continue(Self {
                    content: {
                        content.push(val.clone());
                        content
                    },
                    depth,
                }),
            }
        }

        fn terminate(self) -> SmResult<Self::Output, Self::Error> {
            let Self { content, depth } = self;

            if depth == 0 {
                Ok((AdHocType(content), 0))
            } else if content.len() == 0 {
                Err(AdHocTypeErr::ZeroLengthType)
            } else {
                Err(AdHocTypeErr::UnexpectedTermination(depth))
            }
        }
    }
}
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
    pub type Dollar = FPunct<'$'>;
    pub type DoubleColon = (JColon, Colon);
    pub type Dot = FPunct<'.'>;
    pub type JDot = FJointPunct<'.'>;
    pub type Elipsis = (JDot, JDot, Dot);
    pub type DotDot = (JDot, Dot);
    pub type DotDotEq = (JDot, JDot, Eq);
    pub type Arrow = (FJointPunct<'-'>, FPunct<'>'>);
    pub type FatArrow = (FJointPunct<'='>, FPunct<'>'>);

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
pub use items::*;
pub use keyword::*;
pub use lifetime::*;
pub use macro_invocation::*;
pub use macro_rules::*;
pub use modules::*;
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
pub use use_declarations::*;
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
