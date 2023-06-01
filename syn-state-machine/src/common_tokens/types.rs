use super::*;
use crate::*;
use std::fmt::Debug;

pub enum Type<T: Parsable> {
    NoBounds(TypeNoBounds<T, Self>),
    ImplTrait(Box<ImplTraitType<T, Self>>),
    TraitObject(Box<TraitObjectType<T, Self>>),
}
impl<T: Parsable> Debug for Type<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBounds(arg0) => f.debug_tuple("NoBounds").field(arg0).finish(),
            Self::ImplTrait(arg0) => f.debug_tuple("ImplTrait").field(arg0).finish(),
            Self::TraitObject(arg0) => f.debug_tuple("TraitObject").field(arg0).finish(),
        }
    }
}
impl<T: Parsable> MappedParse for Type<T> {
    type Source =
        Sum3<TypeNoBounds<T, Self>, PBox<ImplTraitType<T, Self>>, PBox<TraitObjectType<T, Self>>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum3::Val0(a) => Self::NoBounds(a),
            Sum3::Val1(a) => Self::ImplTrait(a),
            Sum3::Val2(a) => Self::TraitObject(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(thiserror::Error)]
#[error("Expected type")]
pub struct TypeNoBoundsError<T: Parsable, Ty: Parsable> {
    pub parenthesized: Box<SmErr<ParenthesizedType<Ty>>>,
    pub impl_trait_one_bound: SmErr<ImplTraitTypeOneBound<T, Ty>>,
    pub trait_object_one_bound: SmErr<TraitObjectTypeOneBound<T, Ty>>,
    pub type_path: SmErr<TypePath<Ty>>,
    pub tuple: SmErr<TupleType<T>>,
    pub never: SmErr<NeverType>,
    pub raw_pointer: Box<SmErr<RawPointerType<TypeNoBounds<T, Ty>>>>,
    pub reference: Box<SmErr<ReferenceType<T>>>,
    pub array: Box<SmErr<ArrayType<T>>>,
    pub slice: Box<SmErr<SliceType<Ty>>>,
    pub inferred: SmErr<InferredType>,
    pub qualified_path: Box<SmErr<QualifiedPathInType<Ty>>>,
    pub bare_function: Box<SmErr<BareFunctionType<Tokens, PBox<Ty>>>>,
    pub macro_invocation: SmErr<MacroInvocation>,
}
impl<T: Parsable, Ty: Parsable> Debug for TypeNoBoundsError<T, Ty> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeNoBoundsError")
            .field("parenthesized", &self.parenthesized)
            .field("impl_trait_one_bound", &self.impl_trait_one_bound)
            .field("trait_object_one_bound", &self.trait_object_one_bound)
            .field("type_path", &self.type_path)
            .field("tuple", &self.tuple)
            .field("never", &self.never)
            .field("raw_pointer", &self.raw_pointer)
            .field("reference", &self.reference)
            .field("array", &self.array)
            .field("slice", &self.slice)
            .field("inferred", &self.inferred)
            .field("qualified_path", &self.qualified_path)
            .field("bare_function", &self.bare_function)
            .field("macro_invocation", &self.macro_invocation)
            .finish()
    }
}

