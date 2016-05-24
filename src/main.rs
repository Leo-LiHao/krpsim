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

use krpsim::format::ressource::{Ressource, add, sub, check_ressource};
use krpsim::format::process::Process;
use krpsim::format::queue::Queue;
use krpsim::format::livep::Livep;
use krpsim::parser::Parser;

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

  let delay: u64 = options.value_of("delay").unwrap().parse::<u64>().unwrap();
  let mut cycle: u64 = 0;

  let mut parser = Parser::new(options.value_of("file").unwrap()).unwrap();

  let mut done = false;
  let mut process_queue = Queue::new();
  while !done {
    let processes = get_available_process(&parser.process_list,
                                          &mut parser.ressources,
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
          ended_process.destruct(&mut parser.ressources);
        }
      }
    }
  }
}
