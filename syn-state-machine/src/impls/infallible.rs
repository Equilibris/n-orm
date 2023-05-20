use crate::*;

pub use std::convert::Infallible;

#[derive(Debug, thiserror::Error, Default)]
pub enum ReachedToken {
    #[error("Expected to not attempt construction but got '{}'", .0)]
    Value(TokenTree),
    #[default]
    #[error("Expected to not attempt construction but got termination")]
    Termination,
}

impl Parsable for std::convert::Infallible {
    type StateMachine = InfM;
}

#[derive(Default)]
pub struct InfM;

impl StateMachine for InfM {
    type Output = std::convert::Infallible;
    type Error = ReachedToken;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        ControlFlow::Break(Err(ReachedToken::Value(val.clone())))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Default::default())
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}Infallible", "  ".repeat(depth));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(it_fails_on_all, Infallible:);
}
