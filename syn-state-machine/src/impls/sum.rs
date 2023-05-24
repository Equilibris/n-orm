use crate::*;

mod new_sum {
    use proc_macro2::TokenTree;

    use crate::*;
    use std::ops::ControlFlow::*;

    pub type E1Next<T, A> = <T as ErrorNext<A>>::Next;
    pub type E2Next<T, A, B> = E1Next<E1Next<T, A>, B>;
    pub type E3Next<T, A, B, C> = E1Next<E2Next<T, A, B>, C>;
    pub type E4Next<T, A, B, C, D> = E1Next<E3Next<T, A, B, C>, D>;
    pub type E5Next<T, A, B, C, D, E> = E1Next<E4Next<T, A, B, C, D>, E>;
    pub type E6Next<T, A, B, C, D, E, F> = E1Next<E5Next<T, A, B, C, D, E>, F>;
    pub type E7Next<T, A, B, C, D, E, F, G> = E1Next<E6Next<T, A, B, C, D, E, F>, G>;
    pub type E8Next<T, A, B, C, D, E, F, G, H> = E1Next<E7Next<T, A, B, C, D, E, F, G>, H>;
    pub type E9Next<T, A, B, C, D, E, F, G, H, I> = E1Next<E8Next<T, A, B, C, D, E, F, G, H>, I>;
    pub type E10Next<T, A, B, C, D, E, F, G, H, I, J> =
        E1Next<E9Next<T, A, B, C, D, E, F, G, H, I>, J>;
    pub type E11Next<T, A, B, C, D, E, F, G, H, I, J, K> =
        E1Next<E10Next<T, A, B, C, D, E, F, G, H, I, J>, K>;
    pub type E12Next<T, A, B, C, D, E, F, G, H, I, J, K, L> =
        E1Next<E11Next<T, A, B, C, D, E, F, G, H, I, J, K>, L>;
    pub type E13Next<T, A, B, C, D, E, F, G, H, I, J, K, L, M> =
        E1Next<E12Next<T, A, B, C, D, E, F, G, H, I, J, K, L>, M>;
    pub type E14Next<T, A, B, C, D, E, F, G, H, I, J, K, L, M, N> =
        E1Next<E13Next<T, A, B, C, D, E, F, G, H, I, J, K, L, M>, N>;

    pub trait ErrorNext<T> {
        type Next;

        fn next(self, v: T) -> Self::Next;
    }

    #[derive(Default, Debug, thiserror::Error)]
    #[error("BlackHole")]
    pub struct BlackHole;
    impl<T> ErrorNext<T> for BlackHole {
        type Next = BlackHole;

        fn next(self, _: T) -> Self::Next {
            BlackHole
        }
    }

    #[derive(Default, Debug, thiserror::Error)]
    #[error("")]
    pub struct Sum0Err {}
    impl<T: std::error::Error> ErrorNext<T> for Sum0Err {
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

