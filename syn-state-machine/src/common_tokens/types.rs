use super::*;
use crate::*;

use Either::*;

#[derive(Debug)]
pub enum Type {
    NoBounds(TypeNoBounds),
    ImplTrait(ImplTraitType),
    TraitObject(TraitObjectType),
}
impl MappedParse for Type {
    type Source = Either<TypeNoBounds, Either<ImplTraitType, TraitObjectType>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Left(a) => Self::NoBounds(a),
            Right(Left(a)) => Self::ImplTrait(a),
            Right(Right(a)) => Self::TraitObject(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Expected type")]
pub struct TypeNoBoundsError {
    parenthesized: Box<SmErr<ParenthesizedType>>,
    impl_trait_one_bound: SmErr<ImplTraitTypeOneBound>,
    trait_object_one_bound: SmErr<TraitObjectTypeOneBound>,
    type_path: SmErr<TypePath>,
    tuple: SmErr<TupleType>,
    never: SmErr<NeverType>,
    raw_pointer: Box<SmErr<RawPointerType>>,
    reference: Box<SmErr<ReferenceType>>,
    array: Box<SmErr<ArrayType>>,
    slice: Box<SmErr<SliceType>>,
    inferred: SmErr<InferredType>,
    qualified_path: Box<SmErr<QualifiedPathInType>>,
    bare_function: Box<SmErr<BareFunctionType>>,
    macro_invocation: SmErr<MacroInvocation>,
}

#[derive(Debug)]
pub enum TypeNoBounds {
    Parenthesized(Box<ParenthesizedType>),
    ImplTraitOneBound(ImplTraitTypeOneBound),
    TraitObjectOneBound(TraitObjectTypeOneBound),
    TypePath(TypePath),
    Tuple(TupleType),
    Never(NeverType),
    RawPointer(Box<RawPointerType>),
    Reference(Box<ReferenceType>),
    Array(Box<ArrayType>),
    Slice(Box<SliceType>),
    Inferred(InferredType),
    QualifiedPath(Box<QualifiedPathInType>),
    BareFunction(Box<BareFunctionType>),
    MacroInvocation(MacroInvocation),
}
impl MappedParse for TypeNoBounds {
    type Source = PBox<
        Either<
            Either<
                Either<
                    Either<ParenthesizedType, ImplTraitTypeOneBound>,
                    Either<TraitObjectTypeOneBound, TypePath>,
                >,
                Either<Either<TupleType, NeverType>, Either<RawPointerType, ReferenceType>>,
            >,
            Either<
                Either<Either<ArrayType, SliceType>, Either<InferredType, QualifiedPathInType>>,
                Either<BareFunctionType, MacroInvocation>,
            >,
        >,
    >;

    type Output = Self;
    type Error = TypeNoBoundsError;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match *src {
            Left(Left(Left(Left(a)))) => Self::Parenthesized(Box::new(a)),
            Left(Left(Left(Right(a)))) => Self::ImplTraitOneBound(a),
            Left(Left(Right(Left(a)))) => Self::TraitObjectOneBound(a),
            Left(Left(Right(Right(a)))) => Self::TypePath(a),
            Left(Right(Left(Left(a)))) => Self::Tuple(a),
            Left(Right(Left(Right(a)))) => Self::Never(a),
            Left(Right(Right(Left(a)))) => Self::RawPointer(Box::new(a)),
            Left(Right(Right(Right(a)))) => Self::Reference(Box::new(a)),
            Right(Left(Left(Left(a)))) => Self::Array(Box::new(a)),
            Right(Left(Left(Right(a)))) => Self::Slice(Box::new(a)),
            Right(Left(Right(Left(a)))) => Self::Inferred(a),
            Right(Left(Right(Right(a)))) => Self::QualifiedPath(Box::new(a)),
            Right(Right(Left(a))) => Self::BareFunction(Box::new(a)),
            Right(Right(Right(a))) => Self::MacroInvocation(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        let src = *src;
        let EitherParsingError(
            EitherParsingError(
                EitherParsingError(
                    EitherParsingError(parenthesized, impl_trait_one_bound),
                    EitherParsingError(trait_object_one_bound, type_path),
                ),
                EitherParsingError(
                    EitherParsingError(tuple, never),
                    EitherParsingError(raw_pointer, reference),
                ),
            ),
            EitherParsingError(
                EitherParsingError(
                    EitherParsingError(array, slice),
                    EitherParsingError(inferred, qualified_path),
                ),
                EitherParsingError(bare_function, macro_invocation),
            ),
        ) = src;

        TypeNoBoundsError {
            parenthesized: Box::new(parenthesized),
            impl_trait_one_bound,
            trait_object_one_bound,
            type_path,
            tuple,
            never,
            raw_pointer: Box::new(raw_pointer),
            reference: Box::new(reference),
            array: Box::new(array),
            slice: Box::new(slice),
            inferred,
            qualified_path: Box::new(qualified_path),
            bare_function: Box::new(bare_function),
            macro_invocation,
        }
    }
}

#[derive(Debug)]
pub struct ParenthesizedType(pub Type);
impl MappedParse for ParenthesizedType {
    type Source = Paren<Type>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type InferredType = Underscore;

#[derive(Debug)]
pub struct SliceType(pub Type);
impl MappedParse for SliceType {
    type Source = Brace<Type>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct ArrayType {
    pub ty: Type,
    pub expr: Expression, // TODO
}
impl MappedParse for ArrayType {
    type Source = std::convert::Infallible;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct ReferenceType {
    pub is_mut: bool,
    pub inner: TypeNoBounds,
}
impl MappedParse for ReferenceType {
    type Source = (Amp, Option<KwMut>, TypeNoBounds);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            is_mut: src.1.is_some(),
            inner: src.2,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct RawPointerType {
    pub is_mut: bool,
    pub inner: TypeNoBounds,
}
impl MappedParse for RawPointerType {
    type Source = (Star, Option<KwMut>, TypeNoBounds);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            is_mut: src.1.is_some(),
            inner: src.2,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type NeverType = Exclamation;

#[derive(Debug)]
pub struct TupleType(pub Vec<Type>);
impl MappedParse for TupleType {
    type Source = Paren<Interlace<Type, Comma>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0))
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
    fn it_matches_paren_type() {
        println!("{:#?}", parse_terminal::<Type>(quote!(impl Hi)).unwrap());
        println!("{:#?}", parse_terminal::<Type>(quote!(dyn Hi)).unwrap());
        println!("{:#?}", parse_terminal::<Type>(quote!((u16))).unwrap());
        println!(
            "{:#?}",
            parse_terminal::<Type>(quote!(hello::World)).unwrap()
        );
    }
}
