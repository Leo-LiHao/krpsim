// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module `Process` describes a offer.

extern crate std;

use itertools::Itertools;

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
    pub heuristic: HashMap<String, f64>
}

impl Process {

    /// The `new` constructor function returns the Process.

    pub fn new (
        name: String,
        cycle: usize,
        input: Inventory,
        output: Inventory,
        hash: HashMap<String, f64>,
    ) -> Self {
        let neutral = input.get_neutral(&output);

        Process {
            name: name,
            cycle: cycle,
            input: input,
            output: output,
            neutral: neutral,
            heuristic: hash
        }
    }

    pub fn from_integer (
        name: String,
        cycle: usize,
        input: Inventory,
        output: Inventory,
    ) -> Self {
        let mut hash = HashMap::new();

        input.get_ressource().iter().foreach(|ressource| {
            hash.insert(
                ressource.get_name().to_string(),
                -ressource.get_float_quantity() / cycle as f64
            );
        });
        output.get_ressource().iter().foreach(|ressource| {
            *hash.entry(
                ressource.get_name().to_string()
            ).or_insert(
                ressource.get_float_quantity() / cycle as f64
            ) += ressource.get_float_quantity() / cycle as f64;
        });
        Process::new (
            name,
            cycle,
            input,
            output,
            hash,
        )
    }

    /// The `from_line` constructor function returns a parsed process.

    pub fn from_line (
        name: String,
        need: &str,
        result_and_nb_cycle: &str,
    ) -> std::io::Result<Process> {
        match &result_and_nb_cycle.rsplitn(2, ':').collect::<Vec<&str>>()[..] {
            [nb_cycle, result] => if nb_cycle.parse::<usize>().is_ok() {
                match (Inventory::from_line(need), Inventory::from_line(result)) {
                    (None,    None   ) => Err(from_error!(
                        format!("bad need `{}` and rest `{}`", need, result)
                    )),
                    (None,    Some(_)) => Err(from_error!(
                        format!("bad need `{}`", need)
                    )),
                    (Some(_), None   ) => Err(from_error!(
                        format!("bad rest `{}`", result)
                    )),
                    (Some(n), Some(r)) => Ok(
                        Process::from_integer(
                            name,
                            nb_cycle.parse::<usize>().ok().unwrap_or_default(),
                            n, r
                        )
                    ),
                }
            }
            else {
                Err(from_error!(format!("bad cycle `{}`", nb_cycle)))
            },
            why => Err(from_error!("parse_proces", why)),
        }
    }

    /// The `get_name` accessor function returns the name
    /// of process.

    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// The `get_cycle` accessor function returns the number
    /// of cycle required by the process.

    pub fn get_cycle(&self) -> usize {
        self.cycle
    }

    /// The `buy_with` function substrates *with* argument with *input* and
    /// additions the *output* to *with* argument.

    pub fn buy_with (
      &self,
      with: &mut Inventory, // with
    ) -> bool {
      if self.input.order(with) { // Pay pay pay.
        with.add_from_inventory(&self.output) // Take the list items.
      }
      else {
        false
      }
    }

    pub fn get_h_value(&self, s: &String) -> f64 {
        match self.heuristic.get(s) {
            Some(&number) => number,
            None => 0.0
        }
    }

    pub fn get_distance(need: &Ressource, owned: &Vec<Ressource>) -> usize {
        match owned.iter().find(|&x| x.0 == need.0) {
            Some(a) if a.1 > need.1 => a.1 - need.1,
            Some(_) => 0,
            None => need.1
        }
    }

    pub fn get_producing_process (
        obj: &Ressource,
        process: &Vec<&Process>
    ) -> Vec<Process> {
        let mut ret: Vec<Process> = Vec::new();

        process.iter().foreach(|procs| {
          if procs.get_h_value(&obj.0) > 0.0 {
            ret.push((*procs).clone());
          }
        });
        ret
    }

    pub fn needed_process (
        &self,
        process: &Vec<&Process>,
        ressources: &Inventory,
    ) -> Result<Option<Vec<Process>>, ()> {
        let mut input = self.input.clone();
        if let Some(ref x) = self.neutral {
            // Check if the neutral ressource exist
            input.sub(&x);
        }
        input.sub_from_inventory(ressources);
        if input.is_zero() {
            Ok(None)
        } else {
            let mut ret: Vec<Process> = Vec::new();
            for (_, obj) in input.iter() {
                let tmp = Process::get_producing_process(obj, process);
                if tmp.len() == 0 {
                    return Err(())
                }
                let smt = match tmp.first()/*max_by_key(|&x| x.get_h_value(&obj.0))*/ {
                    None => return Err(()),
                    Some(a) => a
                };
                match smt.needed_process(process, ressources) {
                    Err(_) => return Err(()),
                    Ok(None) => ret.push(smt.clone()),
                    Ok(Some(a)) => ret.extend(a)
                }
            }
             Ok(Some(ret))
        }

    }

    pub fn distance_overall (
        &self,
        owned: &Vec<Ressource>,
    ) -> usize {
        self.input.get_ressource()
                  .iter()
                  .fold(0usize, |acc, b| acc + Process::get_distance(b, owned))
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

impl std::fmt::Debug for Process {

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
