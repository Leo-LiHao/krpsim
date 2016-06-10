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
pub struct Ressource(pub String, pub usize, f64);

impl Ressource {
  /// The `new` constructor function returns the Stock.

  pub fn new(stock_name: String, quantity: usize) -> Self {
    Ressource(stock_name, quantity, quantity as f64)
  }

  /// The `get_name` accessor function returns the name
  /// of ressource.

  pub fn get_name(&self) -> &str {
    &self.0
  }

  /// The `get_quantity` accessor function returns the
  /// quantity of ressource.

  pub fn get_quantity(&self) -> usize {
    self.1
  }

  /// The `get_quantity` accessor function returns the
  /// quantity of ressource.

  pub fn get_float_quantity(&self) -> f64 {
    self.2
  }

  /// The `set_quantity` updates and returns the qte value.

  fn set_quantity (
      &mut self,
      val: usize,
  ) -> usize {
    self.1 = val;
    self.1
  }

  /// The `add_from_ressource` function additiones a item
  /// with a value.

  pub fn add_quantity (
    &mut self,
    val: usize,
  ) -> Option<usize> {
    match self.1.checked_add(val) {
        Some(v) => Some(self.set_quantity(v)),
        None => None,
    }
  }

  /// The `add_from_ressource` function additiones a item
  /// with another item.

  pub fn add_from_ressource (
    &mut self,
    val: &Ressource,
  ) -> Option<usize> {
    self.add_quantity(val.get_quantity())
  }

  /// The `can_sub_quantity` function checks if the substrate is
  /// possible.

  pub fn can_sub_quantity (
    &self,
    val: usize,
  ) -> Option<usize> {
    match self.1.checked_sub(val) {
      Some(v) => Some(v),
      None => None,
    }
  }

  /// The `sub_quantity` function substrates a item
  /// with a value.

  pub fn sub_quantity (
    &mut self,
    val: usize,
  ) -> Option<usize> {
    match self.can_sub_quantity(val) {
      Some(v) => Some(self.set_quantity(v)),
      None => None,
    }
  }

  /// The `can_sub_from_ressource` function checks if the substrate of
  /// a item is possible with another item.

  pub fn can_sub_from_ressource (
    &self,
    val: &Ressource,
  ) -> Option<usize> {
    self.can_sub_quantity(val.get_quantity())
  }

  /// The `sub_from_ressource` function substrates a item
  /// with another item.

  pub fn sub_from_ressource (
    &mut self,
    val: &Ressource,
  ) -> Option<usize> {
    self.sub_quantity(val.get_quantity())
  }

  pub fn can_order (
    &self,
    with: &Ressource,
  ) -> std::io::Result<usize> {
    match with.get_quantity().checked_sub(self.get_quantity()) {
      Some(v) => Ok(v),
      None => Err(from_error!("less qte")),
    }
  }

  pub fn order (
    &self,
    with: &mut Ressource,
  ) -> std::io::Result<usize> {
    match self.can_order(with) {
      Ok(v) => Ok(with.set_quantity(v)),
      w => w,
    }
  }
}

impl std::cmp::Ord for Ressource {
    /// The `cmp` function fast compares two Ressource.

    fn cmp(&self, with: &Self) -> std::cmp::Ordering {
        (self.1).cmp(&with.1)
    }
}

impl std::cmp::PartialOrd for Ressource {
    fn partial_cmp(&self, with: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(with))
    }
}

impl std::cmp::PartialEq for Ressource {
    /// The `eq` function fast checks if two Ressource are equal.

    fn eq(&self, with: &Self) -> bool {
        (&self.0, self.1) == (&with.0, with.1)
    }
}

impl Eq for Ressource {
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
    Ressource(String::new(), 0usize, 0f64)
  }
}

impl std::ops::Sub for Ressource {
  type Output = Ressource;

  fn sub(self, rhs: Ressource) -> Ressource {
    Ressource(self.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}
