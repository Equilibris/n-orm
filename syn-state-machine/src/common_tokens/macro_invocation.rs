use super::*;
use crate::*;

#[derive(Debug)]
pub struct MacroInvocation {
    pub path: SimplePath,
    pub content: TokenTree,
}
impl MappedParse for MacroInvocation {
    type Source = (SimplePath, Exclamation, TokenTree);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            path: src.0,
            content: src.2,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct MacroInvocationSemi {
    pub path: SimplePath,
    pub content: Tokens,
}
impl MappedParse for MacroInvocationSemi {
    type Source = (
        SimplePath,
        Either<(Either<Paren<Tokens>, Bracket<Tokens>>, Semi), Brace<Tokens>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            path: src.0,
            content: match src.1 {
                Either::Left(a) => match a.0 {
                    Either::Left(a) | Either::Right(a) => a,
                },
                Either::Right(a) => a,
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
