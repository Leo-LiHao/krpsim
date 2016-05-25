// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use super::ressource::Ressource;
use super::process::Process;
use std::cmp::Ordering;

pub struct Livep<'a> {
    process: &'a Process,
    pub cycle_end: u64
}

impl <'a> Eq for Livep <'a>{}

impl <'a> Ord for Livep <'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cycle_end.cmp(&(self.cycle_end))
    }
}

impl <'a> PartialEq for Livep <'a>{
    fn eq(&self, other: &Self) -> bool {
        self.cycle_end == other.cycle_end
    }
}

impl <'a> PartialOrd for Livep <'a>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl <'a> Livep <'a> {
    pub fn new(process: &'a Process, cycle: u64) -> Self {
        println!("process created: {} at cyle: {}",
                 process.name, cycle);
        Livep {
            process: process,
            cycle_end: cycle + process.cycle
        }
    }
    pub fn destruct(&self) -> &Vec<Ressource> {
        println!("process finished: {} at cyle: {}",
                 &self.process.name, &self.cycle_end);
        &self.process.output
    }

}
