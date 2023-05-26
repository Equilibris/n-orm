use crate::*;

use proc_macro2::TokenTree;
use std::ops::ControlFlow::*;

pub type E1Next<T, A> = <T as Cons<A>>::Next;
pub type E2Next<T, A, B> = E1Next<E1Next<T, A>, B>;
pub type E3Next<T, A, B, C> = E1Next<E2Next<T, A, B>, C>;
pub type E4Next<T, A, B, C, D> = E1Next<E3Next<T, A, B, C>, D>;
pub type E5Next<T, A, B, C, D, E> = E1Next<E4Next<T, A, B, C, D>, E>;
pub type E6Next<T, A, B, C, D, E, F> = E1Next<E5Next<T, A, B, C, D, E>, F>;
pub type E7Next<T, A, B, C, D, E, F, G> = E1Next<E6Next<T, A, B, C, D, E, F>, G>;
pub type E8Next<T, A, B, C, D, E, F, G, H> = E1Next<E7Next<T, A, B, C, D, E, F, G>, H>;
pub type E9Next<T, A, B, C, D, E, F, G, H, I> = E1Next<E8Next<T, A, B, C, D, E, F, G, H>, I>;
pub type E10Next<T, A, B, C, D, E, F, G, H, I, J> = E1Next<E9Next<T, A, B, C, D, E, F, G, H, I>, J>;
pub type E11Next<T, A, B, C, D, E, F, G, H, I, J, K> =
    E1Next<E10Next<T, A, B, C, D, E, F, G, H, I, J>, K>;
pub type E12Next<T, A, B, C, D, E, F, G, H, I, J, K, L> =
    E1Next<E11Next<T, A, B, C, D, E, F, G, H, I, J, K>, L>;
pub type E13Next<T, A, B, C, D, E, F, G, H, I, J, K, L, M> =
    E1Next<E12Next<T, A, B, C, D, E, F, G, H, I, J, K, L>, M>;
pub type E14Next<T, A, B, C, D, E, F, G, H, I, J, K, L, M, N> =
    E1Next<E13Next<T, A, B, C, D, E, F, G, H, I, J, K, L, M>, N>;

// This may be able to be used to impl monads and TypeClasses in Rust
pub trait Cons<T> {
    type Next;

    fn next(self, v: T) -> Self::Next;
}

#[derive(Default, Debug)]
pub struct EBox<T>(pub Box<T>);

impl<T: std::error::Error> std::error::Error for EBox<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for EBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T, A> Cons<A> for EBox<T>
where
    T: Cons<A>,
{
    type Next = EBox<E1Next<T, A>>;
    fn next(self, v: A) -> Self::Next {
        EBox(Box::new((*self.0).next(v)))
    }
}

#[derive(Default, Debug, thiserror::Error)]
#[error("BlackHole")]
pub struct BlackHole;
impl<T> Cons<T> for BlackHole {
    type Next = BlackHole;

    fn next(self, _: T) -> Self::Next {
        BlackHole
    }
}

#[derive(Default, Debug, thiserror::Error)]
#[error("")]
pub struct Sum0Err {}
impl<T: std::error::Error> Cons<T> for Sum0Err {
    type Next = Sum1Err<T>;

    fn next(self, a: T) -> Self::Next {
        Sum1Err { a }
    }
}

macro_rules! sum_n_err {
    (!$name:ident, $err:literal, $($p:ident $s:ident),*; $fp:ident $fs:ident) => {
        #[derive(Debug, thiserror::Error)]
        #[error($err, $(. $p),*)]
        pub struct $name <$($s: std::error::Error),*>{
            $(pub $p: $s,)*
        }
    };
    ($name:ident, $next:ident, $err:literal, $($p:ident $s:ident),*; $fp:ident $fs:ident) => {
        sum_n_err!(!$name, $err, $($p $s),*; $fp $fs);

        impl<$($s: std::error::Error,)*$fs : std::error::Error> Cons<$fs> for $name <$($s),*> {
            type Next = $next<$($s,)*$fs >;

            fn next(self, $fp: $fs) -> Self::Next {
                let Self { $($p,)* } = self;

                $next { $($p,)* $fp }
            }
        }
    };
}

