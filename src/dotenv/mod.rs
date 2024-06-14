#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]

//! [`dotenv`]: https://crates.io/crates/dotenv
//! A well-maintained fork of the [`dotenv`] crate
//!
//! This library loads environment variables from a *.env* file. This is convenient for dev environments.

mod errors;
mod iter;
mod parse;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub use self::errors::*;
use self::iter::Iter;

pub fn parse_file(path: &Path) -> errors::Result<HashMap<String, String>> {
  let iter = Iter::new(File::open(path).map_err(Error::Io)?);
  let mut output = HashMap::new();

  for v in iter {
    let (key, value) = v?;
    output.insert(key, value);
  }
  Ok(output)
}
