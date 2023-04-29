use crate::*;

pub struct Identifier(Ident);

impl SmFrom<proc_macro2::Ident> for Identifier {
    fn sm_from(src: proc_macro2::Ident) -> Self {
        Self(src)
    }
}

impl Parsable for Identifier {
    type StateMachine = <MapOut<Ident, Identifier> as Parsable>::StateMachine;
}

pub struct Attribute<T: Parsable>(SmOutput<T>);

impl<T: Parsable> SmFrom<(FJointPunct<'#'>, Bracket<T>)> for Attribute<T> {
    fn sm_from(src: (FJointPunct<'#'>, Bracket<T>)) -> Self {
        Self(src.1 .0)
    }
}

impl<T: Parsable> Parsable for Attribute<T> {
    type StateMachine =
        <MapOut<(FJointPunct<'#'>, Bracket<T>), Attribute<T>> as Parsable>::StateMachine;
}

pub enum Visability {
    Pub,
    PubCrate,
    PubSelf,
    PubSuper,
    // PubIn(SimplePath)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_matches_lifetime() {}
}
