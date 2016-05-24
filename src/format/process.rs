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
