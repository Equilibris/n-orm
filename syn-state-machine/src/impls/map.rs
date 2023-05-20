use crate::*;

pub trait MappedParse {
    type Source: Parsable;

    type Output;
    type Error: std::error::Error;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error>;
    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error;
}

pub struct MappedMachine<T: MappedParse>(Sm<T::Source>);

impl<T: MappedParse> MappedMachine<T> {
    fn map(src: SmResult<SmOut<T::Source>, SmErr<T::Source>>) -> SmResult<T::Output, T::Error> {
        match src {
            Err(e) => Err(T::map_err(e)),
            Ok((ok, rl)) => match T::map(ok) {
                Ok(ok) => Ok((ok, rl)),
                Err(e) => Err(e),
            },
        }
    }
}

impl<T: MappedParse> Parsable for T {
    type StateMachine = MappedMachine<T>;
}

impl<T: MappedParse> Default for MappedMachine<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: MappedParse> StateMachine for MappedMachine<T> {
    type Output = T::Output;
    type Error = T::Error;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        self.0.drive(val).map_continue(Self).map_break(Self::map)
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Self::map(self.0.terminate())
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        self.0.inspect(depth);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(Debug)]
    struct V(String);

    impl MappedParse for V {
        type Source = Ident;

        type Output = Self;
        type Error = SmErr<Self::Source>;

        fn map(
            src: SmOut<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(V(src.to_string()))
        }

        fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
            src
        }
    }

    insta_match_test!(it_maps, V: ident);
}
