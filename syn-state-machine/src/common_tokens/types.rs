use super::*;
use crate::*;
use std::fmt::Debug;

pub enum Type<T: Parsable> {
    NoBounds(TypeNoBounds<T>),
    ImplTrait(ImplTraitType<T>),
    TraitObject(TraitObjectType<T>),
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
    type Source = Sum3<TypeNoBounds<T>, MBox<ImplTraitType<T>>, MBox<TraitObjectType<T>>>;

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
pub struct TypeNoBoundsError<T: Parsable> {
    pub parenthesized: Box<SmErr<ParenthesizedType<T>>>,
    pub impl_trait_one_bound: SmErr<ImplTraitTypeOneBound<T>>,
    pub trait_object_one_bound: SmErr<TraitObjectTypeOneBound<T>>,
    pub type_path: SmErr<TypePath<T>>,
    pub tuple: SmErr<TupleType<T>>,
    pub never: SmErr<NeverType>,
    pub raw_pointer: Box<SmErr<RawPointerType<T>>>,
    pub reference: Box<SmErr<ReferenceType<T>>>,
    pub array: Box<SmErr<ArrayType<T>>>,
    pub slice: Box<SmErr<SliceType<T>>>,
    pub inferred: SmErr<InferredType>,
    pub qualified_path: Box<SmErr<QualifiedPathInType<T>>>,
    pub bare_function: Box<SmErr<BareFunctionType<Tokens, PBox<Type<T>>>>>,
    pub macro_invocation: SmErr<MacroInvocation>,
}
impl<T: Parsable> Debug for TypeNoBoundsError<T> {
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

pub enum TypeNoBounds<T: Parsable> {
    Parenthesized(Box<ParenthesizedType<T>>),
    ImplTraitOneBound(ImplTraitTypeOneBound<T>),
    TraitObjectOneBound(TraitObjectTypeOneBound<T>),
    TypePath(TypePath<T>),
    Tuple(TupleType<T>),
    Never(NeverType),
    RawPointer(Box<RawPointerType<T>>),
    Reference(Box<ReferenceType<T>>),
    Array(Box<ArrayType<T>>),
    Slice(Box<SliceType<T>>),
    Inferred(InferredType),
    QualifiedPath(Box<QualifiedPathInType<T>>),
    BareFunction(Box<BareFunctionType<T, PBox<Type<T>>>>),
    MacroInvocation(MacroInvocation),
}
impl<T: Parsable> Debug for TypeNoBounds<T>
where
    SmOut<T>: Debug,
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
impl<T: Parsable> MappedParse for TypeNoBounds<T> {
    type Source = PBox<
        Sum14<
            ParenthesizedType<T>,
            ImplTraitTypeOneBound<T>,
            TraitObjectTypeOneBound<T>,
            TypePath<T>,
            TupleType<T>,
            NeverType,
            RawPointerType<T>,
            ReferenceType<T>,
            ArrayType<T>,
            SliceType<T>,
            InferredType,
            QualifiedPathInType<T>,
            BareFunctionType<T, PBox<Type<T>>>,
            MacroInvocation,
        >,
    >;

    type Output = Self;
    type Error = TypeNoBoundsError<T>;

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

pub struct ParenthesizedType<T: Parsable>(pub Type<T>);
impl<T: Parsable> Debug for ParenthesizedType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ParenthesizedType").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for ParenthesizedType<T> {
    type Source = Paren<Type<T>>;

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

pub struct SliceType<T: Parsable>(pub Type<T>);
impl<T: Parsable> Debug for SliceType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SliceType").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for SliceType<T> {
    type Source = Bracket<Type<T>>;

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

pub struct ArrayType<T: Parsable> {
    pub ty: Type<T>,
    pub expr: Expression, // TODO
}
impl<T: Parsable> Debug for ArrayType<T>
where
    SmOut<T>: Debug,
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
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        todo!()
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct ReferenceType<T: Parsable> {
    pub is_mut: bool,
    pub inner: TypeNoBounds<T>,
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
    type Source = (Amp, Option<KwMut>, TypeNoBounds<T>);

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

pub enum RawPointerType<T: Parsable> {
    Simple(TypeNoBounds<T>),
    Const(TypeNoBounds<T>),
    Mut(TypeNoBounds<T>),
}
impl<T: Parsable> Debug for RawPointerType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(arg0) => f.debug_tuple("Simple").field(arg0).finish(),
            Self::Const(arg0) => f.debug_tuple("Const").field(arg0).finish(),
            Self::Mut(arg0) => f.debug_tuple("Mut").field(arg0).finish(),
        }
    }
}
impl<T: Parsable> MappedParse for RawPointerType<T> {
    type Source = (Star, Option<Sum2<KwMut, KwConst>>, TypeNoBounds<T>);

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

pub struct TupleType<T: Parsable>(pub Vec<Type<T>>);
impl<T: Parsable> Debug for TupleType<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TupleType").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for TupleType<T> {
    type Source =
        Paren<Sum2<MinLength<InterlaceTrail<Type<T>, Comma>, 2>, Option<(Type<T>, Comma)>>>;

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

    // #[test]
    // pub fn sm_size_prune() {
    //     panic!(
    //         "{}",
    //         std::mem::size_of::<<Type<Infallible> as Parsable>::StateMachine>()
    //     );
    // }

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

    insta_match_test!(it_matches_paren_type, ParenthesizedType<Infallible>: u16);

    insta_match_test!(it_matches_tuple_type_unit, TupleType<Infallible>: ());
    insta_match_test!(it_matches_tuple_type_single, TupleType<Infallible>: (Hello,));
    insta_match_test!(it_matches_tuple_type_duo, TupleType<Infallible>: (Hello, World));
    insta_match_test!(it_matches_tuple_type_duo_trail, TupleType<Infallible>: (Hello, World,));

    insta_match_test!(it_matches_raw_pointer_type, RawPointerType<Infallible>: *hello);
    insta_match_test!(it_matches_raw_pointer_type_mut, RawPointerType<Infallible>: *mut hello);

    insta_match_test!(it_matches_reference, ReferenceType<Infallible>: &hello);
    insta_match_test!(it_matches_reference_mut, ReferenceType<Infallible>: &mut hello);

    #[cfg(disable)]
    insta_match_test!(it_matches_array_type, ArrayType<Infallible>: [i64; 10]);

    insta_match_test!(it_matches_slice_type, SliceType<Infallible>: [i64]);

    insta_match_test!(it_matches_never, NeverType: !);
    insta_match_test!(it_matches_inferred, InferredType: _);
}
