// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Optimize` structure is a list of keywords/stock-names who
/// have the priority queue.

pub struct Optimize {
  pub stock: Vec<String>,
  pub time: bool,
}

impl Optimize {

  /// The `new` constructor function returns the optimization's items.

  pub fn new (
    stock: Vec<String>,
    time: bool,
  ) -> Self {
      Optimize {
        stock: stock,
        time: time,
      }
  }

  /// The `from_line` constructor function returns the optimization's item
  /// for a parsed line.

  pub fn from_line (
    line: String,
  ) -> Self {
    let stock: Vec<String> = line.split(&['(', ';', ')'][..])
                                 .filter(|&a| !a.is_empty())
                                 .map(|a| a.to_string())
                                 .collect::<Vec<String>>();
    Optimize::new (
      stock.iter().filter(|&a| a != "time")
                  .map(|a| a.to_string())
                  .collect::<Vec<String>>(),
      stock.iter().any(|a| a == "time"),
    )
  }

  /// The `len` interface function returns the number of elements
  /// in the list.

  pub fn len (
    &self,
  ) -> usize {
    self.stock.len()
  }
}

impl std::default::Default for Optimize {

  /// The `default` constructor function returns a empty optimize.

  fn default() -> Self {
    Optimize {
      stock: Vec::new(),
      time: false,
    }
  }
}

impl std::fmt::Display for Optimize {

  /// The `fmt` function prints the Optimization's items.

  fn fmt (
    &self,
    f: &mut std::fmt::Formatter,
  ) -> Result<(), std::fmt::Error> {
     write!(f, "(optimize: {})", self.stock.iter().map(|a| format!("{}", a))
                                                  .collect::<Vec<String>>()
                                                  .concat())
  }
}
