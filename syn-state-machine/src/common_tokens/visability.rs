use crate::*;

pub enum Visability {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    // PubIn(SimplePath)
}

// impl
//     SmFrom<
//         Either<
//             Either<Pub, (Pub, Parenthesis<FIdent<"self">>)>,
//             Either<(Pub, Parenthesis<FIdent<"crate">>), (Pub, Parenthesis<FIdent<"super">>)>,
//         >,
//     > for Visability
// {
//     fn sm_from(
//         src: Either<
//             Either<FIdent<"pub">, (FIdent<"pub">, Parenthesis<FIdent<"self">>)>,
//             Either<
//                 (FIdent<"pub">, Parenthesis<FIdent<"crate">>),
//                 (FIdent<"pub">, Parenthesis<FIdent<"super">>),
//             >,
//         >,
//     ) -> Self {
//         use Either::*;

//         match src {
//             Left(Left(_)) => Self::Pub,
//             Left(Right(_)) => Self::PubSelf,
//             Right(Left(_)) => Self::PubCrate,
//             Right(Right(_)) => Self::PubSuper,
//         }
//     }
// }

// type Pub = FIdent<"pub">;
// impl Parsable for Visability {
//     type StateMachine = Sm<
//         MapOut<
//             Either<
//                 Either<Pub, (Pub, Parenthesis<FIdent<"self">>)>,
//                 Either<(Pub, Parenthesis<FIdent<"crate">>), (Pub, Parenthesis<FIdent<"super">>)>,
//             >,
//             Visability,
//         >,
//     >;
// }
