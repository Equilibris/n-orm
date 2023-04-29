use proc_macro::TokenStream as Ts1;
use proc_macro2::TokenTree;
use syn_state_machine::*;

enum Content {
    DirectVariable(Ident),
    Value(TokenTree),
}

impl SmFrom<Either<(FJointPunct<'$'>, Ident), Either<(FJointPunct<'$'>, Punct), TokenTree>>>
    for Content
{
    fn sm_from(
        src: Either<(FJointPunct<'$'>, Ident), Either<(FJointPunct<'$'>, Punct), TokenTree>>,
    ) -> Self {
        use Either::*;

        match src {
            Left((_, id)) => Content::DirectVariable(id),
            Right(Left((_, p))) => Content::Value(TokenTree::Punct(p)),
            Right(Right(t)) => Content::Value(t),
        }
    }
}

impl Parsable for Content {
    type StateMachine = <MapOut<
        Either<(FJointPunct<'$'>, Ident), Either<(FJointPunct<'$'>, Punct), TokenTree>>,
        Content,
    > as Parsable>::StateMachine;
}

#[proc_macro_derive(Parsable, attributes(parse))]
pub fn l2parsable(input: Ts1) -> Ts1 {
    input
}