sum_n_err!(Sum1Err,  Sum2Err,  "e0: ({})", a A; b B);
sum_n_err!(Sum2Err,  Sum3Err,  "e0: ({}) e1: ({})", a A, b B; c C);
sum_n_err!(Sum3Err,  Sum4Err,  "e0: ({}) e1: ({}) e2: ({})", a A, b B, c C; d D);
sum_n_err!(Sum4Err,  Sum5Err,  "e0: ({}) e1: ({}) e2: ({}) e3: ({})", a A, b B, c C, d D; e E);
sum_n_err!(Sum5Err,  Sum6Err,  "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({})", a A, b B, c C, d D, e E; f F);
sum_n_err!(Sum6Err,  Sum7Err,  "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({})", a A, b B, c C, d D, e E, f F; g G);
sum_n_err!(Sum7Err,  Sum8Err,  "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({})", a A, b B, c C, d D, e E, f F, g G; h H);
sum_n_err!(Sum8Err,  Sum9Err,  "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({})", a A, b B, c C, d D, e E, f F, g G, h H; i I);
sum_n_err!(Sum9Err,  Sum10Err, "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({})", a A, b B, c C, d D, e E, f F, g G, h H, i I; j J);
sum_n_err!(Sum10Err, Sum11Err, "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({})", a A, b B, c C, d D, e E, f F, g G, h H, i I, j J; k K);
sum_n_err!(Sum11Err, Sum12Err, "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({})", a A, b B, c C, d D, e E, f F, g G, h H, i I, j J, k K; l L);
sum_n_err!(Sum12Err, Sum13Err, "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({}) e11: ({})", a A, b B, c C, d D, e E, f F, g G, h H, i I, j J, k K, l L; m M);
sum_n_err!(Sum13Err, Sum14Err, "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({}) e11: ({}) e12: ({})", a A, b B, c C, d D, e E, f F, g G, h H, i I, j J, k K, l L, m M; n N);
sum_n_err!(!Sum14Err,          "e0: ({}) e1: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({}) e11: ({}) e12: ({}) e13: ({})", a A, b B, c C, d D, e E, f F, g G, h H, i I, j J, k K, l L, m M, n N; p P);

