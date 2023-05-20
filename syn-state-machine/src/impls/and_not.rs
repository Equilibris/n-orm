use std::marker::PhantomData;

use crate::*;

pub struct AndNot<Valid: Parsable, Not: Parsable>(PhantomData<(Valid, Not)>);

impl<Valid: Parsable, Not: Parsable> Parsable for AndNot<Valid, Not> {
    type StateMachine = AndNotMachine<Valid::StateMachine, Not::StateMachine>;
}

pub struct AndNotMachine<Valid: StateMachine, Not: StateMachine> {
    primary: Sum2<Valid, SmResult<Valid::Output, Valid::Error>>,
    negation: Sum2<Not, SmResult<Not::Output, Not::Error>>,
}

impl<Valid: StateMachine, Not: StateMachine> Default for AndNotMachine<Valid, Not> {
    fn default() -> Self {
        Self {
            primary: Sum2::Val0(Default::default()),
            negation: Sum2::Val0(Default::default()),
        }
    }
}

#[derive(thiserror::Error)]
pub enum AndNotError<Valid: StateMachine, Not: StateMachine> {
    #[error("Valid path failed: {}", .0)]
    ValidFailed(Valid::Error),
    #[error("Both paths passed")]
    FailedCondition((Valid::Output, usize), (Not::Output, usize)),
}

impl<Valid: StateMachine, Not: StateMachine> std::fmt::Debug for AndNotError<Valid, Not> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidFailed(arg0) => f.debug_tuple("ValidFailed").field(arg0).finish(),
            Self::FailedCondition(_, _) => f
                .debug_tuple("ValidPassedButErrorAlsoPassed")
                .field(&"<Hidden>")
                .field(&"<Hidden>")
                .finish(),
        }
    }
}

impl<Valid: StateMachine, Not: StateMachine> StateMachine for AndNotMachine<Valid, Not> {
    type Output = Valid::Output;
    type Error = AndNotError<Valid, Not>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        use ControlFlow::*;
        use Sum2::*;

        let v = match self.primary {
            Val0(l) => match l.drive(val) {
                Continue(c) => Val0(c),
                Break(b) => Val1(b),
            },
            Val1(Ok((r, l))) => Val1(Ok((r, l + 1))),
            r => r,
        };

        let n = match self.negation {
            Val0(l) => match l.drive(val) {
                Continue(c) => Val0(c),
                Break(b) => Val1(b),
            },
            Val1(Ok((r, l))) => Val1(Ok((r, l + 1))),
            r => r,
        };

        match (v, n) {
            (Val1(Ok(v)), Val1(Err(_))) => Break(Ok(v)),
            (Val1(Ok(v)), Val1(Ok(n))) => Break(Err(AndNotError::FailedCondition(v, n))),
            (Val1(Err(v)), Val1(_)) => Break(Err(AndNotError::ValidFailed(v))),
            (v, n) => Continue(Self {
                primary: v,
                negation: n,
            }),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        use Sum2::*;

        let v = match self.primary {
            Val0(l) => l.terminate(),
            Val1(r) => r,
        };
        let n = match self.negation {
            Val0(l) => l.terminate(),
            Val1(r) => r,
        };

        match (v, n) {
            (Ok(v), Err(_)) => Ok(v),
            (Ok(v), Ok(n)) => Err(AndNotError::FailedCondition(v, n)),
            (Err(v), _) => Err(AndNotError::ValidFailed(v)),
        }
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}AndNot", "  ".repeat(depth));
        if let Sum2::Val0(ref v) = self.primary {
            println!("{}And:", "  ".repeat(depth + 1));
            v.inspect(depth + 2);
        }
        if let Sum2::Val0(ref v) = self.negation {
            println!("{}Not:", "  ".repeat(depth + 1));
            v.inspect(depth + 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    type V = AndNot<Ident, FIdent<"struct">>;

    insta_match_test!(it_matches_specific_idents, V: pub);
    insta_match_test!(it_fails_on_negation, V: struct);
}
