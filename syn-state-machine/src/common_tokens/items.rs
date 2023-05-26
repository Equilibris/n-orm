use super::*;
use crate::*;

pub enum Item<T: Parsable, Ty: Parsable, Content: Parsable> {
    Vis(VisItem<T, Ty, Content>),
    Macro(MacroItem),
}

pub enum VisItem<T: Parsable, Ty: Parsable, Content: Parsable> {
    Module(Option<Visibility>, Module<Content, Ty>),
    ExternCrate(Option<Visibility>, ExternCrate),
    UseDeclaration(Option<Visibility>, UseDeclaration),
    Function(Option<Visibility>, Function<T, Ty>),
    TypeAlias(Option<Visibility>, TypeAlias<T, Ty>),
    Struct(Option<Visibility>, Struct<T, Ty>),
    Enumeration(Option<Visibility>, Enumeration<T, Ty>),
    Union(Option<Visibility>, Union<T, Ty>),
    ConstantItem(Option<Visibility>, ConstantItem<Ty>),
    StaticItem(Option<Visibility>, StaticItem<Ty>),
    Trait(Option<Visibility>, Trait<T, Ty>),
    Implementation(Option<Visibility>, Implementation<T, Ty>),
    ExternBlock(Option<Visibility>, ExternBlock<T, Ty>),
}
impl<T: Parsable, Ty: Parsable, Content: Parsable> MappedParse for VisItem<T, Ty, Content> {
    type Source = (
        Option<Visibility>,
        ESum13<
            Module<Content, Ty>,
            ExternCrate,
            UseDeclaration,
            Function<T, Ty>,
            TypeAlias<T, Ty>,
            Struct<T, Ty>,
            Enumeration<T, Ty>,
            Union<T, Ty>,
            ConstantItem<Ty>,
            StaticItem<Ty>,
            Trait<T, Ty>,
            Implementation<T, Ty>,
            ExternBlock<T, Ty>,
            EBox<Sum0Err>,
        >,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Sum13::Val0(a) => Self::Module(src.0, a),
            Sum13::Val1(a) => Self::ExternCrate(src.0, a),
            Sum13::Val2(a) => Self::UseDeclaration(src.0, a),
            Sum13::Val3(a) => Self::Function(src.0, a),
            Sum13::Val4(a) => Self::TypeAlias(src.0, a),
            Sum13::Val5(a) => Self::Struct(src.0, a),
            Sum13::Val6(a) => Self::Enumeration(src.0, a),
            Sum13::Val7(a) => Self::Union(src.0, a),
            Sum13::Val8(a) => Self::ConstantItem(src.0, a),
            Sum13::Val9(a) => Self::StaticItem(src.0, a),
            Sum13::Val10(a) => Self::Trait(src.0, a),
            Sum13::Val11(a) => Self::Implementation(src.0, a),
            Sum13::Val12(a) => Self::ExternBlock(src.0, a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub enum MacroItem {
    InvocationSemi(MacroInvocationSemi),
    RulesDefinition(MacroRulesDefinition),
}
impl MappedParse for MacroItem {
    type Source = Sum2<MacroInvocationSemi, MacroRulesDefinition>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::InvocationSemi(a),
            Sum2::Val1(a) => Self::RulesDefinition(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
