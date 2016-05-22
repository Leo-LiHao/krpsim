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
        process: &Livep<'a>,
    ) -> () {
        self.lst.push(process);
    }

    fn getEndedProcess(
        &mut self,
        cycle: u64
    ) -> Option<Vec<&Livep>> {
        if let Some(v) = self.lst.peek() {
            if v.cycle_end == cycle {
                let mut ret = Vec::new();
                ret.push(self.lst.pop().unwrap());
                while let Some(tmp) = self.lst.peek() {
                    if tmp.cycle_end == cycle {
                        ret.push(self.lst.pop().unwrap());
                    } else {
                        break;
                    }
                }
                Some(ret)
            } else {
                None
            }
        } else {
            None
        }
    }
}
