extern crate krpsim;

use krpsim::format::ressource::{Ressource, add, sub, check_ressource};
use krpsim::format::process::Process;
use krpsim::format::queue::Queue;
use krpsim::format::livep::Livep;


fn get_process() -> Vec<Process> {
    vec![
        Process::new("achat_materiel", 10, vec![Ressource::new("euro".to_string(), 8)], vec![Ressource::new("materiel".to_string(), 1)]),
        Process::new("realisation_produit", 30, vec![Ressource::new("materiel".to_string(), 1)], vec![Ressource::new("produit".to_string(), 1)]),
        Process::new("livraison", 20, vec![Ressource::new("produit".to_string(), 1)], vec![Ressource::new("client_content".to_string(), 1)])
    ]
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
    let process_list: Vec<Process> = get_process();
    let mut ressources: Vec<Ressource> = vec![Ressource::new("euro".to_string(), 50),
                                              Ressource::new("client_content".to_string(), 0)];
    get_ressources_from_process(&process_list, &mut ressources);
    let mut cycle: u64 = 0;
    let mut done = false;
    let mut process_queue = Queue::new();
    while !done {
        let processes = get_available_process(&process_list, &mut ressources, cycle);
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
                   ended_process.destruct(&mut ressources);
                }
            }
        }
    }
}
