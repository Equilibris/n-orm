use std::marker::PhantomData;

use proc_macro2::Delimiter;

use crate::*;

/// Matches a general grouping of either (), [], or {}
pub struct Group<T: Parsable>(pub SmOut<T>, pub Delimiter);

impl<T: Parsable> std::fmt::Debug for Group<T>
where
    SmOut<T>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Group")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<T: Parsable> Parsable for Group<T> {
    type StateMachine = GroupMachine<T>;
}

pub struct GroupMachine<T: Parsable>(PhantomData<T>);
impl<T: Parsable> Default for GroupMachine<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GroupError<T: std::error::Error> {
    #[error("A grouping was found but the sub-parsing resulted in the error: {}", .0)]
    NestedError(T),
    #[error("Expected grouping but got: {}", .0)]
    InvalidToken(TokenTree),
    #[error("Expected grouping but got termination")]
    Termination,
}

impl<T: Parsable> StateMachine for GroupMachine<T> {
    type Output = Group<T>;
    type Error = GroupError<TerminalError<SmErr<T>>>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        ControlFlow::Break(match val {
            TokenTree::Group(g) => match parse_terminal::<T>(g.stream()) {
                Ok(a) => Ok((Group(a, g.delimiter()), 0)),
                Err(e) => Err(GroupError::NestedError(e)),
            },
            e => Err(GroupError::InvalidToken(e.clone())),
        })
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(GroupError::Termination)
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}Group:", "  ".repeat(depth));
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SpecifiedGroupError<T: std::error::Error> {
    #[error("A grouping was found but the sub-parsing resulted in the error: {}", .0)]
    NestedError(T),
    #[error("Expected delimiter {:?} but got {:?}", .0, .1)]
    InvalidDelimiter(Delimiter, Delimiter),
    #[error("Expected grouping but got {}", .0)]
    InvalidToken(TokenTree),
    #[error("Expected grouping but got termination")]
    Termination,
}

macro_rules! specified_group {
    ($name:ident, $machine:ident, $delim_ty: path) => {
        #[derive(Debug)]
        pub struct $name<T>(pub T);

        impl<T: Parsable> Parsable for $name<T> {
            type StateMachine = $machine<T>;
        }

        pub struct $machine<T: Parsable>(PhantomData<T>);
        impl<T: Parsable> Default for $machine<T> {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl<T: Parsable> StateMachine for $machine<T> {
            type Output = $name<SmOut<T>>;
            type Error = SpecifiedGroupError<TerminalError<SmErr<T>>>;

            fn drive(
                self,
                val: &TokenTree,
            ) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                ControlFlow::Break(match val {
                    TokenTree::Group(g) => {
                        if g.delimiter() == $delim_ty {
                            match parse_terminal::<T>(g.stream()) {
                                Ok(a) => Ok(($name(a), 0)),
                                Err(e) => Err(SpecifiedGroupError::NestedError(e)),
                            }
                        } else {
                            Err(SpecifiedGroupError::InvalidDelimiter(
                                $delim_ty,
                                g.delimiter(),
                            ))
                        }
                    }
                    e => Err(SpecifiedGroupError::InvalidToken(e.clone())),
                })
            }

            fn terminate(self) -> SmResult<Self::Output, Self::Error> {
                Err(SpecifiedGroupError::Termination)
            }

            #[cfg(feature = "execution-debug")]
            fn inspect(&self, depth: usize) {
                println!("{}Group:", "  ".repeat(depth));
            }
        }
    };
}
specified_group!(Paren, ParenthesisMachine, Delimiter::Parenthesis);
specified_group!(Brace, BraceMachine, Delimiter::Brace);
specified_group!(Bracket, BracketMachine, Delimiter::Bracket);
specified_group!(NoneGroup, NoneMachine, Delimiter::None);

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(it_matches_general_delim, (Ident, Group<Ident>, Ident) : hello (world) hi );
    insta_match_test!(it_matches_parenthesis, (Ident, Paren<Ident>, Ident) : hello (world) hi );
    insta_match_test!(it_fails_on_wrong_delim, (Ident, Paren<Ident>, Ident) : hello {world} hi );
}
