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

use krpsim::input::verif::ProcessVerif;

fn main () {
  let yaml = load_yaml!("cli.yml");
  let options = clap::App::from_yaml(yaml).get_matches();

  let verif = ProcessVerif::new(
    options.value_of("file").unwrap(),
    options.value_of("result_to_test").unwrap(),
  ).unwrap();

  verif.buy();
}
