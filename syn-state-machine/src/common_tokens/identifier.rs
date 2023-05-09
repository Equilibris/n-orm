use super::*;
use crate::*;

pub struct IdentifierOrUnder;

impl Parsable for IdentifierOrUnder {
    type StateMachine = Sm<
        AndNot<
            Ident,
            Either<
                super::keyword::Keyword,
                Either<
                    Either<FIdent<"r#crate">, FIdent<"r#super">>,
                    Either<FIdent<"r#self">, FIdent<"r#Self">>,
                >,
            >,
        >,
    >;
}
pub type Identifier = AndNot<IdentifierOrUnder, Underscore>;
