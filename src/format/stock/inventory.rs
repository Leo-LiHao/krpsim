// @lecorref - github.com/lecorref, @geam - github.com/geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module `Inventory` describes a list of items.

extern crate std;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use super::ressource::Ressource;

pub const ERR_WRONG_QTE: &'static str = "Invalid quantity {} from Inventory";
pub const ERR_NOT_FOUND: &'static str = "Item wasn't found `{}` from Inventory";
pub const ERR_EMPTY: &'static str = "There isn't any command `{}` from Inventory";
pub const ERR_LESS: &'static str = "The {}'s payment is insufficient";

#[derive(Clone)]
pub struct Inventory (std::collections::HashMap<String, Ressource>);

impl Inventory {
    /// The `new` constructor function returns the list of ressources.

    pub fn new (
        ressources: Vec<Ressource>,
    ) -> Self {
        let len: usize = ressources.len();

        Inventory(
          ressources.into_iter().fold_while(
            std::collections::HashMap::with_capacity(len),
            |mut map, ressource| {
              map.insert(ressource.get_name().to_string(), ressource);
              Continue(map)
            }
          )
        )
    }

    /// The `from_result` multi constructor function returns a list
    /// of Item for a list of Result.

    fn from_result(ressources: Vec<std::io::Result<Ressource>>)
                 -> Option<Self> {
        if ressources.iter().any(|x| x.is_err()) {
            None
        }
        else {
            Some (
                Inventory::new (
                    ressources.into_iter()
                              .map(|res| match res {
                                            Ok(ressource) => ressource,
                                            Err(_) => unimplemented!(),
                                         })
                              .collect::<Vec<Ressource>>()
                )
            )
        }
    }

    pub fn print_final(&self) -> () {

     }

    /// The `from_line` multi constructor function returns a list
    /// of Item for a need or result argument from `Process`.

    pub fn from_line(ressources: &str) -> Option<Self> {
        Inventory::from_result(ressources.split(&['(', ':', ';', ')'][..])
          .filter(|&a| !a.is_empty())
          .collect::<Vec<&str>>()
          .chunks(2)
          .map(|ressource|
            match &ressource[..] {
              &[ref n, ref q] => {
                if let Some(quantity) = q.parse::<usize>().ok() {
                  Ok(Ressource::new(n.to_string(), quantity))
                }
                else {
                  try!(from_error!(ERR_WRONG_QTE, &format!("{}", q)))
                }
              }
              _ => unimplemented!(),
            })
          .collect::<Vec<std::io::Result<Ressource>>>())
    }

    /// The `len` interface function returns the number of elements
    /// in the map.

    pub fn len (
      &self,
    ) -> usize {
      self.0.len()
    }

    /// The `iter` interface function returns a iterator.

    pub fn iter (
      &self,
    ) -> std::collections::hash_map::Iter<std::string::String, Ressource> {
        self.0.iter()
    }

    /// The `push` interface function inserts a new item to
    /// the inventory.

    pub fn push (
        &mut self,
        key: String,
        val: Ressource,
    ) -> Option<Ressource> {
        self.0.insert(key, val)
    }

    /// The `is_empty` interface function returns true if
    /// the map contains not elements.

    pub fn is_empty (
        &self,
    ) -> bool {
        self.0.is_empty()
    }

    /// The `is_zero` interface function returns true if
    /// the map contains only nul ressources

    pub fn is_zero (
        &self
    ) -> bool {
        self.iter().all(|(_, x)| x.1 == 0)
    }

    /// The `any` interface function checks if the map contains
    /// the key.

    pub fn any (
        &self,
        key: &str,
    ) -> bool {
        self.0.contains_key(key)
    }

    /// The `any` interface function checks if the map contains
    /// the key from a ressource.

    pub fn any_from_ressource (
        &self,
        val: &Ressource,
    ) -> bool {
        self.any(val.get_name())
    }

    /// The `get` accessor function returns a item.

    pub fn get (
        &self,
        key: &str,
    ) -> Option<&Ressource> {
        self.0.get(key)
    }

    /// The `get_from_ressource` accessor function returns a item.

    pub fn get_from_ressource (
        &self,
        val: &Ressource,
    ) -> Option<&Ressource> {
        self.get(val.get_name())
    }

    /// The `get_mut` mutator function returns a item.

    pub fn get_mut (
        &mut self,
        key: &str,
    ) -> Option<&mut Ressource> {
        self.0.get_mut(key)
    }

    /// The `get_mut` mutator function returns a item.

