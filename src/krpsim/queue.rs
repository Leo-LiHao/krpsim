extern crate std;

use super::livep::Livep;

pub struct Queue<'a> {
    lst: std::collections::BinaryHeap<&'a Livep<'a>>
}

impl <'a> Queue <'a> {
    pub fn new() -> Self {
        Queue {
            lst: std::collections::BinaryHeap::new()
        }
    }

    fn add(
        &mut self,
        process: &'a Livep<'a>,
    ) -> () {
        self.lst.push(process);
    }

    fn get_ended_process(
        &mut self,
        cycle: u64
    ) -> Option<Vec<&Livep>> {
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
}
