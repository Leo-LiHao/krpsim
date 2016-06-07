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
    pub cycle: u64,
    pub input: Inventory,
    pub output: Inventory,
    neutral: Option<Ressource>,
    pub heuristic: HashMap<String, f64>
}

///if exist, return a neutral component
pub fn get_neutral(input: &Inventory, output: &Inventory) -> Option<Ressource> {
    input.into_iter().find(|&x| output.iter().any(|&y| y == x))
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
        let neutral = get_neutral(&input, &output);
        Process {
            name: name,
            cycle: cycle,
            input: input,
            output: output,
            neutral: neutral,
            heuristic: hash
        }
    }

    pub fn get_h_value(&self, s: &String) -> f64 {
        match self.heuristic.get(s) {
            Some(&number) => number,
            None => 0.0
        }
    }

    pub fn get_distance(need: &Ressource, owned: &Vec<Ressource>) -> u64 {
        match owned.iter().find(|&x| x.0 == need.0) {
            Some(a) => std::cmp::max(0, a.1 as u64 - need.1 as u64),
            None => need.1 as u64
        }
    }

    pub fn distance_overall(&self, owned: &Vec<Ressource>) -> u64 {
        self.input.iter().fold(0, |acc, b| acc + Process::get_distance(b, owned)) //Maybe use a closure here?
    }

    pub fn needed_process(&self, process: &Vec<Process>, ressources: &Vec<Ressource>) -> Option<Vec<Process>> {
        let mut input = self.input.clone();
        if let Some(x) = self.neutral {
            // Check if the neutral ressource exist
           input.sub(&x);
        }
        input.sub(&ressources);
        let mut ret: Vec<Process> = Vec::new();

        None
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
