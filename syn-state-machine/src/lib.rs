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

type SmResult<T, E> = Result<(T, usize), E>;
type SmOutput<T> = <<T as Parsable>::StateMachine as StateMachine>::Output;
type SmError<T> = <<T as Parsable>::StateMachine as StateMachine>::Error;

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
