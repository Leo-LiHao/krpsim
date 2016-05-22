extern crate krpsim;

use krpsim::krpsim::ressource::Ressource;
use krpsim::krpsim::process::Process;


fn getProcess() -> Vec<Process> {
    vec![
        Process::new("achat_materiel", 10, vec![("euro".to_string(), 8)], vec![("materiel".to_string(), 1)]),
        Process::new("realisation_produit", 30, vec![("materiel".to_string(), 1)], vec![("produit".to_string(), 1)]),
        Process::new("livraison", 20, vec![("produit".to_string(), 1)], vec![("client_content".to_string(), 1)])
    ]
}

fn getRessourcesFromProcess(process_list: &Vec<Process>, ressources: &mut std::collections::HashMap<String, usize>) -> () {
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

fn main() {
    let process_list: Vec<Process> = getProcess();
    let mut ressources: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    getRessourcesFromProcess(&process_list, &mut ressources);

}
