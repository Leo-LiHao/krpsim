// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module `Ressource` describes a stock.

extern crate std;

/// The `Ressource` structure is the Item implementation.
#[derive(Clone)]
pub struct Ressource (pub String, pub usize);

impl Ressource {
  /// The `new` constructor function returns the Stock.

  pub fn new(stock_name: String, quantity: usize) -> Self {
    Ressource(stock_name, quantity)
  }

  /// The `get_name` accessor function returns the name
  /// of ressource.

  pub fn get_name(&self) -> &str {
    &self.0
  }

  /// The `get_quantity` accessor function returns the
  /// quantity of ressource.

  pub fn get_quantity(&self) -> &usize {
    &self.1
  }

  /// The `set_quantity` updates and returns the qte value.

  fn set_quantity (
      &mut self,
      val: usize,
  ) -> &usize {
    self.1 = val;
    &self.1
  }

  /// The `add_from_ressource` function additiones a item
  /// with a value.

  pub fn add_quantity (
    &mut self,
    val: usize,
  ) -> Option<usize> {
    match self.1.checked_add(val) {
        Some(v) => Some(*self.set_quantity(v)),
        None => None,
    }
  }

  /// The `add_from_ressource` function additiones a item
  /// with another item.

  pub fn add_from_ressource (
    &mut self,
    val: &Ressource,
  ) -> Option<usize> {
    self.add_quantity(*val.get_quantity())
  }

  /// The `sub_from_ressource` function substrates a item
  /// with a value.

  pub fn sub_quantity (
    &mut self,
    val: usize,
  ) -> Option<usize> {
    match self.1.checked_sub(val) {
      Some(v) => Some(*self.set_quantity(v)),
      None => None,
    }
  }

  /// The `sub_from_ressource` function substrates a item
  /// with another item.

  pub fn sub_from_ressource (
    &mut self,
    val: &Ressource,
  ) -> Option<usize> {
    self.sub_quantity(*val.get_quantity())
  }

  /*pub fn can_order (
    &self,
    with: Ressource,
  ) -> std::io::Result<(), usize> {
    match 
  }*/
}

impl std::fmt::Display for Ressource {
  /// The `fmt` function prints the Ressource formated
  /// like `<stock_name> :<quantity>`.

  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{}", self.0, self.1)
  }
}

impl std::default::Default for Ressource {

  /// The `default` constructor function returns a empty Ressource.

  fn default() -> Self {
    Ressource( String::new(), 0usize)
  }
}

impl std::ops::Sub for Ressource {
  type Output = Ressource;

  fn sub(self, rhs: Ressource) -> Ressource {
    Ressource(self.0, self.1 - rhs.1)
  }
}
