use std::fmt::Debug;

use super::*;
use crate::*;

pub enum Struct<T: Parsable = Tokens> {
    Unit(UnitStruct),
    Block(BlockStruct<T>),
    Tuple(TupleStruct<T>),
}
impl<T: Parsable> MappedParse for Struct<T> {
    type Source = Sum3<BlockStruct<T>, TupleStruct<T>, UnitStruct>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum3::Val0(a) => Self::Block(a),
            Sum3::Val1(a) => Self::Tuple(a),
            Sum3::Val2(a) => Self::Unit(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable> Debug for Struct<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(arg0) => f.debug_tuple("Unit").field(arg0).finish(),
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Tuple(arg0) => f.debug_tuple("Tuple").field(arg0).finish(),
        }
    }
}

pub enum StructStruct<T: Parsable = Tokens> {
    Unit(UnitStruct),
    Block(BlockStruct<T>),
}
impl<T: Parsable> MappedParse for StructStruct<T> {
    type Source = Sum2<BlockStruct<T>, UnitStruct>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::Block(a),
            Sum2::Val1(a) => Self::Unit(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable> Debug for StructStruct<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(arg0) => f.debug_tuple("Unit").field(arg0).finish(),
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
        }
    }
}

#[derive(Debug)]
pub struct UnitStruct(pub Ident);
impl MappedParse for UnitStruct {
    type Source = (KwStruct, Identifier, Semi);

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

pub struct BlockStruct<T: Parsable = Tokens> {
    pub id: Ident,
    pub params: Option<GenericParams>,
    pub fields: StructFields<T>,
    pub where_clause: Option<WhereClause>,
}
impl<T: Parsable> Debug for BlockStruct<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockStruct")
            .field("id", &self.id)
            .field("params", &self.params)
            .field("fields", &self.fields)
            .field("where_clause", &self.where_clause)
            .finish()
    }
}
impl<T: Parsable> MappedParse for BlockStruct<T> {
    type Source = (
        KwStruct,
        Identifier,
        Option<GenericParams>,
        Option<WhereClause>,
        Brace<StructFields<T>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            params: src.2,
            fields: src.4,
            where_clause: src.3,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TupleStruct<T: Parsable = Tokens> {
    pub id: Ident,
    pub params: Option<GenericParams>,
    pub fields: TupleFields<T>,
    pub where_clause: Option<WhereClause>,
}
impl<T: Parsable> Debug for TupleStruct<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TupleStruct")
            .field("id", &self.id)
            .field("params", &self.params)
            .field("fields", &self.fields)
            .field("where_clause", &self.where_clause)
            .finish()
    }
}
impl<T: Parsable> MappedParse for TupleStruct<T> {
    type Source = (
        KwStruct,
        Identifier,
        Option<GenericParams>,
        Paren<TupleFields<T>>,
        Option<WhereClause>,
        Semi,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            params: src.2,
            fields: src.3,
            where_clause: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// These are marginally incorrect but in practice it can simply be fixed with a min-length
pub type TupleFields<T> = InterlaceTrail<TupleField<T>, Comma>;
pub type StructFields<T> = InterlaceTrail<StructField<T>, Comma>;

pub struct StructField<T: Parsable = Tokens> {
    pub attr: Attrs<T>,
    pub vis: Option<Visibility>,
    pub id: Ident,
    pub ty: Type,
}
impl<T: Parsable> MappedParse for StructField<T> {
    type Source = (Attrs<T>, Option<Visibility>, Identifier, Colon, Type);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            attr: src.0,
            vis: src.1,
            id: src.2,
            ty: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable> Debug for StructField<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructField")
            .field("attr", &self.attr)
            .field("vis", &self.vis)
            .field("id", &self.id)
            .field("ty", &self.ty)
            .finish()
    }
}

pub struct TupleField<T: Parsable = Tokens> {
    pub attr: Attrs<T>,
    pub vis: Option<Visibility>,
    pub ty: Type,
}
impl<T: Parsable> MappedParse for TupleField<T> {
    type Source = (Attrs<T>, Option<Visibility>, Type);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            attr: src.0,
            vis: src.1,
            ty: src.2,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

impl<T: Parsable> Debug for TupleField<T>
where
    SmOut<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TupleField")
            .field("attr", &self.attr)
            .field("vis", &self.vis)
            .field("ty", &self.ty)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(it_matches_unit, Struct: struct Unit;);
    insta_match_test!(it_matches_tuple, Struct: struct Point<T> (T,T) where T: std::ops::Add<Other = T>;);
    insta_match_test!(it_matches_struct, Struct: struct Point<T> where T: Hi { pub v0: T, pub v1: T });
}
