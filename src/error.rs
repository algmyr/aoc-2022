use core::fmt;
use std::io;
use std::num::ParseIntError;

pub type AocResult<T> = Result<T, AocError>;

#[derive(Debug)]
pub enum AocError {
  Io(io::Error),
  ParseIntError(ParseIntError),
  ParserError(String),
  Custom(String),
}

impl fmt::Display for AocError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      AocError::Io(ref err) => err.fmt(f),
      AocError::ParseIntError(ref err) => err.fmt(f),
      AocError::ParserError(ref err) => err.fmt(f),
      AocError::Custom(ref err) => write!(f, "AoC error: {:?}", err),
    }
  }
}

impl From<io::Error> for AocError {
  fn from(err: io::Error) -> AocError { AocError::Io(err) }
}

impl From<ParseIntError> for AocError {
  fn from(err: ParseIntError) -> AocError { AocError::ParseIntError(err) }
}

//impl<E> From<nom::Err<E>> for AocError
//where
//  nom::Err<E>: std::fmt::Display,
//{
//  fn from(err: nom::Err<E>) -> AocError { AocError::ParserError(err.to_string()) }
//}

pub fn aoc_error(msg: &str) -> AocError { AocError::Custom(msg.to_owned()) }
