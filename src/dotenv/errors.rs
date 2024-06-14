use std::error;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
  LineParse(String, usize),
  Io(io::Error),
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Self::Io(err) => Some(err),
      _ => None,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(
    &self,
    fmt: &mut fmt::Formatter,
  ) -> fmt::Result {
    match self {
      Self::Io(e) => write!(fmt, "{e}"),
      Self::LineParse(line, index) => write!(
        fmt,
        "Error parsing line: '{line}', error at line index: {index}",
      ),
    }
  }
}
