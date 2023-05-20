use crate::*;

mod trivial;
pub use trivial::*;

impl<A: Parsable, B: Parsable> Parsable for (A, B) {
    type StateMachine = MTuple<A::StateMachine, B::StateMachine>;
}
pub enum MTuple<A: StateMachine, B: StateMachine> {
    A(Vec<TokenTree>, A),
    B(A::Output, B),
}

#[derive(Clone, thiserror::Error, Debug)]
pub enum TupleError<A: std::error::Error, B: std::error::Error> {
    #[error("1: ({})", .0)]
    A(A),
    #[error("2: ({})", .0)]
    B(B),

    #[error("Internal token content was of length {} but requested {}", .0, .1)]
    InvalidLength(usize, usize),
}

impl<A: StateMachine, B: StateMachine> Default for MTuple<A, B> {
    fn default() -> Self {
        Self::A(Vec::new(), A::default())
    }
}
impl<A: StateMachine, B: StateMachine> MTuple<A, B> {
    fn process_a_stepup(
        content: Vec<TokenTree>,
        a: A::Output,
        mut rl: usize,
    ) -> ControlFlow<SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error>, Self>
    {
        use ControlFlow::*;

        let mut b = B::default();

        let len = content.len();

        if rl > len {
            return Break(Err(TupleError::InvalidLength(rl, len)));
        }
        while rl > 0 {
            match b.drive(&content[len - rl]) {
                Continue(c) => b = c,
                Break(Ok((ok, inc))) => return Break(Ok(((a, ok), inc + rl - 1))),
                Break(Err(e)) => return Break(Err(TupleError::B(e))),
            }
            rl -= 1;
        }

        Continue(Self::B(a, b))
    }

    fn terminate_b(
        a: A::Output,
        b: B,
    ) -> SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error> {
        match b.terminate() {
            Ok((ok, b)) => Ok(((a, ok), b)),
            Err(b) => Err(TupleError::B(b)),
        }
    }
}

impl<A: StateMachine, B: StateMachine> StateMachine for MTuple<A, B> {
    type Output = (A::Output, B::Output);
    type Error = TupleError<A::Error, B::Error>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        use ControlFlow::*;

        match self {
            Self::A(mut content, a) => match a.drive(val) {
                Break(b) => match b {
                    Ok((a, a_backtrack)) => {
                        content.push(val.clone());
                        Self::process_a_stepup(content, a, a_backtrack)
                    }
                    Err(e) => Break(Err(TupleError::A(e))),
                },
                Continue(v) => Continue({
                    content.push(val.clone());
                    Self::A(content, v)
                }),
            },
            Self::B(a, b) => match b.drive(val) {
                Break(b) => match b {
                    Ok((ok, backtrack)) => Break(Ok(((a, ok), backtrack))),
                    Err(e) => Break(Err(TupleError::B(e))),
                },
                Continue(v) => Continue(Self::B(a, v)),
            },
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        match self {
            Self::A(content, a) => match a.terminate() {
                Ok((a, a_backtrack)) => match Self::process_a_stepup(content, a, a_backtrack) {
                    ControlFlow::Continue(s) => s.terminate(),
                    ControlFlow::Break(a) => a,
                },
                Err(e) => Err(TupleError::A(e)),
            },
            Self::B(a, b) => Self::terminate_b(a, b),
        }
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        match self {
            MTuple::A(_, ref a) => a.inspect(depth),
            MTuple::B(_, ref b) => b.inspect(depth),
        }
    }
}

mod higher_order_tuple_2 {
    use crate::*;

