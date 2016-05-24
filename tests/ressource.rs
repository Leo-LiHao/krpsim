extern crate krpsim;

use self::krpsim::format::ressource::{Ressource, add, sub, check_ressource};
use self::krpsim::format::process::Process;

fn get_process() -> Vec<Process> {
    vec![
        Process::new("achat_materiel".to_string(), 10, vec![Ressource::new("euro".to_string(), 8)], vec![Ressource::new("materiel".to_string(), 1)]),
        Process::new("realisation_produit".to_string(), 30, vec![Ressource::new("materiel".to_string(), 1)], vec![Ressource::new("produit".to_string(), 1)]),
        Process::new("livraison".to_string(), 20, vec![Ressource::new("produit".to_string(), 1)], vec![Ressource::new("client_content".to_string(), 1)])
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

#[test]
fn ressource_lecorref () {
    let process_list: Vec<Process> = get_process();
    let mut ressources: Vec<Ressource> = vec![Ressource::new("euro".to_string(), 50),
                                              Ressource::new("client_content".to_string(), 0)];


    get_ressources_from_process(&process_list, &mut ressources);
}
