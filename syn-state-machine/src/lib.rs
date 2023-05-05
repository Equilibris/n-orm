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
pub type SmOutput<T> = <Sm<T> as StateMachine>::Output;
pub type SmError<T> = <Sm<T> as StateMachine>::Error;

pub trait StateMachine: Default {
    type Output;
    type Error: Error;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self>;
    fn terminate(self) -> SmResult<Self::Output, Self::Error>;
}

pub fn parse<T: Parsable>(stream: TokenStream) -> SmResult<SmOutput<T>, SmError<T>> {
    let mut state_machine: T::StateMachine = Default::default();

    for i in stream {
        use ControlFlow::*;
        match state_machine.drive(&i) {
            Continue(c) => state_machine = c,
            Break(c) => return c,
        }
    }
    state_machine.terminate()
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
) -> Result<SmOutput<T>, TerminalError<SmError<T>>> {
    use Either::*;
    use TerminalError::*;

    let state: T::StateMachine = Default::default();
    let mut state = Either::Left(state);

    let mut stream = stream.into_iter();

    loop {
        if let Some(v) = stream.next() {
            match state {
                Left(m) => match m.drive(&v) {
                    ControlFlow::Continue(c) => state = Left(c),
                    ControlFlow::Break(c) => state = Right(c),
                },
                Right(Err(e)) => break Err(Inner(e)),
                Right(_) => break Err(NonTerminal),
            }
        } else {
            match state {
                Left(state) => {
                    break match state.terminate() {
                        Ok((v, 0)) => Ok(v),
                        Err(e) => Err(Inner(e)),
                        _ => Err(NonTerminal),
                    }
                }

                Right(e) => break e.map(|(o, _)| o).map_err(Inner),
            }
        }
    }
}
