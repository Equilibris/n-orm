use super::*;
use crate::*;

pub struct IdentifierOrUnder;

impl Parsable for IdentifierOrUnder {
    type StateMachine = Sm<
        AndNot<
            Ident,
            Sum2<
                super::keyword::Keyword,
                Sum2<
                    Sum2<FIdent<"r#crate">, FIdent<"r#super">>,
                    Sum2<FIdent<"r#self">, FIdent<"r#Self">>,
                >,
            >,
        >,
    >;
}
pub type Identifier = AndNot<IdentifierOrUnder, Underscore>;
