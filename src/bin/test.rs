extern crate krpsim;

use krpsim::format::stock::inventory::Inventory;
use krpsim::format::stock::ressource::Ressource;

fn main() {
  let mut inventory = Inventory::new(
    vec!(
      Ressource::new("heart".to_string(), 1),
    )
  );
  inventory.sub_from_inventory(
    &Inventory::new(
      vec!(
        Ressource::new("heart".to_string(), 5),
      ) // from
    )
  );
  println!("ss {}", inventory);
}
