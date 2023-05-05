use crate::*;
pub use either::Either;

impl<A: Parsable, B: Parsable> Parsable for Either<A, B> {
    type StateMachine = EitherMachine<A::StateMachine, B::StateMachine>;
}
// This either-machine short circuits to the left (first) value
pub struct EitherMachine<A: StateMachine, B: StateMachine> {
    a: Either<A, SmResult<A::Output, A::Error>>,
    b: Either<B, SmResult<B::Output, B::Error>>,
}

impl<A: StateMachine, B: StateMachine> Default for EitherMachine<A, B> {
    fn default() -> Self {
        Self {
            a: Either::Left(Default::default()),
            b: Either::Left(Default::default()),
        }
    }
}

#[derive(Clone, thiserror::Error, Debug)]
#[error("e1: ({}) e2: ({})", .0, .1)]
pub struct EitherParsingError<A: std::error::Error, B: std::error::Error>(pub A, pub B);

impl<A: StateMachine, B: StateMachine> StateMachine for EitherMachine<A, B> {
    type Output = Either<A::Output, B::Output>;
    type Error = EitherParsingError<A::Error, B::Error>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        use ControlFlow::*;
        use Either::*;

        let a = match self.a {
            Left(v) => match v.drive(val) {
                Continue(v) => Left(v),
                Break(Ok((v, rl))) => return Break(Ok((Left(v), rl))),
                Break(v) => Right(v),
            },
            Right(Ok((v, run_length))) => Right(Ok((v, run_length + 1))),
            e => e,
        };
        let b = match self.b {
            Left(v) => match v.drive(val) {
                Continue(v) => Left(v),
                Break(v) => Right(v),
            },
            Right(Ok((v, run_length))) => Right(Ok((v, run_length + 1))),
            e => e,
        };

        match (a, b) {
            (Right(Ok((a, rl))), Right(_)) => Break(Ok((Left(a), rl))),
            (Right(Err(_)), Right(Ok((b, rl)))) => Break(Ok((Right(b), rl))),
            (Right(Err(a)), Right(Err(b))) => Break(Err(EitherParsingError(a, b))),
            (a, b) => Continue(Self { a, b }),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        use Either::*;

        let a = match self.a {
            Left(v) => match v.terminate() {
                Ok((v, rl)) => return Ok((Left(v), rl)),
                v => v,
            },
            Right(v) => v,
        };

        let b = match self.b {
            Left(v) => v.terminate(),
            Right(v) => v,
        };

        match (a, b) {
            (Ok((a, rl)), _) => Ok((Left(a), rl)),
            (Err(_), Ok((b, rl))) => Ok((Right(b), rl)),
            (Err(a), Err(b)) => Err(EitherParsingError(a, b)),
        }
    }
}
