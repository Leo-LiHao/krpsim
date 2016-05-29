// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use format::ressource::Ressource;
use format::process::Process;
use format::optimize::Optimize;

use std::io::prelude::*;

/// The `Configuration` struct contains the Ressource, Process and Optimize.

pub struct Configuration {
    pub ressources: Vec<Ressource>, // Inventory.
    pub process_list: Vec<Process>, // Shop.
    pub optimize: Vec<Optimize>,
}

impl Configuration {
    /// The `new` constructor function returns a parsed struct of
    /// Ressource, Process and Optimize.

    pub fn new(filename: &str) -> std::io::Result<Self> {
        let file: std::fs::File = try!(std::fs::File::open(filename));
        let reader: std::io::BufReader<std::fs::File> =
            std::io::BufReader::new(file);
        let mut result: Configuration = Configuration::default();

        for readed in reader.lines() {
            if let Ok(line) = readed {
                match &line.splitn(2, ':').collect::<Vec<&str>>()[..] {
                    [comment, ..] if comment.starts_with('#') => {}
                    [name, thing] => {
                        match &thing.splitn(2, "):").collect::<Vec<&str>>()[..] {
                            [optimize] if optimize.starts_with('(') &&
                                optimize.ends_with(')') => {
                                    result.optimize
                                        .push(Optimize::from_line(optimize.to_string()))
                                }
                            [quantity] if quantity.parse::<usize>().is_ok() => {
                                result.ressources
                                    .push(Ressource::new(name.to_string(),
                                    quantity.parse::<usize>()
                                    .unwrap_or(0)))
                            }
                            [need, result_and_nb_cycle] if need.starts_with('(') => {
                                result.process_list
                                    .push(try!(Configuration::parse_process(name,
                                                                            need,
                                                                            result_and_nb_cycle)))
                            }
                            why => {
                                try!(Err(from_error!("Configuration::new - splitn(2, \"):\")",
                                why)))
                            }
                        }
                    }
                    [why..] => {
                        try!(Err(from_error!("Configuration::new - splitn(2, ':')",
                        why)))
                    }
                }
            }
        }
        Ok(result)
    }

    /// The `parse_process` function returns a process.

    fn parse_process(name: &str,
                     need: &str,
                     result_and_nb_cycle: &str)
        -> std::io::Result<Process> {
            match &result_and_nb_cycle.rsplitn(2, ':').collect::<Vec<&str>>()[..] {
                [nb_cycle, result] if nb_cycle.parse::<u64>().is_ok() => {
                    Ok(Process::new(name.to_string(),
                    nb_cycle.parse::<u64>().unwrap(),
                    try!(Ressource::from_line(need)),
                    try!(Ressource::from_line(result))))
                }
               why => Err(from_error!("parse_proces", why)),
            }
        }

    


    pub fn buy_it (
        &mut self,
        it: String
        ) -> Option<&u64> {
        match self.process_list.iter()
                               .find(|&a|
                                  a.name == it
                                ) {
            Some(process) => process.buy_it(&mut self.ressources),
            None => None, // There isn't any resource available at sell.
        }
    }
}

impl std::default::Default for Configuration {
    /// The `default` constructor function returns a empty configuration.

    fn default() -> Self {
        Configuration {
            ressources: Vec::new(),
            process_list: Vec::new(),
            optimize: Vec::new(),
        }
    }
}
