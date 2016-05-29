// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::ressource::Ressource;

pub struct Process {
    pub name: String,
    pub cycle: u64,
    pub input: Vec<Ressource>,
    pub output: Vec<Ressource>
}

impl Process {

    /// The `new` constructor function returns the Process.

    pub fn new(
        name: String,
        cycle: u64,
        input: Vec<Ressource>,
        output: Vec<Ressource>
    ) -> Self {
        Process {
            name: name,
            cycle: cycle,
            input: input,
            output: output
        }
    }

    /// The `can_buy` checks if the ressource can be bought.

    pub fn buy_it (
        &self,
        inventory: &mut Vec<Ressource>,
    ) -> Option<&u64> {
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
    write!(f, "(process: {}, {:?}, {:?}, {})", self.name, self.input, self.output, self.cycle)
  }
}

impl std::fmt::Debug for Process {

   /// The `fmt` function prints the Process formated like `<name> :
   /// (<need> :<qty>[ ;<need> :<qty>[...]]) :
   /// (<result> :<qty>[ ;<result> :<qty>[...]]) :
   /// <nb_cycle>`.
 
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(process: {}, {})", self.name, self.cycle)
  }
}

