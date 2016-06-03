// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;
use std::collections::HashMap;

use format::stock::ressource::Ressource;
use format::stock::inventory::Inventory;

pub struct Process {
    pub name: String,
    pub cycle: u64,
    pub input: Inventory,
    pub output: Inventory,
    pub heuristic: HashMap<String, f64>
}

impl Process {

    /// The `new` constructor function returns the Process.

    pub fn new(
        name: String,
        cycle: u64,
        input: Inventory,
        output: Inventory,
    ) -> Self {
        let mut hash = HashMap::new();
        for ressource in (*input).clone() {
            hash.insert(ressource.0, (0.0 - ressource.1 as f64) / cycle as f64);
        }
        for ressource in (*output).clone() {
            let rec = hash.entry(ressource.0).or_insert(ressource.1 as f64 / cycle as f64);
            *rec += ressource.1 as f64 / cycle as f64;
        }
        Process {
            name: name,
            cycle: cycle,
            input: input,
            output: output,
            heuristic: hash
        }
    }

    pub fn get_h_value(&self, s: &String) -> f64 {
        match self.heuristic.get(s) {
            Some(&number) => number,
            None => 0.0
        }
    }

    /// The `can_buy` checks if the ressource can be bought.

    #[allow(unused_variables)]
    pub fn buy_it (
        &self,
        inventory: &mut Inventory,
    ) -> Option<&u64> {
        //! Work in progress.
        /*
        let mut trunk = inventory.iter_mut();
        let mut ss = self.input.into_iter().map(|required|
            match trunk.skip_while(|has|
              required.get_name() == has.get_name() &&
              required.get_quantity() <= has.get_quantity()
            ).nth(0) {
                Some(ref has) => None,
                None => None,
            }
        ).collect::<Vec<Option<(&Ressource, &mut Ressource)>>>();*/
        None
/*
        self.input.iter().all(|&required|
          trunk.skip_while(|has|
            required.get_name() == has.get_name() &&
            required.get_quantity() <= has.get_quantity()
          )
          match trunk.nth(0) {
            Some(ref has) if required.get_name() == has.get_name() &&
                             required.get_quantity() <= has.get_quantity() &&
                             trunk.next() => {
                Some((has))
            },
            Some(_) | None => false,
          }
        ).collect::<Vec<&Ressource>>().len() == inventory.len();
        if trunk.next().is_none() {
            Some(&self.cycle)
        }
        else {
            None
        }*/
        /*
        match self.input.iter().zip(self.output.iter())
                               .find(|&(&ref i, &_)|
                                 i.get_name() == inventory.get_name()
                               ) {
            Some((i, o)) if i.get_quantity() <= inventory.get_quantity() => {
                inventory.sub_quantity(*i.get_quantity());
                Some(o)
            },
            Some((_, _)) | None => None,
        }
        */
    }
}

impl std::fmt::Display for Process {

   /// The `fmt` function prints the Process formated like `<name> :
   /// (<need> :<qty>[ ;<need> :<qty>[...]]) :
   /// (<result> :<qty>[ ;<result> :<qty>[...]]) :
   /// <nb_cycle>`.

  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(process: {}, {}, {}, {})", self.name, self.input, self.output, self.cycle)
  }
}

impl std::default::Default for Process {

  /// The `default` constructor function returns a empty Proces.

  fn default() -> Self {
    Process {
      name: String::new(),
      cycle: 0u64,
      input: Inventory::default(),
      output: Inventory::default(),
      heuristic: HashMap::new()
    }
  }
}