            impl<$($s: std::error::Error,)*$fs : std::error::Error> ErrorNext<$fs> for $name <$($s),*> {
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
            $name:ident, $mname:ident
            $(
                ; $sum:ident
                , $next:ident
                , $gen:ident
                , $err_type:ty
                , ($($bound:tt)+)
            )*
            : $f_sum:ident
            , $f_gen:ident
            , $f_err_type:ty
            , ($($f_bound:tt)+)
            , $final:ty
            ) => {

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
                E0: ErrorNext<A::Error>,
                $(
                    $err_type: $($bound)+,
                )*
                $f_err_type: $($f_bound)+,
                $final: std::error::Error,
            {
                Val0(Vec<TokenTree>, E0, A),
                // Val1(Vec<TokenTree>, E1Next<E0, A::Error>, B),
                $(
                    $sum(Vec<TokenTree>, $err_type, $gen),
                )*
                $f_sum(Vec<TokenTree>, $f_err_type, $f_gen)
            }

            impl<
                A: StateMachine,
                $($gen: StateMachine,)*
                $f_gen: StateMachine,
                E0
            > Default for $mname<A, $($gen,)* $f_gen, E0>
            where
                E0: Default + ErrorNext<A::Error>,
                $(
                    $err_type: $($bound)+,
                )*
                $f_err_type: $($f_bound)+,
                $final: std::error::Error,
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
                E0: Default + ErrorNext<A::Error>,
                $(
                    $err_type: $($bound)+,
                )*
                $f_err_type: $($f_bound)+,
                $final: std::error::Error,
            {
                fn stepup(
                    mut self,
                ) -> std::ops::ControlFlow<
                    SmResult<
                        <Self as StateMachine>::Output,
                        <Self as StateMachine>::Error
                    >,
                    Self,
                > {
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
                E0: Default + ErrorNext<A::Error>,
                $(
                    $err_type: $($bound)+,
                )*
                $f_err_type: $($f_bound)+,
                $final: std::error::Error,
            {
                type Output = $name<A::Output, $($gen::Output,)* $f_gen::Output>;
                type Error = $final;

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
                            )
                            .stepup(),
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
                                )
                                .stepup(),
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

    // sum_n!(
    //     Sum22, Sum22M
    //     ; Val1, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
    //     : E2Next<E0, A::Error, B::Error>
    // );
    sum_n!(
        Sum2, Sum2M
        : Val1, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>), E2Next<E0, A::Error, B::Error>
    );
    sum_n!(
        Sum3, Sum3M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        : Val2, C,       E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ,                E3Next<E0, A::Error, B::Error, C::Error>
    );
    sum_n!(
        Sum4, Sum4M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3, C, E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        : Val3, D,       E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ,                E4Next<E0, A::Error, B::Error, C::Error, D::Error>
    );
    sum_n!(
        Sum5, Sum5M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3, C, E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4, D, E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        : Val4, E,       E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ,                E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>
    );
    sum_n!(
        Sum6, Sum6M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3, C, E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4, D, E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5, E, E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        : Val5, F,       E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ,                E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>
    );
    sum_n!(
        Sum7, Sum7M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3, C, E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4, D, E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5, E, E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6, F, E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        : Val6, G,       E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        ,                E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>
    );
    sum_n!(
        Sum8, Sum8M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3, C, E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4, D, E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5, E, E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6, F, E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ; Val6, Val7, G, E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        : Val7, H,       E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>, (ErrorNext<H::Error>)
        ,                E8Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error>
    );
    sum_n!(
        Sum9, Sum9M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3, C, E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4, D, E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5, E, E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6, F, E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ; Val6, Val7, G, E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        ; Val7, Val8, H, E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>, (ErrorNext<H::Error>)
        : Val8, I,       E8Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error>, (ErrorNext<I::Error>)
        ,                E9Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error>
    );
    sum_n!(
        Sum10, Sum10M
        ; Val1, Val2, B, E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3, C, E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4, D, E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5, E, E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6, F, E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ; Val6, Val7, G, E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        ; Val7, Val8, H, E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>, (ErrorNext<H::Error>)
        ; Val8, Val9, I, E8Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error>, (ErrorNext<I::Error>)
        : Val9, J,       E9Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error>, (ErrorNext<J::Error>)
        ,                E10Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error>
    );
    sum_n!(
        Sum11, Sum11M
        ; Val1, Val2,  B,  E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3,  C,  E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4,  D,  E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5,  E,  E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6,  F,  E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ; Val6, Val7,  G,  E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        ; Val7, Val8,  H,  E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>, (ErrorNext<H::Error>)
        ; Val8, Val9,  I,  E8Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error>, (ErrorNext<I::Error>)
        ; Val9, Val10, J,  E9Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error>, (ErrorNext<J::Error>)
        : Val10, K,       E10Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error>, (ErrorNext<K::Error>)
        ,                 E11Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error>
    );
    sum_n!(
        Sum12, Sum12M
        ; Val1, Val2,   B,  E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3,   C,  E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4,   D,  E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5,   E,  E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6,   F,  E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ; Val6, Val7,   G,  E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        ; Val7, Val8,   H,  E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>, (ErrorNext<H::Error>)
        ; Val8, Val9,   I,  E8Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error>, (ErrorNext<I::Error>)
        ; Val9, Val10,  J,  E9Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error>, (ErrorNext<J::Error>)
        ; Val10, Val11, K, E10Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error>, (ErrorNext<K::Error>)
        : Val11, L,        E11Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error>, (ErrorNext<L::Error>)
        ,                  E12Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error, L::Error>
    );
    sum_n!(
        Sum13, Sum13M
        ; Val1, Val2,   B,  E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3,   C,  E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4,   D,  E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5,   E,  E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6,   F,  E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ; Val6, Val7,   G,  E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        ; Val7, Val8,   H,  E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>, (ErrorNext<H::Error>)
        ; Val8, Val9,   I,  E8Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error>, (ErrorNext<I::Error>)
        ; Val9, Val10,  J,  E9Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error>, (ErrorNext<J::Error>)
        ; Val10, Val11, K, E10Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error>, (ErrorNext<K::Error>)
        ; Val11, Val12, L, E11Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error>, (ErrorNext<L::Error>)
        : Val12, M,        E12Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error, L::Error>, (ErrorNext<M::Error>)
        ,                  E13Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error, L::Error, M::Error>
    );
    sum_n!(
        Sum14, Sum14M
        ; Val1, Val2,   B,  E1Next<E0, A::Error>, (ErrorNext<B::Error>)
        ; Val2, Val3,   C,  E2Next<E0, A::Error, B::Error>, (ErrorNext<C::Error>)
        ; Val3, Val4,   D,  E3Next<E0, A::Error, B::Error, C::Error>, (ErrorNext<D::Error>)
        ; Val4, Val5,   E,  E4Next<E0, A::Error, B::Error, C::Error, D::Error>, (ErrorNext<E::Error>)
        ; Val5, Val6,   F,  E5Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error>, (ErrorNext<F::Error>)
        ; Val6, Val7,   G,  E6Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error>, (ErrorNext<G::Error>)
        ; Val7, Val8,   H,  E7Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error>, (ErrorNext<H::Error>)
        ; Val8, Val9,   I,  E8Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error>, (ErrorNext<I::Error>)
        ; Val9, Val10,  J,  E9Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error>, (ErrorNext<J::Error>)
        ; Val10, Val11, K, E10Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error>, (ErrorNext<K::Error>)
        ; Val11, Val12, L, E11Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error>, (ErrorNext<L::Error>)
        ; Val12, Val13, M, E12Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error, L::Error>, (ErrorNext<M::Error>)
        : Val13, N,        E13Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error, L::Error, M::Error>, (ErrorNext<N::Error>)
        ,                  E14Next<E0, A::Error, B::Error, C::Error, D::Error, E::Error, F::Error, G::Error, H::Error, I::Error, J::Error, K::Error, L::Error, M::Error, N::Error>
    );

    // #[derive(Clone, Debug)]
    // pub enum Sum2<A, B> {
    //     Val0(A),
    //     Val1(B),
    // }
    // pub enum Sum2M<A: StateMachine, B: StateMachine, E0: ErrorNext<A::Error>>
    // where
    //     E1Next<E0, A::Error>: ErrorNext<B::Error>,
    //     E1Next<E1Next<E0, A::Error>, B::Error>: std::error::Error,
    // {
    //     Val0(Vec<TokenTree>, E0, A),
    //     Val1(Vec<TokenTree>, E1Next<E0, A::Error>, B),
    // }

    // impl<A: StateMachine, B: StateMachine, E0> Default for Sum2M<A, B, E0>
    // where
    //     E0: ErrorNext<A::Error> + Default,
    //     E1Next<E0, A::Error>: ErrorNext<B::Error>,
    //     E2Next<E0, A::Error, B::Error>: std::error::Error,
    // {
    //     fn default() -> Self {
    //         Self::Val0(Default::default(), Default::default(), Default::default())
    //     }
    // }

    // impl<A: Parsable, B: Parsable> Parsable for Sum2<A, B> {
    //     type StateMachine = Sum2M<A::StateMachine, B::StateMachine, Sum0Err>;
    // }

    // impl<A: StateMachine, B: StateMachine, E0> Sum2M<A, B, E0>
    // where
    //     E0: Default + ErrorNext<A::Error>,
    //     E1Next<E0, A::Error>: ErrorNext<B::Error>,
    //     E2Next<E0, A::Error, B::Error>: std::error::Error,
    // {
    //     fn stepup(
    //         mut self,
    //     ) -> std::ops::ControlFlow<
    //         SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error>,
    //         Self,
    //     > {
    //         'main: loop {
    //             match self {
    //                 Sum2M::Val0(vs, e, mut sm) => {
    //                     let len = vs.len();

    //                     for (i, v) in vs.iter().enumerate() {
    //                         match sm.drive(v) {
    //                             Continue(v) => sm = v,
    //                             Break(Ok((a, mut rl))) => {
    //                                 rl += len - i;
    //                                 rl -= 1;

    //                                 return Break(Ok((Sum2::Val0(a), rl)));
    //                             }
    //                             Break(Err(e_next)) => {
    //                                 self = Self::Val1(vs, e.next(e_next), Default::default());
    //                                 continue 'main;
    //                             }
    //                         }
    //                     }
    //                     return Continue(Self::Val0(vs, e, sm));
    //                 }
    //                 Sum2M::Val1(vs, e, mut sm) => {
    //                     let len = vs.len();

    //                     for (i, v) in vs.iter().enumerate() {
    //                         match sm.drive(v) {
    //                             Continue(v) => sm = v,
    //                             Break(Ok((a, mut rl))) => {
    //                                 rl += len - i;
    //                                 rl -= 1;

    //                                 return Break(Ok((Sum2::Val1(a), rl)));
    //                             }
    //                             Break(Err(e_next)) => {
    //                                 return Break(Err(e.next(e_next)));
    //                             }
    //                         }
    //                     }
    //                     return Continue(Self::Val1(vs, e, sm));
    //                 }
    //             }
    //         }
    //     }
    // }
    // impl<A: StateMachine, B: StateMachine, E0> StateMachine for Sum2M<A, B, E0>
    // where
    //     E0: Default + ErrorNext<A::Error>,
    //     E1Next<E0, A::Error>: ErrorNext<B::Error>,
    //     E2Next<E0, A::Error, B::Error>: std::error::Error,
    // {
    //     type Output = Sum2<A::Output, B::Output>;
    //     type Error = E2Next<E0, A::Error, B::Error>;

    //     fn drive(
    //         self,
    //         val: &TokenTree,
    //     ) -> std::ops::ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
    //         use std::ops::ControlFlow::*;

    //         match self {
    //             Sum2M::Val0(mut vs, e, sm) => match sm.drive(val) {
    //                 Continue(sm) => Continue(Self::Val0(
    //                     {
    //                         vs.push(val.clone());
    //                         vs
    //                     },
    //                     e,
    //                     sm,
    //                 )),
    //                 Break(Ok((a, rl))) => Break(Ok((Sum2::Val0(a), rl))),
    //                 Break(Err(e_next)) => Self::Val1(
    //                     {
    //                         vs.push(val.clone());
    //                         vs
    //                     },
    //                     e.next(e_next),
    //                     Default::default(),
    //                 )
    //                 .stepup(),
    //             },
    //             Sum2M::Val1(mut vs, e, sm) => match sm.drive(val) {
    //                 Continue(sm) => Continue(Self::Val1(
    //                     {
    //                         vs.push(val.clone());
    //                         vs
    //                     },
    //                     e,
    //                     sm,
    //                 )),
    //                 Break(Ok((a, rl))) => Break(Ok((Sum2::Val1(a), rl))),
    //                 Break(Err(e_next)) => Break(Err(e.next(e_next))),
    //             },
    //         }
    //     }

    //     fn terminate(mut self) -> SmResult<Self::Output, Self::Error> {
    //         loop {
    //             match self {
    //                 Sum2M::Val0(vs, e, sm) => match sm.terminate() {
    //                     Ok((a, rl)) => return Ok((Sum2::Val0(a), rl)),
    //                     Err(e_next) => {
    //                         match Self::Val1(vs, e.next(e_next), Default::default()).stepup() {
    //                             Continue(a) => self = a,
    //                             Break(o) => return o,
    //                         }
    //                     }
    //                 },
    //                 Sum2M::Val1(_, e, sm) => {
    //                     return match sm.terminate() {
    //                         Ok((a, rl)) => Ok((Sum2::Val1(a), rl)),
    //                         Err(e_next) => Err(e.next(e_next)),
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

// We can actually fix the memory scaling issue by transforming the problem to a state where we
// just iteratively map a series of options until reach a valid value. We must hold an iterative
// error term tho. This could optionally be boxed. An idea is also making it generic across a
// monoid error composer. This would result in a sum that is (with a forgetful error mapping) O(1)
// sized where n is the amount of types its summed over. This would be a great improvement from
// the current O(n) sizing scheme

// pub use new_sum::*;
pub use new_sum::*;
// pub use sum_old::*;
// #[cfg(disable)]
mod sum_old {
    use crate::{ControlFlow, Parsable, SmResult, StateMachine, TokenTree};
    use Sum2::Val1 as V1;

    macro_rules! sum {
        ($name:ident, $mname:ident, $err_name:ident $err:literal, $rl:ident $(; $gen:ident, $sum:ident, $product:ident, $pat:pat)+) => {
            #[derive(Clone, Debug)]
            pub enum $name<A, $($gen),+> {
                Val0(A),
                $($sum ($gen)),+
            }

            impl<A: Parsable, $($gen: Parsable),+> Parsable for $name<A, $($gen),+> {
                type StateMachine = $mname<A::StateMachine, $($gen::StateMachine),+>;
            }
            // This either-machine short circuits to the left (first) value
            pub struct $mname<A: StateMachine, $($gen: StateMachine),+> {
                v0: Sum2<A, A::Error>,
                $($product: Sum2<$gen, SmResult<$gen::Output, $gen::Error>>,)+
            }

            impl<A: StateMachine, $($gen: StateMachine),+> Default for $mname<A, $($gen),+> {
                fn default() -> Self {
                    Self {
                        v0: Sum2::Val0(Default::default()),
                        $($product: Sum2::Val0(Default::default()),)+
                    }
                }
            }

            #[derive(Clone, thiserror::Error, Debug)]
            #[error($err, .v0, $(. $product,)+)]
            pub struct $err_name<A: std::error::Error, $($gen: std::error::Error),+> {
                pub v0: A,
                $(pub $product: $gen,)+
            }

            impl<A: StateMachine, $($gen: StateMachine),+> StateMachine for $mname<A, $($gen),+> {
                type Output = $name<A::Output, $($gen::Output),+>;
                type Error = $err_name<A::Error, $($gen::Error),+>;

                fn drive(
                    self,
                    val: &TokenTree,
                ) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                    use $name::*;
                    use ControlFlow::*;

                    let Self { mut v0, $(mut $product),+ } = self;

                    v0 = match v0 {
                        Sum2::Val0(v) => match v.drive(val) {
                            Continue(v) => Sum2::Val0(v),
                            Break(Ok((v, rl))) => return Break(Ok((Val0(v), rl))),
                            Break(Err(v)) => Sum2::Val1(v),
                        },
                        e => e,
                    };
                    'block: {
                        $({
                            match $product {
                                Sum2::Val0(v) => match v.drive(val) {
                                    Continue(v) => $product = Sum2::Val0(v),
                                    Break(Ok(v)) => {
                                        $product = Sum2::Val1(Ok(v));
                                        break 'block
                                    },
                                    Break(v) => $product = Sum2::Val1(v),
                                },
                                Sum2::Val1(Ok((v, run_length))) => {
                                    $product = Sum2::Val1(Ok((v, run_length + 1)));
                                    break 'block
                                },
                                e => $product = e,
                            };
                        })+
                    };

                    match (v0, $($product,)+) {
                        $($pat => Break(Ok(($sum($product), $rl))),)+

                        (Sum2::Val1(v0), $(Sum2::Val1(Err($product))),+) => Break(Err($err_name { v0, $($product),+ })),
                        (v0, $($product,)+) => Continue(Self { v0, $($product),+ }),
                    }
                }

                fn terminate(self) -> SmResult<Self::Output, Self::Error> {
                    use $name::*;

                    let Self { mut v0, $(mut $product),+ } = self;

                    v0 = match v0 {
                        Sum2::Val0(v) => match v.terminate() {
                            Ok((v, rl)) => return Ok((Val0(v), rl)),
                            Err(v) => Sum2::Val1(v),
                        },
                        v => v
                    };

                    $({
                        $product = match $product {
                            Sum2::Val0($product) => match $product.terminate() {
                                Ok((v, rl)) => return Ok(($sum(v), rl)),
                                v => Sum2::Val1(v),
                            },
                            Sum2::Val1(Ok((v, rl))) => return Ok(($sum(v), rl)),
                            v => v,
                        };
                    })+

                    match (v0, $($product),+) {
                        (Sum2::Val1(v0), $(Sum2::Val1(Err($product))),+) => Err($err_name {
                            v0,
                            $($product),+
                        }),
                        _ => unreachable!()
                    }
                }

                #[cfg(feature = "execution-debug")]
                fn inspect(&self, depth: usize) {
                    $({
                        if let Sum2::Val0(ref v) = self.$product {
                            v.inspect(depth);
                        }
                    })+
                }
            }
        };
    }

