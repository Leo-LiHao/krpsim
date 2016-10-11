// @lecorref - github.com/lecorref, @geam - github.com/geam,
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

pub const ERR_WRONG_CYCLE: &'static str = "Invalid cycle {} from Process";
pub const ERR_PARSE: &'static str = "Can't parse `{}` from Process";
pub const ERR_BUY: &'static str = "Can't buy `{}` from Process";
pub const ERR_NEED: &'static str = "Inventory can't parse `{}`'s\
                                    from need Process";
pub const ERR_REST: &'static str = "Inventory can't parse `{}`'s\
                                    from rest Process";
pub const ERR_BOTH: &'static str = "Inventory can't parse `{}`'s\
                                    from both Process"; // need and rest

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
                -ressource.get_float_quantity()
            );
        });
        output.get_ressource().iter().foreach(|ressource| {
            *hash.entry(
                ressource.get_name().to_string()
            ).or_insert(
                ressource.get_float_quantity()
            ) += ressource.get_float_quantity();
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
            &[ref nb_cycle, ref result] => if nb_cycle.parse::<usize>().is_ok() {
                match (Inventory::from_line(need), Inventory::from_line(result)) {
                    (None, Some(_)) => from_error!(ERR_NEED, need),
                    (Some(_), None   ) => from_error!(ERR_REST, result),
                    (None, None) => from_error!(ERR_BOTH,
                        &format!("{}, {}", need, result)
                    ),
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
                from_error!(ERR_WRONG_CYCLE, &format!("{}", nb_cycle))
            },
            why => from_error!(ERR_PARSE, &format!("{:?}", why)),
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
    ) -> std::io::Result<()> {
        match self.input.order(with) { // Pay pay pay.
            Ok(_) => if with.add_from_inventory(&self.output) {
                Ok(())
            }
        else {
          from_error!(ERR_BUY, &format!("{}", self))
        }, // Take the list items.
        why => why,
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

    /*
    fn number_of_process(&self, obj: &Ressource) -> Vec<Process> {
        if let Some(a) = self.output.get_from_ressource(obj) {
            vec![self.clone();
                 {
                     match self.neutral {
                         Some(_) => 1,
                         None => obj.clone().euclidian_div(a.1)
                     }
                 }
            ]
        } else {
            vec![]
        }
    }
*/
    pub fn get_producing_process (
        obj: &Ressource,
        process: &Vec<&Process>,
        used_process: Vec<Process>
    ) -> Vec<Process> {
        let mut ret: Vec<Process> = Vec::new();

        process.iter().foreach(|procs| {
            if procs.get_h_value(&obj.0) > 0.0 {
                if !used_process.iter().any(|prc| {prc.name == (*procs).name}) {
                    ret.push((*procs).clone());
                }
            }
        });
        ret
    }

    fn get_ressource_number(vect: &Vec<Process>, obj: &Ressource) -> f64 {
        vect.iter().fold(0.0, |acc:f64, find| {
            acc + find.get_h_value(&obj.0)
        })
    }

    pub fn time_cmp(ori:&Result<(Vec<Process>, usize), ()>,
                new: (&Vec<Process>, usize),
                obj: &Ressource) -> bool() {
        match ori {
            &Err(_) => true,
            &Ok((ref a, t)) => {
                (Process::get_ressource_number(&a, obj) / t as f64) < (Process::get_ressource_number(&new.0, obj) / new.1 as f64)
            }
        }
    }

    pub fn needed_process (
        &self,
        process: &Vec<&Process>,
        ressources: &Inventory,
        real_ressources: &Inventory,
        already_used: Vec<Process>,
        delay : usize,
    ) -> Result<(Option<Vec<Process>>, usize), ()> {
        let mut input = self.input.clone();
        let mut real_input = self.input.clone();
        let mut time = self.cycle.clone();
        input.sub_from_inventory(ressources);
        real_input.sub_from_inventory(real_ressources);
        if input.is_zero() {
            if real_input.is_zero() {
                Ok((None, time))
            } else {
                Err(())
            }
        } else {
            let mut ret: Vec<Process> = Vec::new();
            for (_, obj) in input.iter() {

                if obj.1 > 0 {
                    let mut lst = Process::get_producing_process(obj, process, already_used.clone());
                    if lst.len() == 0 {
                        return Err(())
                    }
                    lst.sort_by(|a, b|
                                a.get_h_value(&obj.0).partial_cmp(&b.get_h_value(&obj.0)).unwrap());
                    let mut temp = already_used.clone();
                    match lst.iter().fold(Err(()), |acc:Result<(Vec<Process>, usize), ()>, smt|{
                        temp.push(smt.clone());
                        match smt.needed_process(
                            process, ressources, real_ressources, temp.clone(), delay) {
                            Err(_) => acc,
                            Ok((None, t)) => {
                         //       let vect = smt.number_of_process(&obj);
                         //       let total_time = vect.len() * t;
                                if Process::time_cmp(&acc, (&vec![smt.clone()], t), obj) {
                                    Ok((vec![smt.clone()], t))
                                } else {acc}
                            },

                            Ok((Some(a), t)) => {
                                if Process::time_cmp(&acc, (&a, t), obj) {
                                    Ok((a, t))
                                } else {acc}
                            }
                        }
                    }) {
                        Err(_) => return Err(()),
                        Ok((a, t)) => {
                            time += t;
                            if time > delay {
                                return Err(())
                            }
                            ret.extend(a);
                        }
                    }
                }
            }
            Ok((Some(ret), time))
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
