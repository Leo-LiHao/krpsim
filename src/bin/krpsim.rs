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
                    ressources.add(&Ressource::new(res.0.clone(), 0));
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

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    let delay: f64 = options.value_of("delay").unwrap_or(DEFAULT_DELAY).parse::<f64>().unwrap();
    let mut cycle: f64 = 0f64;

    let mut config = Configuration::new(options.value_of("file").unwrap()).unwrap();

    let mut done = false;
    let mut process_queue = Queue::new();
    get_ressources_from_process(&config.running.get_process(), &mut config.ressources);
    let production: Ressource = match get_optimized_product(&config.optimize.stock, &mut config.ressources) {
        Some(a) => a,
        None => panic!("You should optimize the production of at least one ressources!")
    };
   let mut final_process: Vec<Process> = Process::get_producing_process(&production, &config.running.get_process());

/*    optimization(&mut config.process_list, &production);*/
    while !done {
        let mut usable_process:Vec<(Process, Vec<Process>)> = Vec::new();

        for process in final_process.iter() {
            match process.needed_process(&config.running.get_process(), &config.ressources) {
                Err(_) => {},
                Ok(None) => usable_process.push((process.clone(), vec!(process.clone()))),
                Ok(Some(a)) => usable_process.push(( process.clone(), a ))
            }
        }

        if process_queue.is_empty() {
            println!("Finished at cycle: {}", cycle);
            done = true;
        }
    }
/*        if processes.len() > 0 {
            for process in processes {
                process_queue.add(process);
            }
        }
        match process_queue.get_ended_process(cycle) {
            None => cycle += 1,
            Some(livep_vec) => {
                for ended_process in livep_vec {
                    add(&mut config.ressources, ended_process.destruct(), 1f64);
                }
                if cycle > delay {
                    println!("Finished at cycle: {}", cycle);
                    done = true;
                }
            }
        }
    */
}
