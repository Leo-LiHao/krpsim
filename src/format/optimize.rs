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
}

impl Optimize {

  /// The `new` constructor function returns the optimization's items.

  pub fn new (
    stock: Vec<String>,
  ) -> Self {
    Optimize {
      stock: stock,
    }
  }

  /// The `from_line` constructor function returns the optimization's item
  /// for a parsed line.

  pub fn from_line (
    line: String,
  ) -> Self {
    Optimize::new (
      line.split(&['(', ';', ')'][..])
          .filter(|& a| !a.is_empty())
          .map(|a| a.to_string())
          .collect::<Vec<String>>()
    )
  }
}

impl std::default::Default for Optimize {

  /// The `default` constructor function returns a empty optimize.

  fn default() -> Self {
    Optimize {
      stock: Vec::new(),
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
