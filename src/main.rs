extern crate krpsim;

use krpsim::krpsim::ressource::{Ressource, add, sub, check_ressource};
use krpsim::krpsim::process::Process;
use krpsim::krpsim::queue::Queue;
use krpsim::krpsim::livep::Livep;


fn get_process() -> Vec<Process> {
    vec![
        Process::new("achat_materiel", 10, vec![("euro".to_string(), 8)], vec![("materiel".to_string(), 1)]),
        Process::new("realisation_produit", 30, vec![("materiel".to_string(), 1)], vec![("produit".to_string(), 1)]),
        Process::new("livraison", 20, vec![("produit".to_string(), 1)], vec![("client_content".to_string(), 1)])
    ]
}

fn get_ressources_from_process(process_list: &Vec<Process>, ressources: &mut std::collections::HashMap<String, usize>) -> () {
    for process in process_list {
        let mut add_ressource = |ressources_list: &Vec<Ressource>| -> () {
            for res in ressources_list {
                if ressources.get(&res.0).is_none() {
                    ressources.insert(res.0.clone(), 0);
                }
            }
        };
        add_ressource(&process.input);
        add_ressource(&process.output);
    }
}

fn get_available_process<'a>(process_list: &Vec<Process>, ressources: &mut Vec<Ressource>, cycle: u64)
                         -> Vec<Livep<'a>> {
    let mut vec = Vec::new();

    for process in process_list {
        if check_ressource(&process.input, ressources) {
            sub(ressources, &process.input);
            vec.push(Livep::new(&process, cycle));
        }
    }
    vec
}

fn main() {
    let process_list: Vec<Process> = get_process();
   /* let mut ressources: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    get_ressources(&mut ressources);

    get_ressources_from_process(&process_list, &mut ressources);
     */
    let mut ressources: Vec<Ressource> = vec![("euro".to_string(), 50),
                                            ("materiel".to_string(), 0),
                                            ("produit".to_string(), 0),
                                            ("client_content".to_string(), 0)];
    let mut cycle: u64 = 0;
    let mut done = false;
    let mut process_queue = Queue::new();
    while !done {
        let procs = get_available_process(&process_list, &mut ressources, cycle);
        println!("{:?}", ressources);
        cycle += 1;
        if cycle == 90 {
            done = true;
        }
    }
}