macro_rules! sum_n {
    (
        $name:ident, $mname:ident, $errored_name:ident
        $(
            ; $sum:ident
            , $next:ident
            , $gen:ident
            , ($fst:ident $($err_type:ident)*)
            , $bound:ident
         )*
        : $f_sum:ident
        , $f_gen:ident
        , ($f_fst:ident $($f_err_type:ident)*)
        , $f_bound:ident
        , ($ff_fst:ident $($final:ident)*)
    ) => {
        pub struct $errored_name<
            A: Parsable,
            $($gen: Parsable,)*
            $f_gen: Parsable,
            E0
        > (std::marker::PhantomData<(A, $($gen,)* $f_gen, E0)>);

        impl<
            A: Parsable,
            $($gen: Parsable,)*
            $f_gen: Parsable,
            E0
        > Parsable for $errored_name<
            A,
            $($gen,)*
            $f_gen,
            E0
            >
        where
            E0: Cons<SmErr<A>> + Default,
            $(
                $fst<E0, $(SmErr<$err_type>,)*>: Cons<SmErr<$bound>>,
            )*
            $f_fst<E0, $(SmErr<$f_err_type>,)*>: Cons<SmErr<$f_bound>>,
            $ff_fst<E0, $(SmErr<$final>, )*>: std::error::Error,
        {
            type StateMachine = $mname<
                A::StateMachine,
                $($gen::StateMachine,)*
                    $f_gen::StateMachine,
                    E0
                        >;
        }

        #[derive(Clone, Debug)]
        pub enum $name<A, $($gen,)* $f_gen> {
            Val0(A),
            $($sum($gen),)*
            $f_sum($f_gen)
        }
        pub enum $mname<
            A: StateMachine,
            $($gen: StateMachine,)*
            $f_gen: StateMachine,
            E0
        >
        where
            E0: Cons<A::Error>,
            $(
                $fst<E0, $($err_type::Error,)*>: Cons<$bound::Error>,
            )*
            $f_fst<E0, $($f_err_type::Error,)*>: Cons<$f_bound::Error>,
            $ff_fst<E0, $($final::Error, )*>: std::error::Error,
        {
            Val0(Vec<TokenTree>, E0, A),
            // Val1(Vec<TokenTree>, E1Next<E0, A::Error>, B),
            $(
                $sum(Vec<TokenTree>, $fst<E0, $($err_type::Error,)*>, $gen),
            )*
            $f_sum(Vec<TokenTree>, $f_fst<E0, $($f_err_type::Error,)*>, $f_gen)
        }

        impl<
            A: StateMachine,
            $($gen: StateMachine,)*
            $f_gen: StateMachine,
            E0
        > Default for $mname<A, $($gen,)* $f_gen, E0>
        where
            E0: Default + Cons<A::Error>,
            $(
                $fst<E0, $($err_type::Error,)*>: Cons<$bound::Error>,
            )*
            $f_fst<E0, $($f_err_type::Error,)*>: Cons<$f_bound::Error>,
            $ff_fst<E0, $($final::Error,)*>: std::error::Error,
        {
            fn default() -> Self {
                Self::Val0(
                    Default::default(),
                    Default::default(),
                    Default::default()
                    )
            }
        }

        impl<
            A: Parsable,
            $($gen: Parsable,)*
            $f_gen: Parsable
        > Parsable for $name<A, $($gen,)* $f_gen> {
            type StateMachine = $mname<
                A::StateMachine,
                $($gen::StateMachine,)*
                $f_gen::StateMachine,
                Sum0Err
            >;
        }

        impl<
            A: StateMachine,
            $($gen: StateMachine,)*
            $f_gen: StateMachine,
            E0
        > $mname<A, $($gen,)* $f_gen, E0>
        where
            E0: Default + Cons<A::Error>,
            $(
                $fst<E0, $($err_type::Error,)*>: Cons<$bound::Error>,
            )*
            $f_fst<E0, $($f_err_type::Error,)*>: Cons<$f_bound::Error>,
            $ff_fst<E0, $($final::Error,)*>: std::error::Error,
        {
            fn stepup(mut self) -> std::ops::ControlFlow<SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error>, Self>
            {
                'main: loop {
                    match self {
                        $mname::Val0(vs, e, mut sm) => {
                            let len = vs.len();

                            for (i, v) in vs.iter().enumerate() {
                                match sm.drive(v) {
                                    Continue(v) => sm = v,
                                    Break(Ok((a, mut rl))) => {
                                        rl += len - i;
                                        rl -= 1;

                                        return Break(Ok(($name::Val0(a), rl)));
                                    }
                                    Break(Err(e_next)) => {
                                        self =
                                            Self::Val1(vs, e.next(e_next), Default::default());
                                        continue 'main;
                                    }
                                }
                            }
                            return Continue(Self::Val0(vs, e, sm));
                        }
                        $(
                            $mname::$sum(vs, e, mut sm) => {
                                let len = vs.len();

                                for (i, v) in vs.iter().enumerate() {
                                    match sm.drive(v) {
                                        Continue(v) => sm = v,
                                        Break(Ok((a, mut rl))) => {
                                            rl += len - i;
                                            rl -= 1;

                                            return Break(Ok(($name::$sum(a), rl)));
                                        }
                                        Break(Err(e_next)) => {
                                            self =
                                                Self::$next(vs, e.next(e_next), Default::default());
                                            continue 'main;
                                        }
                                    }
                                }
                                return Continue(Self::$sum(vs, e, sm));
                            }
                        )*
                        $mname::$f_sum(vs, e, mut sm) => {
                            let len = vs.len();

                            for (i, v) in vs.iter().enumerate() {
                                match sm.drive(v) {
                                    Continue(v) => sm = v,
                                    Break(Ok((a, mut rl))) => {
                                        rl += len - i;
                                        rl -= 1;

                                        return Break(Ok(($name::$f_sum(a), rl)));
                                    }
                                    Break(Err(e_next)) => {
                                        return Break(Err(e.next(e_next)));
                                    }
                                }
                            }
                            return Continue(Self::$f_sum(vs, e, sm));
                        }
                    }
                }
            }
        }

        impl<
            A: StateMachine,
            $($gen: StateMachine,)*
            $f_gen: StateMachine,
            E0
        > StateMachine for $mname<A, $($gen,)* $f_gen, E0>
        where
            E0: Default + Cons<A::Error>,
            $(
                $fst<E0, $($err_type::Error,)*>: Cons<$bound::Error>,
            )*
            $f_fst<E0, $($f_err_type::Error,)*>: Cons<$f_bound::Error>,
            $ff_fst<E0, $($final::Error,)*>: std::error::Error,
        {
            type Output = $name<A::Output, $($gen::Output,)* $f_gen::Output>;
            type Error = $ff_fst<E0, $($final::Error,)*>;

            fn drive(
                self,
                val: &TokenTree,
                ) -> std::ops::ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                use std::ops::ControlFlow::*;

                match self {
                    $mname::Val0(mut vs, e, sm) => match sm.drive(val) {
                        Continue(sm) => Continue(Self::Val0(
                            {
                                vs.push(val.clone());
                                vs
                            },
                            e,
                            sm,
                        )),
                        Break(Ok((a, rl))) => Break(Ok(($name::Val0(a), rl))),
                        Break(Err(e_next)) => Self::Val1(
                            {
                                vs.push(val.clone());
                                vs
                            },
                            e.next(e_next),
                            Default::default(),
                        ).stepup(),
                    },
                    $(
                        $mname::$sum(mut vs, e, sm) => match sm.drive(val) {
                            Continue(sm) => Continue(Self::$sum(
                                {
                                    vs.push(val.clone());
                                    vs
                                },
                                e,
                                sm,
                            )),
                            Break(Ok((a, rl))) => Break(Ok(($name::$sum(a), rl))),
                            Break(Err(e_next)) => Self::$next(
                                {
                                    vs.push(val.clone());
                                    vs
                                },
                                e.next(e_next),
                                Default::default(),
                            ) .stepup(),
                        }
                    )*
                    $mname::$f_sum(mut vs, e, sm) => match sm.drive(val) {
                        Continue(sm) => Continue(Self::$f_sum(
                            {
                                vs.push(val.clone());
                                vs
                            },
                            e,
                            sm,
                        )),
                        Break(Ok((a, rl))) => Break(Ok(($name::$f_sum(a), rl))),
                        Break(Err(e_next)) => Break(Err(e.next(e_next))),
                    },
                }
            }

            fn terminate(mut self) -> SmResult<Self::Output, Self::Error> {
                loop {
                    match self {
                        $mname::Val0(vs, e, sm) => match sm.terminate() {
                            Ok((a, rl)) => return Ok(($name::Val0(a), rl)),
                            Err(e_next) => {
                                match Self::Val1(vs, e.next(e_next), Default::default())
                                    .stepup()
                                    {
                                        Continue(a) => self = a,
                                        Break(o) => return o,
                                    }
                            }
                        },
                        $(
                            $mname::$sum(vs, e, sm) => match sm.terminate() {
                                Ok((a, rl)) => return Ok(($name::$sum(a), rl)),
                                Err(e_next) => {
                                    match Self::$next(vs, e.next(e_next), Default::default())
                                        .stepup()
                                        {
                                            Continue(a) => self = a,
                                            Break(o) => return o,
                                        }
                                }
                            },
                        )*
                        $mname::$f_sum(_, e, sm) => {
                            return match sm.terminate() {
                                Ok((a, rl)) => Ok(($name::$f_sum(a), rl)),
                                Err(e_next) => Err(e.next(e_next)),
                            }
                        }
                    }
                }
            }
        }
    };
}