    pub fn get_mut_from_ressource (
        &mut self,
        val: &Ressource,
    ) -> Option<&mut Ressource> {
        self.get_mut(val.get_name())
    }

    /// The `add` interface function additiones a item
    /// with another item.

    pub fn add (
        &mut self,
        val: &Ressource,
    ) -> Option<usize> {
        if self.any_from_ressource(val) {
          match self.get_mut_from_ressource(val) {
            Some(v) => v.add_from_ressource(val),
            None => unimplemented!(),
          }
        }
        else {
          self.push(
           val.get_name().to_string(),
           val.clone()
          );
          Some(val.get_quantity())
        }
    }

    /// The `add` interface function additiones a list of item
    /// to our self-inventory.

    pub fn add_from_inventory (
        &mut self,
        vals: &Inventory,
    ) -> bool {
      !vals.iter().map(|(_, val)| self.add(&val))
                  .collect::<Vec<Option<usize>>>()
                  .iter().any(|item| item.is_none())
    }

    /// The `sub` interface function substractes a item
    /// with another item.

    pub fn sub (
        &mut self,
        val: &Ressource,
    ) -> Option<usize> {
        match self.get_mut_from_ressource(val) {
          Some(ref mut v) => Some(v.sub_from_ressource(val)),
          None => None,
        }
    }

    /// The `sub` interface function substractes a list of item
    /// to our self-inventory.

    pub fn sub_from_inventory (
        &mut self,
        vals: &Inventory,
    ) -> bool {
        !vals.iter().map(|(_, val)|
                     self.sub(&val)).collect::<Vec<Option<usize>>>(
                   )
             .iter().any(|item|
                     item.is_none()
                   )
    }

    /// The `order` takes the payment of command.

    pub fn order (
        &self,
        with: &mut Inventory,
    ) -> std::io::Result<()> {
      self.iter()
          .fold_while(
            from_error!(ERR_EMPTY, &format!("{}", self)),
            |_, (&_, ref must_have)| {
                match with.get_mut_from_ressource(&must_have) {
                Some(ref mut have) if must_have.get_quantity() <= have.get_quantity() => {
                  have.sub_from_ressource(must_have);
                  Continue(Ok(()))
                },
                Some(_) => Done(from_error!(ERR_LESS, &format!("{}", self))),
                None => Done(from_error!(ERR_NOT_FOUND, must_have.get_name())),
              }
            }
          )
    }

    /// The `can_order` checks if the order is possible.

    pub fn can_order (
      &self,
      with: &Inventory,
    ) -> std::io::Result<()> {
        self.order(
          &mut with.clone()
        )
    }

    /// The `get_ressource` function returns a accessor on
    /// the list of ressource.

    pub fn get_ressource(&self) -> Vec<&Ressource> {
        self.0.iter().map(|(&_, ressoure)| ressoure)
            .collect::<Vec<&Ressource>>()
    }

    /// The `get_neutral` function returns return a neutral component
    /// if the output ressource exist.

    pub fn get_neutral (
        &self,
        output: &Inventory,
    ) -> Option<Ressource> {
        match self.iter().find(|&(_, x)| output.any_from_ressource(x)) {
            Some((_, val)) => Some(val.clone()),
            None => None,
        }
    }
    /// The `to_vec` function returns a cloned list of ressource.

    #[deprecated]
    pub fn to_vec (&self) -> Vec<Ressource> {
      self.0.iter().map(|ressoure| ressoure.1.clone())
                   .collect::<Vec<Ressource>>()
    }
}

impl std::cmp::PartialEq for Inventory {

    /// The `eq` function fast checks if two Inventory are equal.

    fn eq(&self, with: &Self) -> bool {
        self.0 == with.0
    }
}

impl std::cmp::Eq for Inventory {
}

impl std::fmt::Display for Inventory {

    /// The `fmt` function prints the multiplication list.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "({})", self.iter().sorted()
                                     .iter().map(|&(_, r)| format!("{}", r))
                                            .collect::<Vec<String>>()
                                            .join(";"))
    }
}

impl std::fmt::Debug for Inventory {

    /// The `fmt` function prints the multiplication list.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.iter().sorted()
                                   .iter().map(|&(_, r)|
                                                 format!(" {} => {}",
                                                   r.get_name(),
                                                   r.get_quantity()
                                                 ))
                                          .collect::<Vec<String>>()
                                          .join("\n"))
    }
}

impl std::default::Default for Inventory {

  /// The `default` constructor function returns a empty Inventory.

  fn default() -> Self {
    Inventory::new(Vec::new())
  }
}
