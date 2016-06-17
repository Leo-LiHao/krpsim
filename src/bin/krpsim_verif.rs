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

use krpsim::parser::config::Configuration;
use krpsim::parser::trace::Trace;

fn parse (
  file: &str,
  result_to_test: &str,
) -> std::io::Result<(Configuration, Trace)> {
  Ok((
    try!(Configuration::new(file)),
    try!(Trace::new(result_to_test))
  ))
}

fn main () {
  let yaml = load_yaml!("cli.yml");
  let options = clap::App::from_yaml(yaml).get_matches();

  match parse(
    options.value_of("file").unwrap(),
    options.value_of("result_to_test").unwrap(),
  ) {
    Ok((ref mut config, ref trace)) => { // parse
      match (config.running.buy_with(trace, &mut config.ressources),
             config.running.can_cycle(trace)) {
        (Ok(()), Ok(cycle)) => { // config
          println!("Nice file ! {} processes, {} stocks, {} to optimize\n\
                    Evaluating .................. done.\n\
                    Main walk\n\
                    {}\n\
                    no more process doable at time {}\n\
                    Stock :\n\
                    {:?}",
            config.running.len(),
            config.ressources.len(),
            config.optimize.len(),
            trace,
            {cycle+1},
            config.ressources
          )
        },
        (Err(why), _) | (_, Err(why)) => panic!(why),
      }
    },
    Err(why) => panic!(why),
  }
}
