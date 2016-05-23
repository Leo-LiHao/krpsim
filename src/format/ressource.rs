extern crate std;

pub struct Ressource(pub String, usize);

impl Ressource {

  /// The `new` constructor function returns the Stock.

  pub fn new(stock_name: String, quantity: usize) -> Self {
    Ressource(stock_name, quantity)
  }
}

impl std::fmt::Display for Ressource {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(ressource: {}, {})", self.0, self.1)
  }
}


fn update_ressource<F>(ori: &mut Vec<Ressource>,
                       update: &Vec<Ressource>,
                       closure: F)
                       -> ()
  where F: Fn(usize, usize) -> usize
{
  for item in update {
    if let Some(needle) = ori.iter_mut().find(|needle| needle.0 == item.0) {
      needle.1 = closure(needle.1, item.1);
    }
  }
}

pub fn add(ori: &mut Vec<Ressource>, update: &Vec<Ressource>, operations: usize) -> () {
  update_ressource(ori, update, |a, b| a + b * operations)
}
pub fn sub(ori: &mut Vec<Ressource>, update: &Vec<Ressource>, operations: usize) -> () {
  update_ressource(ori, update, |a, b| a - b * operations)
}

pub fn check_ressource(need: &Vec<Ressource>, owned: &Vec<Ressource>) -> usize {
  let ret = need.into_iter()
                .map(|&ref n| {
                  match owned.into_iter().find(|&s| *s.0 == *n.0) {
                    None => 0,
                    Some(own) => own.1 as usize / n.1 as usize,
                  }
                })
                .collect::<Vec<_>>();
  *ret.iter().min().unwrap()
}
