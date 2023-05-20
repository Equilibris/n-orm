use crate::*;

pub struct MBox<T: Parsable>(Box<T>);

impl<T: Parsable> Parsable for MBox<T> {
    type StateMachine = MBoxM<T::StateMachine>;
}

#[derive(Default)]
pub struct MBoxM<T: StateMachine>(Box<T>);
impl<T: StateMachine> StateMachine for MBoxM<T> {
    type Output = T::Output;
    type Error = T::Error;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        self.0.drive(val).map_continue(|v| Self(Box::new(v)))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        self.0.terminate()
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}MBox:", "  ".repeat(depth));
        self.0.inspect(depth + 1);
    }
}

pub struct PBox<T: Parsable>(Box<T>);

impl<T: Parsable> Parsable for PBox<T> {
    type StateMachine = PBoxM<T::StateMachine>;
}

#[derive(Default)]
pub struct PBoxM<T: StateMachine>(Box<T>);

impl<T: StateMachine> StateMachine for PBoxM<T> {
    type Output = Box<T::Output>;
    type Error = Box<T::Error>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        self.0
            .drive(val)
            .map_continue(|v| Self(Box::new(v)))
            .map_break(|v| v.map_err(Box::new).map(|(v, rl)| (Box::new(v), rl)))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        self.0
            .terminate()
            .map_err(Box::new)
            .map(|(v, rl)| (Box::new(v), rl))
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}PBox:", "  ".repeat(depth));
        self.0.inspect(depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::*;

    #[derive(Debug)]
    struct Test(pub Punct, pub Box<Option<Test>>);

    impl MappedParse for Test {
        type Source = (Punct, PBox<Option<Self>>);

        type Output = Self;
        type Error = SmErr<Self::Source>;

        fn map(
            src: SmOut<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(Self(src.0, src.1))
        }

        fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
            src
        }
    }

    insta_match_test!(it_parses_simple_infinite_type, Test : <><);
}
