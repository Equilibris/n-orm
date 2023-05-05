use super::*;
use crate::*;

pub enum GenericParam {
    Lifetime(Ident, Option<SmOutput<LifetimeBounds>>),
    TypeParam(Ident, Option<SmOutput<TypeParamBounds>>),
    Const(Ident, KwType),
}

pub struct GenericParams {}

// TODO
impl MappedParse for GenericParams {
    type Source = Ident;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_matches_lifetime() {
        let v = parse_terminal::<LifetimeOrLable>(quote::quote!('a)).unwrap();

        assert_eq!(v.to_string().as_str(), "a");
    }
}
