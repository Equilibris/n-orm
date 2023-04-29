use crate::*;

impl<A: Parsable> Parsable for Vec<A> {
    type StateMachine = VecMachine<A::StateMachine>;
}

pub struct VecMachine<A: StateMachine> {
    contents: Vec<A::Output>,
    machine: A,

    history: Vec<TokenTree>,
    checkpoint: usize,
}

impl<A: StateMachine> VecMachine<A> {
    fn process_value_stepup(
        self,
        mut rl: usize,
    ) -> ControlFlow<SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error>, Self>
    {
        use ControlFlow::*;

        let Self {
            mut contents,
            mut machine,

            history,
            mut checkpoint,
        } = self;

        let len = history.len();

        while rl > 0 {
            match machine.drive(&history[len - rl]) {
                Continue(c) => machine = c,
                Break(Ok((ok, backtrack))) => {
                    rl += backtrack + 1;
                    checkpoint = rl - 1;

                    contents.push(ok);
                    machine = A::default();
                }
                Break(Err(_)) => return Break(Ok((contents, checkpoint))),
            }
            rl -= 1;
        }

        Continue(Self {
            contents,
            machine,
            history,
            checkpoint,
        })
    }
}

impl<A: StateMachine> Default for VecMachine<A> {
    fn default() -> Self {
        Self {
            contents: Default::default(),
            machine: Default::default(),

            history: Default::default(),
            checkpoint: Default::default(),
        }
    }
}

// Maybe there should be a limit on this if it goes back to the original checkpoint twice on
// success??
// #[derive(Default, Debug, thiserror::Error)]
// #[error("Detected infinite loop")]
// pub struct VecError;

impl<A: StateMachine> StateMachine for VecMachine<A> {
    type Output = Vec<A::Output>;
    type Error = std::convert::Infallible;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        use ControlFlow::*;

        let Self {
            mut contents,
            machine,

            mut history,
            mut checkpoint,
        } = self;
        history.push(val.clone());
        checkpoint += 1;

        match machine.drive(val) {
            Continue(machine) => Continue(Self {
                contents,
                machine,
                history,
                checkpoint,
            }),
            Break(Ok((v, rl))) => {
                checkpoint = rl;
                contents.push(v);

                Self {
                    contents,
                    machine: A::default(),
                    history,
                    checkpoint,
                }
                .process_value_stepup(rl)
            }
            Break(Err(_)) => Break(Ok((contents, checkpoint))),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        use ControlFlow::*;

        let Self {
            mut contents,
            machine,

            history,
            mut checkpoint,
        } = self;

        match machine.terminate() {
            Ok((v, rl)) => {
                checkpoint = rl;
                contents.push(v);

                match (Self {
                    contents,
                    machine: A::default(),
                    history,
                    checkpoint,
                }
                .process_value_stepup(rl))
                {
                    Continue(c) => c.terminate(),
                    Break(b) => b,
                }
            }
            Err(_) => Ok((contents, checkpoint)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_matches_basic_iteration() {
        let (ls, l) = parse::<Vec<Ident>>(quote::quote! { hello world hi }).unwrap();

        assert_eq!(l, 0);
        assert_eq!(ls.len(), 3);
    }

    #[test]
    fn it_specifies_correct_backstep() {
        let (ls, l) = parse::<Vec<(Ident, Ident)>>(quote::quote! { hello world hi }).unwrap();

        assert_eq!(l, 1);
        assert_eq!(ls.len(), 1);
    }
    #[test]
    fn it_can_work_on_individual_backtracks() {
        let (ls, l) =
            parse::<Vec<(Ident, Option<Punct>)>>(quote::quote! { hello world hi }).unwrap();

        assert_eq!(l, 0);
        assert_eq!(ls.len(), 3);
    }
}
