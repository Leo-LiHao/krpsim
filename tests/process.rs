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

#[test]
fn test_process_constructor_new() {
  assert_eq!(
    format!("{}",
      Process::from_integer(
        "knight".to_string(), // name
        10, // cycle
        Inventory::new(
          vec!(
            Ressource::new("silver-rupee".to_string(), 200),
          ) // need
        ),
        Inventory::new(
          vec!(
            Ressource::new("heart".to_string(), 10),
          ) // result
        ),
      )
    ),
    "knight:(silver-rupee:200):(heart:10):10"
  );
}
