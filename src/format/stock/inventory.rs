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
pub struct Inventory (std::collections::HashMap<String, Ressource>);

impl Inventory {
    /// The `new` constructor function returns the list of ressources.

    pub fn new (
        ressources: Vec<Ressource>,
    ) -> Self {
        let mut map: std::collections::HashMap<String, Ressource> = std::collections::HashMap::with_capacity(ressources.len());

        ressources.into_iter().all(|ressource|
            map.insert(ressource.get_name().to_string(), ressource).is_some()
        );
        Inventory(map)
    }
    
    /// The `from_result` multi constructor function returns a list
    /// of Item for a list of Result.
    
    fn from_result(ressources: Vec<std::io::Result<Ressource>>)
                 -> Option<Self> {
        if ressources.iter().any(|x| x.is_err()) {
            None
        } else {
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
                Ok(Ressource::new(n.to_string(), quantity))
              } else {
                try!(Err(from_error!("quantity isn't parsable")))
              }
            }
            _ => try!(Err(from_error!("unimplemented"))),
          })
          .collect::<Vec<std::io::Result<Ressource>>>())
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
        match self.get_mut_from_ressource(val) {
            Some(ref mut v) => v.add_from_ressource(val),
            None => None,
        }
    }

    /// The `add` interface function additiones a list of item
    /// to our self-inventory.

    pub fn add_from_inventory (
        &mut self,
        vals: &Inventory,
    ) -> bool {
        !vals.iter().map(|(_, val)|
                           self.add(&val)
                   ).collect::<Vec<Option<usize>>>()
             .iter().any(|item|
                           item.is_none()
                   )
    }

    /// The `sub` interface function substractes a item
    /// with another item.

    pub fn sub (
        &mut self,
        val: &Ressource,
    ) -> Option<usize> {
        match {
            match self.get_mut_from_ressource(val) {
                Some(ref mut v) => v.sub_from_ressource(val),
                None => None,
            }
        } {
            Some(0) => {
              if self.0.remove(val.get_name()).is_some() {
                Some(0)
              }
              else {
                None
              }
            },
            v => v,
        }
    }

    /// The `sub` interface function substractes a list of item
    /// to our self-inventory.

    pub fn sub_from_inventory (
        &mut self,
        vals: &Inventory,
    ) -> bool {
        !vals.iter().map(|(_, val)|
                           self.add(&val)).collect::<Vec<Option<usize>>>(
                   )
             .iter().any(|item|
                           item.is_none()
                   )
    }

    /// The `to_vec` function returns a cloned list of ressource.

    #[deprecated]
    pub fn to_vec (&self) -> Vec<Ressource> {
      self.0.iter().map(|ressoure| ressoure.1.clone())
                   .collect::<Vec<Ressource>>()
    }
}

impl std::fmt::Display for Inventory {

    /// The `fmt` function prints the multiplication list.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "({})", self.0.iter().map(|a| format!("{}", a.1))
                                       .collect::<Vec<String>>()
                                       .join(";"))
    }
}

impl std::default::Default for Inventory {

  /// The `default` constructor function returns a empty Inventory.

  fn default() -> Self {
    Inventory::new(Vec::new())
  }
}
