use std::fmt::Debug;

use super::*;
use crate::*;

pub enum Struct<T: Parsable, Ty: Parsable> {
    Unit(UnitStruct),
    Block(BlockStruct<T, Ty>),
    Tuple(TupleStruct<T, Ty>),
}
impl<T: Parsable, Ty: Parsable> MappedParse for Struct<T, Ty> {
    type Source = Sum3<BlockStruct<T, Ty>, TupleStruct<T, Ty>, UnitStruct>;

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
impl<T: Parsable, Ty: Parsable> Debug for Struct<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(arg0) => f.debug_tuple("Unit").field(arg0).finish(),
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Tuple(arg0) => f.debug_tuple("Tuple").field(arg0).finish(),
        }
    }
}

pub enum StructStruct<T: Parsable, Ty: Parsable> {
    Unit(UnitStruct),
    Block(BlockStruct<T, Ty>),
}
impl<T: Parsable, Ty: Parsable> MappedParse for StructStruct<T, Ty> {
    type Source = Sum2<BlockStruct<T, Ty>, UnitStruct>;

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
impl<T: Parsable, Ty: Parsable> Debug for StructStruct<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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

pub struct BlockStruct<T: Parsable, Ty: Parsable> {
    pub id: Ident,
    pub params: Option<GenericParams<T, Ty>>,
    pub fields: StructFields<T, Ty>,
    pub where_clause: Option<WhereClause<T, Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for BlockStruct<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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
impl<T: Parsable, Ty: Parsable> MappedParse for BlockStruct<T, Ty> {
    type Source = (
        KwStruct,
        Identifier,
        Option<GenericParams<T, Ty>>,
        Option<WhereClause<T, Ty>>,
        Brace<StructFields<T, Ty>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            id: src.1,
            params: src.2,
            fields: src.4 .0,
            where_clause: src.3,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct TupleStruct<T: Parsable, Ty: Parsable> {
    pub id: Ident,
    pub params: Option<GenericParams<T, Ty>>,
    pub fields: TupleFields<T, Ty>,
    pub where_clause: Option<WhereClause<T, Ty>>,
}
impl<T: Parsable, Ty: Parsable> Debug for TupleStruct<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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
impl<T: Parsable, Ty: Parsable> MappedParse for TupleStruct<T, Ty> {
    type Source = (
        KwStruct,
        Identifier,
        Option<GenericParams<T, Ty>>,
        Paren<TupleFields<T, Ty>>,
        Option<WhereClause<T, Ty>>,
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
            fields: src.3 .0,
            where_clause: src.4,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

// These are marginally incorrect but in practice it can simply be fixed with a min-length
pub type TupleFields<T, Ty> = InterlaceTrail<TupleField<T, Ty>, Comma>;
pub type StructFields<T, Ty> = InterlaceTrail<StructField<T, Ty>, Comma>;

pub struct StructField<T: Parsable, Ty: Parsable> {
    pub attr: Attrs<T>,
    pub vis: Option<Visibility>,
    pub id: Ident,
    pub ty: SmOut<Ty>,
}
impl<T: Parsable, Ty: Parsable> MappedParse for StructField<T, Ty> {
    type Source = (Attrs<T>, Option<Visibility>, Identifier, Colon, Ty);

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
impl<T: Parsable, Ty: Parsable> Debug for StructField<T, Ty>
where
    SmOut<Ty>: Debug,
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

pub struct TupleField<T: Parsable, Ty: Parsable> {
    pub attr: Attrs<T>,
    pub vis: Option<Visibility>,
    pub ty: SmOut<Ty>,
}
impl<T: Parsable, Ty: Parsable> MappedParse for TupleField<T, Ty> {
    type Source = (Attrs<T>, Option<Visibility>, Ty);

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

impl<T: Parsable, Ty: Parsable> Debug for TupleField<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
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

    insta_match_test!(it_matches_unit, Struct<Infallible,std::convert::Infallible>: struct Unit;);
    insta_match_test!(it_matches_tuple, Struct<Infallible,Ident>: struct Point<T> (T,T) where T: std::ops::Add<Other = T>;);
    insta_match_test!(it_matches_struct, Struct<Infallible,Ident>: struct Point<T> where T: Hi { pub v0: T, pub v1: T });
}
