// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::ressource::Ressource;

#[derive(Clone)]
pub struct Inventory (pub Vec<Ressource>);

impl Inventory {
    /// The `new` constructor function returns the list ressources.
    
    pub fn new (
        ressources: Vec<Ressource>,
    ) -> Self {
        Inventory(ressources)
    }
    
    /// The `from_result` multi constructor function returns a list
    /// of Item for a list of Result.
    
    fn from_result(ressources: Vec<std::io::Result<(String, usize)>>)
                 -> Option<Self> {
        if ressources.iter().any(|x| x.is_err()) {
            None
        } else {
            Some (
                Inventory::new (
                    ressources.into_iter()
                              .map(|res| match res {
                                            Ok((n, qte)) => Ressource::new(n, qte),
                                            Err(_) => unimplemented!(),
                                         })
                              .collect::<Vec<Ressource>>()
                )
            )
        }
    }
    
    /// The `from_line` multi constructor function returns a list
    /// of Item for a need or result argument from `Process`.
    
    pub fn from_line(ressources: &str) -> Option<Self> {
        Inventory::from_result(ressources.split(&['(', ':', ';', ')'][..])
          .filter(|&a| !a.is_empty())
          .collect::<Vec<&str>>()
          .chunks(2)
          .map(|ressource| match &ressource[..] {
            [n, q] => {
              if let Some(quantity) = q.parse::<usize>().ok() {
                Ok((n.to_string(), quantity))
              } else {
                try!(Err(from_error!("quantity isn't parsable")))
              }
            }
            _ => try!(Err(from_error!("unimplemented"))),
          })
          .collect::<Vec<std::io::Result<(String, usize)>>>())
    }
}

impl std::default::Default for Inventory {

  /// The `default` constructor function returns a empty Inventory.

  fn default() -> Self {
    Inventory(Vec::new())
  }
}

impl std::fmt::Display for Inventory {

    /// The `fmt` function prints the multiplication list.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.iter().map(|a| format!("{}", a))
                                     .collect::<Vec<String>>()
                                     .join(", "))
    }
}

impl std::ops::Deref for Inventory {
    type Target = std::vec::Vec<Ressource>;

    fn deref(&self) -> &std::vec::Vec<Ressource> {
        &self.0
    }
}

impl std::ops::DerefMut for Inventory {
    fn deref_mut(&mut self) -> &mut std::vec::Vec<Ressource> {
        &mut self.0
    }
}
