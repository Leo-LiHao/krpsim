// @lecorref - github.com/lecorref, @geam - github.com/geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clap;
extern crate itertools;
extern crate krpsim;

const DEFAULT_DELAY: &'static str = "100";

use krpsim::format::stock::ressource::Ressource;
use krpsim::format::stock::inventory::Inventory;
use krpsim::format::operate::process::Process;
use krpsim::format::queue::Queue;
use krpsim::format::livep::Livep;
use krpsim::parser::config::Configuration;

use itertools::Itertools;

fn get_ressources_from_process(process_list: &Vec<&Process>, ressources: &mut Inventory) -> () {
    process_list.iter().foreach(|process| {
        let mut add_ressource = |ressources_list: &Inventory| -> () {
            ressources_list.iter().foreach(|res| {
                if ressources.iter().find(|tmp| tmp.0 == res.0).is_none() {
                    ressources.push(res.0.clone(), Ressource::new(res.0.clone(), 0));
                }
            });
        };
        add_ressource(&process.input);
        add_ressource(&process.output);
    })
}

fn get_optimized_product(opti: &Vec<String>, ressources: &mut Inventory) -> Option<Ressource> {
    match opti.iter().find(|s| ressources.any(s)) {
        Some(s) => if let Some(x) = ressources.get(&s) {
            Some(x.clone())
        }
        else {
            None
        },
        None => None,
    }
}

fn get_best(prcs: &Vec<(Vec<Process>, usize)>, obj: &Ressource) -> Result<(Vec<Process>, usize), ()> {
    prcs.into_iter().fold(Err(()), |acc:Result<(Vec<Process>, usize), ()>, smt|{
        match acc {
            Err(()) => Ok(smt.clone()),
            acc => {
                if Process::time_cmp(&acc, (&smt.0, smt.1), obj) {
                    Ok(smt.clone())
                } else {acc}
            }
        }
    })
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    let delay: usize = options.value_of("delay").unwrap_or(DEFAULT_DELAY).parse::<usize>().unwrap();
    let verbose: bool = options.is_present("verbose");
    let mut config = Configuration::new(options.value_of("file").unwrap()).unwrap();

    let mut cycle: usize = 0usize;

    let mut done = false;
    let mut process_queue = Queue::new();
    let mut tmp_inventory = config.ressources.clone();
    get_ressources_from_process(&config.running.get_process(), &mut config.ressources);
    let mut production: Ressource = match get_optimized_product(&config.optimize.stock, &mut config.ressources) {
        Some(a) => a,
        None => panic!("You should optimize the production of at least one ressources!")
    };
    production.add_quantity(1);
    let final_process: Vec<Process> = Process::get_producing_process(&production,
                                                                     &config.running.get_process(),
                                                                     Vec::new());
    while !done {
        let mut usable_process:Vec<(Vec<Process>, usize)> = Vec::new();

        final_process.iter().foreach(|process| {
            match process.needed_process(
                &config.running.get_process(),
                &mut tmp_inventory.clone(),
                &mut config.ressources.clone(),
                final_process.clone(), delay) {
                Err(_) => {},
                Ok((None, t)) => usable_process.push((vec!(process.clone()), t)),
                Ok((Some(a), t)) => {
                    usable_process.push((a, t))
                }
            }
        });
        match get_best(&usable_process, &production) {
            Ok((ref a, _)) => {
                for process in a {
                    config.ressources.sub_from_inventory(&process.input);
                    tmp_inventory.sub_from_inventory(&process.input);
                    tmp_inventory.add_from_inventory(&process.output);
                    process_queue.add(Livep::new(process.clone(), cycle, verbose));
                    if verbose {
                        println!("# inventory: {}", config.ressources);
                    }
                }
                if a.len() == 0 {
                    println!("# already got the optimized ressource\n# {}", config.ressources);
                    done = true;
                }
            },
            Err(_) => {
                if process_queue.is_empty() {
                    println!("# no more process doable at time {}\n# {}", cycle, config.ressources);
                    done = true;
                }
                match process_queue.get_ended_process(cycle) {
                    None => cycle += 1,
                    Some(livep_vec) => {
                        livep_vec.iter().foreach(|ended_process| {
                            config.ressources.add_from_inventory(ended_process.destruct(verbose));
                            if verbose {
                                println!("# inventory: {}", config.ressources);
                            }
                        });
                        if cycle > delay {
                            println!("# last {}\n# {}", cycle, config.ressources);
                            done = true;
                        }
                    }
                }
            }
        }
    }
}
