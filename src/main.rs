extern crate krpsim;

use krpsim::krpsim::ressource::Ressource;
use krpsim::krpsim::process::Process;


fn get_process() -> Vec<Process> {
    vec![
        Process::new("achat_materiel", 10, vec![("euro".to_string(), 8)], vec![("materiel".to_string(), 1)]),
        Process::new("realisation_produit", 30, vec![("materiel".to_string(), 1)], vec![("produit".to_string(), 1)]),
        Process::new("livraison", 20, vec![("produit".to_string(), 1)], vec![("client_content".to_string(), 1)])
    ]
}

fn get_ressources_from_process(process_list: &Vec<Process>, ressources: &mut Vec<Ressource>) -> () {
    for process in process_list {
        let mut add_ressource = |ressources_list: &Vec<Ressource>| -> () {
            for res in ressources_list {
                if ressources.iter().find(|tmp| tmp.0 == res.0).is_none() {
                    ressources.push((res.0.clone(), 0));
                }
            }
        };
        add_ressource(&process.input);
        add_ressource(&process.output);
    }
}

fn main() {
    
    let process_list: Vec<Process> = get_process();
    let mut ressources: Vec<Ressource> = Vec::new();

    get_ressources_from_process(&process_list, &mut ressources);

}
