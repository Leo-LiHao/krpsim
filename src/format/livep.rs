// @lecorref - github.com/lecorref, @geam - github.com/geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use super::stock::inventory::Inventory;
use super::operate::process::Process;
use std::cmp::Ordering;

pub struct Livep {
    process: Process,
    pub cycle_end: usize,
}

impl  Eq for Livep {}

impl  Ord for Livep  {
    fn cmp(&self, other: &Self) -> Ordering {
        match (other.cycle_end, self.cycle_end) {
            (a, b) if a < b => Ordering::Less,
            (a, b) if a > b => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl  PartialEq for Livep {
    fn eq(&self, other: &Self) -> bool {
        self.cycle_end == other.cycle_end
    }
}

impl  PartialOrd for Livep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl  Livep  {
    pub fn new(process: Process, cycle: usize) -> Self {
        println!("process created: {} at cyle: {}",
                 process.name, cycle);
        Livep {
            process: process.clone(),
            cycle_end: cycle + process.get_cycle()
        }
    }
    pub fn destruct(&self) -> &Inventory {
        println!("process finished: {} at cyle: {}",
                 &self.process.name, &self.cycle_end);
        &self.process.output
    }

}
