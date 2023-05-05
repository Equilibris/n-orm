use crate::*;

impl<T: Parsable> Parsable for Option<T> {
    type StateMachine = OptionMachine<T::StateMachine>;
}

#[derive(Default)]
pub struct OptionMachine<T: StateMachine>(T, usize);

impl<T: StateMachine> StateMachine for OptionMachine<T> {
    type Output = Option<T::Output>;
    type Error = std::convert::Infallible;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        let Self(v, run_length) = self;
        v.drive(val)
            .map_continue(move |v| Self(v, run_length + 1))
            .map_break(|v| {
                Ok(match v {
                    Ok((v, r)) => (Some(v), r),
                    Err(_) => (None, run_length + 1),
                })
            })
    }
    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        let Self(v, run_length) = self;
        Ok(match v.terminate() {
            Ok((v, r)) => (Some(v), r),
            Err(_) => (None, run_length),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_matches_only() {
        let p = parse::<Option<Ident>>(quote::quote! { < }).unwrap();

        assert_eq!(p.0, None);
        assert_eq!(p.1, 1);
    }
    #[test]
    fn it_returns_the_correct_length() {
        let p = parse::<Option<(Ident, Ident)>>(quote::quote! { hi < }).unwrap();

        assert_eq!(p.0, None);
        assert_eq!(p.1, 2);
    }
}
