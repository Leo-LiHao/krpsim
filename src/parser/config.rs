// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module `config` describes a configuration of industrial process.

extern crate std;

use format::stock::ressource::Ressource;
use format::stock::inventory::Inventory;
use format::operate::process::Process;
use format::operate::running::Running;
use format::optimize::Optimize;

use std::io::prelude::*;

/// The `Configuration` struct contains the Ressource, Process and Optimize.

pub struct Configuration {
  pub ressources: Inventory, // Stock.
  pub running: Running, // Shop.
  pub optimize: Optimize,
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
          [comment, _..] if comment.starts_with('#') => {}
          [name, thing] => {
            match &thing.splitn(2, "):").collect::<Vec<&str>>()[..] {
              [optimize] if optimize.starts_with('(') &&
                            optimize.ends_with(')') => {
                result.optimize = Optimize::from_line(optimize.to_string())
              }
              [quantity] if quantity.parse::<usize>().is_ok() => {
                result.ressources
                       .push(name.to_string(),
                             Ressource::new(name.to_string(),
                                            quantity.parse::<usize>().unwrap_or(0usize)));
              }
              [need, result_and_nb_cycle] if need.starts_with('(') => {
                 result.running
                       .push(name.to_string(),
                             try!(Process::from_line(name.to_string(),
                                                     need,
                                                     result_and_nb_cycle)));
              }
              why => {
                try!(Err(from_error!("Configuration::new - splitn(2, \"):\")",
                                     why)))
              }
            }
          }
          [why..] => {
            try!(Err(from_error!("Configuration::new - splitn(2, ':')", why)))
          }
        }
      }
    }
    Ok(result)
  }

  /// The `can_order` function check if the configuration can support
  /// the command,

  pub fn can_order (
    &self,
    must_command: Vec<String>,
  ) {
    let mut config = self.clone();

  }
}

impl std::default::Default for Configuration {
  /// The `default` constructor function returns a empty configuration.

  fn default() -> Self {
    Configuration {
      ressources: Inventory::default(),
      running: Running::default(),
      optimize: Optimize::default(),
    }
  }
}
