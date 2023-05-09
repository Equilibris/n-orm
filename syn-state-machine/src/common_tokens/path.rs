mod expressions;
mod qualified;
mod simple;
mod types;

pub use expressions::*;
pub use qualified::*;
pub use simple::*;
pub use types::*;

use super::*;
use crate::*;

use Either::*;

#[derive(Debug)]
pub enum PathIdentSegment {
    Id(Ident),
    DCrate,
}
impl MappedParse for PathIdentSegment {
    type Source = Either<
        FlatEither<
            FlatEither<FlatEither<Identifier, KwSuper>, FlatEither<KwLowerSelf, KwCrate, Ident>>,
            KwUpperSelf,
        >,
        DollarCrate,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Either::Left(v) => Self::Id(v),
            Either::Right(_) => Self::DCrate,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct GenericArgsBinding(pub Ident, pub Type);
impl MappedParse for GenericArgsBinding {
    type Source = (Identifier, Eq, Type);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.2))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub enum GenericArg {
    Lifetime(Lifetime),
    Type(Type),
    Const(TokenTree), // TODO:
    ArgsBinding(GenericArgsBinding),
}
impl MappedParse for GenericArg {
    type Source = Either<Either<Lifetime, Type>, Either<GenericArgsBinding, TokenTree>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Right(Left(a)) => Self::ArgsBinding(a),
            Left(Left(a)) => Self::Lifetime(a),
            Left(Right(a)) => Self::Type(a),
            Right(Right(a)) => todo!(),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct GenericArgs(pub Vec<GenericArg>);
impl MappedParse for GenericArgs {
    type Source = (Lt, Interlace<GenericArg, Comma>, Option<Comma>, Gt);

    type Output = GenericArgs;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1 .0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub enum PathExpression {
    PathInExpression(PathInExpression),
    QualifiedPathInExpression(QualifiedPathInExpression),
}
impl MappedParse for PathExpression {
    type Source = Either<PathInExpression, QualifiedPathInExpression>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(a) => Self::PathInExpression(a),
            Right(a) => Self::QualifiedPathInExpression(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;
    use crate::*;

    #[test]
    fn it_matches_simple_path() {
        parse_terminal::<SimplePath>(quote!(usize::hello)).unwrap();
    }
    #[test]
    fn it_matches_expr_path() {
        println!(
            "{:#?}",
            parse_terminal::<PathInExpression>(quote!(usize::hello::<Hello, World>)).unwrap()
        );
    }
}
