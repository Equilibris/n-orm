use crate::*;

impl<T: Parsable> MappedParse for Box<T> {
    type Source = T;

    type Output = Box<SmOutput<T>>;
    type Error = Box<SmError<T>>;

    fn map(src: SmOutput<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(Box::new(src))
    }

    fn map_err(src: SmError<<Self as MappedParse>::Source>) -> Self::Error {
        Box::new(src)
    }
}
