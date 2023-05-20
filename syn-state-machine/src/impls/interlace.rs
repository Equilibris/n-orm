use std::fmt::Debug;

use crate::*;

// pub struct Interlace<A: Parsable, B: Parsable>(pub Vec<SmOut<A>>, pub Vec<SmOut<B>>);
#[derive(Debug)]
pub struct Interlace<A, B>(pub Vec<A>, pub Vec<B>);

impl<A: Parsable, B: Parsable> Parsable for Interlace<A, B> {
    type StateMachine = InterlaceMachine<A::StateMachine, B::StateMachine>;
}

pub struct InterlaceMachine<A: StateMachine, B: StateMachine> {
    contents_a: Vec<A::Output>,
    contents_b: Vec<B::Output>,

    b_parking: Option<B::Output>,

    machine: Sum2<A, B>,

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
        use Sum2::*;

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
                Val0(v) => match v.drive(&history[len - rl]) {
                    Continue(c) => machine = Val0(c),
                    Break(Ok((ok, backtrack))) => {
                        rl += backtrack;
                        checkpoint = rl;

                        if let Some(v) = b_parking {
                            contents_b.push(v)
                        }
                        b_parking = None;

                        contents_a.push(ok);
                        machine = Val1(B::default());
                    }
                    Break(Err(_)) => {
                        return Break(Ok((Interlace(contents_a, contents_b), checkpoint)));
                    }
                },
                Val1(v) => match v.drive(&history[len - rl]) {
                    Continue(c) => machine = Val1(c),
                    Break(Ok((ok, backtrack))) => {
                        rl += backtrack;

                        b_parking = Some(ok);
                        machine = Val0(A::default());
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

            machine: Sum2::Val0(Default::default()),

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
        use Sum2::*;

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
            Val0(machine) => match machine.drive(val) {
                Continue(machine) => Continue(Self {
                    contents_a,
                    contents_b,

                    b_parking,

                    machine: Val0(machine),
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

                        machine: Val1(B::default()),
                        history,
                        checkpoint,
                    }
                    .process_value_stepup(rl)
                }
                Break(Err(_)) => Break(Ok((Interlace(contents_a, contents_b), checkpoint))),
            },
            Val1(machine) => match machine.drive(val) {
                Continue(machine) => Continue(Self {
                    contents_a,
                    contents_b,

                    b_parking,

                    machine: Val1(machine),
                    history,
                    checkpoint,
                }),
                Break(Ok((v, rl))) => {
                    b_parking = Some(v);

                    Self {
                        contents_a,
                        contents_b,

                        b_parking,

                        machine: Val0(A::default()),
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
        use Sum2::*;

        let Self {
            mut contents_a,
            mut contents_b,

            mut b_parking,

            machine,

            history,
            mut checkpoint,
        } = self;

        match machine {
            Val0(machine) => match machine.terminate() {
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

                        machine: Val1(B::default()),
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
            Val1(machine) => match machine.terminate() {
                Ok((v, rl)) => {
                    b_parking = Some(v);

                    match (Self {
                        contents_a,
                        contents_b,

                        b_parking,

                        machine: Val0(A::default()),
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

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        match self.machine {
            Sum2::Val0(ref a) => {
                println!("{}Interlace Main :", "  ".repeat(depth));
                a.inspect(depth + 1)
            }
            Sum2::Val1(ref a) => {
                println!("{}Interlace Secondary :", "  ".repeat(depth));
                a.inspect(depth + 1)
            }
        }
    }
}

#[derive(Debug)]
pub struct InterlaceTrail<A, B>(pub Vec<A>, pub Vec<B>);
impl<A: Parsable, B: Parsable> MappedParse for InterlaceTrail<A, B> {
    type Source = Option<(MinLength<Interlace<A, B>>, Option<B>)>;

    type Output = InterlaceTrail<SmOut<A>, SmOut<B>>;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Some((Interlace(a, mut b), Some(c))) => {
                b.push(c);
                InterlaceTrail(a, b)
            }
            Some((Interlace(a, b), None)) => InterlaceTrail(a, b),
            None => InterlaceTrail(Vec::new(), Vec::new()),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::*;

    type Two = (FPunct<':'>, FPunct<':'>);

    insta_match_test!(it_matches_esoterics, Interlace<(Ident, Option<(Two, Ident)>), Two> : r1::r2::r3::r4::r5);
    insta_match_test!(it_matches_empty, Interlace<Ident, FPunct<','>> : );
    insta_match_test!(it_matches_tokens_after_interlace, 
                      (Interlace<Ident, (FJointPunct<':'>, FPunct<':'>)>, (FPunct<'>'>, FPunct<';'>),) 
                      :  hello > ;);
    insta_match_test!(it_matches_comma_seperation, Interlace<Ident, FPunct<','>> :  hello, world, hi, there,);
    insta_match_test!(it_matches_comma_seperation_with_backstep, Interlace<(Ident, Option<Ident>), FPunct<','>> :  hello, world, hi, there);
    insta_match_test!(it_matches_with_arbitrarilly_sized_interlacing, Interlace<(Ident, Option<Ident>), Vec<FPunct<','>>> : hello hi world,,, hi, there  hello, world, hi, there);

    insta_match_test!(it_matches_with_arbitrarilly, Interlace<(Ident, Vec<Ident>), Vec<FPunct<','>>> :  hello hi world,,, hi, there );

    insta_match_test!(it_matches_trailing, InterlaceTrail<Ident, FPunct<','>>: hi, hello);
    insta_match_test!(it_matches_trailing_with, InterlaceTrail<Ident, FPunct<','>>: hi, hello,);
}
