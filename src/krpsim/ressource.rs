pub type Ressource = (String, i32);

fn check_ressource(need: Vec<Ressource>, owned: Vec<Ressource>) -> bool  {
    need.iter().all(
        |&(ref a, b)| owned.clone().into_iter().any(
            |(ref s, x)| *s == *a && x >= b))
}

fn update_ressource(ori: Vec<Ressource>, update: Vec<Ressource>) -> Vec<Ressource> {
    ori.iter().map(
        |&(ref a, b)| {
            match update.clone().into_iter().find(|&(ref s, _)| *s == *a) {
                Some(tup) => (tup.0, tup.1 + b),
                None => (a.clone(), b),
            }
        }
    ).collect::<Vec<Ressource>>()
}
