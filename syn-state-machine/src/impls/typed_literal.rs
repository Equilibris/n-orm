use either::Either;

use crate::*;

impl MappedParse for bool {
    type Source = Either<FIdent<"true">, FIdent<"false">>;

    type Output = Self;
    type Error = SmError<Self::Source>;

    fn map(src: SmOutput<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(src.is_left())
    }

    fn map_err(src: SmError<<Self as MappedParse>::Source>) -> Self::Error {
        src
    }
}

#[derive(Debug, thiserror::Error, Default)]
pub enum TypedLit {
    #[error("Expected literal got {}", .0)]
    Val(TokenTree),

    #[default]
    #[error("Expected lit got termination")]
    Termination,
}

macro_rules! typed_lit {
    ($v:ident, $machine:ident) => {
        impl Parsable for $v {
            type StateMachine = $machine;
        }

        #[derive(Default)]
        pub struct $machine;

        impl StateMachine for $machine {
            type Output = $v;
            type Error = TypedLit;

            fn drive(
                self,
                val: &TokenTree,
            ) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                let val = val.clone();

                match $v::try_from(val.clone()) {
                    Ok(v) => ControlFlow::Break(Ok((v, 0))),
                    Err(_) => ControlFlow::Break(Err(Self::Error::Val(val))),
                }
            }

            fn terminate(self) -> SmResult<Self::Output, Self::Error> {
                Err(Default::default())
            }
        }
    };
}

pub type ByteStringLit = litrs::ByteStringLit<String>;
pub type FloatLit = litrs::FloatLit<String>;
pub type IntegerLit = litrs::IntegerLit<String>;
pub type StringLit = litrs::StringLit<String>;

pub type SignedIntegerLit = (FPunct<'-'>, IntegerLit);
pub type SignedFloatLit = (FPunct<'-'>, FloatLit);

typed_lit!(ByteStringLit, ByteStringLitMachine);
typed_lit!(FloatLit, FloatLitMachine);
typed_lit!(IntegerLit, IntegerLitMachine);
typed_lit!(StringLit, StringLitMachine);

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::*;

    #[test]
    fn it_matches_int() {
        parse_terminal::<IntegerLit>(quote!(0)).unwrap();
    }
    #[test]
    fn it_matches_signed_int() {
        parse_terminal::<SignedIntegerLit>(quote!(-10)).unwrap();
    }
    #[test]
    fn it_matches_float() {
        parse_terminal::<FloatLit>(quote!(0.0)).unwrap();
    }
    #[test]
    fn it_matches_signed_float() {
        parse_terminal::<SignedFloatLit>(quote!(-10.0)).unwrap();
    }
}
