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
fn test_inventory_can_order() {
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
    ).is_ok()
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
    ).is_ok()
  );
  assert!( // less items
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
        )
      ) // with
    ).is_err()
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
    ).is_ok()
  );
  assert!( // less qte of items
    Inventory::new(
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
    ).is_err()
  );
}

#[test]
fn test_inventory_fmt() {
  assert_eq!(
    format!("{}",
      Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 10),
          Ressource::new("arrows".to_string(), 20),
          Ressource::new("deku-seeds".to_string(), 30),
          Ressource::new("deku-nuts".to_string(), 15),
          Ressource::new("deku-stick".to_string(), 10),
          Ressource::new("deku-shield".to_string(), 40),
        )
      )
    ),
    "(arrows:20;deku-nuts:15;deku-seeds:30;deku-shield:40;deku-stick:10;heart:10)"
  )
}

#[test]
fn test_inventory_sub_from_inventory() {
  let mut inventory = Inventory::new(
    vec!(
      Ressource::new("heart".to_string(), 10),
    )
  );

  assert!(
    !inventory.sub_from_inventory(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 5),
          Ressource::new("arrows".to_string(), 20),
        ) // from
      )
    )
  );
  assert_eq!(
    format!("{}", inventory),
    "(heart:5)"
  );
 let mut inventory = Inventory::new(
    vec!(
      Ressource::new("heart".to_string(), 8),
    )
  );

  assert!(
    inventory.sub_from_inventory(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 5),
        ) // from
      )
    )
  );
  assert_eq!(
    format!("{}", inventory),
    "(heart:3)"
  );
}

#[test]
fn test_inventory_frale_co() {
  let mut inventory = Inventory::new(
    vec!(
      Ressource::new("heart".to_string(), 1),
    )
  );

  assert!(
    inventory.sub_from_inventory(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 5),
        ) // from
      )
    )
  );
  assert_eq!(
    format!("{}", inventory),
    "(heart:0)"
  );
}

#[test]
fn test_inventory_add_from_inventory() {
  let mut inventory = Inventory::new(
    vec!(
      Ressource::new("heart".to_string(), 10),
    )
  );

  assert!(
    inventory.add_from_inventory(
      &Inventory::new(
        vec!(
          Ressource::new("heart".to_string(), 10),
          Ressource::new("arrows".to_string(), 20),
        ) // from
      )
    )
  );
  assert_eq!(
    format!("{}", inventory),
    "(arrows:20;heart:20)"
  );
}

#[test]
fn test_inventory_equal() {
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
    ) == Inventory::new(
      vec!(
        Ressource::new("heart".to_string(), 10),
        Ressource::new("arrows".to_string(), 20),
        Ressource::new("deku-seeds".to_string(), 30),
        Ressource::new("deku-nuts".to_string(), 15),
        Ressource::new("deku-stick".to_string(), 10),
        Ressource::new("deku-shield".to_string(), 40),
      ) // with
    )
  );
}
