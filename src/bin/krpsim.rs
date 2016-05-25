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

const DEFAULT_DELAY: &'static str = "100";

use krpsim::format::ressource::{Ressource, add, sub, check_ressource};
use krpsim::format::optimize::Optimize;
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

fn get_ressources_from_process(process_list: &Vec<Process>, ressources: &mut Vec<Ressource>) -> () {
    for process in process_list {
        let mut add_ressource = |ressources_list: &Vec<Ressource>| -> () {
            for res in ressources_list {
                if ressources.iter().find(|tmp| tmp.0 == res.0).is_none() {
                    ressources.push(Ressource::new(res.0.clone(), 0));
                }
            }
        };
        add_ressource(&process.input);
        add_ressource(&process.output);
    }

}

fn optimization(proc_list: &mut Vec<Process>, optimize: &mut Optimize ) -> (){
    let s = optimize.stock.pop().unwrap();
    //println!("optimize: {}", s);
    proc_list.sort_by(|a, b| b.get_h_value(&s).partial_cmp(&a.get_h_value(&s)).unwrap())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    let delay: u64 = options.value_of("delay").unwrap_or(DEFAULT_DELAY).parse::<u64>().unwrap();
    let mut cycle: u64 = 0;

    let mut parser = Parser::new(options.value_of("file").unwrap()).unwrap();

    let mut done = false;
    let mut process_queue = Queue::new();
    get_ressources_from_process(&parser.process_list, &mut parser.ressources);
    optimization(&mut parser.process_list, &mut parser.optimize);
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
                    add(&mut parser.ressources, ended_process.destruct(), 1);
                }
            }
        }
    }
}
