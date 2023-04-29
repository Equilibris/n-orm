use crate::*;

pub use proc_macro2::TokenTree;

impl Parsable for TokenTree {
    type StateMachine = TokenTreeMachine;
}

#[derive(Default)]
pub struct TokenTreeMachine;

#[derive(Debug, Clone, thiserror::Error, Default)]
#[error("Expected ident but got termination")]
pub struct TokenTreeError;

impl StateMachine for TokenTreeMachine {
    type Output = TokenTree;
    type Error = TokenTreeError;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        ControlFlow::Break(Ok((val.clone(), 0)))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Default::default())
    }
}
