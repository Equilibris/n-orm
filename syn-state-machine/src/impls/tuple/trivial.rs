use crate::*;

impl Parsable for () {
    type StateMachine = ();
}

impl StateMachine for () {
    type Output = ();

    type Error = std::convert::Infallible;

    fn drive(self, _: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        ControlFlow::Break(Ok(((), 1)))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Ok(((), 0))
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}Unit", "  ".repeat(depth));
    }
}

impl<A: Parsable> Parsable for (A,) {
    type StateMachine = (A::StateMachine,);
}

impl<A: StateMachine> StateMachine for (A,) {
    type Output = (A::Output,);

    type Error = A::Error;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        self.0
            .drive(val)
            .map_continue(|v| (v,))
            .map_break(|v| v.map(|(v, rl)| ((v,), rl)))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        self.0.terminate().map(|(v, rl)| ((v,), rl))
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        self.0.inspect(depth)
    }
}
