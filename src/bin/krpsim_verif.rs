// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clap;
extern crate krpsim;

use krpsim::format::operate::process::Process;
use krpsim::format::stock::inventory::Inventory;
use krpsim::parser::config::Configuration;

use std::io::prelude::*;

fn trace (
  file: &str,
  result_to_test: &str,
) -> std::io::Result<usize> {
  let configuration = try!(Configuration::new(file));
  let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(
    try!(std::fs::File::open(result_to_test))
  );
    

 /*  for readed in reader.lines() {
      if let Ok(line) = readed {
        if let Ok(process) = Inventory(line) {
          result.process_list.push(process);
        }
      }
   }*/
  Ok(42usize)
}

fn main () {
  let yaml = load_yaml!("cli.yml");
  let options = clap::App::from_yaml(yaml).get_matches();

  trace(
    options.value_of("file").unwrap(),
    options.value_of("result_to_test").unwrap(),
  );
}
