pub type Ressource = (String, u32);

pub fn check_ressource(need: &Vec<Ressource>, owned: &Vec<Ressource>) -> bool  {
    need.iter().all(
        |&(ref a, b)| owned.clone().into_iter().any(
            |(ref s, x)| *s == *a && x >= b))
}

fn update_ressource<F>(ori: &mut Vec<Ressource>, update: &Vec<Ressource>, closure: F) -> ()
    where F: Fn(u32, u32) -> u32 {
    for item in update {
        if let Some(needle) = ori.iter_mut().find(|needle| needle.0 == item.0) {
            needle.1 = closure(needle.1, item.1);
        }
    }
}

pub fn add(ori: &mut Vec<Ressource>, update: &Vec<Ressource>) -> () {
    update_ressource(ori, update, |a, b| a + b)
}
pub fn sub(ori: &mut Vec<Ressource>, update: &Vec<Ressource>) -> () {
    update_ressource(ori, update, |a, b| a - b)
}
