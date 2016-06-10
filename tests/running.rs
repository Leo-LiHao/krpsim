// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate krpsim;

use self::krpsim::format::stock::inventory::Inventory;
use self::krpsim::format::stock::ressource::Ressource;
use self::krpsim::format::operate::process::Process;
use self::krpsim::format::operate::running::Running;

#[test]
fn test_can_cycle() {
/*  assert_eq!( // identical inventory
    Running::new(
      vec!(
        Process::new(
          "knight".to_string(), // name
          10usize, // cycle
          Inventory::new(
            vec!(
              Ressource::new("green-rupee".to_string(), 1),
              Ressource::new("blue-rupee".to_string(), 5),
              Ressource::new("red-rupee".to_string(), 20),
              Ressource::new("purple-rupee".to_string(), 50),
              Ressource::new("orange-rupee".to_string(), 100),
              Ressource::new("silver-rupee".to_string(), 200),
              Ressource::new("gold-rupee".to_string(), 300),
            ) // need
          ),
          Inventory::new(
            vec!(
              Ressource::new("heart".to_string(), 10),
              Ressource::new("sword".to_string(), 40),
            ) // result
          ),
        )
      )
    ).can_cycle(
      &Running::new(
        vec!(
          Process::new(
            "knight".to_string(),
            0usize,
            Inventory::default(),
            Inventory::default(),
          )
        )
      )
    ),
    Some(10)
  );*/
}
