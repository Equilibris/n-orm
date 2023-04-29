use crate::*;

impl<A: Parsable, B: Parsable> Parsable for (A, B) {
    type StateMachine = TupleMachine<A::StateMachine, B::StateMachine>;
}
pub enum TupleMachine<A: StateMachine, B: StateMachine> {
    A(Vec<TokenTree>, A),
    B(A::Output, B),
}

#[derive(Clone, thiserror::Error)]
pub enum TupleError<A: std::error::Error, B: std::error::Error, Out> {
    #[error("First tuple variant failed: {}", .0)]
    AFailed(A),
    #[error("Second tuple variant failed: {}", .1)]
    BFailed(Out, B),

    #[error("Internal token content was of length {} but requested {}", .0, .1)]
    InvalidLength(usize, usize),
}

impl<A: std::error::Error, B: std::error::Error, Out> std::fmt::Debug for TupleError<A, B, Out> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AFailed(arg0) => f.debug_tuple("AFailed").field(arg0).finish(),
            Self::BFailed(_, arg1) => f
                .debug_tuple("BFailed")
                .field(&"<Hidden>")
                .field(arg1)
                .finish(),
            Self::InvalidLength(arg0, arg1) => f
                .debug_tuple("InvalidLength")
                .field(arg0)
                .field(arg1)
                .finish(),
        }
    }
}

impl<A: StateMachine, B: StateMachine> Default for TupleMachine<A, B> {
    fn default() -> Self {
        Self::A(Vec::new(), A::default())
    }
}
impl<A: StateMachine, B: StateMachine> TupleMachine<A, B> {
    fn process_a_stepup(
        content: Vec<TokenTree>,
        a: A::Output,
        mut a_backtrack: usize,
    ) -> ControlFlow<SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error>, Self>
    {
        use ControlFlow::*;

        let mut b = B::default();

        let len = content.len();

        if a_backtrack > len {
            return Break(Err(TupleError::InvalidLength(a_backtrack, len)));
        }
        while a_backtrack > 0 {
            match b.drive(&content[len - a_backtrack]) {
                Continue(c) => b = c,
                Break(Ok((ok, backtrack))) => return Break(Ok(((a, ok), backtrack + a_backtrack))),
                Break(Err(e)) => return Break(Err(TupleError::BFailed(a, e))),
            }
            a_backtrack -= 1;
        }

        Continue(TupleMachine::B(a, b))
    }

    fn terminate_b(
        a: A::Output,
        b: B,
    ) -> SmResult<<Self as StateMachine>::Output, <Self as StateMachine>::Error> {
        match b.terminate() {
            Ok((ok, b)) => Ok(((a, ok), b)),
            Err(b) => Err(TupleError::BFailed(a, b)),
        }
    }
}

impl<A: StateMachine, B: StateMachine> StateMachine for TupleMachine<A, B> {
    type Output = (A::Output, B::Output);
    type Error = TupleError<A::Error, B::Error, A::Output>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        use ControlFlow::*;

        match self {
            TupleMachine::A(mut content, a) => match a.drive(val) {
                Break(b) => match b {
                    Ok((a, a_backtrack)) => {
                        content.push(val.clone());
                        Self::process_a_stepup(content, a, a_backtrack)
                    }
                    Err(e) => Break(Err(TupleError::AFailed(e))),
                },
                Continue(v) => Continue({
                    content.push(val.clone());
                    Self::A(content, v)
                }),
            },
            TupleMachine::B(a, b) => match b.drive(val) {
                Break(b) => match b {
                    Ok((ok, backtrack)) => Break(Ok(((a, ok), backtrack))),
                    Err(e) => Break(Err(TupleError::BFailed(a, e))),
                },
                Continue(v) => Continue(Self::B(a, v)),
            },
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        match self {
            TupleMachine::A(content, a) => match a.terminate() {
                Ok((a, a_backtrack)) => match Self::process_a_stepup(content, a, a_backtrack) {
                    ControlFlow::Continue(s) => s.terminate(),
                    ControlFlow::Break(a) => a,
                },
                Err(e) => Err(TupleError::AFailed(e)),
            },
            TupleMachine::B(a, b) => Self::terminate_b(a, b),
        }
    }
}
mod higher_order_tuple {
    use crate::*;
    macro_rules! implSmFrom {
        ($($i:ident)+; $($t:ident)+) => {
            impl<$($t,)+ ZZ> SmFrom<(($($t),+), ZZ)> for ($($t,)+ ZZ) {
                fn sm_from((($($i,)+), zz): (($($t,)+), ZZ)) -> Self {
                    ($($i,)+ zz)
                }
            }
            impl<$($t : L2Parsable,)+ ZZ: L2Parsable> L2Parsable for ($($t,)+ ZZ) {
                type StateMachine= <MapOut<
                    (($($t,)+), ZZ),
                    ($(SmOutput<$t>,)+ SmOutput<ZZ>),
                > as L2Parsable>::StateMachine;
            }
        };
    }
    implSmFrom!(a b; A B);
    implSmFrom!(a b c; A B C);
    implSmFrom!(a b c d; A B C D);
    implSmFrom!(a b c d e; A B C D E);
    implSmFrom!(a b c d e f; A B C D E F);
    implSmFrom!(a b c d e f g; A B C D E F G);
    implSmFrom!(a b c d e f g h; A B C D E F G H);
    implSmFrom!(a b c d e f g h i; A B C D E F G H I);
    implSmFrom!(a b c d e f g h i j; A B C D E F G H I J);
    implSmFrom!(a b c d e f g h i j k; A B C D E F G H I J K);
    implSmFrom!(a b c d e f g h i j k l; A B C D E F G H I J K L);
    implSmFrom!(a b c d e f g h i j k l m; A B C D E F G H I J K L M);
    implSmFrom!(a b c d e f g h i j k l m n; A B C D E F G H I J K L M N);
    implSmFrom!(a b c d e f g h i j k l m n o; A B C D E F G H I J K L M N O);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_matches() {
        let p = parse::<(Ident, FIdent<"world">)>(quote::quote! {hello world}).unwrap();

        assert_eq!(p.0 .0.to_string().as_str(), "hello");
    }
    #[test]
    fn it_steps_back_for_options() {
        let ((a, b), _) = parse::<(Option<Ident>, Option<Punct>)>(quote::quote! { < }).unwrap();
        let b = b.unwrap().as_char();

        assert!(a.is_none());
        assert_eq!(b, '<');
    }
    #[test]
    fn it_only_steps_back_on_fail_for_options() {
        let ((a, b), _) = parse::<(Option<Ident>, Option<Punct>)>(quote::quote! { hi }).unwrap();
        let a = a.unwrap().to_string();

        assert_eq!(a.as_str(), "hi");
        assert!(b.is_none());
    }
    #[test]
    fn it_steps_back_for_multi_tuples() {
        let ((a, b, c, d), _) =
            parse::<(Option<Ident>, Option<Punct>, Option<Ident>, Option<Punct>)>(
                quote::quote! { hi <>},
            )
            .unwrap();
        let a = a.unwrap().to_string();
        let b = b.unwrap().as_char();
        let d = d.unwrap().as_char();

        assert_eq!(a.as_str(), "hi");
        assert_eq!(b, '<');
        assert!(c.is_none());
        assert_eq!(d, '>');
    }
}
