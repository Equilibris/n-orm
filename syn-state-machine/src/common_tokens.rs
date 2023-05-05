mod attr;
mod bounds;
mod expr;
mod extern_crate;
mod genetic_params;
mod identifier;
mod keyword;
mod lifetime;
mod modules;
mod path;
mod use_declarations;
mod visability;
mod macro_invocation {
    use super::*;
    use crate::*;

    pub struct MacroInvocation {
        path: SimplePath,
        content: TokenTree,
    }

    impl MappedParse for MacroInvocation {
        type Source = (SimplePath, Exclamation, TokenTree);

        type Output = Self;
        type Error = SmError<Self::Source>;

        fn map(
            src: SmOutput<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(Self {
                path: src.0,
                content: src.2,
            })
        }

        fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
            src
        }
    }

    pub struct MacroInvocationSemi {
        path: SimplePath,
        content: Tokens,
    }

    impl MappedParse for MacroInvocationSemi {
        type Source = (
            SimplePath,
            Either<(Either<Parenthesis<Tokens>, Bracket<Tokens>>, Semi), Brace<Tokens>>,
        );

        type Output = Self;
        type Error = SmError<Self::Source>;

        fn map(
            src: SmOutput<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(Self {
                path: src.0,
                content: match src.1 {
                    Either::Left(Either::Right(a)) => a,
                    Either::Left(Either::Left(a)) => a,
                    Either::Right(a) => a,
                },
            })
        }

        fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
            todo!()
        }
    }
}
mod complex_types {
    use super::*;
    use crate::*;

    pub struct ImplTraitType;
    pub struct TraitBound;

    impl MappedParse for ImplTraitType {
        type Source = (KwImpl, TypeParamBounds);

        type Output = TypeParamBounds;
        type Error = SmError<Self::Source>;

        fn map(
            src: SmOutput<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            todo!()
        }

        fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
            todo!()
        }
    }
}
mod types {
    use super::*;
    use crate::*;

    pub enum Type {
        // ImplTrait(ImplTrait),
        // TraitObjectType(todo!()),
        UnboundedType(TypeNoBound),
    }
    pub enum TypeNoBound {
        Parenthesized(Box<Self>),
        // ImplOneBound(Box<Self>),
        // DynOneBound(Box<Self>),
        Path(SmOutput<TypePath>),
        Tuple(Vec<Type>),
        Never,
        RawPointer {
            is_mut: bool,
            inner: Box<TypeNoBounds>,
        },
        Refrence {
            is_mut: bool,
            inner: Box<TypeNoBounds>,
        },
        Array {
            ty: Box<Type>,
            expr: Expression,
        },
        Slice(Box<Type>),
        Inferred,
        QualifiedType(QualifiedPathType),
        // BareFunctionType(...),
        MacroInvocation(MacroInvocation),
    }

    pub enum TypeNoBounds {}
    impl MappedParse for TypeNoBounds {}
}
mod punctual {
    use crate::*;

    pub type Eq = FPunct<'='>;
    pub type Lt = FPunct<'<'>;
    pub type Gt = FPunct<'>'>;
    pub type Semi = FPunct<';'>;
    pub type Comma = FPunct<','>;
    pub type Star = FPunct<'*'>;
    pub type Colon = FPunct<':'>;
    pub type Exclamation = FPunct<'!'>;
    pub type DoubleColon = (FJointPunct<':'>, FPunct<':'>);
}

pub use attr::*;
pub use bounds::*;
pub use expr::*;
pub use extern_crate::*;
pub use genetic_params::*;
pub use identifier::*;
pub use keyword::*;
pub use lifetime::*;
pub use macro_invocation::*;
pub use path::*;
pub use punctual::*;
pub use types::*;
pub use visability::*;
