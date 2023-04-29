use proc_macro2::Delimiter;

use crate::*;

/// Matches a general grouping of either (), [], or {}
pub struct Group<T: Parsable>(pub SmOutput<T>, pub Delimiter);

impl<T: Parsable> Parsable for Group<T> {
    type StateMachine = GroupMachine<T>;
}

pub struct GroupMachine<T: Parsable>(Option<T::StateMachine>);
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
    type Error = GroupError<TupleError<SmError<T>, TerminateError, SmOutput<T>>>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        ControlFlow::Break(match val {
            TokenTree::Group(g) => match parse::<(T, Terminate)>(g.stream()) {
                Ok(((a, _), _)) => Ok((Group(a, g.delimiter()), 0)),
                Err(e) => Err(GroupError::NestedError(e)),
            },
            e => Err(GroupError::InvalidToken(e.clone())),
        })
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(GroupError::Termination)
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
        pub struct $name<T: Parsable>(pub SmOutput<T>);

        impl<T: Parsable> Parsable for $name<T> {
            type StateMachine = $machine<T>;
        }

        pub struct $machine<T: Parsable>(Option<T::StateMachine>);
        impl<T: Parsable> Default for $machine<T> {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl<T: Parsable> StateMachine for $machine<T> {
            type Output = $name<T>;
            type Error = SpecifiedGroupError<TupleError<SmError<T>, TerminateError, SmOutput<T>>>;

            fn drive(
                self,
                val: &TokenTree,
            ) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                ControlFlow::Break(match val {
                    TokenTree::Group(g) => {
                        if g.delimiter() == $delim_ty {
                            match parse::<(T, Terminate)>(g.stream()) {
                                Ok(((a, _), _)) => Ok(($name(a), 0)),
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
        }
    };
}
specified_group!(Parenthesis, ParenthesisMachine, Delimiter::Parenthesis);
specified_group!(Brace, BraceMachine, Delimiter::Brace);
specified_group!(Bracket, BracketMachine, Delimiter::Bracket);
specified_group!(NoneGroup, NoneMachine, Delimiter::None);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_matches_general_delim() {
        let ((_, Group(content, _), _), l) =
            parse::<(Ident, Group<Ident>, Ident)>(quote::quote! { hello (world) hi }).unwrap();

        assert_eq!(l, 0);
        assert_eq!(content.to_string().as_str(), "world");
    }
    #[test]
    fn it_matches_parenthesis() {
        let ((_, Parenthesis(content), _), l) =
            parse::<(Ident, Parenthesis<Ident>, Ident)>(quote::quote! { hello (world) hi })
                .unwrap();

        assert_eq!(l, 0);
        assert_eq!(content.to_string().as_str(), "world");

        let a =
            match parse::<(Ident, Parenthesis<Ident>, Ident)>(quote::quote! { hello {world} hi }) {
                Ok(_) => panic!(),
                Err(e) => e,
            };

        use TupleError::*;
        match a {
            AFailed(BFailed(_, SpecifiedGroupError::InvalidDelimiter(_, _))) => (),
            _ => panic!(),
        }
    }
}
