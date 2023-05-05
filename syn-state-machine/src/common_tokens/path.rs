mod simple;
mod types {
    use super::super::*;
    use crate::*;

    #[derive(Debug)]
    pub enum TypeSegmentIdent {
        Id(Ident),
        DCrate,
    }

    impl MappedParse for TypeSegmentIdent {
        type Source = Either<
            FlatEither<
                FlatEither<
                    FlatEither<Identifier, KwSuper>,
                    FlatEither<KwLowerSelf, KwCrate, Ident>,
                >,
                KwUpperSelf,
            >,
            (FPunct<'$'>, FIdent<"crate">),
        >;

        type Output = Self;
        type Error = SmError<Self::Source>;

        fn map(
            src: SmOutput<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            Ok(match src {
                Either::Left(v) => Self::Id(v),
                Either::Right(_) => Self::DCrate,
            })
        }

        fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
            src
        }
    }

    pub struct TypeSegment {
        id: TypeSegmentIdent,
        generic: GenericArg,
    }

    pub enum GenericArg {
        Lifetime(Lifetime),
        Type(Type),
        Const(TokenTree), // TODO:
        ArgsBinding(Ident, Type),
    }

    impl MappedParse for GenericArg {
        type Source = Either<Either<Lifetime, Type>, Either<(Identifier, Eq, Type), TokenTree>>;

        type Output = Self;
        type Error = SmError<Self::Source>;

        fn map(
            src: SmOutput<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            todo!()
        }

        fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
            todo!()
        }
    }

    pub struct GenericArgs;

    impl MappedParse for GenericArgs {
        type Source = (Lt, Interlace<GenericArg, Comma>, Option<Comma>, Gt);

        type Output = Vec<GenericArg>;
        type Error = SmError<Self::Source>;

        fn map(
            src: SmOutput<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            todo!()
        }

        fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
            todo!()
        }
    }

    pub struct TypePath;

    // TODO
    impl MappedParse for TypePath {
        type Source = Punct;

        type Output = ();
        type Error = SmError<Self::Source>;

        fn map(
            src: SmOutput<Self::Source>,
        ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
            todo!()
        }

        fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
            todo!()
        }
    }
}
mod qualified {
    pub struct QualifiedPathInType;
}

pub use qualified::*;
pub use simple::*;
pub use types::*;

// impl MappedParse for TypePath {
//     type Source;

//     type Output;
//     type Error;

//     fn map(
//         src: SmOutput<Self::Source>,
//     ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
//         todo!()
//     }

//     fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
//         todo!()
//     }
// }
