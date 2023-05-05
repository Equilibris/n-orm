use crate::*;

#[derive(Debug)]
pub struct Interlace<A, B>(pub Vec<A>, pub Vec<B>);

impl<A: Parsable, B: Parsable> Parsable for Interlace<A, B> {
    type StateMachine = InterlaceMachine<A::StateMachine, B::StateMachine>;
}

pub struct InterlaceMachine<A: StateMachine, B: StateMachine> {
    contents_a: Vec<A::Output>,
    contents_b: Vec<B::Output>,

    b_parking: Option<B::Output>,

    machine: Either<A, B>,

    history: Vec<TokenTree>,
    checkpoint: usize,
}

impl<A: StateMachine, B: StateMachine> InterlaceMachine<A, B> {
    fn process_value_stepup(
        self,
        mut rl: usize,
    ) -> ControlFlow<SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error>, Self>
    {
        use ControlFlow::*;
        use Either::*;

        let Self {
            mut contents_a,
            mut contents_b,

            mut b_parking,

            mut machine,

            history,
            mut checkpoint,
        } = self;

        let len = history.len();

        while rl > 0 {
            match machine {
                Left(v) => match v.drive(&history[len - rl]) {
                    Continue(c) => machine = Left(c),
                    Break(Ok((ok, backtrack))) => {
                        rl += backtrack;
                        checkpoint = rl;

                        if let Some(v) = b_parking {
                            contents_b.push(v)
                        }
                        b_parking = None;

                        contents_a.push(ok);
                        machine = Right(B::default());
                    }
                    Break(Err(_)) => {
                        return Break(Ok((Interlace(contents_a, contents_b), checkpoint)));
                    }
                },
                Right(v) => match v.drive(&history[len - rl]) {
                    Continue(c) => machine = Right(c),
                    Break(Ok((ok, backtrack))) => {
                        rl += backtrack;

                        b_parking = Some(ok);
                        machine = Left(A::default());
                    }
                    Break(Err(_)) => {
                        return Break(Ok((Interlace(contents_a, contents_b), checkpoint)));
                    }
                },
            }
            rl -= 1;
        }

        Continue(Self {
            contents_a,
            contents_b,

            b_parking,

            machine,
            history,
            checkpoint,
        })
    }
}

impl<A: StateMachine, B: StateMachine> Default for InterlaceMachine<A, B> {
    fn default() -> Self {
        Self {
            contents_a: Default::default(),
            contents_b: Default::default(),

            b_parking: Default::default(),

            machine: Either::Left(Default::default()),

            history: Default::default(),
            checkpoint: Default::default(),
        }
    }
}

impl<A: StateMachine, B: StateMachine> StateMachine for InterlaceMachine<A, B> {
    type Output = Interlace<A::Output, B::Output>;
    type Error = std::convert::Infallible;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        use ControlFlow::*;
        use Either::*;

        let Self {
            mut contents_a,
            mut contents_b,

            mut b_parking,

            machine,

            mut history,
            mut checkpoint,
        } = self;
        history.push(val.clone());
        checkpoint += 1;

        match machine {
            Left(machine) => match machine.drive(val) {
                Continue(machine) => Continue(Self {
                    contents_a,
                    contents_b,

                    b_parking,

                    machine: Left(machine),
                    history,
                    checkpoint,
                }),
                Break(Ok((v, rl))) => {
                    checkpoint = rl;
                    contents_a.push(v);

                    if let Some(v) = b_parking {
                        contents_b.push(v)
                    }
                    b_parking = None;

                    Self {
                        contents_a,
                        contents_b,

                        b_parking,

                        machine: Right(B::default()),
                        history,
                        checkpoint,
                    }
                    .process_value_stepup(rl)
                }
                Break(Err(_)) => Break(Ok((Interlace(contents_a, contents_b), checkpoint))),
            },
            Right(machine) => match machine.drive(val) {
                Continue(machine) => Continue(Self {
                    contents_a,
                    contents_b,

                    b_parking,

                    machine: Right(machine),
                    history,
                    checkpoint,
                }),
                Break(Ok((v, rl))) => {
                    b_parking = Some(v);

                    Self {
                        contents_a,
                        contents_b,

                        b_parking,

                        machine: Left(A::default()),
                        history,
                        checkpoint,
                    }
                    .process_value_stepup(rl)
                }
                Break(Err(_)) => Break(Ok((Interlace(contents_a, contents_b), checkpoint))),
            },
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        use ControlFlow::*;
        use Either::*;

        let Self {
            mut contents_a,
            mut contents_b,

            mut b_parking,

            machine,

            history,
            mut checkpoint,
        } = self;

        match machine {
            Left(machine) => match machine.terminate() {
                Ok((v, rl)) => {
                    checkpoint = rl;
                    contents_a.push(v);

                    if let Some(v) = b_parking {
                        contents_b.push(v)
                    }
                    b_parking = None;

                    match (Self {
                        contents_a,
                        contents_b,

                        b_parking,

                        machine: Right(B::default()),
                        history,
                        checkpoint,
                    }
                    .process_value_stepup(rl))
                    {
                        Continue(c) => c.terminate(),
                        Break(b) => b,
                    }
                }
                Err(_) => Ok((Interlace(contents_a, contents_b), checkpoint)),
            },
            Right(machine) => match machine.terminate() {
                Ok((v, rl)) => {
                    b_parking = Some(v);

                    match (Self {
                        contents_a,
                        contents_b,

                        b_parking,

                        machine: Left(A::default()),
                        history,
                        checkpoint,
                    }
                    .process_value_stepup(rl))
                    {
                        Continue(c) => c.terminate(),
                        Break(b) => b,
                    }
                }
                Err(_) => Ok((Interlace(contents_a, contents_b), checkpoint)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_matches_tokens_after_interlace() {
        let path = parse::<(
            Interlace<Ident, (FJointPunct<':'>, FPunct<':'>)>,
            (FPunct<'>'>, FPunct<';'>),
        )>(quote::quote! { hello > ;});

        match path {
            Ok(p) => println!("{:#?}", p),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn it_matches_comma_seperation() {
        let (v, l) =
            parse::<Interlace<Ident, FPunct<','>>>(quote::quote! { hello, world, hi, there, })
                .unwrap();

        assert_eq!(v.0.len(), 4);
        assert_eq!(v.1.len(), 3);
        assert_eq!(l, 1);
    }
    #[test]
    fn it_matches_comma_seperation_with_backstep() {
        let (v, l) = parse::<Interlace<(Ident, Option<Ident>), FPunct<','>>>(
            quote::quote! { hello hi, world, hi, there },
        )
        .unwrap();

        assert_eq!(v.0.len(), 4);
        assert_eq!(v.1.len(), 3);
        assert_eq!(l, 0);
    }

    #[test]
    fn it_matches_with_arbitrarilly_sized_interlacing() {
        let (v, l) = parse::<Interlace<(Ident, Option<Ident>), Vec<FPunct<','>>>>(
            quote::quote! { hello hi world,,, hi, there },
        )
        .unwrap();

        assert_eq!(v.0.len(), 4);
        assert_eq!(v.1.len(), 3);
        assert_eq!(l, 0);
    }
    #[test]
    fn it_matches_with_arbitrarilly() {
        let (v, l) = parse::<Interlace<(Ident, Vec<Ident>), Vec<FPunct<','>>>>(
            quote::quote! { hello hi world,,, hi, there },
        )
        .unwrap();

        assert_eq!(l, 0);
        assert_eq!(v.0.len(), 3);
        assert_eq!(v.1.len(), 2);
    }
}
