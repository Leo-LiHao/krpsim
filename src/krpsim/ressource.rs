pub type Ressource = (String, u32);

pub fn check_ressource(need: &Vec<Ressource>, owned: &Vec<Ressource>) -> u32  {
    let ret = need.iter().map(
        |&(ref a, b)|
        match owned.into_iter().find(|&&(ref s, _)| *s == *a) {
            None => 0,
            Some(own) => own.1 as u32 / b as u32
        }
    ).collect::<Vec<_>>();
    *ret.iter().min().unwrap()
}

fn update_ressource<F>(ori: &mut Vec<Ressource>, update: &Vec<Ressource>, closure: F) -> ()
    where F: Fn(u32, u32) -> u32 {
    for item in update {
        if let Some(needle) = ori.iter_mut().find(|needle| needle.0 == item.0) {
            needle.1 = closure(needle.1, item.1);
        }
    }
}

pub fn add(ori: &mut Vec<Ressource>, update: &Vec<Ressource>, operations: u32) -> () {
    update_ressource(ori, update, |a, b| a + b * operations)
}
pub fn sub(ori: &mut Vec<Ressource>, update: &Vec<Ressource>, operations: u32) -> () {
    update_ressource(ori, update, |a, b| a - b * operations)
}
