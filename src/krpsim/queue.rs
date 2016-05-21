extern crate std;

use super::process::Process;

pub struct Queue<'a> {
    lst: std::collections::LinkedList<(u64, Vec<&'a Process>)>
}

impl <'a> Queue <'a> {
    pub fn new() -> Self {
        Queue {
            lst: std::collections::LinkedList::new()
        }
    }

    fn add(
        process: &Process,
        current_cycle: u64
    ) -> () {
        let cycle = current_cycle + process.cycle;
    }
}