    macro_rules! hot {
        (
            $mname:ident, $err_name:ident, $fst:ident;
            $($gen:ident, $next:ident, $(($under:pat) $low:ident $cap:ident)*);*
            : $final:ident, $(($f_under:pat) $f_low:ident $f_cap:ident)*
        ) => {

            impl<$($gen: Parsable,)* $final: Parsable> Parsable for ($($gen,)* $final) {
                type StateMachine = $mname<$($gen::StateMachine,)* $final::StateMachine>;
            }
            pub enum $mname<$($gen: StateMachine,)* $final: StateMachine> {
                $(
                    $gen($($cap::Output,)* Vec<TokenTree>, $gen),
                )*
                $final($($f_cap::Output,)* Vec<TokenTree>, $final),
            }

            #[derive(Clone, thiserror::Error, Debug)]
            pub enum $err_name<
                $($gen: std::error::Error,)*
                $final: std::error::Error,
            > {
                $(
                    #[error("{}: ({})", stringify!($gen), .0)]
                    $gen($gen),
                )*

                #[error("{}: ({})", stringify!($final), .0)]
                $final($final),

                #[error("Internal token content was of length {} but requested {}", .0, .1)]
                InvalidLength(usize, usize),
            }

            impl<$($gen : StateMachine,)* $final: StateMachine> Default
                for $mname<$($gen,)* $final>
            {
                fn default() -> Self {
                    Self::$fst(Vec::new(), A::default())
                }
            }
            impl<$($gen : StateMachine,)* $final: StateMachine> $mname<$($gen,)* $final> {
                fn process_stepup(
                    mut self,
                    mut rl: usize,
                ) -> ControlFlow<
                    SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error>,
                    Self,
                > {
                    use ControlFlow::*;
                    use $mname::*;
                    use $err_name as InnerErr;

                    match self {
                        $($gen($($under,)* ref vs, _))|* => {
                            if rl > vs.len() {
                                return Break(Err($err_name::InvalidLength(rl, vs.len())));
                            }
                        }
                        $final($($f_under,)* ref vs, _) => {
                            if rl > vs.len() {
                                return Break(Err($err_name::InvalidLength(rl, vs.len())));
                            }
                        }
                    }
                    while rl > 0 {
                        match self {
                            $(
                                $gen($($low,)* vs, machine) => match machine.drive(&vs[vs.len() - rl]) {
                                    Continue(machine) => self = $gen($($low,)* vs, machine),
                                    Break(Ok((ok, inc))) => {
                                        rl += inc;
                                        self = $next($($low,)* ok, vs, Default::default());
                                    }
                                    Break(Err(e)) => return Break(Err(InnerErr::$gen(e))),
                                },

                            )*
                            $final($($f_low,)* vs, machine) => match machine.drive(&vs[vs.len() - rl]) {
                                Continue(machine) => self = $final($($f_low,)* vs, machine),
                                Break(Ok((zz, inc))) => return Break(Ok((($($f_low,)* zz), inc + rl - 1))),
                                Break(Err(e)) => return Break(Err(InnerErr::$final(e))),
                            },
                        }
                        rl -= 1;
                    }

                    Continue(self)
                }
            }

            impl<$($gen: StateMachine,)* $final: StateMachine> StateMachine
                for $mname<$($gen,)* $final>
            {
                type Output = ($($gen::Output,)* $final::Output);
                type Error = $err_name<$($gen::Error,)* $final::Error>;

                fn drive(
                    self,
                    val: &TokenTree,
                ) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                    use ControlFlow::*;

                    match self {
                        $(
                            Self::$gen($($low,)* mut vs, machine) => match machine.drive(val) {
                                Break(Ok((zz, rl))) => {
                                    vs.push(val.clone());
                                    Self::$next($($low,)* zz, vs, Default::default()).process_stepup(rl)
                                }
                                Break(Err(e)) => Break(Err($err_name::$gen(e))),
                                Continue(v) => Continue({
                                    vs.push(val.clone());
                                    Self::$gen($($low,)* vs, v)
                                }),
                            },
                        )*
                        Self::$final($($f_low,)* vs, machine) => match machine.drive(val) {
                            Break(Ok((zz, rl))) => Break(Ok((($($f_low,)* zz), rl))),
                            Break(Err(e)) => Break(Err($err_name::$final(e))),
                            Continue(machine) => Continue(Self::$final($($f_low,)* vs, machine)),
                        },
                    }
                }

                fn terminate(mut self) -> SmResult<Self::Output, Self::Error> {
                    loop {
                        let (v, rl) = match self {
                            $(
                                $mname::$gen($($low,)* vs, machine) => match machine.terminate() {
                                    Ok((zz, rl)) => (Self::$next($($low,)* zz, vs, Default::default()), rl),
                                    Err(e) => return Err($err_name::$gen(e)),
                                },
                            )*
                            $mname::$final($($f_low,)* _, machine) => match machine.terminate() {
                                Ok((zz, rl)) => return Ok((($($f_low,)* zz), rl)),
                                Err(e) => return Err($err_name::$final(e)),
                            },
                        };
                        match v.process_stepup(rl) {
                            ControlFlow::Continue(machine) => self = machine,
                            ControlFlow::Break(out) => return out,
                        }
                    }
                }

                #[cfg(feature = "execution-debug")]
                fn inspect(&self, depth: usize) {
                    match self {
                        MTuple::A(_, ref a) => a.inspect(depth),
                        MTuple::B(_, ref b) => b.inspect(depth),
                    }
                }
            }
        };
    }

    hot!(
        M2Tuple, TupleError2, A
        ; A, B,
        ; B, C, (_) a A
        : C, (_) a A (_) b B
    );
    hot!(
        M3Tuple, TupleError3, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        : D,    (_) a A (_) b B (_) c C
    );
    hot!(
        M4Tuple, TupleError4, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        ; D, E, (_) a A (_) b B (_) c C
        : E,    (_) a A (_) b B (_) c C (_) d D
    );
    hot!(
        M5Tuple, TupleError5, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        ; D, E, (_) a A (_) b B (_) c C
        ; E, F, (_) a A (_) b B (_) c C (_) d D
        : F,    (_) a A (_) b B (_) c C (_) d D (_) e E
    );
    hot!(
        M6Tuple, TupleError6, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        ; D, E, (_) a A (_) b B (_) c C
        ; E, F, (_) a A (_) b B (_) c C (_) d D
        ; F, G, (_) a A (_) b B (_) c C (_) d D (_) e E
        : G,    (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F
    );
    hot!(
        M7Tuple, TupleError7, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        ; D, E, (_) a A (_) b B (_) c C
        ; E, F, (_) a A (_) b B (_) c C (_) d D
        ; F, G, (_) a A (_) b B (_) c C (_) d D (_) e E
        ; G, H, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F
        : H,    (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G
    );
    hot!(
        M8Tuple, TupleError8, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        ; D, E, (_) a A (_) b B (_) c C
        ; E, F, (_) a A (_) b B (_) c C (_) d D
        ; F, G, (_) a A (_) b B (_) c C (_) d D (_) e E
        ; G, H, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F
        ; H, I, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G
        : I,    (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G (_) h H
    );
    hot!(
        M9Tuple, TupleError9, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        ; D, E, (_) a A (_) b B (_) c C
        ; E, F, (_) a A (_) b B (_) c C (_) d D
        ; F, G, (_) a A (_) b B (_) c C (_) d D (_) e E
        ; G, H, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F
        ; H, I, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G
        ; I, J, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G (_) h H
        : J,    (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G (_) h H (_) i I
    );
    hot!(
        M10Tuple, TupleError10, A
        ; A, B,
        ; B, C, (_) a A
        ; C, D, (_) a A (_) b B
        ; D, E, (_) a A (_) b B (_) c C
        ; E, F, (_) a A (_) b B (_) c C (_) d D
        ; F, G, (_) a A (_) b B (_) c C (_) d D (_) e E
        ; G, H, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F
        ; H, I, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G
        ; I, J, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G (_) h H
        ; J, K, (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G (_) h H (_) i I
        : K,    (_) a A (_) b B (_) c C (_) d D (_) e E (_) f F (_) g G (_) h H (_) i I (_) j J
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sizes_do_not_grow_exponentially() {
        use std::mem::size_of;

        type V<T> = (T, T, T, T, T, T, T, T, T, T, T);

        dbg!(size_of::<<V<()> as Parsable>::StateMachine>());
    }

    insta_match_test!(it_matches_2_tuple, (Ident, FIdent<"world">) : hello world);
    insta_match_test!(it_steps_back_for_options, (Option<Ident>, Option<Punct>) : <);
    insta_match_test!(it_only_steps_back_on_fail_for_options, (Option<Ident>, Option<Punct>) : hi);
    insta_match_test!(it_steps_back_for_multi_tuples, (Option<Ident>, Option<Punct>, Option<Ident>, Option<Punct>) : hi <>);
    insta_match_test!(it_sums_tuple_backtracking, (Vec<(Punct, Punct, Ident, Ident)>, Punct) : >>h1 h2>>h3 h4 !);
}
