use std::error::Error;
use std::fmt;

/// ParserError represents all the possible errors that can be gotten
/// from parsing a JSON string.
#[derive(Debug)]
pub enum ParserError {
  UnexpectedCharacterError(char, usize),
  UnexpectedNumberError(usize),
  UnexpectedEndError,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParserError::UnexpectedCharacterError(c, p) => {
        write!(f, "Unexpected character {} at position {}", c, p)
      }
      ParserError::UnexpectedNumberError(p) => write!(f, "Unexpected number at position {}", p),
      ParserError::UnexpectedEndError => write!(f, "Unexpected end of JSON input"),
    }
  }
}
