use crate::*;
pub use proc_macro2::Literal;

impl Parsable for Literal {
    type StateMachine = LiteralMachine;
}

#[derive(Default)]
pub struct LiteralMachine;

#[derive(Debug, Clone, thiserror::Error, Default)]
pub enum LiteralError {
    #[error("Expected literal but got {}", .0)]
    Val(TokenTree),
    #[default]
    #[error("Expected literal but got termination")]
    Termination,
}

impl StateMachine for LiteralMachine {
    type Output = Literal;
    type Error = LiteralError;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        match val {
            TokenTree::Literal(p) => ControlFlow::Break(Ok((p.clone(), 0))),
            _ => ControlFlow::Break(Err(LiteralError::default())),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Default::default())
    }
}
