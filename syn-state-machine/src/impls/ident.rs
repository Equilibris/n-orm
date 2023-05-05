use crate::*;

#[derive(Debug, Clone)]
pub struct FIdent<const IDENT: &'static str>(pub Ident);

impl<const IDENT: &'static str> From<Ident> for FIdent<IDENT> {
    fn from(value: Ident) -> Self {
        Self(value)
    }
}
#[allow(clippy::from_over_into)]
impl<const IDENT: &'static str> Into<Ident> for FIdent<IDENT> {
    fn into(self) -> Ident {
        self.0
    }
}

impl<const IDENT: &'static str> Parsable for FIdent<IDENT> {
    type StateMachine = FixedIdentMachine<IDENT>;
}

#[derive(Default)]
pub struct FixedIdentMachine<const IDENT: &'static str>;

#[derive(Debug, thiserror::Error, Default)]
pub enum FixedIdentError<const IDENT: &'static str> {
    #[error("Expected ident \"{}\" but got {}", IDENT, .0)]
    Val(TokenTree),
    #[default]
    #[error("Expected ident \"{}\" but got termination", IDENT)]
    Termination,
}

impl<const IDENT: &'static str> StateMachine for FixedIdentMachine<IDENT> {
    type Output = FIdent<IDENT>;
    type Error = FixedIdentError<IDENT>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        match val {
            TokenTree::Ident(p) if p.to_string().as_str() == IDENT => {
                ControlFlow::Break(Ok((FIdent::<IDENT>(p.clone()), 0)))
            }
            _ => ControlFlow::Break(Err(FixedIdentError::Val(val.clone()))),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Default::default())
    }
}

pub use proc_macro2::Ident;

impl Parsable for Ident {
    type StateMachine = IdentMachine;
}

#[derive(Default)]
pub struct IdentMachine;

#[derive(Debug, Clone, thiserror::Error, Default)]
pub enum IdentError {
    #[error("Expected ident but got {}", .0)]
    Val(TokenTree),
    #[default]
    #[error("Expected ident but got termination")]
    Termination,
}

impl StateMachine for IdentMachine {
    type Output = Ident;
    type Error = IdentError;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        match val {
            TokenTree::Ident(p) => ControlFlow::Break(Ok((p.clone(), 0))),
            _ => ControlFlow::Break(Err(IdentError::default())),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_matches_only() {
        let v = parse_terminal::<Ident>(quote::quote! { id }).unwrap();

        assert_eq!(v.to_string().as_str(), "id");
    }
    #[test]
    fn it_matches_fixed() {
        parse_terminal::<FIdent<"id">>(quote::quote! { id }).unwrap();
    }
    #[test]
    fn it_fails_on_incorrect() {
        parse_terminal::<FIdent<"id">>(quote::quote! { ident }).unwrap_err();
    }
}
