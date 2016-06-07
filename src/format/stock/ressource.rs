// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Ressource` structure is the Item implementation.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Ressource(pub String, pub usize);

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

  pub fn sub_quantity(&mut self, sub: usize) {
    self.1 -= sub;
  }
}

impl std::fmt::Display for Ressource {
  /// The `fmt` function prints the Ressource formated
  /// like `<stock_name> :<quantity>`.

  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(ressource: {}, {})", self.0, self.1)
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

fn update_ressource<F>(ori: &mut Vec<Ressource>,
                       update: &Vec<Ressource>,
                       closure: F)
                       -> ()
  where F: Fn(usize, usize) -> usize
{
  for item in update {
    if let Some(needle) = ori.iter_mut().find(|needle| needle.0 == item.0) {
      needle.1 = closure(needle.1, item.1);
    }
  }
}

pub fn add(ori: &mut Vec<Ressource>,
           update: &Vec<Ressource>,
           operations: usize)
           -> () {
  update_ressource(ori, update, |a, b| a + b * operations)
}
pub fn sub(ori: &mut Vec<Ressource>,
           update: &Vec<Ressource>,
           operations: usize)
           -> () {
  update_ressource(ori, update, |a, b| a - b * operations)
}

pub fn check_ressource(need: &Vec<Ressource>,
                       owned: &Vec<Ressource>)
                       -> usize {
  let ret = need.into_iter()
    .map(|&ref n| {
      match owned.into_iter().find(|&s| *s.0 == *n.0) {
        None => 0,
        Some(own) => own.1 as usize / n.1 as usize,
      }
    })
    .collect::<Vec<_>>();
  *ret.iter().min().unwrap()
}
