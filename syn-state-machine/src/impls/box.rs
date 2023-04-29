use crate::*;

impl<A: Parsable> Parsable for Box<A> {
    type StateMachine = BoxMachine<A::StateMachine>;
}
#[derive(Default)]
pub struct BoxMachine<A: StateMachine>(Box<A>);

impl<A: StateMachine> StateMachine for BoxMachine<A> {
    type Output = Box<A::Output>;
    type Error = Box<A::Error>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        self.0
            .drive(val)
            .map_break(|v| v.map_err(Box::new).map(|(v, rl)| (Box::new(v), rl)))
            .map_continue(|v| Self(Box::new(v)))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        self.0
            .terminate()
            .map_err(Box::new)
            .map(|(v, rl)| (Box::new(v), rl))
    }
}
