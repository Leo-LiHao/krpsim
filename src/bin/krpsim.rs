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

const DEFAULT_DELAY: &'static str = "1";

use krpsim::format::ressource::{Ressource, add, sub, check_ressource};
use krpsim::format::process::Process;
use krpsim::format::queue::Queue;
use krpsim::format::livep::Livep;
use krpsim::input::config::Configuration;

fn get_available_process<'a>(process_list: &'a Vec<Process>, ressources: &mut Vec<Ressource>, cycle: u64)
                         -> Vec<Livep<'a>> {
    let mut vec = Vec::new();

    for process in process_list {
        let nb = check_ressource(&process.input, ressources);
        sub(ressources, &process.input, nb);
        for _ in 0..nb {
            vec.push(Livep::new(&process, cycle));
        }
    }
    vec
}

fn main() {
  let yaml = load_yaml!("cli.yml");
  let options = clap::App::from_yaml(yaml).get_matches();

  let delay: u64 = options.value_of("delay").unwrap_or(DEFAULT_DELAY).parse::<u64>().unwrap();
  let mut cycle: u64 = 0;

  let mut config = Configuration::new(options.value_of("file").unwrap()).unwrap();

  let mut done = false;
  let mut process_queue = Queue::new();
  while !done {
    let processes = get_available_process(&config.process_list,
                                          &mut config.ressources,
                                          cycle);
    if processes.len() > 0 {
      for process in processes {
        process_queue.add(process);
      }
    }
    if process_queue.is_empty() {
      println!("Finished at cycle: {}", cycle);
      done = true;
    }
    match process_queue.get_ended_process(cycle) {
      None => cycle += 1,
      Some(livep_vec) => {
        for ended_process in livep_vec {
          add(&mut config.ressources, ended_process.destruct(), 1);
        }
      }
    }
  }
}
