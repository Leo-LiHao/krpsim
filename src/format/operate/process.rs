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

#[derive(Clone)]
pub struct Process {
    pub name: String,
    pub cycle: usize,
    pub input: Inventory,
    pub output: Inventory,
    pub heuristic: HashMap<String, usize>
}

impl Process {

    /// The `new` constructor function returns the Process.

    pub fn new (
        name: String,
        cycle: usize,
        input: Inventory,
        output: Inventory,
    ) -> Self {
        let mut hash = HashMap::new();
        for ressource in input.to_vec() {
            hash.insert(ressource.0, (0 - ressource.1) / cycle);
        }
        for ressource in output.to_vec() {
            let rec = hash.entry(ressource.0).or_insert(ressource.1 / cycle);
            *rec += ressource.1 / cycle;
        }
        Process {
            name: name,
            cycle: cycle,
            input: input,
            output: output,
            heuristic: hash
        }
    }

    /// The `from_line` constructor function returns a parsed process.

    pub fn from_line (
      name: String,
      need: &str,
      result_and_nb_cycle: &str,
    ) -> std::io::Result<Process> {
      match &result_and_nb_cycle.rsplitn(2, ':').collect::<Vec<&str>>()[..] {
        [nb_cycle, result] if nb_cycle.parse::<usize>().is_ok() => Ok(
                    Process::new (
                       name,
                       nb_cycle.parse::<usize>().unwrap_or(try!(Err(from_error!("bad number of cycle")))),
                       Inventory::from_line(need).unwrap_or(try!(Err(from_error!("bad need")))),
                       Inventory::from_line(result).unwrap_or(try!(Err(from_error!("bad result")))),
                    )
                 ),
        why => Err(from_error!("parse_proces", why)),
      }
    }

    /// The `get_name` accessor function returns the name
    /// of process.

    pub fn get_name(&self) -> &str {
        &self.name
    }


    pub fn get_h_value(&self, s: &String) -> usize {
        match self.heuristic.get(s) {
            Some(&number) => number,
            None => 0
        }
    }

    pub fn get_distance(need: &Ressource, owned: &Vec<Ressource>) -> usize {
        match owned.iter().find(|&x| x.0 == need.0) {
            Some(a) if a.1 > need.1 => a.1 - need.1,
            Some(_) => 0,
            None => need.1
        }
    }

    pub fn distance_overall(&self, owned: &Vec<Ressource>) -> usize {
        self.input.to_vec().iter().fold(0usize, |acc, b| acc + Process::get_distance(b, owned)) //Maybe use a closure here?
    }
}

impl std::fmt::Display for Process {

   /// The `fmt` function prints the Process formated like `<name> :
   /// (<need> :<qty>[ ;<need> :<qty>[...]]) :
   /// (<result> :<qty>[ ;<result> :<qty>[...]]) :
   /// <nb_cycle>`.

  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{}:{}:{}", self.name, self.input, self.output, self.cycle)
  }
}

impl std::default::Default for Process {

  /// The `default` constructor function returns a empty Proces.

  fn default() -> Self {
    Process {
      name: String::new(),
      cycle: 0usize,
      input: Inventory::default(),
      output: Inventory::default(),
      heuristic: HashMap::new()
    }
  }
}
