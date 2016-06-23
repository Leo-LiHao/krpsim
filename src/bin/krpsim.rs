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

use krpsim::format::stock::ressource::Ressource;
use krpsim::format::stock::inventory::Inventory;
use krpsim::format::optimize::Optimize;
use krpsim::format::operate::process::Process;
use krpsim::format::queue::Queue;
use krpsim::format::livep::Livep;
use krpsim::parser::config::Configuration;


fn get_ressources_from_process(process_list: &Vec<&Process>, ressources: &mut Inventory) -> () {
    for process in process_list {
        let mut add_ressource = |ressources_list: &Inventory| -> () {
            for res in ressources_list.iter() {
                if ressources.iter().find(|tmp| tmp.0 == res.0).is_none() {
                    ressources.push(res.0.clone(), Ressource::new(res.0.clone(), 0));
                }
            }
        };
        add_ressource(&process.input);
        add_ressource(&process.output);
    }
}

fn get_optimized_product(opti: &Vec<String>, ressources: &mut Inventory) -> Option<Ressource> {
    for s in opti {
        if let Some(x) = ressources.get(&s) {
            return Some(x.clone())
        }
    }
    None
}

fn get_best(prcs: &Vec<(Process, Vec<Process>)>) -> Option<Vec<Process>> {
    match prcs.first() {
        Some(&(_, ref b)) => Some(b.clone()),
        None => None
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    let delay: usize = options.value_of("delay").unwrap_or(DEFAULT_DELAY).parse::<usize>().unwrap();
    let mut cycle: usize = 0usize;

    let mut config = Configuration::new(options.value_of("file").unwrap()).unwrap();

    let mut done = false;
    let mut process_queue = Queue::new();
    get_ressources_from_process(&config.running.get_process(), &mut config.ressources);
    let mut production: Ressource = match get_optimized_product(&config.optimize.stock, &mut config.ressources) {
        Some(a) => a,
        None => panic!("You should optimize the production of at least one ressources!")
    };
    production.add_quantity(1);
    //println!("{}", production);
    let final_process: Vec<Process> = Process::get_producing_process(&production,
                                                                     &config.running.get_process(),
                                                                     Vec::new());
    /*    optimization(&mut config.process_list, &production);*/
    while !done {
        let mut usable_process:Vec<(Process, Vec<Process>)> = Vec::new();

        for process in final_process.iter() {
            match process.needed_process(
                &config.running.get_process(), &config.ressources,
                final_process.clone()) {
                Err(_) => {},
                Ok((None, _)) => usable_process.push((process.clone(), vec!(process.clone()))),
                Ok((Some(a), _)) => usable_process.push(( process.clone(), a ))
            }
        }
        match get_best(&usable_process) {
            Some(a) => {
                for process in a {
                    config.ressources.sub_from_inventory(&process.input);
                    process_queue.add(Livep::new(process.clone(), cycle));
                    println!("inventory: {}", config.ressources);
                }
            },
            None => {
                if process_queue.is_empty() {
                    println!("Finished at cycle: {}", cycle);
                    done = true;
                }
                match process_queue.get_ended_process(cycle) {
                    None => cycle += 1,
                    Some(livep_vec) => {
                        for ended_process in livep_vec {
                            config.ressources.add_from_inventory(ended_process.destruct());
                            println!("inventory: {}", config.ressources);
                        }
                        if cycle > delay {
                            println!("Finished at cycle: {}", cycle);
                            done = true;
                        }
                    }
                }
            }
        }
    }
}
