// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module `config` describes a configuration of industrial process.

extern crate std;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use format::stock::ressource::Ressource;
use format::stock::inventory::Inventory;
use format::operate::process::Process;
use format::operate::running::Running;
use format::optimize::Optimize;

use std::io::prelude::*;

pub const ERR_READ: &'static str = "Can't read `{}` from Config";
pub const ERR_DOUBLE: &'static str = "Double optimization `{}` from Config";
pub const ERR_SPLITN: &'static str = "Can't split `{}` from Config";

/// The `Configuration` struct contains the Ressource, Process and Optimize.

pub struct Configuration {
  pub ressources: Inventory, // Stock.
  pub running: Running, // Shop.
  pub optimize: Optimize,
}

impl Configuration {
  /// The `new` constructor function returns a parsed struct of
  /// Ressource, Process and Optimize.

  pub fn new(file: &str) -> std::io::Result<Self> {
    std::io::BufReader::new(
      try!(std::fs::File::open(file))
    ).lines().fold_while(Ok(Configuration::default()), |acc, readed| {
      if let (Ok(mut config), Ok(line)) = (acc, readed) {
        match &line.splitn(2, ':')
                   .collect::<Vec<&str>>()[..] {
          [comment, _..] if comment.starts_with('#') => Continue(Ok(config)),
          [name, thing] => {
            match &thing.splitn(2, "):")
                        .collect::<Vec<&str>>()[..] {
              [quantity] if quantity.parse::<usize>().is_ok() => {
                config.ressources.push(name.to_string(),
                  Ressource::new(
                    name.to_string(),
                    quantity.parse::<usize>()
                            .unwrap_or(0usize)
                  )
                );
                Continue(Ok(config))
              },
              [optimize] if optimize.starts_with('(') &&
                            optimize.ends_with(')') => {
                if config.optimize.is_empty() {
                  config.optimize = Optimize::from_line(optimize.to_string());
                  Continue(Ok(config))
                }
                else {
                  Done(from_error!(ERR_DOUBLE, &format!("{}", config.optimize)))
                }
              },
              [need, result_and_nb_cycle] if need.starts_with('(') => {
                match Process::from_line(
                  name.to_string(),
                  need,
                  result_and_nb_cycle
                ) {
                  Ok(run) => {
                    config.running.push(name.to_string(), run);
                    Continue(Ok(config))
                  },
                  Err(why) => Done(Err(why)),
                }
              },
              why => Done(from_error!(ERR_SPLITN, &format!("{:?}", why))),
            }
          },
          [why..] => Done(from_error!(ERR_SPLITN, &format!("{:?}", why))),
        }
      }
      else {
        Done(from_error!(ERR_READ, file))
      }
    })
  }
}

impl std::fmt::Display for Configuration {

  /// The `fmt` function prints the configuration.

  fn fmt (
    &self,
    f: &mut std::fmt::Formatter,
  ) -> Result<(), std::fmt::Error> {
    write!(f, "{}\n{}\n{}",
      self.ressources,
      self.running,
      self.optimize
    )
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
