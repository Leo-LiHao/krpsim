extern crate std;

pub struct Optimize <'a> {
  stock: Vec<&'a str>,
}

impl <'a> Optimize <'a> {

  /// The `new` constructor function returns the item list to optimize.

  pub fn new (
    stock: Vec<&'a str>,
  ) -> Self {
    Optimize {
      stock: stock,
    }
  }
}

impl <'a> std::fmt::Display for Optimize <'a> {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter,
  ) -> Result<(), std::fmt::Error> {
      Ok(())
//    write!(f, "(optimize: {})", self.stock.iter().map(|a| format!("{}", a))
  //                                               .collect::<Vec<String>>())
  }
}
