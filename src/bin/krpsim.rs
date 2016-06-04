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

use krpsim::format::stock::ressource::{Ressource, add, sub, check_ressource};
use krpsim::format::optimize::Optimize;
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

fn optimization(proc_list: &mut Vec<Process>, optimize: &String ) -> (){
    proc_list.sort_by(|a, b| b.get_h_value(optimize).partial_cmp(&a.get_h_value(&optimize)).unwrap())
}

fn get_optimized_product(opti: &Vec<String>, ressources: &mut Vec<Ressource>) -> Option<String> {
    for s in opti {
        if ressources.iter().any(|ref x| x.0 == *s) {
            return Some(s.clone())
        }
    }
    None
}

fn get_producing_process(optimized: &String, proc_list: &Vec<Process>) -> Vec<Process> {
    let mut ret = Vec::new();
    for procs in proc_list {
        if procs.get_h_value(optimized) > 0.0 {
            ret.push(procs.clone());
        }
    }
    ret
}

fn get_needed_ressources(input_list: &Vec<Ressource>, owned: &Vec<Ressource>) -> Vec<Ressource> {
    let mut ret = Vec::new();
    for resrc in input_list {
        let tmp = Process::get_distance(resrc, owned);
        if tmp > 0 {
            ret.push(Ressource::new(resrc.0.clone(), tmp as usize));
        }
    }
    ret
}

fn get_needed_process(input_list: Vec<Ressource>, proc_list: &Vec<Process>, ressource_list: &Vec<Ressource>)-> Vec<Process>{
    let mut ret = Vec::new();
    for ressource in input_list {
        let prc = get_producing_process(&ressource.0, proc_list);
        if prc.len() > 0 {
            let tmp = prc.iter().min_by_key(|x| x.distance_overall(ressource_list)).unwrap().clone();
            if tmp.distance_overall(ressource_list) > 0 {
                ret.extend(get_needed_process(get_needed_ressources(&tmp.input, ressource_list), proc_list, ressource_list));
            }
            else {
                ret.push(tmp);
            }
        }
    }
    ret
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    let delay: u64 = options.value_of("delay").unwrap_or(DEFAULT_DELAY).parse::<u64>().unwrap();
    let mut cycle: u64 = 0;

    let mut config = Configuration::new(options.value_of("file").unwrap()).unwrap();

    let mut done = false;
    let mut process_queue = Queue::new();
    get_ressources_from_process(&config.process_list, &mut config.ressources);
    let production: String = match get_optimized_product(&config.optimize.stock, &mut config.ressources) {
        Some(a) => a,
        None => panic!("You should optimize the production of at least one ressources!")
    };
    let mut final_process: Vec<Process> = get_producing_process(&production, &config.process_list);

    optimization(&mut config.process_list, &production);
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
                if cycle > delay {
                    println!("Finished at cycle: {}", cycle);
                    done = true;
                }
            }
        }
    }
}
