use super::*;
use crate::*;

use Sum2::*;

#[derive(Debug)]
pub enum Type {
    NoBounds(TypeNoBounds),
    ImplTrait(ImplTraitType),
    TraitObject(TraitObjectType),
}
impl MappedParse for Type {
    type Source = Sum2<TypeNoBounds, Sum2<ImplTraitType, TraitObjectType>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Val0(a) => Self::NoBounds(a),
            Val1(Val0(a)) => Self::ImplTrait(a),
            Val1(Val1(a)) => Self::TraitObject(a),
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
        Sum14<
            ParenthesizedType,
            ImplTraitTypeOneBound,
            TraitObjectTypeOneBound,
            TypePath,
            TupleType,
            NeverType,
            RawPointerType,
            ReferenceType,
            ArrayType,
            SliceType,
            InferredType,
            QualifiedPathInType,
            BareFunctionType,
            MacroInvocation,
        >,
    >;

    type Output = Self;
    type Error = TypeNoBoundsError;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match *src {
            Sum14::Val0(a) => Self::Parenthesized(Box::new(a)),
            Sum14::Val1(a) => Self::ImplTraitOneBound(a),
            Sum14::Val2(a) => Self::TraitObjectOneBound(a),
            Sum14::Val3(a) => Self::TypePath(a),
            Sum14::Val4(a) => Self::Tuple(a),
            Sum14::Val5(a) => Self::Never(a),
            Sum14::Val6(a) => Self::RawPointer(Box::new(a)),
            Sum14::Val7(a) => Self::Reference(Box::new(a)),
            Sum14::Val8(a) => Self::Array(Box::new(a)),
            Sum14::Val9(a) => Self::Slice(Box::new(a)),
            Sum14::Val10(a) => Self::Inferred(a),
            Sum14::Val11(a) => Self::QualifiedPath(Box::new(a)),
            Sum14::Val12(a) => Self::BareFunction(Box::new(a)),
            Sum14::Val13(a) => Self::MacroInvocation(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        let Sum14Err {
            v0: parenthesized,
            v1: impl_trait_one_bound,
            v2: trait_object_one_bound,
            v3: type_path,
            v4: tuple,
            v5: never,
            v6: raw_pointer,
            v7: reference,
            v8: array,
            v9: slice,
            v10: inferred,
            v11: qualified_path,
            v12: bare_function,
            v13: macro_invocation,
        } = *src;

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
    type Source = Bracket<Type>;

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
    type Source = Paren<Sum2<MinLength<InterlaceTrail<Type, Comma>, 2>, Option<(Type, Comma)>>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(match src {
            Val0(a) => a.0,
            Val1(Some(a)) => vec![a.0],
            Val1(None) => Vec::new(),
        }))
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

    // The primary issue for type matching is stack-overflows. This is me trying to avoid this.
    #[test]
    fn it_does_not_overrun_stack_for_reasonable_types() {
        for i in 1..=5 {
            let thread = std::thread::spawn(move || {
                let mut src = quote!(i8);
                for _ in 0..i {
                    src = quote!(Box < #src >);
                }

                parse_terminal::<Type>(src).is_ok()
            });
            let thread = thread.join();

            assert!(thread.is_ok());
            assert!(thread.unwrap());
        }
    }

    insta_match_test!(it_matches_type_impl, Type: impl Hi);
    insta_match_test!(it_matches_type_dyn, Type: dyn Hi);
    insta_match_test!(it_matches_type_direct, Type: u16);
    insta_match_test!(it_matches_type_path, Type: hello::World);

    insta_match_test!(it_matches_paren_type, ParenthesizedType: u16);

    insta_match_test!(it_matches_tuple_type_unit, TupleType: ());
    insta_match_test!(it_matches_tuple_type_single, TupleType: (Hello,));
    insta_match_test!(it_matches_tuple_type_duo, TupleType: (Hello, World));
    insta_match_test!(it_matches_tuple_type_duo_trail, TupleType: (Hello, World,));

    insta_match_test!(it_matches_raw_pointer_type, RawPointerType: *hello);
    insta_match_test!(it_matches_raw_pointer_type_mut, RawPointerType: *mut hello);

    insta_match_test!(it_matches_reference, ReferenceType: &hello);
    insta_match_test!(it_matches_reference_mut, ReferenceType: &mut hello);

    #[cfg(disable)]
    insta_match_test!(it_matches_array_type, ArrayType: [i64; 10]);

    insta_match_test!(it_matches_slice_type, SliceType: [i64]);

    insta_match_test!(it_matches_never, NeverType: !);
    insta_match_test!(it_matches_inferred, InferredType: _);
}
