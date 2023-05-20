use crate::*;

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
    insta_match_test!(it_prioritizes_on_joint_match, Sum2<FIdent<"hello">, Ident> : hello);
}