sum_n!(
    Sum2, Sum2M, ESum2
    : Val1, B, (E1Next A), B
    ,          (E2Next A B)
);
sum_n!(
    Sum3, Sum3M, ESum3
    ; Val1, Val2, B, (E1Next A), B
    : Val2, C,       (E2Next A B), C
    ,                (E3Next A B C)
);
sum_n!(
    Sum4, Sum4M, ESum4
    ; Val1, Val2, B, (E1Next A), B
    ; Val2, Val3, C, (E2Next A B), C
    : Val3, D,       (E3Next A B C), D
    ,                (E4Next A B C D)
);
sum_n!(
    Sum5, Sum5M, ESum5
    ; Val1, Val2, B, (E1Next A), B
    ; Val2, Val3, C, (E2Next A B), C
    ; Val3, Val4, D, (E3Next A B C), D
    : Val4, E,       (E4Next A B C D), E
    ,                (E5Next A B C D E)
);
sum_n!(
    Sum6, Sum6M, ESum6
    ; Val1, Val2, B, (E1Next A), B
    ; Val2, Val3, C, (E2Next A B), C
    ; Val3, Val4, D, (E3Next A B C), D
    ; Val4, Val5, E, (E4Next A B C D), E
    : Val5, F,       (E5Next A B C D E), F
    ,                (E6Next A B C D E F)
);
sum_n!(
    Sum7, Sum7M, ESum7
    ; Val1, Val2, B, (E1Next A), B
    ; Val2, Val3, C, (E2Next A B), C
    ; Val3, Val4, D, (E3Next A B C), D
    ; Val4, Val5, E, (E4Next A B C D), E
    ; Val5, Val6, F, (E5Next A B C D E), F
    : Val6, G,       (E6Next A B C D E F), G
    ,                (E7Next A B C D E F G)
);
sum_n!(
    Sum8, Sum8M, ESum8
    ; Val1, Val2, B, (E1Next A), B
    ; Val2, Val3, C, (E2Next A B), C
    ; Val3, Val4, D, (E3Next A B C), D
    ; Val4, Val5, E, (E4Next A B C D), E
    ; Val5, Val6, F, (E5Next A B C D E), F
    ; Val6, Val7, G, (E6Next A B C D E F), G
    : Val7, H,       (E7Next A B C D E F G), H
    ,                (E8Next A B C D E F G H)
);
sum_n!(
    Sum9, Sum9M, ESum9
    ; Val1, Val2, B, (E1Next A), B
    ; Val2, Val3, C, (E2Next A B), C
    ; Val3, Val4, D, (E3Next A B C), D
    ; Val4, Val5, E, (E4Next A B C D), E
    ; Val5, Val6, F, (E5Next A B C D E), F
    ; Val6, Val7, G, (E6Next A B C D E F), G
    ; Val7, Val8, H, (E7Next A B C D E F G), H
    : Val8, I,       (E8Next A B C D E F G H), I
    ,                (E9Next A B C D E F G H I)
);
sum_n!(
    Sum10, Sum10M, ESum10
    ; Val1, Val2, B, (E1Next A), B
    ; Val2, Val3, C, (E2Next A B), C
    ; Val3, Val4, D, (E3Next A B C), D
    ; Val4, Val5, E, (E4Next A B C D), E
    ; Val5, Val6, F, (E5Next A B C D E), F
    ; Val6, Val7, G, (E6Next A B C D E F), G
    ; Val7, Val8, H, (E7Next A B C D E F G), H
    ; Val8, Val9, I, (E8Next A B C D E F G H), I
    : Val9, J,       (E9Next A B C D E F G H I), J
    ,                (E10Next A B C D E F G H I J)
);
sum_n!(
    Sum11, Sum11M, ESum11
    ; Val1, Val2,  B,  (E1Next A), B
    ; Val2, Val3,  C,  (E2Next A B), C
    ; Val3, Val4,  D,  (E3Next A B C), D
    ; Val4, Val5,  E,  (E4Next A B C D), E
    ; Val5, Val6,  F,  (E5Next A B C D E), F
    ; Val6, Val7,  G,  (E6Next A B C D E F), G
    ; Val7, Val8,  H,  (E7Next A B C D E F G), H
    ; Val8, Val9,  I,  (E8Next A B C D E F G H), I
    ; Val9, Val10, J,  (E9Next A B C D E F G H I), J
    : Val10, K,       (E10Next A B C D E F G H I J), K
    ,                 (E11Next A B C D E F G H I J K)
);
sum_n!(
    Sum12, Sum12M, ESum12
    ; Val1, Val2,   B,  (E1Next A), B
    ; Val2, Val3,   C,  (E2Next A B), C
    ; Val3, Val4,   D,  (E3Next A B C), D
    ; Val4, Val5,   E,  (E4Next A B C D), E
    ; Val5, Val6,   F,  (E5Next A B C D E), F
    ; Val6, Val7,   G,  (E6Next A B C D E F), G
    ; Val7, Val8,   H,  (E7Next A B C D E F G), H
    ; Val8, Val9,   I,  (E8Next A B C D E F G H), I
    ; Val9, Val10,  J,  (E9Next A B C D E F G H I), J
    ; Val10, Val11, K, (E10Next A B C D E F G H I J), K
    : Val11, L,        (E11Next A B C D E F G H I J K), L
    ,                  (E12Next A B C D E F G H I J K L)
);
sum_n!(
    Sum13, Sum13M, ESum13
    ; Val1, Val2,   B,  (E1Next A), B
    ; Val2, Val3,   C,  (E2Next A B), C
    ; Val3, Val4,   D,  (E3Next A B C), D
    ; Val4, Val5,   E,  (E4Next A B C D), E
    ; Val5, Val6,   F,  (E5Next A B C D E), F
    ; Val6, Val7,   G,  (E6Next A B C D E F), G
    ; Val7, Val8,   H,  (E7Next A B C D E F G), H
    ; Val8, Val9,   I,  (E8Next A B C D E F G H), I
    ; Val9, Val10,  J,  (E9Next A B C D E F G H I), J
    ; Val10, Val11, K, (E10Next A B C D E F G H I J), K
    ; Val11, Val12, L, (E11Next A B C D E F G H I J K), L
    : Val12, M,        (E12Next A B C D E F G H I J K L), M
    ,                  (E13Next A B C D E F G H I J K L M)
);
sum_n!(
    Sum14, Sum14M, ESum14
    ; Val1, Val2,   B,  (E1Next A), B
    ; Val2, Val3,   C,  (E2Next A B), C
    ; Val3, Val4,   D,  (E3Next A B C), D
    ; Val4, Val5,   E,  (E4Next A B C D), E
    ; Val5, Val6,   F,  (E5Next A B C D E), F
    ; Val6, Val7,   G,  (E6Next A B C D E F), G
    ; Val7, Val8,   H,  (E7Next A B C D E F G), H
    ; Val8, Val9,   I,  (E8Next A B C D E F G H), I
    ; Val9, Val10,  J,  (E9Next A B C D E F G H I), J
    ; Val10, Val11, K, (E10Next A B C D E F G H I J), K
    ; Val11, Val12, L, (E11Next A B C D E F G H I J K), L
    ; Val12, Val13, M, (E12Next A B C D E F G H I J K L), M
    : Val13, N,        (E13Next A B C D E F G H I J K L M), N
    ,                  (E14Next A B C D E F G H I J K L M N)
);

#[cfg(test)]
mod tests {
    use crate::*;

    type P = Punct;

    insta_match_test!(
        it_matches_highest_priority,
        Sum5<
            (P,P,P,P,P),
            (P,P,P,P,),
            (P,P,P,),
            (P,P,),
            (P,),
        > : ....);

    insta_match_test!(it_matches_sum_2_0, Sum2<Ident, Punct> : hello);
    insta_match_test!(it_matches_sum_2_1, Sum2<Ident, Punct> : <);
}
