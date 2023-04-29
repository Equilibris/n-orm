// TODO: add the rest of the literals later
use either::Either;

use crate::*;

impl SmFrom<Either<FIdent<"true">, FIdent<"false">>> for bool {
    fn sm_from(src: Either<FIdent<"true">, FIdent<"false">>) -> Self {
        src.is_left()
    }
}

impl Parsable for bool {
    type StateMachine =
        <MapOut<Either<FIdent<"true">, FIdent<"false">>, bool> as Parsable>::StateMachine;
}
