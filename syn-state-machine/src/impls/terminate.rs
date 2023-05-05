use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Terminate;

#[derive(Debug, Clone, thiserror::Error)]
#[error("Expected termination but got {:#?}",.0)]
pub struct TerminateError(TokenTree);

impl Parsable for Terminate {
    type StateMachine = Self;
}
impl StateMachine for Terminate {
    type Output = Self;
    type Error = TerminateError;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        ControlFlow::Break(Err(TerminateError(val.clone())))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Ok((Self, 0))
    }
}
