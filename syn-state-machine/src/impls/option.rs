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

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}Option:", "  ".repeat(depth));
        self.0.inspect(depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(it_matches_only, Option<Ident> : <);
    insta_match_test!(it_returns_the_correct_length, Option<(Ident, Ident)> : hi <);
}
