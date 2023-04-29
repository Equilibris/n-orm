use crate::*;

pub struct Identifier(Ident);

impl SmFrom<proc_macro2::Ident> for Identifier {
    fn sm_from(src: proc_macro2::Ident) -> Self {
        Self(src)
    }
}

impl L2Parsablea for Identifier {
    type StateMachine = <MapOut<Ident, Identifier> as L2Parsable>::StateMachine;
}

pub struct Attribute<T: L2Parsable>(SmOutput<T>);

impl<T: L2Parsable> SmFrom<(FJointPunct<'#'>, Bracket<T>)> for Attribute<T> {
    fn sm_from(src: (FJointPunct<'#'>, Bracket<T>)) -> Self {
        Self(src.1 .0)
    }
}

impl<T: L2Parsable> L2Parsable for Attribute<T> {
    type StateMachine =
        <MapOut<(FJointPunct<'#'>, Bracket<T>), Attribute<T>> as L2Parsable>::StateMachine;
}

pub enum Visability {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    // PubIn(SimplePath)
}

impl L2Parsable

#[cfg(test)]
mod tests {
    #[test]
    fn it_matches_lifetime() {}
}
