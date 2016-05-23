extern crate std;

use super::format::ressource::Ressource;
use std::io::prelude::*;

pub struct Parser {
  stock: Vec<String>,
}

impl Parser {
  pub fn new(filename: &str) -> std::io::Result<()> {
    let file: std::fs::File = try!(std::fs::File::open(filename));
    let reader: std::io::BufReader<std::fs::File> =
      std::io::BufReader::new(file);

    for readed in reader.lines() {
      if let Ok(line) = readed {
        match &line.splitn(2, ':').collect::<Vec<&str>>()[..] {
          [comment, ..] if comment.starts_with('#') => {},
          [name, thing] => {
            print!("{}: ", name);
            match &thing.split("):").collect::<Vec<&str>>()[..] {
              [optimize] if optimize.starts_with('(') &&
                            optimize.ends_with(')') => {
                println!("{{optimize: {}}}", optimize)
              }
              [quantity] if quantity.parse::<usize>()
                                    .is_ok() => {
                println!("{}", Ressource::new(name.to_string(), quantity.parse::<usize>().unwrap()))
              }
              [processus.., nb_cycle] if nb_cycle.parse::<usize>()
                                                 .is_ok() => {
                println!("{{nb_cycle: {}}}",
                         nb_cycle.parse::<usize>().unwrap());
                println!("{{processus: {:?}}}", processus)
              }
              _ => unimplemented!(),
            }
          }
          _ => unimplemented!(),
        }
      }
    }
    Ok(())
  }
}