// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use format::operate::process::Process;
use format::stock::inventory::Inventory;
use super::config::Configuration;

use std::io::prelude::*;

/// The `ProcessVerif` struct contains a krpsim's configuration
/// and trace who must be verified.

pub struct ProcessVerif {
    configuration: Configuration,
    process_list: Vec<Process>, // Trace.
}

impl ProcessVerif {

    /// The `new` constructor function returns a `ProcessVerif` interface.
    pub fn new (
        filename_configuration: &str,
        filename: &str,
    ) -> std::io::Result<Self> {
        let file: std::fs::File = try!(std::fs::File::open(filename));
        let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
        let mut result = ProcessVerif::default();
    
        result.configuration = try!(Configuration::new(filename_configuration));
        for readed in reader.lines() {
            if let Ok(line) = readed {
                match &line.splitn(2, ':').collect::<Vec<&str>>()[..] {
                    [cycle, name] if cycle.parse::<usize>().is_ok() => result.process_list.push(
                        Process::new(name.to_string(), cycle.parse::<usize>().unwrap(), Inventory::default(), Inventory::default())
                    ),
                    [_..] => continue ,
                }
            }
        }
        Ok(result)
    }

    pub fn buy (&self) -> std::io::Result<()> {
        /*let cycle = self.process_list.iter()
                                     .zip(self.configuration.process_list.iter())
                                     .fold(0, |cycle, (ref krpsim, ref verif)|
                                        match (krpsim.name == verif.name, verif.cycle == cycle) {
                                          (false, false) => panic!("!process: {}!={}, !cycle: {}!={}",
                                                                   krpsim.name, verif.name, verif.cycle, cycle
                                          ),
                                          (false,  true) => panic!("!process: {}!={}", krpsim.name, verif.name),
                                          ( true, false) => panic!("!cycle: {}!={}", verif.cycle, cycle),
                                          ( true,  true) => krpsim.cycle + cycle,
                                        }
                                     );*/
        Ok(())
    }
}

impl std::default::Default for ProcessVerif {

    /// The `default` constructor function returns a empty ProcessVerif.

    fn default() -> Self {
        ProcessVerif {
            configuration: Configuration::default(),
            process_list: Vec::new(),
        }
    }
}
