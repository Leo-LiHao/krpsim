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
    neutral: Option<Ressource>,
    pub heuristic: HashMap<String, usize>
}

///if exist, return a neutral component
pub fn get_neutral(input: &Inventory, output: &Inventory) -> Option<Ressource> {
    match input.iter().find(|&(_, x)| output.any_from_ressource(x)) {
        Some((_, val)) => Some(val.clone()),
        None => None,
    }
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
            hash.insert(ressource.0, (0 - ressource.1));
        }
        for ressource in output.to_vec() {
            let rec = hash.entry(ressource.0).or_insert(ressource.1);
            *rec += ressource.1;
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

    pub fn get_producing_process(obj: Ressource, process: &Vec<Process>) ->Vec<Process> {
        let mut ret = Vec::new();
        for procs in process {
            if procs.get_h_value(&obj.0) > 0 {
                ret.push(procs.clone());
            }
        }
        ret
    }

    pub fn get_time(&self, process: &Vec<Process>, ressources: &Inventory) -> usize {
        1
    }

    pub fn needed_process(&self, process: &Vec<Process>, ressources: &Inventory)
                          -> Result<Option<Vec<Process>>, ()> {
        let mut input = self.input.clone();
        if let Some(ref x) = self.neutral {
            // Check if the neutral ressource exist
           input.sub(&x);
        }
        input.sub_from_inventory(ressources);
        if input.is_empty() {
            Ok(None)
        } else {
            let mut ret: Vec<Process> = Vec::new();
            for obj in input.iter() {
                let tmp = Process::get_producing_process(&obj, process);
                if tmp.len() == 0 {
                    Err(());
                }
                //temporary: should cycle tmp and give the fast
                let smt = match tmp.iter().max_by_key(|&x| x.get_h_value(&obj.0)) {
                    None => return Err(()),
                    Some(a) => a
                };
                match smt.needed_process(process, ressources) {
                    Err(_) => return Err(()),
                    Ok(None) => ret.push(smt.clone()),
                    Ok(Some(a)) => ret.extend(a)
                };
            }
            Ok(Some(ret))
        }

    }

    pub fn distance_overall(&self, owned: &Vec<Ressource>) -> usize {
        self.input.to_vec().iter().fold(0usize, |acc, b| acc + Process::get_distance(b, owned))
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
      neutral: None,
      heuristic: HashMap::new()
    }
  }
}
