use super::*;
use crate::*;
use std::fmt::Debug;

pub struct Enumeration<T: Parsable = Tokens> {
    pub id: Ident,
    pub generic_params: Option<GenericParams>,
    pub where_clause: Option<WhereClause>,

    pub items: EnumItems<T>,
}

impl<T: Parsable> Debug for Enumeration<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enumeration")
            .field("id", &self.id)
            .field("generic_params", &self.generic_params)
            .field("where_clause", &self.where_clause)
            .field("items", &self.items)
            .finish()
    }
}
impl<T: Parsable> MappedParse for Enumeration<T> {
    type Source = (
        KwEnum,
        Identifier,
        Option<GenericParams>,
        Option<WhereClause>,
        Brace<EnumItems<T>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            generic_params: src.2,
            where_clause: src.3,
            items: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type EnumItems<T = Tokens> = InterlaceTrail<EnumItem<T>, Comma>;

pub enum EnumItem<T: Parsable = Tokens> {
    Unit {
        id: Ident,

        attrs: Attrs<T>,
        vis: Option<Visibility>,
        desc: Option<EnumItemDiscriminant>,
    },
    Tuple {
        id: Ident,

        attrs: Attrs<T>,
        tuple: TupleFields<T>,
        vis: Option<Visibility>,
        desc: Option<EnumItemDiscriminant>,
    },
    Block {
        id: Ident,

        attrs: Attrs<T>,
        block: StructFields<T>,
        vis: Option<Visibility>,
        desc: Option<EnumItemDiscriminant>,
    },
}
impl<T: Parsable> Debug for EnumItem<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit {
                id,
                attrs,
                vis,
                desc,
            } => f
                .debug_struct("Unit")
                .field("id", id)
                .field("attrs", attrs)
                .field("vis", vis)
                .field("desc", desc)
                .finish(),
            Self::Tuple {
                id,
                attrs,
                tuple,
                vis,
                desc,
            } => f
                .debug_struct("Tuple")
                .field("id", id)
                .field("attrs", attrs)
                .field("tuple", tuple)
                .field("vis", vis)
                .field("desc", desc)
                .finish(),
            Self::Block {
                id,
                attrs,
                block,
                vis,
                desc,
            } => f
                .debug_struct("Block")
                .field("id", id)
                .field("attrs", attrs)
                .field("block", block)
                .field("vis", vis)
                .field("desc", desc)
                .finish(),
        }
    }
}
impl<T: Parsable> MappedParse for EnumItem<T> {
    type Source = (
        Attrs<T>,
        Option<Visibility>,
        Identifier,
        Option<Sum2<EnumItemStruct<T>, EnumItemTuple<T>>>,
        Option<EnumItemDiscriminant>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.3 {
            Some(Sum2::Val0(a)) => Self::Block {
                id: src.2,

                attrs: src.0,
                vis: src.1,
                desc: src.4,

                block: a.0,
            },
            Some(Sum2::Val1(a)) => Self::Tuple {
                id: src.2,

                attrs: src.0,
                vis: src.1,
                desc: src.4,

                tuple: a.0,
            },
            None => Self::Unit {
                id: src.2,

                attrs: src.0,
                vis: src.1,
                desc: src.4,
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct EnumItemStruct<T: Parsable = Tokens>(pub StructFields<T>);
impl<T: Parsable> Debug for EnumItemStruct<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EnumItemStruct").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for EnumItemStruct<T> {
    type Source = Brace<StructFields<T>>;

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

pub struct EnumItemTuple<T: Parsable = Tokens>(pub TupleFields<T>);
impl<T: Parsable> Debug for EnumItemTuple<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EnumItemStruct").field(&self.0).finish()
    }
}
impl<T: Parsable> MappedParse for EnumItemTuple<T> {
    type Source = Paren<TupleFields<T>>;

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
pub struct EnumItemDiscriminant(pub Expression);
impl MappedParse for EnumItemDiscriminant {
    type Source = (Eq, Expression);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.1))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(it_matches_enum_item_unit, EnumItem: Block);
    insta_match_test!(it_matches_enum_item_struct, EnumItem: Block { hello : World });
    insta_match_test!(it_matches_enum_item_tuple, EnumItem: Block(World));

    insta_match_test!(
        it_matches_enum, Enumeration :
        enum HelloWorld <F,T> where F: Into<T> {
            Unit,
            From(F),
            To { result: T },
        }
    );
}
