use std::marker::PhantomData;

use crate::*;

pub trait SmFrom<A> {
    fn sm_from(src: A) -> Self;
}

pub struct MapOut<A: Parsable, B>(PhantomData<A>, PhantomData<B>);
pub struct MapErr<A: Parsable, B: std::error::Error>(PhantomData<A>, PhantomData<B>);

pub struct MapOutMachine<A, V>(A, PhantomData<V>);
pub struct MapErrMachine<A, V>(A, PhantomData<(A, V)>);
impl<A: Default, V> Default for MapOutMachine<A, V> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<A: Default, V> Default for MapErrMachine<A, V> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<A: Parsable, V: SmFrom<SmOutput<A>>> Parsable for MapOut<A, V> {
    type StateMachine = MapOutMachine<A::StateMachine, V>;
}
impl<A: Parsable, V: std::error::Error + SmFrom<SmError<A>>> Parsable for MapErr<A, V> {
    type StateMachine = MapErrMachine<A::StateMachine, V>;
}

impl<A: StateMachine, V: SmFrom<A::Output>> StateMachine for MapOutMachine<A, V> {
    type Output = V;
    type Error = A::Error;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        self.0
            .drive(val)
            .map_continue(|v| Self(v, PhantomData))
            .map_break(|v| v.map(|(a, r)| (V::sm_from(a), r)))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        self.0.terminate().map(|(a, r)| (V::sm_from(a), r))
    }
}
impl<A: StateMachine, V: std::error::Error + SmFrom<A::Error>> StateMachine
    for MapErrMachine<A, V>
{
    type Output = A::Output;
    type Error = V;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        self.0
            .drive(val)
            .map_continue(|v| Self(v, PhantomData))
            .map_break(|v| v.map_err(V::sm_from))
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        self.0.terminate().map_err(V::sm_from)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_maps() {
        struct V(String);

        impl SmFrom<proc_macro2::Ident> for V {
            fn sm_from(src: proc_macro2::Ident) -> Self {
                Self(src.to_string())
            }
        }

        let (V(v), _) = parse::<MapOut<Ident, V>>(quote::quote! { hello }).unwrap();

        assert_eq!(v.as_str(), "hello")
    }
}
