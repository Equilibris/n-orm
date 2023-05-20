#![warn(clippy::nursery)]
#![allow(incomplete_features)]
#![feature(control_flow_enum, adt_const_params)]
mod common_tokens;
mod impls;

use std::{error::Error, ops::ControlFlow};

use proc_macro2::{TokenStream, TokenTree};

pub use impls::*;

pub trait Parsable {
    type StateMachine: StateMachine;
}

pub type Sm<T> = <T as Parsable>::StateMachine;
pub type SmResult<T, E> = Result<(T, usize), E>;
pub type SmOut<T> = <Sm<T> as StateMachine>::Output;
pub type SmErr<T> = <Sm<T> as StateMachine>::Error;

pub trait StateMachine: Default {
    type Output;
    type Error: Error;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self>;
    fn terminate(self) -> SmResult<Self::Output, Self::Error>;

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize);
}

pub fn parse<T: Parsable>(stream: TokenStream) -> SmResult<SmOut<T>, SmErr<T>> {
    let mut state_machine: T::StateMachine = Default::default();

    for i in stream {
        use ControlFlow::*;

        #[cfg(feature = "execution-debug")]
        {
            println!(":: {}", i);
            state_machine.inspect(0);
        }

        match state_machine.drive(&i) {
            Continue(c) => state_machine = c,
            Break(c) => return c,
        }
    }
    state_machine.terminate()
}

#[cfg(test)]
#[macro_export]
macro_rules! insta_match_test {
    ($test_name:ident, $ty:ty : $($t:tt)*) => {
        #[test]
        fn $test_name() {
            ::insta::assert_debug_snapshot!(
                $crate::parse::<$ty>(::quote::quote!{$($t)*})
            );
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum TerminalError<T: std::error::Error> {
    #[error("{}",.0)]
    Inner(T),
    #[error("Did not terminate")]
    NonTerminal,
}

pub fn parse_terminal<T: Parsable>(
    stream: TokenStream,
) -> Result<SmOut<T>, TerminalError<SmErr<T>>> {
    use Sum2::*;
    use TerminalError::*;

    let state: T::StateMachine = Default::default();
    let mut state = Sum2::Val0(state);

    let mut stream = stream.into_iter();

    loop {
        if let Some(v) = stream.next() {
            match state {
                Val0(m) => {
                    #[cfg(feature = "execution-debug")]
                    {
                        println!(":: {}", v);
                        m.inspect(0);
                    }

                    match m.drive(&v) {
                        ControlFlow::Continue(c) => state = Val0(c),
                        ControlFlow::Break(c) => state = Val1(c),
                    }
                }
                Val1(Err(e)) => break Err(Inner(e)),
                Val1(_) => break Err(NonTerminal),
            }
        } else {
            match state {
                Val0(state) => {
                    break match state.terminate() {
                        Ok((v, 0)) => Ok(v),
                        Err(e) => Err(Inner(e)),
                        _ => Err(NonTerminal),
                    }
                }

                Val1(e) => break e.map(|(o, _)| o).map_err(Inner),
            }
        }
    }
}
