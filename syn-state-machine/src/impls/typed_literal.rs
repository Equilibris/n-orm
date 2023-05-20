use crate::*;

impl MappedParse for bool {
    type Source = Sum2<FIdent<"true">, FIdent<"false">>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(src: SmOut<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(matches!(src, Sum2::Val0(_)))
    }

    fn map_err(src: SmErr<<Self as MappedParse>::Source>) -> Self::Error {
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

            #[cfg(feature = "execution-debug")]
            fn inspect(&self, depth: usize) {
                println!("{}{}", "  ".repeat(depth), stringify!($machine));
            }
        }
    };
}

pub type ByteStringLit = litrs::ByteStringLit<String>;
pub type FloatLit = litrs::FloatLit<String>;
pub type IntegerLit = litrs::IntegerLit<String>;
pub type StringLit = litrs::StringLit<String>;
pub type CharLit = litrs::CharLit<String>;
pub type ByteLit = litrs::ByteLit<String>;

pub type NegativeIntegerLit = (FPunct<'-'>, IntegerLit);
pub type NegativeFloatLit = (FPunct<'-'>, FloatLit);

pub type SignedIntegerLit = (Option<FPunct<'-'>>, IntegerLit);
pub type SignedFloatLit = (Option<FPunct<'-'>>, FloatLit);

typed_lit!(ByteStringLit, ByteStringLitMachine);
typed_lit!(FloatLit, FloatLitMachine);
typed_lit!(IntegerLit, IntegerLitMachine);
typed_lit!(StringLit, StringLitMachine);
typed_lit!(CharLit, CharLitMachine);
typed_lit!(ByteLit, ByteLitMachine);

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::*;

    insta_match_test!(it_matches_int, IntegerLit : 0);
    insta_match_test!(it_matches_signed_int, SignedIntegerLit : -10);
    insta_match_test!(it_matches_float, FloatLit : 0.0);
    insta_match_test!(it_matches_signed_float, SignedFloatLit : -0.0e10);
}
