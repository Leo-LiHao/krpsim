// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::livep::Livep;

pub struct Queue  {
    lst: std::collections::BinaryHeap<Livep>
}

impl  Queue  {
    pub fn new() -> Self {
        Queue {
            lst: std::collections::BinaryHeap::new()
        }
    }

    pub fn add(
        &mut self,
        process: Livep,
    ) -> () {
        self.lst.push(process);
    }

    pub fn get_ended_process(
        &mut self,
        cycle: usize
    ) -> Option<Vec<Livep>> {
        if self.lst.peek().is_some() &&
           self.lst.peek().unwrap().cycle_end == cycle {
            let mut ret = Vec::new();
            ret.push(self.lst.pop().unwrap());
            while self.lst.peek().is_some() &&
                  self.lst.peek().unwrap().cycle_end == cycle {
                ret.push(self.lst.pop().unwrap());
            }
            Some(ret)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lst.is_empty()
    }
}
