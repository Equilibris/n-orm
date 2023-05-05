use crate::*;

pub trait MappedParse {
    type Source: Parsable;

    type Output;
    type Error: std::error::Error;

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error>;
    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error;
}

pub struct MappedMachine<T: MappedParse>(Sm<T::Source>);

impl<T: MappedParse> MappedMachine<T> {
    fn map(
        src: SmResult<SmOutput<T::Source>, SmError<T::Source>>,
    ) -> SmResult<T::Output, T::Error> {
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
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_maps() {
        struct V(String);

        impl MappedParse for V {
            type Source = Ident;

            type Output = Self;
            type Error = SmError<Self::Source>;

            fn map(
                src: SmOutput<Self::Source>,
            ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
                Ok(V(src.to_string()))
            }

            fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
                src
            }
        }

        let (V(v), _) = parse::<V>(quote::quote! { hello }).unwrap();

        assert_eq!(v.as_str(), "hello")
    }
}
