// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module `Trace` describes a list of orders.

extern crate std;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use std::io::prelude::*;

pub struct Trace(Vec<(String, usize)>);

impl Trace {

  /// The `new` constructor function returns a list of order by cycle.

  pub fn new (
    file: &str,
  ) -> std::io::Result<Self> {
    std::io::BufReader::new(
      try!(std::fs::File::open(file))
    ).lines().fold_while(Ok(Trace::default()), |mut acc, readed| {
      if let (Ok(mut trace), Ok(line)) = (acc, readed) {
        match &line.splitn(2, ":")
                   .collect::<Vec<&str>>()[..] {
          [comment, _..] if comment.starts_with('#') => Continue(Ok(trace)),
          [quantity, product] if quantity.parse::<usize>().is_ok() => {
            trace.push(
              product.to_string(),
              quantity.parse::<usize>().ok().unwrap_or_default()
            );
            Continue(Ok(trace))
          },
          [why..] => Done(
            Err(from_error!("Trace::new", why))
          ),
        }
      }
      else {Done(
        Err(from_error!("Trace::new - unimplemented"))
      )}
    })
  }

  /// The `from_vec` constructor function returns a list of order by cycle
  /// according to a vector.

  pub fn from_vec (
    list: Vec<(String, usize)>
  ) -> Self {
    Trace(list)
  }

  /// The `iter` interface function returns a iterator.

  pub fn iter (
    &self,
  ) -> std::slice::Iter<(std::string::String, usize)> {
    self.0.iter()
  }

  /// The `push` interface function inserts a new order to
  /// the list.

  pub fn push (
    &mut self,
    product: String,
    quantity: usize,
  ) {
    self.0.push((
      product,
      quantity
    ))
  }
}

impl std::fmt::Display for Trace {

  /// The `fmt` function prints the list order.

  fn fmt (
      &self,
      f: &mut std::fmt::Formatter,
  ) -> Result<(), std::fmt::Error> {
    write!(f, "{}", self.0.iter().map(|a| format!("{}:{}", a.1, a.0))
                                 .collect::<Vec<String>>()
                                 .join("\n"))
  }
}

impl std::default::Default for Trace {

  /// The `default` constructor function returns a empty Trace.

  fn default() -> Self {
    Trace(Vec::new())
  }
}
