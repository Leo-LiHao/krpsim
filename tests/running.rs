// @lecorref - github.com/lecorref, @geam - github.com/geam,
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
use self::krpsim::parser::trace::Trace;

#[test]
fn test_can_cycle() {
  assert_eq!( // identical inventory
    Running::new(
      vec!(
        Process::from_integer(
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
        ),
        Process::from_integer(
          "guard".to_string(), // name
          12usize, // cycle
          Inventory::new(
            vec!(
              Ressource::new("blue-rupee".to_string(), 5),
            ) // need
          ),
          Inventory::new(
            vec!(
              Ressource::new("sword".to_string(), 1),
            ) // result
          ),
        ),
      )
    ).can_cycle(
      &Trace::from_vec(vec!(
        (
          "knight".to_string(),
          0usize,
        ),
      )) // will cycle
    ).ok(),
    Some(10usize)
  );
  assert_eq!( // identical inventory
    Running::new(
      vec!(
        Process::from_integer(
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
        ),
        Process::from_integer(
          "guard".to_string(), // name
          12usize, // cycle
          Inventory::new(
            vec!(
              Ressource::new("blue-rupee".to_string(), 5),
            ) // need
          ),
          Inventory::new(
            vec!(
              Ressource::new("sword".to_string(), 1),
            ) // result
          ),
        ),
      )
    ).can_cycle(
      &Trace::from_vec(vec!(
        (
          "knight".to_string(),
          0usize,
        ),
        (
          "knight".to_string(),
          10usize,
        ),
      )) // will cycle
    ).ok(),
    Some(20usize)
  );
}

#[test]
fn test_order() {
    let running = Running::new(
      vec!(
        Process::from_integer(
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
        ),
        Process::from_integer(
          "guard".to_string(), // name
          12usize, // cycle
          Inventory::new(
            vec!(
              Ressource::new("blue-rupee".to_string(), 5),
            ) // need
          ),
          Inventory::new(
            vec!(
              Ressource::new("sword".to_string(), 1),
            ) // result
          ),
        ),
      )
    );
    let mut inventory = Inventory::new(
      vec!(
        Ressource::new("green-rupee".to_string(), 1),
        Ressource::new("blue-rupee".to_string(), 5),
        Ressource::new("red-rupee".to_string(), 20),
        Ressource::new("purple-rupee".to_string(), 50),
        Ressource::new("orange-rupee".to_string(), 100),
        Ressource::new("silver-rupee".to_string(), 200),
        Ressource::new("gold-rupee".to_string(), 300),
      ) // have
    );
   
    assert!(
      running.buy_with(
        &Trace::from_vec(
          vec!(
            ("knight".to_string(), 0usize),
          )
        ),
        &mut inventory
      ).is_ok()
    );
    assert_eq!(
      format!("{}", inventory),
      "(blue-rupee:0;gold-rupee:0;green-rupee:0;heart:10;orange-rupee:0;purple-rupee:0;red-rupee:0;silver-rupee:0;sword:40)"
    );
}