pub enum TypeNoBounds<T: Parsable, Ty: Parsable> {
    Parenthesized(Box<ParenthesizedType<Ty>>),
    ImplTraitOneBound(ImplTraitTypeOneBound<T, Ty>),
    TraitObjectOneBound(TraitObjectTypeOneBound<T, Ty>),
    TypePath(TypePath<Ty>),
    Tuple(TupleType<Ty>),
    Never(NeverType),
    RawPointer(Box<RawPointerType<Self>>),
    Reference(Box<ReferenceType<T>>),
    Array(Box<ArrayType<Ty>>),
    Slice(Box<SliceType<Ty>>),
    Inferred(InferredType),
    QualifiedPath(Box<QualifiedPathInType<Ty>>),
    BareFunction(Box<BareFunctionType<T, PBox<Self>>>),
    MacroInvocation(MacroInvocation),
}
impl<T: Parsable, Ty: Parsable> Debug for TypeNoBounds<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parenthesized(arg0) => f.debug_tuple("Parenthesized").field(arg0).finish(),
            Self::ImplTraitOneBound(arg0) => {
                f.debug_tuple("ImplTraitOneBound").field(arg0).finish()
            }
            Self::TraitObjectOneBound(arg0) => {
                f.debug_tuple("TraitObjectOneBound").field(arg0).finish()
            }
            Self::TypePath(arg0) => f.debug_tuple("TypePath").field(arg0).finish(),
            Self::Tuple(arg0) => f.debug_tuple("Tuple").field(arg0).finish(),
            Self::Never(arg0) => f.debug_tuple("Never").field(arg0).finish(),
            Self::RawPointer(arg0) => f.debug_tuple("RawPointer").field(arg0).finish(),
            Self::Reference(arg0) => f.debug_tuple("Reference").field(arg0).finish(),
            Self::Array(arg0) => f.debug_tuple("Array").field(arg0).finish(),
            Self::Slice(arg0) => f.debug_tuple("Slice").field(arg0).finish(),
            Self::Inferred(arg0) => f.debug_tuple("Inferred").field(arg0).finish(),
            Self::QualifiedPath(arg0) => f.debug_tuple("QualifiedPath").field(arg0).finish(),
            Self::BareFunction(arg0) => f.debug_tuple("BareFunction").field(arg0).finish(),
            Self::MacroInvocation(arg0) => f.debug_tuple("MacroInvocation").field(arg0).finish(),
        }
    }
}
impl<T: Parsable, Ty: Parsable> MappedParse for TypeNoBounds<T, Ty> {
    type Source = PBox<
        Sum14<
            ParenthesizedType<Ty>,
            ImplTraitTypeOneBound<T, Ty>,
            TraitObjectTypeOneBound<T, Ty>,
            TypePath<Ty>,
            TupleType<Ty>,
            NeverType,
            RawPointerType<Self>,
            ReferenceType<T>,
            ArrayType<Ty>,
            SliceType<Ty>,
            InferredType,
            QualifiedPathInType<Ty>,
            BareFunctionType<T, PBox<Self>>,
            MacroInvocation,
        >,
    >;

