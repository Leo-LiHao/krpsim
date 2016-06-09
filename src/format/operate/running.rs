// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use super::process::Process;

pub struct Running(std::collections::HashMap<String, Process>);

impl Running {
    /// The `new` constructor function returns the list of process.

    pub fn new (
        process: Vec<Process>,
    ) -> Self {
        let mut map: std::collections::HashMap<String, Process> = std::collections::HashMap::with_capacity(process.len());

        process.into_iter().all(|task|
            map.insert(task.get_name().to_string(), task).is_some()
        );
        Running(map)
    }

    /// The `is_empty` interface function returns true if
    /// the map contains not elements.

    pub fn is_empty (
        &self,
    ) -> bool {
        self.0.is_empty()
    }


    /// The `iter` interface function returns a iterator.

    pub fn iter (
        &self,
    ) -> std::collections::hash_map::Iter<std::string::String, Process> {
       self.0.iter()
    }    

    /// The `push` interface function inserts a new item to 
    /// the inventory.

    pub fn push (
        &mut self,
        key: String,
        val: Process,
    ) -> Option<Process> {
        self.0.insert(key, val)
    }
}

impl std::fmt::Display for Running {

    /// The `fmt` function prints the multiplication list.

    fn fmt (
        &self,
        f: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.iter().map(|a| format!("{}", a.1))
                                     .collect::<Vec<String>>()
                                     .join("\n"))
    }
}

impl std::default::Default for Running {

  /// The `default` constructor function returns a empty Running.

  fn default() -> Self {
    Running::new(Vec::new())
  }
}
