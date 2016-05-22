pub type Ressource = (String, u32);

pub fn check_ressource(need: &Vec<Ressource>, owned: &Vec<Ressource>) -> bool  {
    need.iter().all(
        |&(ref a, b)| owned.clone().into_iter().any(
            |(ref s, x)| *s == *a && x >= b))
}

fn update_ressource<F>(ori: &Vec<Ressource>, update: &Vec<Ressource>, closure: F)
                       -> Vec<Ressource>
    where F: Fn(u32, u32) -> u32 {
    ori.iter().map(
        |&(ref a, b)| {
            match update.clone().into_iter().find(|&(ref s, _)| *s == *a) {
                Some(tup) => (tup.0, closure(tup.1, b)),
                None => (a.clone(), b),
            }
        }
    ).collect::<Vec<Ressource>>()
}

pub fn add(ori: &Vec<Ressource>, update: &Vec<Ressource>) -> Vec<Ressource> {
    update_ressource(ori, update, |a, b| a + b)
}
pub fn sub(ori: &Vec<Ressource>, update: &Vec<Ressource>) -> Vec<Ressource> {
    update_ressource(ori, update, |a, b| a - b)
}
