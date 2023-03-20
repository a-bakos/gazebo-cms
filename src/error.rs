use std::fmt::Formatter;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    OutOfBounds,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                // Why ref err?
                write!(f, "Cannot parse parameter {err}")
            }
            Error::MissingParameters => {
                write!(f, "Missing parameter")
            }
            Error::OutOfBounds => {
                write!(f, "Out of bounds")
            }
        }
    }
}
