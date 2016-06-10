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

#[test]
fn test_can_order() {
  assert!( // identical inventory
    Inventory::new(
      vec!(
        Ressource::new("heart".to_string(), 10),
        Ressource::new("arrows".to_string(), 20),
        Ressource::new("deku-seeds".to_string(), 30),
        Ressource::new("deku-nuts".to_string(), 15),
        Ressource::new("deku-stick".to_string(), 10),
        Ressource::new("deku-shield".to_string(), 40),
      )
    ).can_order(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 10),
          Ressource::new("arrows".to_string(), 20),
          Ressource::new("deku-seeds".to_string(), 30),
          Ressource::new("deku-nuts".to_string(), 15),
          Ressource::new("deku-stick".to_string(), 10),
          Ressource::new("deku-shield".to_string(), 40),
        ) // with
      )
    )
  );
  assert!( // more items
    Inventory::new(
      vec!(
        Ressource::new("heart".to_string(), 10),
      )
    ).can_order(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 10),
          Ressource::new("arrows".to_string(), 20),
          Ressource::new("deku-seeds".to_string(), 30),
          Ressource::new("deku-nuts".to_string(), 15),
          Ressource::new("deku-stick".to_string(), 10),
          Ressource::new("deku-shield".to_string(), 40),
        )
      ) // with
    )
  );
  assert!( // less items
    !Inventory::new(
      vec!(
        Ressource::new("heart".to_string(), 10), 
        Ressource::new("arrows".to_string(), 20),
        Ressource::new("deku-seeds".to_string(), 30),
        Ressource::new("deku-nuts".to_string(), 15),
        Ressource::new("deku-stick".to_string(), 10),
        Ressource::new("deku-shield".to_string(), 40),
      )
    ).can_order(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 10),
        )
      ) // with
    )
  );
  assert!( // more qte of items
    Inventory::new(
      vec!(
        Ressource::new("heart".to_string(), 10), 
        Ressource::new("arrows".to_string(), 20),
        Ressource::new("deku-seeds".to_string(), 30),
        Ressource::new("deku-nuts".to_string(), 15),
        Ressource::new("deku-stick".to_string(), 10),
        Ressource::new("deku-shield".to_string(), 40),

        )
    ).can_order(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 100),
          Ressource::new("arrows".to_string(), 200),
          Ressource::new("deku-seeds".to_string(), 300),
          Ressource::new("deku-nuts".to_string(), 150),
          Ressource::new("deku-stick".to_string(), 100),
          Ressource::new("deku-shield".to_string(), 400),
        )
      ) // with
    )
  );
  assert!( // less qte of items
    !Inventory::new(
      vec!(
        Ressource::new("heart".to_string(), 100), 
        Ressource::new("arrows".to_string(), 200),
        Ressource::new("deku-seeds".to_string(), 300),
        Ressource::new("deku-nuts".to_string(), 150),
        Ressource::new("deku-stick".to_string(), 100),
        Ressource::new("deku-shield".to_string(), 400),
      )
    ).can_order(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 10), 
          Ressource::new("arrows".to_string(), 20),
          Ressource::new("deku-seeds".to_string(), 30),
          Ressource::new("deku-nuts".to_string(), 15),
          Ressource::new("deku-stick".to_string(), 10),
          Ressource::new("deku-shield".to_string(), 40),
        )
      ) // with
    )
  );
}