    type Output = Self;
    type Error = TypeNoBoundsError<T, Ty>;

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
            a: parenthesized,
            b: impl_trait_one_bound,
            c: trait_object_one_bound,
            d: type_path,
            e: tuple,
            f: never,
            g: raw_pointer,
            h: reference,
            i: array,
            j: slice,
            k: inferred,
            l: qualified_path,
            m: bare_function,
            n: macro_invocation,
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

pub struct ParenthesizedType<Ty: Parsable>(pub SmOut<Ty>);
impl<Ty: Parsable> Debug for ParenthesizedType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ParenthesizedType").field(&self.0).finish()
    }
}
impl<Ty: Parsable> MappedParse for ParenthesizedType<Ty> {
    type Source = Paren<Ty>;

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

pub type InferredType = Underscore;

pub struct SliceType<Ty: Parsable>(pub SmOut<Ty>);
impl<Ty: Parsable> Debug for SliceType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SliceType").field(&self.0).finish()
    }
}
impl<Ty: Parsable> MappedParse for SliceType<Ty> {
    type Source = Bracket<Ty>;

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

pub struct ArrayType<Ty: Parsable> {
    pub ty: SmOut<Ty>,
    pub expr: Expression, // TODO
}
impl<Ty: Parsable> Debug for ArrayType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayType")
            .field("ty", &self.ty)
            .field("expr", &self.expr)
            .finish()
    }
}
impl<T: Parsable> MappedParse for ArrayType<T> {
    type Source = std::convert::Infallible;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        _: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct ReferenceType<T: Parsable> {
    pub is_mut: bool,
    pub inner: TypeNoBounds<T, Type<T>>,
}
impl<T: Parsable> Debug for ReferenceType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReferenceType")
            .field("is_mut", &self.is_mut)
            .field("inner", &self.inner)
            .finish()
    }
}
impl<T: Parsable> MappedParse for ReferenceType<T> {
    type Source = (Amp, Option<KwMut>, TypeNoBounds<T, Type<T>>);

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

pub enum RawPointerType<TyNB: Parsable> {
    Simple(SmOut<TyNB>),
    Const(SmOut<TyNB>),
    Mut(SmOut<TyNB>),
}
impl<TyNB: Parsable> Debug for RawPointerType<TyNB>
where
    SmOut<TyNB>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(arg0) => f.debug_tuple("Simple").field(arg0).finish(),
            Self::Const(arg0) => f.debug_tuple("Const").field(arg0).finish(),
            Self::Mut(arg0) => f.debug_tuple("Mut").field(arg0).finish(),
        }
    }
}
impl<TyNB: Parsable> MappedParse for RawPointerType<TyNB> {
    type Source = (Star, Option<Sum2<KwMut, KwConst>>, TyNB);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Some(Sum2::Val0(_)) => Self::Mut(src.2),
            Some(Sum2::Val1(_)) => Self::Const(src.2),
            None => Self::Simple(src.2),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type NeverType = Exclamation;

pub struct TupleType<Ty: Parsable>(pub Vec<SmOut<Ty>>);
impl<Ty: Parsable> Debug for TupleType<Ty>
where
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TupleType").field(&self.0).finish()
    }
}
impl<Ty: Parsable> MappedParse for TupleType<Ty> {
    type Source = Paren<Sum2<MinLength<InterlaceTrail<Ty, Comma>, 2>, Option<(Ty, Comma)>>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(match src.0 {
            Sum2::Val0(a) => a.0,
            Sum2::Val1(Some(a)) => vec![a.0],
            Sum2::Val1(None) => Vec::new(),
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

    // The primary issue for type matching is stack-overflows. This is me trying to avoid this.
    #[test]
    fn it_does_not_overrun_stack_for_reasonable_types() {
        for i in 1..=5 {
            let thread = std::thread::spawn(move || {
                let mut src = quote!(i8);
                for _ in 0..i {
                    src = quote!(Box < #src >);
                }

                parse_terminal::<Type<Infallible>>(src).is_ok()
            });
            let thread = thread.join();

            assert!(thread.is_ok());
            assert!(thread.unwrap());
        }
    }

    insta_match_test!(it_matches_type_impl, Type<Infallible>: impl Hi);
    insta_match_test!(it_matches_type_dyn, Type<Infallible>: dyn Hi);
    insta_match_test!(it_matches_type_direct, Type<Infallible>: u16);
    insta_match_test!(it_matches_type_path, Type<Infallible>: hello::World);

    insta_match_test!(it_matches_paren_type, ParenthesizedType<Ident>: (u16));

    insta_match_test!(it_matches_tuple_type_unit, TupleType<Infallible>: ());
    insta_match_test!(it_matches_tuple_type_single, TupleType<Ident>: (Hello,));
    insta_match_test!(it_matches_tuple_type_duo, TupleType<Ident>: (Hello, World));
    insta_match_test!(it_matches_tuple_type_duo_trail, TupleType<Ident>: (Hello, World,));

    insta_match_test!(it_matches_raw_pointer_type, RawPointerType<Ident>: *hello);
    insta_match_test!(it_matches_raw_pointer_type_mut, RawPointerType<Ident>: *mut hello);

    insta_match_test!(it_matches_reference, ReferenceType<Infallible>: &hello);
    insta_match_test!(it_matches_reference_mut, ReferenceType<Infallible>: &mut hello);

    #[cfg(disable)]
    insta_match_test!(it_matches_array_type, ArrayType<Infallible>: [i64; 10]);

    insta_match_test!(it_matches_slice_type, SliceType<Ident>: [i64]);

    insta_match_test!(it_matches_never, NeverType: !);
    insta_match_test!(it_matches_inferred, InferredType: _);
}
