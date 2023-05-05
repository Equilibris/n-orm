use crate::*;

pub struct IdentifierUnder;

impl Parsable for IdentifierUnder {
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
pub type Identifier = AndNot<IdentifierUnder, FIdent<"_">>;
