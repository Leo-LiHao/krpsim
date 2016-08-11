// @lecorref - github.com/lecorref, @geam - github.com/geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module `Running` describes a shop.

extern crate std;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use parser::trace::Trace;
use format::stock::inventory::Inventory;
use super::process::Process;

pub const ERR_NOT_FOUND: &'static str = "Item wasn't found `{}` from Running";
pub const ERR_WRONG_CYCLE: &'static str = "Invalid cycle {} from Running";
pub const ERR_EMPTY: &'static str = "There isn't any command `{}` from Running";

pub struct Running (std::collections::HashMap<String, Process>);

impl Running {

    /// The `new` constructor function returns the list of process.

    pub fn new (
        process: Vec<Process>,
    ) -> Self {
        let len: usize = process.len();

        Running(
          process.into_iter().fold_while(
            std::collections::HashMap::with_capacity(len),
            |mut map, task| {
              map.insert(task.get_name().to_string(), task);
              Continue(map)
          })
        )
    }

    /// The `is_empty` interface function returns true if
    /// the map contains not elements.

    pub fn is_empty (
        &self,
    ) -> bool {
        self.0.is_empty()
    }

    /// The `len` interface function returns the number of elements
    /// in the map.

    pub fn len (
        &self,
    ) -> usize {
        self.0.len()
    }

    /// The `iter` interface function returns a iterator.

    pub fn iter (
        &self,
    ) -> std::collections::hash_map::Iter<std::string::String, Process> {
        self.0.iter()
    }

    /// The `push` interface function inserts a new item to
    /// the inventory.

    pub fn push (
        &mut self,
        key: String,
        val: Process,
    ) -> Option<Process> {
        self.0.insert(key, val)
    }

    /// The `get_process` accesor function returns a process from map
    /// according to a key name.

    pub fn get (
        &self,
        key: &String,
    ) -> Option<&Process> {
        self.0.get(key)
    }

    /// The `buy_with` function buys one command with an inventary.

    pub fn buy_with (
        &self,
        commands: &Trace, // product
        inventory: &mut Inventory, // with
    ) -> std::io::Result<()> {
        commands.iter()
                .fold_while(from_error!(ERR_EMPTY, &format!("{}", commands)),
                           |_, &(ref command_name, _)| {
                  match self.get(&command_name) {
                    Some(process) => {
                      match process.buy_with(inventory) {
                        Ok(_) => Continue(Ok(())),
                        why => Done(why),
                      }
                    },
                    None => Done(from_error!(ERR_NOT_FOUND, command_name)),
                  }
                })
    }

    /// The `can_cycle` checks if the number and name of cycle is right
    /// between two process.

    pub fn can_cycle (
        &self,
        checks: &Trace,
    ) -> std::io::Result<usize> {
        checks.iter()
              .fold_while(Ok(0usize),
                         |acc, &(ref check_name, check_cycle)| {

                match (self.get(check_name), acc) {
                  (Some(ref process), Ok(cycle)) => {
                    if check_cycle <= cycle {
                      Continue(Ok(process.get_cycle() + cycle))
                    }
                    else {
                      Done(from_error!(ERR_WRONG_CYCLE, &format!("{}", process)))
                    }
                  },
                  _ => Done(from_error!(ERR_NOT_FOUND, check_name)),
                }
              })
    }

    /// The `get_process` function returns a accessor on
    /// the list of process.

    pub fn get_process(&self) -> Vec<&Process> {
        self.0.iter().map(|(&_, process)| process)
                     .collect::<Vec<&Process>>()
    }

    pub fn get_cloned_process(&self) -> Vec<Process> {
        self.0.iter().map(|(&_, process)| process.clone())
            .collect::<Vec<Process>>()
    }
}

impl std::fmt::Display for Running {

    /// The `fmt` function prints the multiplication list.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.iter().map(|a| format!("{}", a.1))
                                     .collect::<Vec<String>>()
                                     .join("\n"))
    }
}

impl std::default::Default for Running {

  /// The `default` constructor function returns a empty Running.

  fn default() -> Self {
    Running::new(Vec::new())
  }
}
