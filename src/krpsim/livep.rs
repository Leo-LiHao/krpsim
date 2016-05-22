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
        Livep {
            process: process,
            cycle_end: cycle + process.cycle
        }
    }

}
