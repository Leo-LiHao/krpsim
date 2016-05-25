// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;
use std::collections::HashMap;

use super::ressource::Ressource;

pub struct Process {
    pub name: String,
    pub cycle: u64,
    pub input: Vec<Ressource>,
    pub output: Vec<Ressource>,
    pub heuristic: HashMap<String, f64>
}

impl Process {

    /// The `new` constructor function returns the Process.

    pub fn new(
        name: String,
        cycle: u64,
        input: Vec<Ressource>,
        output: Vec<Ressource>
    ) -> Self {
        let mut hash = HashMap::new();
        for ressource in input.clone() {
            hash.insert(ressource.0, (0.0 - ressource.1 as f64) / cycle as f64);
        }
        for ressource in output.clone() {
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
}


impl std::fmt::Debug for Process {

    /// The `fmt` function prints the Process formated like `<name> :
    /// (<need> :<qty>[ ;<need> :<qty>[...]]) :
    /// (<result> :<qty>[ ;<result> :<qty>[...]]) :
    /// <nb_cycle>`.

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(process: {}, {:?}, {:?}, {})", self.name, self.input, self.output, self.cycle)
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
