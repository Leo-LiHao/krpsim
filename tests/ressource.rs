// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate krpsim;

use self::krpsim::format::stock::ressource::Ressource;

#[test]
fn test_ressource_can_order() {
  assert_eq!(
    Ressource::new("apple".to_string(), 5).order(
      &mut Ressource::new("apple".to_string(), 5) // with
    ),
    0usize
  );
  assert_eq!(
    Ressource::new("apple".to_string(), 0).order(
      &mut Ressource::new("apple".to_string(), 0) // with
    ),
    0usize
  );
  assert_eq!(
    Ressource::new("apple".to_string(), 4).order(
      &mut Ressource::new("apple".to_string(), 5) // with
    ),
    1usize
  );
}
/*
#[test]
fn test_cannot_order() {
  assert!(
    Ressource::new("apple".to_string(), 5).order(
      &mut Ressource::new("apple".to_string(), 0) // with
    )
  );
}*/