    sum!(Sum2, Sum2M, Sum2Err "e0: ({}) e1: ({})", rl; B, Val1, v1, (V1(_), V1(Ok((v1, rl)))));
    sum!(Sum3, Sum3M, Sum3Err "e0: ({}) e0: ({}) e2: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))))
    );
    sum!(Sum4, Sum4M, Sum4Err "e0: ({}) e0: ({}) e2: ({}) e3: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))))
    );
    sum!(Sum5, Sum5M, Sum5Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))))
    );
    sum!(Sum6, Sum6M, Sum6Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))))
    );
    sum!(Sum7, Sum7M, Sum7Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))))
    );
    sum!(Sum8, Sum8M, Sum8Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _, _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))), _);
         H, Val7, v7, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v7, rl))))
    );
    sum!(Sum9, Sum9M, Sum9Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _, _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _, _, _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))), _, _);
         H, Val7, v7, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v7, rl))), _);
         I, Val8, v8, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v8, rl))))
    );
    sum!(Sum10, Sum10M, Sum10Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _, _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _, _, _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _, _, _, _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))), _, _, _);
         H, Val7, v7, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v7, rl))), _, _);
         I, Val8, v8, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v8, rl))), _);
         J, Val9, v9, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v9, rl))))
    );
    sum!(Sum11, Sum11M, Sum11Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _, _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _, _, _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _, _, _, _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _, _, _, _, _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))), _, _, _, _);
         H, Val7, v7, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v7, rl))), _, _, _);
         I, Val8, v8, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v8, rl))), _, _);
         J, Val9, v9, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v9, rl))), _);
         K, Val10, v10, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v10, rl))))
    );
    sum!(Sum12, Sum12M, Sum12Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({}) e11: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _, _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _, _, _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _, _, _, _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _, _, _, _, _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _, _, _, _, _, _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))), _, _, _, _, _);
         H, Val7, v7, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v7, rl))), _, _, _, _);
         I, Val8, v8, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v8, rl))), _, _, _);
         J, Val9, v9, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v9, rl))), _, _);
         K, Val10, v10, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v10, rl))), _);
         L, Val11, v11, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v11, rl))))
    );
    sum!(Sum13, Sum13M, Sum13Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({}) e11: ({}) e12: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _, _, _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _, _, _, _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _, _, _, _, _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _, _, _, _, _, _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _, _, _, _, _, _, _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))), _, _, _, _, _, _);
         H, Val7, v7, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v7, rl))), _, _, _, _, _);
         I, Val8, v8, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v8, rl))), _, _, _, _);
         J, Val9, v9, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v9, rl))), _, _, _);
         K, Val10, v10, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v10, rl))), _, _);
         L, Val11, v11, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v11, rl))), _);
         M, Val12, v12, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v12, rl))))
    );
    sum!(Sum14, Sum14M, Sum14Err "e0: ({}) e0: ({}) e2: ({}) e3: ({}) e4: ({}) e5: ({}) e6: ({}) e7: ({}) e8: ({}) e9: ({}) e10: ({}) e11: ({}) e12: ({}) e13: ({})", rl;
         B, Val1, v1, (V1(_), V1(Ok((v1, rl))), _, _, _, _, _, _, _, _, _, _, _, _);
         C, Val2, v2, (V1(_), V1(_), V1(Ok((v2, rl))), _, _, _, _, _, _, _, _, _, _, _);
         D, Val3, v3, (V1(_), V1(_), V1(_), V1(Ok((v3, rl))), _, _, _, _, _, _, _, _, _, _);
         E, Val4, v4, (V1(_), V1(_), V1(_), V1(_), V1(Ok((v4, rl))), _, _, _, _, _, _, _, _, _);
         F, Val5, v5, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v5, rl))), _, _, _, _, _, _, _, _);
         G, Val6, v6, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v6, rl))), _, _, _, _, _, _, _);
         H, Val7, v7, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v7, rl))), _, _, _, _, _, _);
         I, Val8, v8, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v8, rl))), _, _, _, _, _);
         J, Val9, v9, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v9, rl))), _, _, _, _);
         K, Val10, v10, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v10, rl))), _, _, _);
         L, Val11, v11, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v11, rl))), _, _);
         M, Val12, v12, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v12, rl))), _);
         N, Val13, v13, (V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(_), V1(Ok((v13, rl))))
    );
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::sum_old;
    use crate::*;

    type P = Punct;

    #[test]
    fn size_delta_calculation() {
        panic!(
            "{} {}",
            size_of::<
                <Sum14<
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                > as Parsable>::StateMachine,
            >(),
            size_of::<
                <sum_old::Sum14<
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                    Ident,
                > as Parsable>::StateMachine,
            >(),
        );
    }

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
    insta_match_test!(it_prioritizes_on_joint_match, Sum2<FIdent<"hello">, Ident> : hello);
}
