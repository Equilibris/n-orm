use super::super::*;
use crate::*;
#[derive(Debug)]
pub enum SimplePathSegment {
    Id(Ident),
    DCrate,
}

impl MappedParse for SimplePathSegment {
    type Source = Sum2<FlatSum4<Identifier, KwSuper, KwLowerSelf, KwCrate>, DollarCrate>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum2::Val0(v) => Self::Id(v),
            Sum2::Val1(_) => Self::DCrate,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug, Default)]
pub struct SimplePathOrNone {
    pub leading_double_colon: bool,
    pub segments: Vec<SimplePathSegment>,
}

impl MappedParse for SimplePathOrNone {
    type Source = (
        Option<DoubleColon>,
        Interlace<SimplePathSegment, DoubleColon>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            leading_double_colon: src.0.is_some(),
            segments: src.1 .0,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct SimplePath {
    pub leading_double_colon: bool,
    pub segments: Vec<SimplePathSegment>,
}

#[derive(Debug, thiserror::Error)]
pub enum SimplePathError<T: std::error::Error> {
    #[error("{}", .0)]
    Inner(T),
    #[error("Sample path required length")]
    NoLength,
}

impl MappedParse for SimplePath {
    type Source = SimplePathOrNone;

    type Output = Self;
    type Error = SimplePathError<SmErr<Self::Source>>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        if src.segments.len() == 0 {
            Err(SimplePathError::NoLength)
        } else {
            Ok(Self {
                leading_double_colon: src.leading_double_colon,
                segments: src.segments,
            })
        }
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        SimplePathError::Inner(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_stops_on_invalid_token() {
        parse_terminal::<(Vec<Ident>, FPunct<'>'>, Semi)>(quote::quote! { id >; }).unwrap();
    }

    #[test]
    fn it_matches_ident() {
        let path = parse_terminal::<SimplePath>(quote::quote! { hello }).unwrap();

        assert_eq!(path.leading_double_colon, false);
        assert_eq!(path.segments.len(), 1);
        if let Some(SimplePathSegment::Id(v)) = path.segments.into_iter().next() {
            assert_eq!(v.to_string().as_str(), "hello");
        }
    }
    #[test]
    fn it_matches_common_paths() {
        let path =
            parse_terminal::<SimplePath>(quote::quote! { ::hello::world::super::crate::self })
                .unwrap();
        assert!(path.leading_double_colon);

        for (a, v) in path
            .segments
            .into_iter()
            .zip(["hello", "world", "super", "crate", "self"].into_iter())
        {
            if let SimplePathSegment::Id(a) = a {
                assert_eq!(a.to_string().as_str(), v);
            } else {
                unreachable!()
            }
        }
    }
}
