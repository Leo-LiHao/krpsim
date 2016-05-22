extern crate std;

use super::livep::Livep;

pub struct Queue<'a> {
    lst: std::collections::BinaryHeap<Livep>
}

impl <'a> Queue <'a> {
    pub fn new() -> Self {
        Queue {
            lst: Vec::new()
        }
    }

    fn add(
        &self,
        process: &Livep,
    ) -> () {
        self.lst.push(process);
    }

    fn getEndedProcess(
        &self,
        cycle: u64
    ) -> Option<&Vec<Livep>> {
        if let Some(v) = self.lst.peek() && v.cycle == cycle {
            let mut ret = Vec::new();
            ret.push(self.pop().unwrap());
            while let Some(tmp) = self.lst.peek() && tmp.cycle == cycle {
                ret.push(self.pop().unwrap());
            }
            Some(ret)
        } else {
            None
        }
    }
}
