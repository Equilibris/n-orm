use std::marker::PhantomData;

use either::Either;

use crate::*;

pub struct AndNot<Valid: Parsable, Not: Parsable>(PhantomData<(Valid, Not)>);

impl<Valid: Parsable, Not: Parsable> Parsable for AndNot<Valid, Not> {
    type StateMachine = AndNotMachine<Valid::StateMachine, Not::StateMachine>;
}

pub struct AndNotMachine<Valid: StateMachine, Not: StateMachine> {
    v: Either<Valid, SmResult<Valid::Output, Valid::Error>>,
    n: Either<Not, SmResult<Not::Output, Not::Error>>,
}

impl<Valid: StateMachine, Not: StateMachine> Default for AndNotMachine<Valid, Not> {
    fn default() -> Self {
        Self {
            v: Either::Left(Default::default()),
            n: Either::Left(Default::default()),
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
        use Either::*;

        let v = match self.v {
            Left(l) => match l.drive(val) {
                Continue(c) => Left(c),
                Break(b) => Right(b),
            },
            Right(Ok((r, l))) => Right(Ok((r, l + 1))),
            r => r,
        };

        let n = match self.n {
            Left(l) => match l.drive(val) {
                Continue(c) => Left(c),
                Break(b) => Right(b),
            },
            Right(Ok((r, l))) => Right(Ok((r, l + 1))),
            r => r,
        };

        match (v, n) {
            (Right(Ok(v)), Right(Err(_))) => Break(Ok(v)),
            (Right(Ok(v)), Right(Ok(n))) => Break(Err(AndNotError::FailedCondition(v, n))),
            (Right(Err(v)), Right(_)) => Break(Err(AndNotError::ValidFailed(v))),
            (v, n) => Continue(Self { v, n }),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        use Either::*;

        let v = match self.v {
            Left(l) => l.terminate(),
            Right(r) => r,
        };
        let n = match self.n {
            Left(l) => l.terminate(),
            Right(r) => r,
        };

        match (v, n) {
            (Ok(v), Err(_)) => Ok(v),
            (Ok(v), Ok(n)) => Err(AndNotError::FailedCondition(v, n)),
            (Err(v), _) => Err(AndNotError::ValidFailed(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_matches_specific_idents() {
        type V = AndNot<Ident, FIdent<"struct">>;

        let (id, _) = parse::<V>(quote::quote! { pub }).unwrap();
        assert_eq!(id.to_string().as_str(), "pub");

        parse::<V>(quote::quote! { struct }).unwrap_err();
    }
}
