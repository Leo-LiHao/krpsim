type Ressource = (String, i32);


fn check_ressource(need: Ressource, owned: Vec<Ressource>) -> bool  {
    let (stri, num) = need;
    let ret = owned.into_iter().any(|(ref s, x)| *s == stri && x >= num);
}
