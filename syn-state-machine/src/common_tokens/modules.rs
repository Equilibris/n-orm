use super::*;
use crate::*;

pub enum Module<Content: Parsable = Tokens, Inner: Parsable = Tokens> {
    Extern {
        id: Ident,
        r#unsafe: bool,
    },
    Inline {
        id: Ident,

        r#unsafe: bool,
        inner_attrs: InnerAttrs<Inner>,
        content: SmOutput<Content>,
    },
}

impl<Content: Parsable, Inner: Parsable> MappedParse for Module<Content, Inner> {
    type Source = (
        Option<KwUnsafe>,
        KwMod,
        Identifier,
        Either<Brace<(InnerAttrs<Inner>, Content)>, Semi>,
    );

    type Output = Self;
    type Error = SmError<Self::Source>;

    fn map(
        src: SmOutput<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        let r#unsafe = src.0.is_some();
        let id = src.2;
        Ok(match src.3 {
            Either::Left((inner_attrs, content)) => Self::Inline {
                id,

                r#unsafe,
                inner_attrs,
                content,
            },
            Either::Right(_) => Self::Extern { id, r#unsafe },
        })
    }

    fn map_err(src: SmError<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
